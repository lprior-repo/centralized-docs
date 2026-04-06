//! Integration tests for the persisted output record types and conversions.
//!
//! Covers behaviors B01–B68 from the test plan for bead cdocs-bvh.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::float_cmp)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::match_same_arms)]

use chrono::{TimeZone, Utc};
use contextual_chunker::{ChunkLevel, ChunkType};
use doc_transformer::analyze::{Analysis, AnalyzeResult, FailedFile, Heading, Link, LinkKind};
use doc_transformer::assign::IdMapping;
use doc_transformer::chunking_adapter::{Chunk, ChunksResult};
use doc_transformer::persisted::*;
use doc_transformer::scrape::validation::{Header, PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::transform::{TransformError, TransformResult};
use doc_transformer::watch::{
    ChangeKind, ChangePlan, ChangeSummary, PageChange, PageHash, Snapshot,
};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Test Helpers
// ---------------------------------------------------------------------------

fn sample_heading() -> Heading {
    Heading {
        level: 2,
        text: "Introduction".to_string(),
        line: 42,
    }
}

fn sample_link() -> Link {
    Link {
        text: "docs".to_string(),
        target: "/docs".to_string(),
        kind: LinkKind::Internal,
    }
}

fn sample_analysis() -> Analysis {
    let mut fm = HashMap::new();
    fm.insert("z-key".to_string(), "z-val".to_string());
    fm.insert("a-key".to_string(), "a-val".to_string());
    Analysis {
        source_path: "docs/guide.md".to_string(),
        title: "Getting Started".to_string(),
        frontmatter: Some(fm),
        headings: vec![sample_heading()],
        links: vec![sample_link()],
        first_paragraph: "Welcome to the guide.".to_string(),
        word_count: 100,
        has_code: true,
        has_tables: false,
        category: "general".to_string(),
        content: Arc::<str>::from("Full content here."),
    }
}

fn sample_analyze_result() -> AnalyzeResult {
    AnalyzeResult {
        analyses: vec![sample_analysis()],
        failed_files: vec![FailedFile {
            source_path: "bad.md".to_string(),
            error: "parse error".to_string(),
        }],
        total_discovered: 5,
    }
}

fn sample_chunk() -> Chunk {
    Chunk {
        chunk_id: "doc#0".to_string(),
        doc_id: "doc".to_string(),
        doc_title: "My Doc".to_string(),
        chunk_index: 0,
        content: "Some content here.".to_string(),
        token_count: 10,
        heading: Some("Intro".to_string()),
        heading_path: vec!["Intro".to_string()],
        chunk_type: ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: Some("doc#1".to_string()),
        related_chunk_ids: vec!["other#0".to_string()],
        summary: "A summary.".to_string(),
        chunk_level: ChunkLevel::Standard,
        parent_chunk_id: Some("doc#0-summary".to_string()),
        child_chunk_ids: vec!["doc#0-detailed".to_string()],
        context_prefix: Some("Previous context...".to_string()),
    }
}

fn sample_chunks_result() -> ChunksResult {
    ChunksResult {
        total_chunks: 3,
        document_count: 1,
        chunks_metadata: vec![sample_chunk()],
        summary_chunks: 1,
        standard_chunks: 1,
        detailed_chunks: 1,
    }
}

fn sample_scraped_page() -> ScrapedPage {
    ScrapedPage {
        url: "https://example.com/docs".to_string(),
        markdown: "# Docs\nContent.".to_string(),
        title: "Docs".to_string(),
        links: vec!["https://example.com/other".to_string()],
        headers: vec![Header {
            level: 1,
            text: "Docs".to_string(),
        }],
        word_count: 50,
        slug: "docs".to_string(),
        filter_status: PageFilterStatus::Filtered,
        elements_removed: 3,
        density_score: 0.85,
    }
}

fn sample_scrape_result() -> ScrapeResult {
    ScrapeResult {
        pages: vec![sample_scraped_page()],
        total_urls: 10,
        success_count: 8,
        error_count: 2,
        errors: vec![("https://bad.url".to_string(), "timeout".to_string())],
        base_url: "https://example.com".to_string(),
    }
}

fn sample_page_hash() -> PageHash {
    PageHash {
        url: "https://example.com/page".to_string(),
        content_hash: [1u8; 32],
        title: "Page Title".to_string(),
    }
}

fn sample_snapshot() -> Snapshot {
    let ts = Utc.with_ymd_and_hms(2025, 1, 15, 10, 30, 0).unwrap();
    let mut pages = BTreeMap::new();
    pages.insert(
        "https://example.com/a".to_string(),
        PageHash {
            url: "https://example.com/a".to_string(),
            content_hash: [1u8; 32],
            title: "A".to_string(),
        },
    );
    pages.insert(
        "https://example.com/b".to_string(),
        PageHash {
            url: "https://example.com/b".to_string(),
            content_hash: [2u8; 32],
            title: "B".to_string(),
        },
    );
    Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: ts,
        pages,
    }
}

