//! Persisted output records for archive-safe storage with rkyv zero-copy deserialization.
//!
//! Defines append-only persisted record types that mirror the runtime domain types across
//! all five pipeline phases. These records derive `rkyv::Archive`, `rkyv::Serialize`, and
//! `rkyv::Deserialize` for zero-copy deserialization from mmapped files.
//!
//! # Key Conversions
//!
//! - `Arc<str>` → `String`
//! - `HashMap<K, V>` → sorted `Vec<(K, V)>` (deterministic serialization)
//! - `DateTime<Utc>` → `i64` (unix epoch seconds, lossy: sub-second precision dropped)
//!
//! # Design Principles
//!
//! - Infallible `*_to_persisted` conversions (runtime data is already validated)
//! - Fallible `persisted_*_to_runtime` conversions (validates field constraints)
//! - All top-level batch records carry `schema_version: u32` (currently 1)
//! - Zero mutation in conversion functions; pure transforms only

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]

use crate::analyze::{Analysis, AnalyzeResult, FailedFile, Heading, Link, LinkKind};
use crate::assign::IdMapping;
use crate::chunking_adapter::{Chunk, ChunksResult};
use crate::scrape::validation::{Header, PageFilterStatus, ScrapeResult, ScrapedPage};
use crate::transform::{TransformError, TransformResult};
use crate::watch::{ChangeKind, ChangePlan, ChangeSummary, PageChange, PageHash, Snapshot};
use chrono::{TimeZone, Utc};
use contextual_chunker::{ChunkLevel, ChunkType};
use itertools::Itertools;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Error Type
// ---------------------------------------------------------------------------

/// Errors that can occur during persisted ↔ runtime type conversions.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PersistError {
    /// A required String field was empty or whitespace-only.
    #[error("field '{field}' must be non-empty")]
    EmptyField {
        /// Name of the empty field.
        field: String,
    },

    /// A numeric field was outside its valid range.
    #[error("field '{field}' value {value} is out of range {min}..={max}")]
    OutOfRange {
        /// Name of the out-of-range field.
        field: String,
        /// The actual value observed.
        value: i64,
        /// Minimum allowed value (inclusive).
        min: i64,
        /// Maximum allowed value (inclusive).
        max: i64,
    },

    /// The persisted record's schema version does not match expected.
    #[error("schema version mismatch: expected {expected}, got {actual}")]
    SchemaVersionMismatch {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version found in the record.
        actual: u32,
    },

    /// rkyv serialization failed (buffer allocation or write error).
    #[error("serialization failed: {reason}")]
    SerializationFailed {
        /// Description of the failure.
        reason: String,
    },

    /// rkyv deserialization / validation failed (corrupted bytes).
    #[error("deserialization failed: {reason}")]
    DeserializationFailed {
        /// Description of the failure.
        reason: String,
    },

    /// An enum variant that doesn't map to any known runtime value.
    #[error("unknown enum variant for '{type_name}'")]
    UnknownVariant {
        /// The type name whose variant was unknown.
        type_name: String,
    },

    /// A float field was `NaN` or Infinite where finite was expected.
    #[error("field '{field}' must be a finite number, got {value}")]
    NonFiniteFloat {
        /// Name of the non-finite field.
        field: String,
        /// String representation of the invalid value.
        value: String,
    },

    /// A content hash was not exactly 32 bytes.
    #[error("content hash must be exactly 32 bytes, got {actual_len}")]
    InvalidHashLength {
        /// The actual byte length observed.
        actual_len: usize,
    },
}

// ---------------------------------------------------------------------------
// Helper Validation Functions (pure)
// ---------------------------------------------------------------------------

fn require_non_empty(value: &str, field: &str) -> Result<(), PersistError> {
    if value.trim().is_empty() {
        return Err(PersistError::EmptyField {
            field: field.to_string(),
        });
    }
    Ok(())
}

fn require_range(value: i64, min: i64, max: i64, field: &str) -> Result<(), PersistError> {
    if !(min..=max).contains(&value) {
        return Err(PersistError::OutOfRange {
            field: field.to_string(),
            value,
            min,
            max,
        });
    }
    Ok(())
}

fn require_schema_v1(version: u32) -> Result<(), PersistError> {
    if version != 1 {
        return Err(PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: version,
        });
    }
    Ok(())
}

