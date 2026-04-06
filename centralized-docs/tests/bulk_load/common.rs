//! Shared test fixtures and helpers for bulk_load integration tests.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::type_complexity)]

use doc_transformer::persisted::{
    PersistedAnalysis, PersistedAnalyzeResult, PersistedChunk, PersistedChunkLevel,
    PersistedChunkType, PersistedChunksResult, PersistedHeader, PersistedPageFilterStatus,
    PersistedScrapeResult, PersistedScrapedPage, PersistedTransformResult,
};
use doc_transformer::state::bulk_load::StateReadSession;
use doc_transformer::state::{
    analysis_outputs_table, chunk_outputs_table, initialize_tables, scrape_outputs_table,
    transform_outputs_table,
};
use redb::{Database, TableDefinition};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// rkyv serialization helper (macro avoids complex trait bound paths)
// ---------------------------------------------------------------------------

/// Serialize a value to rkyv bytes (Vec<u8>).
/// Uses a macro to avoid exposing private rkyv type paths in a where clause.
macro_rules! rkyv_serialize {
    ($value:expr) => {
        rkyv::to_bytes::<rkyv::rancor::Error>($value)
            .unwrap()
            .to_vec()
    };
}

pub(crate) use rkyv_serialize;

// ---------------------------------------------------------------------------
// Hex encoding (mirrors the one in bulk_load.rs)
// ---------------------------------------------------------------------------

/// Encode a [u8; 32] as lowercase hex string.
pub fn hex_encode_32(key: &[u8; 32]) -> String {
    key.iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------------------
// Database fixtures
// ---------------------------------------------------------------------------

/// Open a fresh redb database with all tables initialized.
pub fn open_db_with_tables() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}

/// Open a database and selectively create tables, excluding `excluded_table`.
pub fn open_db_without_table(excluded_table: &str) -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();

    let rkyv_defs: Vec<(&str, TableDefinition<&[u8], &[u8]>)> = vec![
        ("analysis_outputs", TableDefinition::new("analysis_outputs")),
        (
            "transform_outputs",
            TableDefinition::new("transform_outputs"),
        ),
        ("chunk_outputs", TableDefinition::new("chunk_outputs")),
        ("scrape_outputs", TableDefinition::new("scrape_outputs")),
        ("snapshots", TableDefinition::new("snapshots")),
        ("file_state", TableDefinition::new("file_state")),
        ("url_state", TableDefinition::new("url_state")),
        ("metadata", TableDefinition::new("metadata")),
    ];

    let write_tx = db.begin_write().unwrap();
    {
        for (name, def) in &rkyv_defs {
            if *name != excluded_table {
                let _ = write_tx.open_table(*def).unwrap();
            }
        }
    }
    write_tx.commit().unwrap();

    (temp_dir, db)
}

/// Create a StateReadSession from the database.
pub fn create_session(db: &Database) -> StateReadSession<'_> {
    StateReadSession::new(db).unwrap()
}

// ---------------------------------------------------------------------------
// Data insert helpers
// ---------------------------------------------------------------------------