fn sample_change_plan() -> ChangePlan {
    let ts = Utc.with_ymd_and_hms(2025, 1, 15, 10, 30, 0).unwrap();
    ChangePlan {
        target_url: "https://example.com".to_string(),
        timestamp: ts,
        changes: vec![PageChange {
            url: "https://example.com/new".to_string(),
            kind: ChangeKind::Added,
            old_hash: None,
            new_hash: Some([2u8; 32]),
            title: "New Page".to_string(),
        }],
        summary: ChangeSummary {
            added: 2,
            removed: 1,
            modified: 3,
            unchanged: 10,
            total_current: 15,
            total_previous: 14,
        },
        pending_snapshot: sample_snapshot(),
    }
}

fn sample_id_mapping() -> IdMapping {
    IdMapping {
        id: "concept/guide".to_string(),
        filename: "concept-guide.md".to_string(),
        subcategory: "concept".to_string(),
        slug: "guide".to_string(),
    }
}

// ===========================================================================
// B01-B06: Infallible Conversions — Analysis Family
// ===========================================================================

#[test]
fn heading_to_persisted_produces_identical_fields() {
    let h = sample_heading();
    let p = heading_to_persisted(&h);
    assert_eq!(p.level, 2);
    assert_eq!(p.text, "Introduction");
    assert_eq!(p.line, 42);
}

#[test]
fn link_kind_to_persisted_internal() {
    assert_eq!(
        link_kind_to_persisted(&LinkKind::Internal),
        PersistedLinkKind::Internal
    );
}

#[test]
fn link_kind_to_persisted_external() {
    assert_eq!(
        link_kind_to_persisted(&LinkKind::External),
        PersistedLinkKind::External
    );
}

#[test]
fn link_to_persisted_produces_identical_fields() {
    let l = sample_link();
    let p = link_to_persisted(&l);
    assert_eq!(p.text, "docs");
    assert_eq!(p.target, "/docs");
    assert_eq!(p.kind, PersistedLinkKind::Internal);
}

#[test]
fn analysis_to_persisted_schema_version_and_sorted_frontmatter() {
    let a = sample_analysis();
    let p = analysis_to_persisted(&a);
    assert_eq!(p.schema_version, 1);
    let fm = p.frontmatter.as_ref().unwrap();
    assert_eq!(fm[0].0, "a-key");
    assert_eq!(fm[1].0, "z-key");
    assert_eq!(p.content, "Full content here.");
    assert_eq!(p.category, "general");
}

#[test]
fn analyze_result_to_persisted_schema_version_1() {
    let r = sample_analyze_result();
    let p = analyze_result_to_persisted(&r);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.total_discovered, 5);
    assert_eq!(p.analyses.len(), 1);
    assert_eq!(p.failed_files.len(), 1);
}

// ===========================================================================
// B07-B08: Infallible Conversions — Transform Family
// ===========================================================================

#[test]
fn transform_error_to_persisted_produces_identical_fields() {
    let e = TransformError {
        source_path: "a.md".to_string(),
        error: "bad".to_string(),
    };
    let p = transform_error_to_persisted(&e);
    assert_eq!(p.source_path, "a.md");
    assert_eq!(p.error, "bad");
}

#[test]
fn transform_result_to_persisted_schema_version_1() {
    let r = TransformResult {
        success_count: 3,
        total_count: 5,
        error_count: 2,
        errors: vec![TransformError {
            source_path: "x.md".to_string(),
            error: "fail".to_string(),
        }],
    };
    let p = transform_result_to_persisted(&r);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.success_count, 3);
    assert_eq!(p.error_count, 2);
    assert_eq!(p.errors.len(), 1);
}

// ===========================================================================
// B09-B16: Infallible Conversions — Chunk Family
// ===========================================================================

#[test]
fn chunk_type_to_persisted_code() {
    assert_eq!(
        chunk_type_to_persisted(&ChunkType::Code),
        PersistedChunkType::Code
    );
}
#[test]
fn chunk_type_to_persisted_table() {
    assert_eq!(
        chunk_type_to_persisted(&ChunkType::Table),
        PersistedChunkType::Table
    );
}
#[test]
fn chunk_type_to_persisted_prose() {
    assert_eq!(
        chunk_type_to_persisted(&ChunkType::Prose),
        PersistedChunkType::Prose
    );
}

#[test]
fn chunk_level_to_persisted_summary() {
    assert_eq!(
        chunk_level_to_persisted(&ChunkLevel::Summary),
        PersistedChunkLevel::Summary
    );
}
#[test]
fn chunk_level_to_persisted_standard() {
    assert_eq!(
        chunk_level_to_persisted(&ChunkLevel::Standard),
        PersistedChunkLevel::Standard
    );
}
#[test]
fn chunk_level_to_persisted_detailed() {
    assert_eq!(
        chunk_level_to_persisted(&ChunkLevel::Detailed),
        PersistedChunkLevel::Detailed
    );
}

#[test]
fn chunk_to_persisted_schema_version_1() {
    let c = sample_chunk();
    let p = chunk_to_persisted(&c);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.chunk_id, "doc#0");
    assert_eq!(p.token_count, 10);
    assert_eq!(p.chunk_type, PersistedChunkType::Prose);
    assert_eq!(p.chunk_level, PersistedChunkLevel::Standard);
    assert_eq!(p.context_prefix.as_deref(), Some("Previous context..."));
}

#[test]
fn chunks_result_to_persisted_schema_version_1() {
    let r = sample_chunks_result();
    let p = chunks_result_to_persisted(&r);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.total_chunks, 3);
    assert_eq!(p.document_count, 1);
}