fn require_finite_f32(value: f32, field: &str) -> Result<(), PersistError> {
    if !value.is_finite() {
        return Err(PersistError::NonFiniteFloat {
            field: field.to_string(),
            value: format!("{value}"),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Analysis Family
// ---------------------------------------------------------------------------

/// Persisted heading: level (1-6), text, line number.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedHeading {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text (non-empty after trim).
    pub text: String,
    /// 0-based line number in source.
    pub line: usize,
}

/// Persisted link kind: Internal or External.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedLinkKind {
    /// Internal link (within the documentation site).
    Internal,
    /// External link (outside the documentation site).
    External,
}

/// Persisted link: text, target, kind.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedLink {
    /// Link display text.
    pub text: String,
    /// Link target URL (non-empty).
    pub target: String,
    /// Whether the link is internal or external.
    pub kind: PersistedLinkKind,
}

/// Persisted analysis: full per-file metadata extraction.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedAnalysis {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Source file path (non-empty).
    pub source_path: String,
    /// Document title (non-empty).
    pub title: String,
    /// Frontmatter key-value pairs, sorted by key for deterministic serialization.
    pub frontmatter: Option<Vec<(String, String)>>,
    /// Extracted headings.
    pub headings: Vec<PersistedHeading>,
    /// Extracted links.
    pub links: Vec<PersistedLink>,
    /// First paragraph text.
    pub first_paragraph: String,
    /// Total word count.
    pub word_count: usize,
    /// Whether the document contains code blocks.
    pub has_code: bool,
    /// Whether the document contains tables.
    pub has_tables: bool,
    /// Auto-categorized category (non-empty, lowercase).
    pub category: String,
    /// Full cleaned content.
    pub content: String,
}

/// Persisted failed file: `source_path` + error message.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedFailedFile {
    /// Path of the file that failed analysis.
    pub source_path: String,
    /// Error message from the failed analysis.
    pub error: String,
}

/// Persisted batch analysis result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedAnalyzeResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Successful analyses.
    pub analyses: Vec<PersistedAnalysis>,
    /// Files that failed analysis.
    pub failed_files: Vec<PersistedFailedFile>,
    /// Total files discovered (including skipped/failed).
    pub total_discovered: usize,
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Transform Family
// ---------------------------------------------------------------------------

/// Persisted transform error: `source_path` + error message.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformError {
    /// Path of the file that failed transform.
    pub source_path: String,
    /// Error message from the failed transform.
    pub error: String,
}

/// Persisted batch transform result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Number of successfully transformed documents.
    pub success_count: usize,
    /// Total documents attempted.
    pub total_count: usize,
    /// Number of failed transforms.
    pub error_count: usize,
    /// Detailed errors.
    pub errors: Vec<PersistedTransformError>,
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Chunk Family
// ---------------------------------------------------------------------------

/// Persisted chunk type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkType {
    /// Code-dominated chunk.
    Code,
    /// Table-containing chunk.
    Table,
    /// General prose chunk.
    Prose,
}

/// Persisted chunk level hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkLevel {
    /// High-level overview (~128 tokens).
    Summary,
    /// Balanced detail (~512 tokens).
    Standard,
    /// Full context (~1024 tokens).
    Detailed,
}

/// Persisted extended chunk with knowledge graph relationships.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunk {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Chunk identifier (format: "{`doc_id`}#{`index`}").
    pub chunk_id: String,
    /// Parent document identifier.
    pub doc_id: String,
    /// Parent document title.
    pub doc_title: String,
    /// Index of this chunk within the document.
    pub chunk_index: usize,
    /// Chunk text content (non-empty).
    pub content: String,
    /// Estimated token count (> 0).
    pub token_count: usize,
    /// Optional heading this chunk falls under.
    pub heading: Option<String>,
    /// Full heading path from root.
    pub heading_path: Vec<String>,
    /// Content type classification.
    pub chunk_type: PersistedChunkType,
    /// Previous chunk in document sequence.
    pub previous_chunk_id: Option<String>,
    /// Next chunk in document sequence.
    pub next_chunk_id: Option<String>,
    /// Related chunks via knowledge graph.
    pub related_chunk_ids: Vec<String>,
    /// Extractive summary of chunk content.
    pub summary: String,
    /// Hierarchical level.
    pub chunk_level: PersistedChunkLevel,
    /// Parent chunk in hierarchy (Summary → Standard).
    pub parent_chunk_id: Option<String>,
    /// Child chunks in hierarchy (Standard → Detailed).
    pub child_chunk_ids: Vec<String>,
    /// Context preserved from previous chunk.
    pub context_prefix: Option<String>,
}

/// Persisted batch chunking result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunksResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Total chunks created across all documents.
    pub total_chunks: usize,
    /// Number of documents chunked.
    pub document_count: usize,
    /// Metadata for each chunk.
    pub chunks_metadata: Vec<PersistedChunk>,
    /// Count of summary-level chunks.
    pub summary_chunks: usize,
    /// Count of standard-level chunks.
    pub standard_chunks: usize,
    /// Count of detailed-level chunks.
    pub detailed_chunks: usize,
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Scrape Family
// ---------------------------------------------------------------------------

/// Persisted header extracted from a scraped page.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedHeader {
    /// Header level (1-6).
    pub level: u8,
    /// Header text (non-empty).
    pub text: String,
}

