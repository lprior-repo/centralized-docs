//! Persisted types and conversions for the Analysis pipeline phase.

use super::error::{require_non_empty, require_range, require_schema_v1, PersistError};
use crate::analyze::{Analysis, AnalyzeResult, FailedFile, Heading, Link, LinkKind};
use itertools::Itertools;
use std::sync::Arc;

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

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

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

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

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