// ===========================================================================
// B17-B21: Infallible Conversions — Scrape Family
// ===========================================================================

#[test]
fn header_to_persisted_identical_fields() {
    let h = Header {
        level: 2,
        text: "Section".to_string(),
    };
    let p = header_to_persisted(&h);
    assert_eq!(p.level, 2);
    assert_eq!(p.text, "Section");
}

#[test]
fn page_filter_status_to_persisted_filtered() {
    assert_eq!(
        page_filter_status_to_persisted(&PageFilterStatus::Filtered),
        PersistedPageFilterStatus::Filtered
    );
}
#[test]
fn page_filter_status_to_persisted_unfiltered() {
    assert_eq!(
        page_filter_status_to_persisted(&PageFilterStatus::Unfiltered),
        PersistedPageFilterStatus::Unfiltered
    );
}

#[test]
fn scraped_page_to_persisted_identical_fields() {
    let sp = sample_scraped_page();
    let p = scraped_page_to_persisted(&sp);
    assert_eq!(p.url, sp.url);
    assert!((p.density_score - 0.85f32).abs() < f32::EPSILON);
    assert_eq!(p.headers.len(), sp.headers.len());
    assert_eq!(p.links.len(), 1);
}

#[test]
fn scrape_result_to_persisted_schema_version_1() {
    let r = sample_scrape_result();
    let p = scrape_result_to_persisted(&r);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.base_url, "https://example.com");
    assert_eq!(p.pages.len(), 1);
    assert_eq!(p.errors.len(), 1);
}

// ===========================================================================
// B22-B29: Infallible Conversions — Watch/Snapshot Family
// ===========================================================================

#[test]
fn page_hash_to_persisted_identical_fields() {
    let ph = sample_page_hash();
    let p = page_hash_to_persisted(&ph);
    assert_eq!(p.url, ph.url);
    assert_eq!(p.content_hash, [1u8; 32]);
    assert_eq!(p.title, ph.title);
}

#[test]
fn change_kind_to_persisted_added() {
    assert_eq!(
        change_kind_to_persisted(&ChangeKind::Added),
        PersistedChangeKind::Added
    );
}
#[test]
fn change_kind_to_persisted_modified() {
    assert_eq!(
        change_kind_to_persisted(&ChangeKind::Modified),
        PersistedChangeKind::Modified
    );
}
#[test]
fn change_kind_to_persisted_removed() {
    assert_eq!(
        change_kind_to_persisted(&ChangeKind::Removed),
        PersistedChangeKind::Removed
    );
}

#[test]
fn page_change_to_persisted_identical_fields() {
    let pc = PageChange {
        url: "https://example.com".to_string(),
        kind: ChangeKind::Added,
        old_hash: None,
        new_hash: Some([2u8; 32]),
        title: "New".to_string(),
    };
    let p = page_change_to_persisted(&pc);
    assert_eq!(p.kind, PersistedChangeKind::Added);
    assert!(p.old_hash.is_none());
    assert_eq!(p.new_hash, Some([2u8; 32]));
}

#[test]
fn change_summary_to_persisted_identical_fields() {
    let cs = ChangeSummary {
        added: 2,
        removed: 1,
        modified: 3,
        unchanged: 10,
        total_current: 15,
        total_previous: 14,
    };
    let p = change_summary_to_persisted(&cs);
    assert_eq!(p.added, 2);
    assert_eq!(p.removed, 1);
    assert_eq!(p.total_current, 15);
}

#[test]
fn snapshot_to_persisted_schema_version_and_epoch_secs() {
    let s = sample_snapshot();
    let p = snapshot_to_persisted(&s);
    assert_eq!(p.schema_version, 1);
    // 2025-01-15T10:30:00Z = 1736937000
    assert_eq!(p.timestamp_secs, 1736937000);
    // pages sorted by URL key (BTreeMap already sorted)
    assert_eq!(p.pages.len(), 2);
    assert_eq!(p.pages[0].0, "https://example.com/a");
    assert_eq!(p.pages[1].0, "https://example.com/b");
}

#[test]
fn change_plan_to_persisted_schema_version_1() {
    let cp = sample_change_plan();
    let p = change_plan_to_persisted(&cp);
    assert_eq!(p.schema_version, 1);
    assert_eq!(p.changes.len(), 1);
    assert_eq!(p.summary.added, 2);
}

// ===========================================================================
// B30: Infallible Conversion — Assign Family
// ===========================================================================

#[test]
fn id_mapping_to_persisted_with_source_path() {
    let m = sample_id_mapping();
    let p = id_mapping_to_persisted("docs/guide.md", &m);
    assert_eq!(p.source_path, "docs/guide.md");
    assert_eq!(p.id, "concept/guide");
}

// ===========================================================================
// B31-B34: Fallible Conversions — Heading
// ===========================================================================

#[test]
fn persisted_heading_to_runtime_returns_heading_when_valid() {
    let p = PersistedHeading {
        level: 3,
        text: "Details".to_string(),
        line: 10,
    };
    let h = persisted_heading_to_runtime(&p).unwrap();
    assert_eq!(h.level, 3);
    assert_eq!(h.text, "Details");
    assert_eq!(h.line, 10);
}