/// Persisted page filter status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedPageFilterStatus {
    /// Content-density filtering was applied.
    Filtered,
    /// Raw markdown stored without filtering.
    Unfiltered,
}

/// Persisted scraped page.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapedPage {
    /// Page URL.
    pub url: String,
    /// Extracted markdown content.
    pub markdown: String,
    /// Page title.
    pub title: String,
    /// Links found on the page.
    pub links: Vec<String>,
    /// Headers found on the page.
    pub headers: Vec<PersistedHeader>,
    /// Word count of the markdown content.
    pub word_count: usize,
    /// URL-derived slug.
    pub slug: String,
    /// Whether filtering was applied.
    pub filter_status: PersistedPageFilterStatus,
    /// Number of elements removed by filtering.
    pub elements_removed: usize,
    /// Content density score (0.0-1.0, must be finite).
    pub density_score: f32,
}

/// Persisted batch scrape result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapeResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Successfully scraped pages.
    pub pages: Vec<PersistedScrapedPage>,
    /// Total URLs discovered.
    pub total_urls: usize,
    /// Number of successfully scraped pages.
    pub success_count: usize,
    /// Number of failed scrapes.
    pub error_count: usize,
    /// Errors as (url, `error_message`) pairs.
    pub errors: Vec<(String, String)>,
    /// Base URL of the scrape.
    pub base_url: String,
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Watch/Snapshot Family
// ---------------------------------------------------------------------------

/// Persisted page hash: URL + SHA-256 content hash + title.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageHash {
    /// Canonical URL of the page.
    pub url: String,
    /// SHA-256 hash of the page's markdown content.
    pub content_hash: [u8; 32],
    /// Page title for display.
    pub title: String,
}

/// Persisted change kind: Added / Modified / Removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChangeKind {
    /// Page appeared that wasn't in the previous snapshot.
    Added,
    /// Page content hash changed.
    Modified,
    /// Page disappeared from the current scrape.
    Removed,
}

/// Persisted page-level change between two snapshots.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageChange {
    /// URL that changed.
    pub url: String,
    /// Kind of change.
    pub kind: PersistedChangeKind,
    /// Previous content hash (None for Added).
    pub old_hash: Option<[u8; 32]>,
    /// New content hash (None for Removed).
    pub new_hash: Option<[u8; 32]>,
    /// Page title for display.
    pub title: String,
}

/// Persisted change summary counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangeSummary {
    /// Pages added.
    pub added: usize,
    /// Pages removed.
    pub removed: usize,
    /// Pages modified.
    pub modified: usize,
    /// Pages unchanged.
    pub unchanged: usize,
    /// Total pages in current snapshot.
    pub total_current: usize,
    /// Total pages in previous snapshot.
    pub total_previous: usize,
}

/// Persisted point-in-time snapshot of all scraped pages.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedSnapshot {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Target URL that was scraped.
    pub target_url: String,
    /// Unix epoch seconds (replaces `DateTime<Utc>`).
    pub timestamp_secs: i64,
    /// Page hashes sorted by URL key for deterministic serialization.
    pub pages: Vec<(String, PersistedPageHash)>,
}

/// Persisted change plan (Terraform-style plan/apply model).
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangePlan {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Target URL being watched.
    pub target_url: String,
    /// Unix epoch seconds (replaces `DateTime<Utc>`).
    pub timestamp_secs: i64,
    /// All detected changes.
    pub changes: Vec<PersistedPageChange>,
    /// Summary counts.
    pub summary: PersistedChangeSummary,
    /// The new snapshot (ready to be applied).
    pub pending_snapshot: PersistedSnapshot,
}

// ---------------------------------------------------------------------------
// Persisted Record Types — Assign Family
// ---------------------------------------------------------------------------

/// Persisted ID mapping: `source_path` + assigned document identity.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedIdMapping {
    /// Source file path (key into `link_map`).
    pub source_path: String,
    /// Assigned document ID (e.g., "concept/general/my-doc").
    pub id: String,
    /// Output filename (e.g., "concept-general-my-doc.md").
    pub filename: String,
    /// Subcategory extracted from path.
    pub subcategory: String,
    /// URL-safe slug.
    pub slug: String,
}

// ===========================================================================
// Conversion Functions: Runtime → Persisted (Infallible)
// ===========================================================================

// --- Analysis Family ---

/// Convert a runtime [`Heading`] to its persisted form.
#[must_use]
pub fn heading_to_persisted(h: &Heading) -> PersistedHeading {
    PersistedHeading {
        level: h.level,
        text: h.text.clone(),
        line: h.line,
    }
}

/// Convert a runtime [`LinkKind`] to its persisted form.
#[must_use]
pub fn link_kind_to_persisted(k: &LinkKind) -> PersistedLinkKind {
    match k {
        LinkKind::Internal => PersistedLinkKind::Internal,
        LinkKind::External => PersistedLinkKind::External,
    }
}