/// Insert rkyv-serialized analysis data at the given hash key.
pub fn insert_analysis(db: &Database, hash: &[u8; 32], value: &PersistedAnalyzeResult) {
    let bytes = rkyv_serialize!(value);
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(analysis_outputs_table()).unwrap();
        table.insert(hash.as_slice(), bytes.as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Insert rkyv-serialized transform data at the given hash key.
pub fn insert_transform(db: &Database, hash: &[u8; 32], value: &PersistedTransformResult) {
    let bytes = rkyv_serialize!(value);
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(transform_outputs_table()).unwrap();
        table.insert(hash.as_slice(), bytes.as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Insert rkyv-serialized chunks data at the given hash key.
pub fn insert_chunks(db: &Database, hash: &[u8; 32], value: &PersistedChunksResult) {
    let bytes = rkyv_serialize!(value);
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(chunk_outputs_table()).unwrap();
        table.insert(hash.as_slice(), bytes.as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Insert rkyv-serialized scrape data at the given hash key.
pub fn insert_scrape(db: &Database, hash: &[u8; 32], value: &PersistedScrapeResult) {
    let bytes = rkyv_serialize!(value);
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(scrape_outputs_table()).unwrap();
        table.insert(hash.as_slice(), bytes.as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Insert raw garbage bytes at the given hash key in the specified table.
pub fn insert_garbage(
    db: &Database,
    table_def: TableDefinition<'static, &'static [u8], &'static [u8]>,
    hash: &[u8; 32],
) {
    let garbage: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF];
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(table_def).unwrap();
        table.insert(hash.as_slice(), garbage).unwrap();
    }
    write_tx.commit().unwrap();
}

// ---------------------------------------------------------------------------
// Sample data factories
// ---------------------------------------------------------------------------

/// Sample PersistedAnalyzeResult with configurable source_path and word_count.
pub fn sample_analysis(source_path: &str, word_count: usize) -> PersistedAnalyzeResult {
    PersistedAnalyzeResult {
        schema_version: 1,
        analyses: vec![PersistedAnalysis {
            schema_version: 1,
            source_path: source_path.to_string(),
            title: "Test".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "fp".to_string(),
            word_count,
            has_code: false,
            has_tables: false,
            category: "cat".to_string(),
            content: "body".to_string(),
        }],
        failed_files: vec![],
        total_discovered: 1,
    }
}

/// Sample PersistedTransformResult with configurable counts.
pub fn sample_transform_result(success: usize, total: usize) -> PersistedTransformResult {
    PersistedTransformResult {
        schema_version: 1,
        success_count: success,
        total_count: total,
        error_count: 0,
        errors: vec![],
    }
}

/// Sample PersistedChunk with configurable chunk_index.
pub fn sample_chunk(doc_id: &str, chunk_index: usize) -> PersistedChunk {
    PersistedChunk {
        schema_version: 1,
        chunk_id: format!("{doc_id}#{chunk_index}"),
        doc_id: doc_id.to_string(),
        doc_title: "Test Doc".to_string(),
        chunk_index,
        content: format!("chunk content {chunk_index}"),
        token_count: 10,
        heading: None,
        heading_path: vec![],
        chunk_type: PersistedChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: format!("summary {chunk_index}"),
        chunk_level: PersistedChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

/// Sample PersistedChunksResult with `n` chunks for a single doc.
pub fn sample_chunks_result(n: usize) -> PersistedChunksResult {
    let chunks = (0..n).map(|i| sample_chunk("doc1", i)).collect();
    PersistedChunksResult {
        schema_version: 1,
        total_chunks: n,
        document_count: 1,
        chunks_metadata: chunks,
        summary_chunks: 0,
        standard_chunks: n,
        detailed_chunks: 0,
    }
}

/// Sample PersistedScrapedPage.
pub fn sample_scraped_page(url: &str) -> PersistedScrapedPage {
    PersistedScrapedPage {
        url: url.to_string(),
        markdown: "content".to_string(),
        title: url.to_string(),
        links: vec![],
        headers: vec![PersistedHeader {
            level: 1,
            text: url.to_string(),
        }],
        word_count: 1,
        slug: url.to_string(),
        filter_status: PersistedPageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

/// Sample PersistedScrapeResult with one page.
pub fn sample_scrape_result(url: &str) -> PersistedScrapeResult {
    PersistedScrapeResult {
        schema_version: 1,
        pages: vec![sample_scraped_page(url)],
        total_urls: 1,
        success_count: 1,
        error_count: 0,
        errors: vec![],
        base_url: url.to_string(),
    }
}

/// Generate a deterministic hash from a seed byte.
pub fn hash_from_byte(b: u8) -> [u8; 32] {
    [b; 32]
}