#[test]
fn persisted_heading_to_runtime_rejects_level_zero() {
    let p = PersistedHeading {
        level: 0,
        text: "Bad".to_string(),
        line: 1,
    };
    let err = persisted_heading_to_runtime(&p).unwrap_err();
    assert!(
        matches!(err, PersistError::OutOfRange { field, value, min, max }
        if field == "level" && value == 0 && min == 1 && max == 6)
    );
}

#[test]
fn persisted_heading_to_runtime_rejects_level_seven() {
    let p = PersistedHeading {
        level: 7,
        text: "Bad".to_string(),
        line: 1,
    };
    let err = persisted_heading_to_runtime(&p).unwrap_err();
    assert!(
        matches!(err, PersistError::OutOfRange { field, value, min, max }
        if field == "level" && value == 7 && min == 1 && max == 6)
    );
}

#[test]
fn persisted_heading_to_runtime_rejects_whitespace_text() {
    let p = PersistedHeading {
        level: 1,
        text: "   ".to_string(),
        line: 1,
    };
    let err = persisted_heading_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "text"));
}

// ===========================================================================
// B35-B36: Fallible Conversions — Link
// ===========================================================================

#[test]
fn persisted_link_to_runtime_returns_link_when_valid() {
    let p = PersistedLink {
        text: "guide".to_string(),
        target: "/guide".to_string(),
        kind: PersistedLinkKind::External,
    };
    let l = persisted_link_to_runtime(&p).unwrap();
    assert_eq!(l.text, "guide");
    assert_eq!(l.target, "/guide");
    assert_eq!(l.kind, LinkKind::External);
}

#[test]
fn persisted_link_to_runtime_rejects_empty_target() {
    let p = PersistedLink {
        text: "x".to_string(),
        target: String::new(),
        kind: PersistedLinkKind::Internal,
    };
    let err = persisted_link_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "target"));
}

// ===========================================================================
// B37-B41: Fallible Conversions — Analysis
// ===========================================================================

#[test]
fn persisted_analysis_to_runtime_returns_analysis_when_valid() {
    let p = analysis_to_persisted(&sample_analysis());
    let a = persisted_analysis_to_runtime(&p).unwrap();
    assert_eq!(a.source_path, "docs/guide.md");
    assert_eq!(a.title, "Getting Started");
    assert_eq!(&*a.content, "Full content here.");
    assert_eq!(a.frontmatter.as_ref().map(|fm| fm.len()), Some(2));
    assert_eq!(a.headings.len(), 1);
    assert_eq!(a.links.len(), 1);
}

#[test]
fn persisted_analysis_to_runtime_rejects_empty_source_path() {
    let mut p = analysis_to_persisted(&sample_analysis());
    p.source_path = String::new();
    let err = persisted_analysis_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "source_path"));
}

#[test]
fn persisted_analysis_to_runtime_rejects_empty_title() {
    let mut p = analysis_to_persisted(&sample_analysis());
    p.title = String::new();
    let err = persisted_analysis_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "title"));
}

#[test]
fn persisted_analysis_to_runtime_rejects_empty_category() {
    let mut p = analysis_to_persisted(&sample_analysis());
    p.category = String::new();
    let err = persisted_analysis_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "category"));
}

#[test]
fn persisted_analysis_to_runtime_rejects_schema_version_2() {
    let mut p = analysis_to_persisted(&sample_analysis());
    p.schema_version = 2;
    let err = persisted_analysis_to_runtime(&p).unwrap_err();
    assert!(matches!(
        err,
        PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: 2
        }
    ));
}

// ===========================================================================
// B42-B43: Fallible Conversions — AnalyzeResult
// ===========================================================================

#[test]
fn persisted_analyze_result_to_runtime_returns_result_when_valid() {
    let p = analyze_result_to_persisted(&sample_analyze_result());
    let r = persisted_analyze_result_to_runtime(&p).unwrap();
    assert_eq!(r.total_discovered, 5);
    assert_eq!(r.analyses.len(), 1);
}

#[test]
fn persisted_analyze_result_to_runtime_rejects_schema_version_zero() {
    let mut p = analyze_result_to_persisted(&sample_analyze_result());
    p.schema_version = 0;
    let err = persisted_analyze_result_to_runtime(&p).unwrap_err();
    assert!(matches!(
        err,
        PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: 0
        }
    ));
}

// ===========================================================================
// B44-B47: Fallible Conversions — Chunk
// ===========================================================================

#[test]
fn persisted_chunk_to_runtime_returns_chunk_when_valid() {
    let p = chunk_to_persisted(&sample_chunk());
    let c = persisted_chunk_to_runtime(&p).unwrap();
    assert_eq!(c.chunk_id, "doc#0");
    assert_eq!(c.token_count, 10);
    assert_eq!(c.chunk_type, ChunkType::Prose);
}

#[test]
fn persisted_chunk_to_runtime_rejects_empty_chunk_id() {
    let mut p = chunk_to_persisted(&sample_chunk());
    p.chunk_id = String::new();
    let err = persisted_chunk_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "chunk_id"));
}

#[test]
fn persisted_chunk_to_runtime_rejects_empty_content() {
    let mut p = chunk_to_persisted(&sample_chunk());
    p.content = String::new();
    let err = persisted_chunk_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "content"));
}