/// Convert a runtime [`Link`] to its persisted form.
#[must_use]
pub fn link_to_persisted(l: &Link) -> PersistedLink {
    PersistedLink {
        text: l.text.clone(),
        target: l.target.clone(),
        kind: link_kind_to_persisted(&l.kind),
    }
}

/// Convert a runtime [`Analysis`] to its persisted form.
///
/// Frontmatter `HashMap` entries are sorted by key for deterministic serialization.
/// `Arc<str>` content is converted to `String`.
#[must_use]
pub fn analysis_to_persisted(a: &Analysis) -> PersistedAnalysis {
    PersistedAnalysis {
        schema_version: 1,
        source_path: a.source_path.clone(),
        title: a.title.clone(),
        frontmatter: a.frontmatter.as_ref().map(|fm| {
            fm.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .sorted_by(|a, b| a.0.cmp(&b.0))
                .collect()
        }),
        headings: a.headings.iter().map(heading_to_persisted).collect(),
        links: a.links.iter().map(link_to_persisted).collect(),
        first_paragraph: a.first_paragraph.clone(),
        word_count: a.word_count,
        has_code: a.has_code,
        has_tables: a.has_tables,
        category: a.category.clone(),
        content: a.content.to_string(),
    }
}

/// Convert a runtime [`FailedFile`] to its persisted form.
#[must_use]
pub fn failed_file_to_persisted(f: &FailedFile) -> PersistedFailedFile {
    PersistedFailedFile {
        source_path: f.source_path.clone(),
        error: f.error.clone(),
    }
}

/// Convert a runtime [`AnalyzeResult`] to its persisted form.
#[must_use]
pub fn analyze_result_to_persisted(r: &AnalyzeResult) -> PersistedAnalyzeResult {
    PersistedAnalyzeResult {
        schema_version: 1,
        analyses: r.analyses.iter().map(analysis_to_persisted).collect(),
        failed_files: r
            .failed_files
            .iter()
            .map(failed_file_to_persisted)
            .collect(),
        total_discovered: r.total_discovered,
    }
}

// --- Transform Family ---

/// Convert a runtime [`TransformError`] to its persisted form.
#[must_use]
pub fn transform_error_to_persisted(e: &TransformError) -> PersistedTransformError {
    PersistedTransformError {
        source_path: e.source_path.clone(),
        error: e.error.clone(),
    }
}

/// Convert a runtime [`TransformResult`] to its persisted form.
#[must_use]
pub fn transform_result_to_persisted(r: &TransformResult) -> PersistedTransformResult {
    PersistedTransformResult {
        schema_version: 1,
        success_count: r.success_count,
        total_count: r.total_count,
        error_count: r.error_count,
        errors: r.errors.iter().map(transform_error_to_persisted).collect(),
    }
}

// --- Chunk Family ---

/// Convert a runtime [`ChunkType`] to its persisted form.
#[must_use]
pub fn chunk_type_to_persisted(t: &ChunkType) -> PersistedChunkType {
    match t {
        ChunkType::Code => PersistedChunkType::Code,
        ChunkType::Table => PersistedChunkType::Table,
        ChunkType::Prose => PersistedChunkType::Prose,
    }
}

/// Convert a runtime [`ChunkLevel`] to its persisted form.
#[must_use]
pub fn chunk_level_to_persisted(l: &ChunkLevel) -> PersistedChunkLevel {
    match l {
        ChunkLevel::Summary => PersistedChunkLevel::Summary,
        ChunkLevel::Standard => PersistedChunkLevel::Standard,
        ChunkLevel::Detailed => PersistedChunkLevel::Detailed,
    }
}

/// Convert a runtime ctd [`Chunk`] to its persisted form.
#[must_use]
pub fn chunk_to_persisted(c: &Chunk) -> PersistedChunk {
    PersistedChunk {
        schema_version: 1,
        chunk_id: c.chunk_id.clone(),
        doc_id: c.doc_id.clone(),
        doc_title: c.doc_title.clone(),
        chunk_index: c.chunk_index,
        content: c.content.clone(),
        token_count: c.token_count,
        heading: c.heading.clone(),
        heading_path: c.heading_path.clone(),
        chunk_type: chunk_type_to_persisted(&c.chunk_type),
        previous_chunk_id: c.previous_chunk_id.clone(),
        next_chunk_id: c.next_chunk_id.clone(),
        related_chunk_ids: c.related_chunk_ids.clone(),
        summary: c.summary.clone(),
        chunk_level: chunk_level_to_persisted(&c.chunk_level),
        parent_chunk_id: c.parent_chunk_id.clone(),
        child_chunk_ids: c.child_chunk_ids.clone(),
        context_prefix: c.context_prefix.clone(),
    }
}