#[test]
fn persisted_chunk_to_runtime_rejects_zero_token_count() {
    let mut p = chunk_to_persisted(&sample_chunk());
    p.token_count = 0;
    let err = persisted_chunk_to_runtime(&p).unwrap_err();
    assert!(
        matches!(err, PersistError::OutOfRange { field, value, min, .. }
        if field == "token_count" && value == 0 && min == 1)
    );
}

// ===========================================================================
// B48-B50: Fallible Conversions — ScrapedPage (density_score)
// ===========================================================================

#[test]
fn persisted_scraped_page_to_runtime_returns_page_when_valid() {
    let p = scraped_page_to_persisted(&sample_scraped_page());
    let sp = persisted_scraped_page_to_runtime(&p).unwrap();
    assert!((sp.density_score - 0.85f32).abs() < f32::EPSILON);
    assert_eq!(sp.url, "https://example.com/docs");
}

#[test]
fn persisted_scraped_page_to_runtime_rejects_nan_density() {
    let mut p = scraped_page_to_persisted(&sample_scraped_page());
    p.density_score = f32::NAN;
    let err = persisted_scraped_page_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::NonFiniteFloat { field, value }
        if field == "density_score" && value == "NaN"));
}

#[test]
fn persisted_scraped_page_to_runtime_rejects_inf_density() {
    let mut p = scraped_page_to_persisted(&sample_scraped_page());
    p.density_score = f32::INFINITY;
    let err = persisted_scraped_page_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::NonFiniteFloat { field, value }
        if field == "density_score" && value == "inf"));
}

// ===========================================================================
// B51-B52: Fallible Conversions — ScrapeResult
// ===========================================================================

#[test]
fn persisted_scrape_result_to_runtime_returns_result_when_valid() {
    let p = scrape_result_to_persisted(&sample_scrape_result());
    let r = persisted_scrape_result_to_runtime(&p).unwrap();
    assert_eq!(r.base_url, "https://example.com");
    assert_eq!(r.pages.len(), 1);
}

#[test]
fn persisted_scrape_result_to_runtime_rejects_schema_version_99() {
    let mut p = scrape_result_to_persisted(&sample_scrape_result());
    p.schema_version = 99;
    let err = persisted_scrape_result_to_runtime(&p).unwrap_err();
    assert!(matches!(
        err,
        PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: 99
        }
    ));
}

// ===========================================================================
// B53: Fallible Conversion — PageHash
// ===========================================================================

#[test]
fn persisted_page_hash_to_runtime_returns_page_hash_when_valid() {
    let p = PersistedPageHash {
        url: "https://example.com".to_string(),
        content_hash: [3u8; 32],
        title: "Test".to_string(),
    };
    let ph = persisted_page_hash_to_runtime(&p).unwrap();
    assert_eq!(ph.content_hash, [3u8; 32]);
    assert_eq!(ph.url, "https://example.com");
}

// ===========================================================================
// B54-B55: Fallible Conversions — Snapshot
// ===========================================================================

#[test]
fn persisted_snapshot_to_runtime_returns_snapshot_when_valid() {
    let p = snapshot_to_persisted(&sample_snapshot());
    let s = persisted_snapshot_to_runtime(&p).unwrap();
    assert_eq!(s.target_url, "https://example.com");
    assert_eq!(s.timestamp.timestamp(), 1736937000);
    assert_eq!(s.pages.len(), 2);
}

#[test]
fn persisted_snapshot_to_runtime_rejects_schema_version_5() {
    let mut p = snapshot_to_persisted(&sample_snapshot());
    p.schema_version = 5;
    let err = persisted_snapshot_to_runtime(&p).unwrap_err();
    assert!(matches!(
        err,
        PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: 5
        }
    ));
}

// ===========================================================================
// B56-B57: Fallible Conversions — ChangePlan
// ===========================================================================

#[test]
fn persisted_change_plan_to_runtime_returns_plan_when_valid() {
    let p = change_plan_to_persisted(&sample_change_plan());
    let cp = persisted_change_plan_to_runtime(&p).unwrap();
    assert_eq!(cp.changes.len(), 1);
    assert_eq!(cp.summary.added, 2);
}

#[test]
fn persisted_change_plan_to_runtime_rejects_schema_version_3() {
    let mut p = change_plan_to_persisted(&sample_change_plan());
    p.schema_version = 3;
    let err = persisted_change_plan_to_runtime(&p).unwrap_err();
    assert!(matches!(
        err,
        PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: 3
        }
    ));
}

// ===========================================================================
// B58-B59: Fallible Conversions — IdMapping
// ===========================================================================

#[test]
fn persisted_id_mapping_to_runtime_returns_tuple_when_valid() {
    let p = id_mapping_to_persisted("docs/guide.md", &sample_id_mapping());
    let (source, m) = persisted_id_mapping_to_runtime(&p).unwrap();
    assert_eq!(source, "docs/guide.md");
    assert_eq!(m.id, "concept/guide");
}

#[test]
fn persisted_id_mapping_to_runtime_rejects_empty_id() {
    let mut p = id_mapping_to_persisted("docs/guide.md", &sample_id_mapping());
    p.id = String::new();
    let err = persisted_id_mapping_to_runtime(&p).unwrap_err();
    assert!(matches!(err, PersistError::EmptyField { field } if field == "id"));
}

// ===========================================================================
// B60: Fallible Conversion — TransformResult
// ===========================================================================

#[test]
fn persisted_transform_result_to_runtime_returns_result_when_valid() {
    let r = TransformResult {
        success_count: 3,
        total_count: 5,
        error_count: 2,
        errors: vec![TransformError {
            source_path: "x.md".to_string(),
            error: "fail".to_string(),
        }],
    };
    let p = transform_result_to_persisted(&r);
    let rt = persisted_transform_result_to_runtime(&p).unwrap();
    assert_eq!(rt.success_count, 3);
    assert_eq!(rt.total_count, 5);
}

// ===========================================================================
// B61: Fallible Conversion — ChunksResult
// ===========================================================================

#[test]
fn persisted_chunks_result_to_runtime_returns_result_when_valid() {
    let p = chunks_result_to_persisted(&sample_chunks_result());
    let r = persisted_chunks_result_to_runtime(&p).unwrap();
    assert_eq!(r.total_chunks, 3);
    assert_eq!(r.chunks_metadata.len(), 1);
}

// ===========================================================================
// B62: rkyv Round-Trip for Every Record Type
// ===========================================================================

/// Serialize a value to bytes and deserialize it back. Uses rkyv 0.8 high-level API.
macro_rules! rkyv_roundtrip {
    ($ty:ty, $value:expr) => {{
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>($value).unwrap();
        rkyv::from_bytes::<$ty, rkyv::rancor::Error>(&bytes).unwrap()
    }};
}