/// Convert a runtime [`ChunksResult`] to its persisted form.
#[must_use]
pub fn chunks_result_to_persisted(r: &ChunksResult) -> PersistedChunksResult {
    PersistedChunksResult {
        schema_version: 1,
        total_chunks: r.total_chunks,
        document_count: r.document_count,
        chunks_metadata: r.chunks_metadata.iter().map(chunk_to_persisted).collect(),
        summary_chunks: r.summary_chunks,
        standard_chunks: r.standard_chunks,
        detailed_chunks: r.detailed_chunks,
    }
}

// --- Scrape Family ---

/// Convert a runtime scrape [`Header`] to its persisted form.
#[must_use]
pub fn header_to_persisted(h: &Header) -> PersistedHeader {
    PersistedHeader {
        level: h.level,
        text: h.text.clone(),
    }
}

/// Convert a runtime [`PageFilterStatus`] to its persisted form.
#[must_use]
pub fn page_filter_status_to_persisted(s: &PageFilterStatus) -> PersistedPageFilterStatus {
    match s {
        PageFilterStatus::Filtered => PersistedPageFilterStatus::Filtered,
        PageFilterStatus::Unfiltered => PersistedPageFilterStatus::Unfiltered,
    }
}

/// Convert a runtime [`ScrapedPage`] to its persisted form.
#[must_use]
pub fn scraped_page_to_persisted(p: &ScrapedPage) -> PersistedScrapedPage {
    PersistedScrapedPage {
        url: p.url.clone(),
        markdown: p.markdown.clone(),
        title: p.title.clone(),
        links: p.links.clone(),
        headers: p.headers.iter().map(header_to_persisted).collect(),
        word_count: p.word_count,
        slug: p.slug.clone(),
        filter_status: page_filter_status_to_persisted(&p.filter_status),
        elements_removed: p.elements_removed,
        density_score: p.density_score,
    }
}

/// Convert a runtime [`ScrapeResult`] to its persisted form.
#[must_use]
pub fn scrape_result_to_persisted(r: &ScrapeResult) -> PersistedScrapeResult {
    PersistedScrapeResult {
        schema_version: 1,
        pages: r.pages.iter().map(scraped_page_to_persisted).collect(),
        total_urls: r.total_urls,
        success_count: r.success_count,
        error_count: r.error_count,
        errors: r.errors.clone(),
        base_url: r.base_url.clone(),
    }
}

// --- Watch/Snapshot Family ---

/// Convert a runtime [`PageHash`] to its persisted form.
#[must_use]
pub fn page_hash_to_persisted(p: &PageHash) -> PersistedPageHash {
    PersistedPageHash {
        url: p.url.clone(),
        content_hash: p.content_hash,
        title: p.title.clone(),
    }
}

/// Convert a runtime [`ChangeKind`] to its persisted form.
#[must_use]
pub fn change_kind_to_persisted(k: &ChangeKind) -> PersistedChangeKind {
    match k {
        ChangeKind::Added => PersistedChangeKind::Added,
        ChangeKind::Modified => PersistedChangeKind::Modified,
        ChangeKind::Removed => PersistedChangeKind::Removed,
    }
}

/// Convert a runtime [`PageChange`] to its persisted form.
#[must_use]
pub fn page_change_to_persisted(p: &PageChange) -> PersistedPageChange {
    PersistedPageChange {
        url: p.url.clone(),
        kind: change_kind_to_persisted(&p.kind),
        old_hash: p.old_hash,
        new_hash: p.new_hash,
        title: p.title.clone(),
    }
}

/// Convert a runtime [`ChangeSummary`] to its persisted form.
#[must_use]
pub fn change_summary_to_persisted(s: &ChangeSummary) -> PersistedChangeSummary {
    PersistedChangeSummary {
        added: s.added,
        removed: s.removed,
        modified: s.modified,
        unchanged: s.unchanged,
        total_current: s.total_current,
        total_previous: s.total_previous,
    }
}

/// Convert a runtime [`Snapshot`] to its persisted form.
///
/// `DateTime<Utc>` is converted to unix epoch seconds (i64).
/// Pages are sorted by URL key for deterministic serialization.
#[must_use]
pub fn snapshot_to_persisted(s: &Snapshot) -> PersistedSnapshot {
    PersistedSnapshot {
        schema_version: 1,
        target_url: s.target_url.clone(),
        timestamp_secs: s.timestamp.timestamp(),
        pages: s
            .pages
            .iter()
            .map(|(url, ph)| (url.clone(), page_hash_to_persisted(ph)))
            .collect(),
    }
}

/// Convert a runtime [`ChangePlan`] to its persisted form.
#[must_use]
pub fn change_plan_to_persisted(p: &ChangePlan) -> PersistedChangePlan {
    PersistedChangePlan {
        schema_version: 1,
        target_url: p.target_url.clone(),
        timestamp_secs: p.timestamp.timestamp(),
        changes: p.changes.iter().map(page_change_to_persisted).collect(),
        summary: change_summary_to_persisted(&p.summary),
        pending_snapshot: snapshot_to_persisted(&p.pending_snapshot),
    }
}