#[test]
fn rkyv_roundtrip_preserves_persisted_heading() {
    let p = PersistedHeading {
        level: 2,
        text: "Hello".to_string(),
        line: 5,
    };
    let rt = rkyv_roundtrip!(PersistedHeading, &p);
    assert_eq!(rt, p);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_analysis() {
    let p = analysis_to_persisted(&sample_analysis());
    let rt = rkyv_roundtrip!(PersistedAnalysis, &p);
    assert_eq!(rt.schema_version, p.schema_version);
    assert_eq!(rt.source_path, p.source_path);
    assert_eq!(rt.title, p.title);
    assert_eq!(rt.content, p.content);
    assert_eq!(rt.category, p.category);
    assert_eq!(rt.word_count, p.word_count);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_analyze_result() {
    let p = analyze_result_to_persisted(&sample_analyze_result());
    let rt = rkyv_roundtrip!(PersistedAnalyzeResult, &p);
    assert_eq!(rt.schema_version, p.schema_version);
    assert_eq!(rt.total_discovered, p.total_discovered);
    assert_eq!(rt.analyses.len(), p.analyses.len());
}

#[test]
fn rkyv_roundtrip_preserves_persisted_transform_result() {
    let r = TransformResult {
        success_count: 3,
        total_count: 5,
        error_count: 2,
        errors: vec![TransformError {
            source_path: "x".to_string(),
            error: "e".to_string(),
        }],
    };
    let p = transform_result_to_persisted(&r);
    let rt = rkyv_roundtrip!(PersistedTransformResult, &p);
    assert_eq!(rt.schema_version, 1);
    assert_eq!(rt.success_count, 3);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_chunk() {
    let p = chunk_to_persisted(&sample_chunk());
    let rt = rkyv_roundtrip!(PersistedChunk, &p);
    assert_eq!(rt.chunk_id, p.chunk_id);
    assert_eq!(rt.token_count, p.token_count);
    assert_eq!(rt.content, p.content);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_chunks_result() {
    let p = chunks_result_to_persisted(&sample_chunks_result());
    let rt = rkyv_roundtrip!(PersistedChunksResult, &p);
    assert_eq!(rt.total_chunks, p.total_chunks);
    assert_eq!(rt.chunks_metadata.len(), p.chunks_metadata.len());
}

#[test]
fn rkyv_roundtrip_preserves_persisted_scraped_page() {
    let p = scraped_page_to_persisted(&sample_scraped_page());
    let rt = rkyv_roundtrip!(PersistedScrapedPage, &p);
    assert_eq!(rt.url, p.url);
    assert_eq!(rt.title, p.title);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_scrape_result() {
    let p = scrape_result_to_persisted(&sample_scrape_result());
    let rt = rkyv_roundtrip!(PersistedScrapeResult, &p);
    assert_eq!(rt.base_url, p.base_url);
    assert_eq!(rt.pages.len(), p.pages.len());
}

#[test]
fn rkyv_roundtrip_preserves_persisted_snapshot() {
    let p = snapshot_to_persisted(&sample_snapshot());
    let rt = rkyv_roundtrip!(PersistedSnapshot, &p);
    assert_eq!(rt.target_url, p.target_url);
    assert_eq!(rt.timestamp_secs, p.timestamp_secs);
    assert_eq!(rt.pages.len(), p.pages.len());
}

#[test]
fn rkyv_roundtrip_preserves_persisted_change_plan() {
    let p = change_plan_to_persisted(&sample_change_plan());
    let rt = rkyv_roundtrip!(PersistedChangePlan, &p);
    assert_eq!(rt.target_url, p.target_url);
    assert_eq!(rt.changes.len(), p.changes.len());
}

#[test]
fn rkyv_roundtrip_preserves_persisted_page_hash() {
    let p = PersistedPageHash {
        url: "https://example.com".to_string(),
        content_hash: [42u8; 32],
        title: "Test".to_string(),
    };
    let rt = rkyv_roundtrip!(PersistedPageHash, &p);
    assert_eq!(rt.url, p.url);
    assert_eq!(rt.content_hash, p.content_hash);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_change_summary() {
    let p = PersistedChangeSummary {
        added: 1,
        removed: 2,
        modified: 3,
        unchanged: 4,
        total_current: 10,
        total_previous: 9,
    };
    let rt = rkyv_roundtrip!(PersistedChangeSummary, &p);
    assert_eq!(rt, p);
}

#[test]
fn rkyv_roundtrip_preserves_persisted_id_mapping() {
    let p = id_mapping_to_persisted("docs/guide.md", &sample_id_mapping());
    let rt = rkyv_roundtrip!(PersistedIdMapping, &p);
    assert_eq!(rt.source_path, p.source_path);
    assert_eq!(rt.id, p.id);
}

// ===========================================================================
// B63: Deterministic Serialization
// ===========================================================================

#[test]
fn rkyv_serialization_is_deterministic_for_persisted_analysis() {
    let p = analysis_to_persisted(&sample_analysis());
    let bytes_a = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    let bytes_b = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    assert_eq!(bytes_a.as_slice(), bytes_b.as_slice());
}

#[test]
fn rkyv_serialization_is_deterministic_for_persisted_snapshot() {
    let p = snapshot_to_persisted(&sample_snapshot());
    let bytes_a = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    let bytes_b = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    assert_eq!(bytes_a.as_slice(), bytes_b.as_slice());
}

// ===========================================================================
// B64-B67: Invalid Archived Bytes
// ===========================================================================

#[test]
fn rkyv_from_bytes_fails_on_truncated_bytes() {
    let p = analysis_to_persisted(&sample_analysis());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    let truncated = &bytes[..bytes.len() / 2];
    let result: Result<PersistedAnalysis, _> =
        rkyv::from_bytes::<PersistedAnalysis, rkyv::rancor::Error>(truncated);
    assert!(result.is_err());
}

#[test]
fn rkyv_from_bytes_fails_on_bit_flipped_bytes() {
    let p = analysis_to_persisted(&sample_analysis());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    let mut corrupted = bytes.to_vec();
    if corrupted.len() > 4 {
        corrupted[4] ^= 0xFF;
    }
    let result: Result<PersistedAnalysis, _> =
        rkyv::from_bytes::<PersistedAnalysis, rkyv::rancor::Error>(&corrupted);
    assert!(result.is_err());
}

#[test]
fn rkyv_from_bytes_fails_on_zeroed_bytes() {
    let p = analysis_to_persisted(&sample_analysis());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&p).unwrap();
    let zeroed = vec![0u8; bytes.len()];
    let result: Result<PersistedAnalysis, _> =
        rkyv::from_bytes::<PersistedAnalysis, rkyv::rancor::Error>(&zeroed);
    assert!(result.is_err());
}

#[test]
fn rkyv_from_bytes_fails_on_random_noise() {
    let noise: Vec<u8> = (0u8..=255)
        .map(|i| i.wrapping_mul(137).wrapping_add(37))
        .collect();
    let result: Result<PersistedAnalysis, _> =
        rkyv::from_bytes::<PersistedAnalysis, rkyv::rancor::Error>(&noise);
    assert!(result.is_err());
}

// ===========================================================================
// B68: Deterministic Frontmatter Ordering
// ===========================================================================

#[test]
fn analysis_to_persisted_sorts_frontmatter_regardless_of_hashmap_order() {
    let mut fm1 = HashMap::new();
    fm1.insert("z-key".to_string(), "val".to_string());
    fm1.insert("a-key".to_string(), "val".to_string());
    fm1.insert("m-key".to_string(), "val".to_string());

    let mut fm2 = HashMap::new();
    fm2.insert("m-key".to_string(), "val".to_string());
    fm2.insert("z-key".to_string(), "val".to_string());
    fm2.insert("a-key".to_string(), "val".to_string());

    let a1 = Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        frontmatter: Some(fm1),
        headings: vec![],
        links: vec![],
        first_paragraph: String::new(),
        word_count: 0,
        has_code: false,
        has_tables: false,
        category: "general".to_string(),
        content: Arc::<str>::from(""),
    };
    let a2 = Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        frontmatter: Some(fm2),
        headings: vec![],
        links: vec![],
        first_paragraph: String::new(),
        word_count: 0,
        has_code: false,
        has_tables: false,
        category: "general".to_string(),
        content: Arc::<str>::from(""),
    };

    let p1 = analysis_to_persisted(&a1);
    let p2 = analysis_to_persisted(&a2);

    let keys1: Vec<&str> = p1
        .frontmatter
        .as_ref()
        .unwrap()
        .iter()
        .map(|(k, _)| k.as_str())
        .collect();
    let keys2: Vec<&str> = p2
        .frontmatter
        .as_ref()
        .unwrap()
        .iter()
        .map(|(k, _)| k.as_str())
        .collect();
    assert_eq!(keys1, keys2);
    assert_eq!(keys1, vec!["a-key", "m-key", "z-key"]);
}

// ===========================================================================
// Full Pipeline Round-Trip Tests (Runtime → Persisted → rkyv → Persisted → Runtime)
// ===========================================================================

#[test]
fn full_roundtrip_preserves_analysis_fields() {
    let original = sample_analysis();
    let persisted = analysis_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedAnalysis =
        rkyv::from_bytes::<PersistedAnalysis, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_analysis_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.source_path, original.source_path);
    assert_eq!(restored.title, original.title);
    assert_eq!(&*restored.content, &*original.content);
    assert_eq!(restored.category, original.category);
    assert_eq!(restored.word_count, original.word_count);
    assert_eq!(restored.has_code, original.has_code);
    assert_eq!(restored.has_tables, original.has_tables);
}

#[test]
fn full_roundtrip_preserves_analyze_result_fields() {
    let original = sample_analyze_result();
    let persisted = analyze_result_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedAnalyzeResult =
        rkyv::from_bytes::<PersistedAnalyzeResult, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_analyze_result_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.total_discovered, original.total_discovered);
    assert_eq!(restored.analyses.len(), original.analyses.len());
}

#[test]
fn full_roundtrip_preserves_transform_result_fields() {
    let original = TransformResult {
        success_count: 3,
        total_count: 5,
        error_count: 2,
        errors: vec![TransformError {
            source_path: "x.md".to_string(),
            error: "fail".to_string(),
        }],
    };
    let persisted = transform_result_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedTransformResult =
        rkyv::from_bytes::<PersistedTransformResult, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_transform_result_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.success_count, original.success_count);
    assert_eq!(restored.total_count, original.total_count);
}

#[test]
fn full_roundtrip_preserves_chunk_fields() {
    let original = sample_chunk();
    let persisted = chunk_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedChunk =
        rkyv::from_bytes::<PersistedChunk, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_chunk_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.chunk_id, original.chunk_id);
    assert_eq!(restored.token_count, original.token_count);
    assert_eq!(restored.content, original.content);
    assert_eq!(restored.chunk_type, original.chunk_type);
}

#[test]
fn full_roundtrip_preserves_chunks_result_fields() {
    let original = sample_chunks_result();
    let persisted = chunks_result_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedChunksResult =
        rkyv::from_bytes::<PersistedChunksResult, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_chunks_result_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.total_chunks, original.total_chunks);
    assert_eq!(
        restored.chunks_metadata.len(),
        original.chunks_metadata.len()
    );
}

#[test]
fn full_roundtrip_preserves_scraped_page_fields() {
    let original = sample_scraped_page();
    let persisted = scraped_page_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedScrapedPage =
        rkyv::from_bytes::<PersistedScrapedPage, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_scraped_page_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.url, original.url);
    assert_eq!(restored.title, original.title);
    assert!((restored.density_score - original.density_score).abs() < f32::EPSILON);
}

#[test]
fn full_roundtrip_preserves_scrape_result_fields() {
    let original = sample_scrape_result();
    let persisted = scrape_result_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedScrapeResult =
        rkyv::from_bytes::<PersistedScrapeResult, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_scrape_result_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.base_url, original.base_url);
    assert_eq!(restored.pages.len(), original.pages.len());
}

#[test]
fn full_roundtrip_preserves_snapshot_fields_with_datetime_lossy() {
    let original = sample_snapshot();
    let persisted = snapshot_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedSnapshot =
        rkyv::from_bytes::<PersistedSnapshot, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_snapshot_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.target_url, original.target_url);
    assert_eq!(
        restored.timestamp.timestamp(),
        original.timestamp.timestamp()
    );
    assert_eq!(restored.pages.len(), original.pages.len());
}

#[test]
fn full_roundtrip_preserves_change_plan_fields_with_datetime_lossy() {
    let original = sample_change_plan();
    let persisted = change_plan_to_persisted(&original);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedChangePlan =
        rkyv::from_bytes::<PersistedChangePlan, rkyv::rancor::Error>(&bytes).unwrap();
    let restored = persisted_change_plan_to_runtime(&restored_persisted).unwrap();

    assert_eq!(restored.target_url, original.target_url);
    assert_eq!(
        restored.timestamp.timestamp(),
        original.timestamp.timestamp()
    );
    assert_eq!(restored.changes.len(), original.changes.len());
    assert_eq!(restored.summary.added, original.summary.added);
}

#[test]
fn full_roundtrip_preserves_id_mapping_fields() {
    let original_mapping = sample_id_mapping();
    let persisted = id_mapping_to_persisted("docs/guide.md", &original_mapping);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted).unwrap();
    let restored_persisted: PersistedIdMapping =
        rkyv::from_bytes::<PersistedIdMapping, rkyv::rancor::Error>(&bytes).unwrap();
    let (source, restored) = persisted_id_mapping_to_runtime(&restored_persisted).unwrap();

    assert_eq!(source, "docs/guide.md");
    assert_eq!(restored.id, original_mapping.id);
    assert_eq!(restored.filename, original_mapping.filename);
    assert_eq!(restored.slug, original_mapping.slug);
}