// --- Assign Family ---

/// Convert a `source_path` and runtime [`IdMapping`] to its persisted form.
#[must_use]
pub fn id_mapping_to_persisted(source_path: &str, m: &IdMapping) -> PersistedIdMapping {
    PersistedIdMapping {
        source_path: source_path.to_string(),
        id: m.id.clone(),
        filename: m.filename.clone(),
        subcategory: m.subcategory.clone(),
        slug: m.slug.clone(),
    }
}

// ===========================================================================
// Conversion Functions: Persisted → Runtime (Fallible)
// ===========================================================================

// --- Analysis Family ---

/// Convert a persisted heading back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::OutOfRange`] if level is not in 1..=6.
/// Returns [`PersistError::EmptyField`] if text is empty/whitespace after trim.
pub fn persisted_heading_to_runtime(p: &PersistedHeading) -> Result<Heading, PersistError> {
    require_range(i64::from(p.level), 1, 6, "level")?;
    require_non_empty(&p.text, "text")?;
    Ok(Heading {
        level: p.level,
        text: p.text.clone(),
        line: p.line,
    })
}

/// Convert a persisted link kind back to runtime form (1:1 mapping, always succeeds).
pub fn persisted_link_kind_to_runtime(p: PersistedLinkKind) -> Result<LinkKind, PersistError> {
    match p {
        PersistedLinkKind::Internal => Ok(LinkKind::Internal),
        PersistedLinkKind::External => Ok(LinkKind::External),
    }
}

/// Convert a persisted link back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::EmptyField`] if target is empty.
pub fn persisted_link_to_runtime(p: &PersistedLink) -> Result<Link, PersistError> {
    require_non_empty(&p.target, "target")?;
    Ok(Link {
        text: p.text.clone(),
        target: p.target.clone(),
        kind: persisted_link_kind_to_runtime(p.kind)?,
    })
}

/// Convert a persisted analysis back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Returns [`PersistError::EmptyField`] if `source_path`, title, or category is empty.
/// Propagates any errors from nested heading/link conversions.
pub fn persisted_analysis_to_runtime(p: &PersistedAnalysis) -> Result<Analysis, PersistError> {
    require_schema_v1(p.schema_version)?;
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.title, "title")?;
    require_non_empty(&p.category, "category")?;

    let headings = p
        .headings
        .iter()
        .map(persisted_heading_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;

    let links = p
        .links
        .iter()
        .map(persisted_link_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;

    let frontmatter = p
        .frontmatter
        .as_ref()
        .map(|pairs| pairs.iter().map(|(k, v)| (k.clone(), v.clone())).collect());

    Ok(Analysis {
        source_path: p.source_path.clone(),
        title: p.title.clone(),
        frontmatter,
        headings,
        links,
        first_paragraph: p.first_paragraph.clone(),
        word_count: p.word_count,
        has_code: p.has_code,
        has_tables: p.has_tables,
        category: p.category.clone(),
        content: Arc::<str>::from(p.content.as_str()),
    })
}

fn persisted_failed_file_to_runtime(p: &PersistedFailedFile) -> Result<FailedFile, PersistError> {
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.error, "error")?;
    Ok(FailedFile {
        source_path: p.source_path.clone(),
        error: p.error.clone(),
    })
}

/// Convert a persisted analyze result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested `analysis/failed_file` conversions.
pub fn persisted_analyze_result_to_runtime(
    p: &PersistedAnalyzeResult,
) -> Result<AnalyzeResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let analyses = p
        .analyses
        .iter()
        .map(persisted_analysis_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    let failed_files = p
        .failed_files
        .iter()
        .map(persisted_failed_file_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AnalyzeResult {
        analyses,
        failed_files,
        total_discovered: p.total_discovered,
    })
}

// --- Transform Family ---

/// Convert a persisted transform error back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::EmptyField`] if `source_path` or error is empty.
pub fn persisted_transform_error_to_runtime(
    p: &PersistedTransformError,
) -> Result<TransformError, PersistError> {
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.error, "error")?;
    Ok(TransformError {
        source_path: p.source_path.clone(),
        error: p.error.clone(),
    })
}

/// Convert a persisted transform result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested error conversions.
pub fn persisted_transform_result_to_runtime(
    p: &PersistedTransformResult,
) -> Result<TransformResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let errors = p
        .errors
        .iter()
        .map(persisted_transform_error_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransformResult {
        success_count: p.success_count,
        total_count: p.total_count,
        error_count: p.error_count,
        errors,
    })
}

// --- Chunk Family ---

/// Convert a persisted chunk type back to runtime form (1:1 mapping, always succeeds).
pub fn persisted_chunk_type_to_runtime(p: PersistedChunkType) -> Result<ChunkType, PersistError> {
    match p {
        PersistedChunkType::Code => Ok(ChunkType::Code),
        PersistedChunkType::Table => Ok(ChunkType::Table),
        PersistedChunkType::Prose => Ok(ChunkType::Prose),
    }
}

/// Convert a persisted chunk level back to runtime form (1:1 mapping, always succeeds).
pub fn persisted_chunk_level_to_runtime(
    p: PersistedChunkLevel,
) -> Result<ChunkLevel, PersistError> {
    match p {
        PersistedChunkLevel::Summary => Ok(ChunkLevel::Summary),
        PersistedChunkLevel::Standard => Ok(ChunkLevel::Standard),
        PersistedChunkLevel::Detailed => Ok(ChunkLevel::Detailed),
    }
}

/// Convert a persisted chunk back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Returns [`PersistError::EmptyField`] if `chunk_id`, `doc_id`, or content is empty.
/// Returns [`PersistError::OutOfRange`] if `token_count` == 0.
pub fn persisted_chunk_to_runtime(p: &PersistedChunk) -> Result<Chunk, PersistError> {
    require_schema_v1(p.schema_version)?;
    require_non_empty(&p.chunk_id, "chunk_id")?;
    require_non_empty(&p.doc_id, "doc_id")?;
    require_non_empty(&p.content, "content")?;
    if p.token_count == 0 {
        return Err(PersistError::OutOfRange {
            field: "token_count".to_string(),
            value: 0,
            min: 1,
            max: i64::MAX,
        });
    }

    Ok(Chunk {
        chunk_id: p.chunk_id.clone(),
        doc_id: p.doc_id.clone(),
        doc_title: p.doc_title.clone(),
        chunk_index: p.chunk_index,
        content: p.content.clone(),
        token_count: p.token_count,
        heading: p.heading.clone(),
        heading_path: p.heading_path.clone(),
        chunk_type: persisted_chunk_type_to_runtime(p.chunk_type)?,
        previous_chunk_id: p.previous_chunk_id.clone(),
        next_chunk_id: p.next_chunk_id.clone(),
        related_chunk_ids: p.related_chunk_ids.clone(),
        summary: p.summary.clone(),
        chunk_level: persisted_chunk_level_to_runtime(p.chunk_level)?,
        parent_chunk_id: p.parent_chunk_id.clone(),
        child_chunk_ids: p.child_chunk_ids.clone(),
        context_prefix: p.context_prefix.clone(),
    })
}

/// Convert a persisted chunks result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested chunk conversions.
pub fn persisted_chunks_result_to_runtime(
    p: &PersistedChunksResult,
) -> Result<ChunksResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let chunks_metadata = p
        .chunks_metadata
        .iter()
        .map(persisted_chunk_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ChunksResult {
        total_chunks: p.total_chunks,
        document_count: p.document_count,
        chunks_metadata,
        summary_chunks: p.summary_chunks,
        standard_chunks: p.standard_chunks,
        detailed_chunks: p.detailed_chunks,
    })
}

// --- Scrape Family ---

/// Convert a persisted scrape header back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::OutOfRange`] if level is not in 1..=6.
/// Returns [`PersistError::EmptyField`] if text is empty.
pub fn persisted_header_to_runtime(p: &PersistedHeader) -> Result<Header, PersistError> {
    require_range(i64::from(p.level), 1, 6, "level")?;
    require_non_empty(&p.text, "text")?;
    Ok(Header {
        level: p.level,
        text: p.text.clone(),
    })
}

/// Convert a persisted page filter status back to runtime form (1:1 mapping).
pub fn persisted_page_filter_status_to_runtime(
    p: PersistedPageFilterStatus,
) -> Result<PageFilterStatus, PersistError> {
    match p {
        PersistedPageFilterStatus::Filtered => Ok(PageFilterStatus::Filtered),
        PersistedPageFilterStatus::Unfiltered => Ok(PageFilterStatus::Unfiltered),
    }
}

/// Convert a persisted scraped page back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::NonFiniteFloat`] if `density_score` is `NaN` or Infinite.
/// Propagates any errors from nested header conversions.
pub fn persisted_scraped_page_to_runtime(
    p: &PersistedScrapedPage,
) -> Result<ScrapedPage, PersistError> {
    require_finite_f32(p.density_score, "density_score")?;
    let headers = p
        .headers
        .iter()
        .map(persisted_header_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    let filter_status = persisted_page_filter_status_to_runtime(p.filter_status)?;
    Ok(ScrapedPage {
        url: p.url.clone(),
        markdown: p.markdown.clone(),
        title: p.title.clone(),
        links: p.links.clone(),
        headers,
        word_count: p.word_count,
        slug: p.slug.clone(),
        filter_status,
        elements_removed: p.elements_removed,
        density_score: p.density_score,
    })
}

/// Convert a persisted scrape result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested page conversions.
pub fn persisted_scrape_result_to_runtime(
    p: &PersistedScrapeResult,
) -> Result<ScrapeResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let pages = p
        .pages
        .iter()
        .map(persisted_scraped_page_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ScrapeResult {
        pages,
        total_urls: p.total_urls,
        success_count: p.success_count,
        error_count: p.error_count,
        errors: p.errors.clone(),
        base_url: p.base_url.clone(),
    })
}

// --- Watch/Snapshot Family ---

/// Convert a persisted page hash back to runtime form.
pub fn persisted_page_hash_to_runtime(p: &PersistedPageHash) -> Result<PageHash, PersistError> {
    Ok(PageHash {
        url: p.url.clone(),
        content_hash: p.content_hash,
        title: p.title.clone(),
    })
}

/// Convert a persisted change kind back to runtime form (1:1 mapping).
pub fn persisted_change_kind_to_runtime(
    p: PersistedChangeKind,
) -> Result<ChangeKind, PersistError> {
    match p {
        PersistedChangeKind::Added => Ok(ChangeKind::Added),
        PersistedChangeKind::Modified => Ok(ChangeKind::Modified),
        PersistedChangeKind::Removed => Ok(ChangeKind::Removed),
    }
}

/// Convert a persisted page change back to runtime form.
pub fn persisted_page_change_to_runtime(
    p: &PersistedPageChange,
) -> Result<PageChange, PersistError> {
    Ok(PageChange {
        url: p.url.clone(),
        kind: persisted_change_kind_to_runtime(p.kind)?,
        old_hash: p.old_hash,
        new_hash: p.new_hash,
        title: p.title.clone(),
    })
}

/// Convert a persisted change summary back to runtime form (direct field copy).
pub fn persisted_change_summary_to_runtime(
    p: &PersistedChangeSummary,
) -> Result<ChangeSummary, PersistError> {
    Ok(ChangeSummary {
        added: p.added,
        removed: p.removed,
        modified: p.modified,
        unchanged: p.unchanged,
        total_current: p.total_current,
        total_previous: p.total_previous,
    })
}

/// Convert a persisted snapshot back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Returns [`PersistError::DeserializationFailed`] if `timestamp_secs` is invalid.
/// Propagates any errors from nested page hash conversions.
pub fn persisted_snapshot_to_runtime(p: &PersistedSnapshot) -> Result<Snapshot, PersistError> {
    require_schema_v1(p.schema_version)?;
    let timestamp = Utc
        .timestamp_opt(p.timestamp_secs, 0)
        .single()
        .ok_or_else(|| PersistError::DeserializationFailed {
            reason: format!("invalid unix timestamp: {}", p.timestamp_secs),
        })?;
    let pages = p
        .pages
        .iter()
        .map(|(url, ph)| persisted_page_hash_to_runtime(ph).map(|h| (url.clone(), h)))
        .collect::<Result<std::collections::BTreeMap<String, PageHash>, _>>()?;
    Ok(Snapshot {
        target_url: p.target_url.clone(),
        timestamp,
        pages,
    })
}

/// Convert a persisted change plan back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested change/snapshot conversions.
pub fn persisted_change_plan_to_runtime(
    p: &PersistedChangePlan,
) -> Result<ChangePlan, PersistError> {
    require_schema_v1(p.schema_version)?;
    let timestamp = Utc
        .timestamp_opt(p.timestamp_secs, 0)
        .single()
        .ok_or_else(|| PersistError::DeserializationFailed {
            reason: format!("invalid unix timestamp: {}", p.timestamp_secs),
        })?;
    let changes = p
        .changes
        .iter()
        .map(persisted_page_change_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    let summary = persisted_change_summary_to_runtime(&p.summary)?;
    let pending_snapshot = persisted_snapshot_to_runtime(&p.pending_snapshot)?;
    Ok(ChangePlan {
        target_url: p.target_url.clone(),
        timestamp,
        changes,
        summary,
        pending_snapshot,
    })
}

// --- Assign Family ---

/// Convert a persisted ID mapping back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::EmptyField`] if id, filename, subcategory, or slug is empty.
pub fn persisted_id_mapping_to_runtime(
    p: &PersistedIdMapping,
) -> Result<(String, IdMapping), PersistError> {
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.id, "id")?;
    require_non_empty(&p.filename, "filename")?;
    require_non_empty(&p.subcategory, "subcategory")?;
    require_non_empty(&p.slug, "slug")?;
    Ok((
        p.source_path.clone(),
        IdMapping {
            id: p.id.clone(),
            filename: p.filename.clone(),
            subcategory: p.subcategory.clone(),
            slug: p.slug.clone(),
        },
    ))
}
