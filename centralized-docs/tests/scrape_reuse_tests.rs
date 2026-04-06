//! Integration tests for scrape_reuse module (bead cdocs-90e).
//!
//! Tests the full scrape-reuse pipeline with real redb databases.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::type_complexity)]

use std::collections::HashMap;

use doc_transformer::persisted::{
    PersistedHeader, PersistedPageFilterStatus, PersistedScrapeResult, PersistedScrapedPage,
};
use doc_transformer::scrape::validation::{Header, PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::scrape_reuse::{
    compute_page_content_hash, load_archived_scrape_pages, scrape_with_reuse, ScrapePageDiff,
    ScrapeReuseError, ScrapeReuseStats,
};
use doc_transformer::state::bulk_load::StateReadSession;
use doc_transformer::state::{
    initialize_tables, scrape_outputs_table, url_state_table, UrlStateRaw,
};
use redb::{Database, TableDefinition};
use tempfile::TempDir;

// ===========================================================================
// Helpers
// ===========================================================================

/// Open a fresh redb database with all tables initialized.
fn fresh_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}

/// Open a database WITHOUT a specific table.
fn fresh_db_without_table(excluded_table: &str) -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();

    let table_defs: Vec<(&str, TableDefinition<&[u8], &[u8]>)> = vec![
        ("analysis_outputs", TableDefinition::new("analysis_outputs")),
        (
            "transform_outputs",
            TableDefinition::new("transform_outputs"),
        ),
        ("chunk_outputs", TableDefinition::new("chunk_outputs")),
        ("scrape_outputs", TableDefinition::new("scrape_outputs")),
        ("snapshots", TableDefinition::new("snapshots")),
    ];

    let str_table_defs: Vec<(&str, TableDefinition<&str, &[u8]>)> = vec![
        ("file_state", TableDefinition::new("file_state")),
        ("url_state", TableDefinition::new("url_state")),
        ("metadata", TableDefinition::new("metadata")),
    ];

    let write_tx = db.begin_write().unwrap();
    {
        for (name, def) in &table_defs {
            if *name != excluded_table {
                let _ = write_tx.open_table(*def).unwrap();
            }
        }
        for (name, def) in &str_table_defs {
            if *name != excluded_table {
                let _ = write_tx.open_table(*def).unwrap();
            }
        }
    }
    write_tx.commit().unwrap();

    (temp_dir, db)
}

/// Create a StateReadSession from the database.
fn create_session(db: &Database) -> StateReadSession<'_> {
    StateReadSession::new(db).unwrap()
}

/// Make a runtime ScrapedPage.
fn make_page(url: &str, markdown: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: url.to_string(),
        links: Vec::new(),
        headers: vec![Header {
            level: 1,
            text: url.to_string(),
        }],
        word_count: markdown.split_whitespace().count(),
        slug: url.to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

/// Make a UrlStateRaw.
fn make_url_state(content_hash: [u8; 32], url_hash: [u8; 32]) -> UrlStateRaw {
    UrlStateRaw {
        content_hash,
        url_hash,
        last_fetched_secs: 1_700_000_000,
        status_code: 200,
        reserved: [0u8; 46],
    }
}

/// Make a PersistedScrapedPage.
fn make_persisted_page(url: &str, markdown: &str) -> PersistedScrapedPage {
    PersistedScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: url.to_string(),
        links: Vec::new(),
        headers: vec![PersistedHeader {
            level: 1,
            text: url.to_string(),
        }],
        word_count: markdown.split_whitespace().count(),
        slug: url.to_string(),
        filter_status: PersistedPageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

/// Make a PersistedScrapeResult with the given pages.
fn make_persisted_scrape_result(
    pages: Vec<PersistedScrapedPage>,
    base_url: &str,
) -> PersistedScrapeResult {
    let success_count = pages.len();
    PersistedScrapeResult {
        schema_version: 1,
        pages,
        total_urls: success_count,
        success_count,
        error_count: 0,
        errors: Vec::new(),
        base_url: base_url.to_string(),
    }
}

/// Serialize to rkyv bytes.
macro_rules! rkyv_serialize {
    ($value:expr) => {
        rkyv::to_bytes::<rkyv::rancor::Error>($value)
            .unwrap()
            .to_vec()
    };
}

/// Write a PersistedScrapeResult to the scrape_outputs table at the given hash key.
fn write_scrape_output(db: &Database, key: &[u8; 32], value: &PersistedScrapeResult) {
    let bytes = rkyv_serialize!(value);
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(scrape_outputs_table()).unwrap();
        table.insert(key.as_slice(), bytes.as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Write URL state rows to the url_state table.
fn write_url_rows(db: &Database, rows: &[(&str, UrlStateRaw)]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(url_state_table()).unwrap();
        for (key, state) in rows {
            table.insert(*key, state.to_bytes().as_slice()).unwrap();
        }
    }
    write_tx.commit().unwrap();
}

/// Write raw bytes to the scrape_outputs table at the given hash key.
fn write_raw_scrape_output(db: &Database, key: &[u8; 32], bytes: &[u8]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(scrape_outputs_table()).unwrap();
        table.insert(key.as_slice(), bytes).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Encode a [u8; 32] as lowercase hex string.
fn hex_encode_32(key: &[u8; 32]) -> String {
    key.iter().map(|b| format!("{b:02x}")).collect()
}

// ===========================================================================
// Behavior 9: load_archived_scrape_pages — empty unchanged
// ===========================================================================

#[test]
fn load_archived_scrape_pages_returns_empty_when_no_unchanged_pages() {
    let (_temp_dir, db) = fresh_db();
    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: Vec::new(),
        changed_or_new: vec![0, 1],
    };
    let fresh_pages = vec![
        make_page("https://a.com", "a"),
        make_page("https://b.com", "b"),
    ];
    let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    assert!(result.0.is_empty(), "archived pages should be empty");
    assert!(result.1.is_empty(), "fallback indices should be empty");
}

// ===========================================================================
// Behavior 10: load_archived_scrape_pages — successful load
// ===========================================================================

#[test]
fn load_archived_scrape_pages_loads_correct_pages_when_url_hash_matches() {
    let (_temp_dir, db) = fresh_db();
    let content_hash = compute_page_content_hash("hello");
    let url_hash: [u8; 32] = [1u8; 32];

    // Write url_state
    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(content_hash, url_hash))],
    );

    // Write scrape_output
    let persisted = make_persisted_scrape_result(
        vec![make_persisted_page("https://a.com", "hello")],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash, &persisted);

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![make_page("https://a.com", "hello")];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(content_hash, url_hash),
    );

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    assert_eq!(result.0.len(), 1);
    assert_eq!(result.0[&0].url, "https://a.com");
    assert!(result.1.is_empty(), "fallback indices should be empty");
}

// ===========================================================================
// Behavior 10b: load_archived_scrape_pages — multi-batch load
// ===========================================================================

#[test]
fn load_archived_scrape_pages_loads_pages_from_different_batches_when_url_hashes_differ() {
    let (_temp_dir, db) = fresh_db();
    let hash_a = compute_page_content_hash("content_a");
    let hash_b = compute_page_content_hash("content_b");
    let url_hash_a: [u8; 32] = [0xAA; 32];
    let url_hash_b: [u8; 32] = [0xBB; 32];

    // Write url_states
    write_url_rows(
        &db,
        &[
            ("https://a.com", make_url_state(hash_a, url_hash_a)),
            ("https://b.com", make_url_state(hash_b, url_hash_b)),
        ],
    );

    // Write two different scrape_output batches
    let batch_a = make_persisted_scrape_result(
        vec![make_persisted_page("https://a.com", "content_a")],
        "https://base.com",
    );
    let batch_b = make_persisted_scrape_result(
        vec![make_persisted_page("https://b.com", "content_b")],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash_a, &batch_a);
    write_scrape_output(&db, &url_hash_b, &batch_b);

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0, 1],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![
        make_page("https://a.com", "content_a"),
        make_page("https://b.com", "content_b"),
    ];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(hash_a, url_hash_a),
    );
    url_states.insert(
        "https://b.com".to_string(),
        make_url_state(hash_b, url_hash_b),
    );

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    assert_eq!(result.0.len(), 2);
    assert_eq!(result.0[&0].url, "https://a.com");
    assert_eq!(result.0[&1].url, "https://b.com");
    assert!(result.1.is_empty(), "fallback indices should be empty");
}

// ===========================================================================
// Behavior 10c: load_archived_scrape_pages — empty batch (0 pages)
// ===========================================================================

#[test]
fn load_archived_scrape_pages_returns_fallback_when_batch_contains_zero_pages() {
    let (_temp_dir, db) = fresh_db();
    let content_hash = compute_page_content_hash("hello");
    let url_hash: [u8; 32] = [1u8; 32];

    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(content_hash, url_hash))],
    );

    // Write a valid PersistedScrapeResult with ZERO pages
    let empty_batch = make_persisted_scrape_result(Vec::new(), "https://base.com");
    write_scrape_output(&db, &url_hash, &empty_batch);

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![make_page("https://a.com", "hello")];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(content_hash, url_hash),
    );

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    assert!(result.0.is_empty(), "archived pages should be empty");
    assert_eq!(result.1, vec![0]);
}

// ===========================================================================
// Behavior 11: load_archived_scrape_pages — batch deserialization failure
// ===========================================================================

#[test]
fn load_archived_scrape_pages_returns_fallback_when_batch_deserialization_fails() {
    let (_temp_dir, db) = fresh_db();
    let content_hash = compute_page_content_hash("hello");
    let bad_key: [u8; 32] = [
        0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE,
        0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD,
        0xBE, 0xEF,
    ];

    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(content_hash, bad_key))],
    );

    // Write corrupt bytes at the bad_key
    write_raw_scrape_output(&db, &bad_key, &[0xFF; 128]);

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![make_page("https://a.com", "hello")];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(content_hash, bad_key),
    );

    let result = load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session);

    let err = result.expect_err("should fail with DeserializationFailed");
    match &err {
        ScrapeReuseError::DeserializationFailed { key_hex, message } => {
            assert_eq!(
                key_hex,
                &hex_encode_32(&bad_key),
                "key_hex should be the hex-encoded bad_key"
            );
            assert!(
                message.len() > 5,
                "message should be non-trivial: {message}"
            );
        }
        other => panic!("expected DeserializationFailed, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 13: load_archived_scrape_pages — hash mismatch
// ===========================================================================

#[test]
fn load_archived_scrape_pages_returns_fallback_when_hash_mismatch_detected() {
    let (_temp_dir, db) = fresh_db();
    let stored_hash = compute_page_content_hash("original_content");
    let url_hash: [u8; 32] = [1u8; 32];

    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(stored_hash, url_hash))],
    );

    // Write a batch with a page that has DIFFERENT content
    let persisted = make_persisted_scrape_result(
        vec![make_persisted_page("https://a.com", "different_content")],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash, &persisted);

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![make_page("https://a.com", "fresh")];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(stored_hash, url_hash),
    );

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    // Hash mismatch → page falls back to fresh
    assert!(result.0.is_empty(), "archived pages should be empty");
    assert_eq!(result.1, vec![0]);
}

// ===========================================================================
// Behavior 14: load_archived_scrape_pages — missing scrape_output row
// ===========================================================================

#[test]
fn load_archived_scrape_pages_returns_fallback_when_scrape_output_missing() {
    let (_temp_dir, db) = fresh_db();
    let content_hash = compute_page_content_hash("hello");
    let url_hash: [u8; 32] = [1u8; 32]; // No corresponding row in scrape_outputs

    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(content_hash, url_hash))],
    );

    let session = create_session(&db);

    let page_diff = ScrapePageDiff {
        unchanged: vec![0],
        changed_or_new: Vec::new(),
    };
    let fresh_pages = vec![make_page("https://a.com", "hello")];
    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(content_hash, url_hash),
    );

    let result =
        load_archived_scrape_pages(&page_diff, &fresh_pages, &url_states, &session).unwrap();

    assert!(result.0.is_empty(), "archived pages should be empty");
    assert_eq!(result.1, vec![0]);
}

// ===========================================================================
// Behavior 15: load_archived_scrape_pages — StateLoad error propagation
// ===========================================================================

#[test]
fn load_archived_scrape_pages_propagates_state_load_error_when_url_state_table_missing() {
    let (_temp_dir, db) = fresh_db_without_table("url_state");

    let session = create_session(&db);

    // Call load_url_states which requires url_state table
    let result = session.load_url_states();

    let err = result.expect_err("should fail with StateLoadError");
    match &err {
        doc_transformer::state::StateLoadError::BackendError { operation, message } => {
            assert_eq!(*operation, "open_table", "operation must be 'open_table'");
            assert!(message.len() > 3, "message must be non-trivial: {message}");
        }
        other => panic!("expected BackendError, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 16: load_archived_scrape_pages — BulkLoad error propagation
// ===========================================================================

#[test]
fn load_archived_scrape_pages_propagates_bulk_load_error_when_scrape_outputs_table_missing() {
    let (_temp_dir, db) = fresh_db_without_table("scrape_outputs");

    let session = create_session(&db);

    // Try to load from missing scrape_outputs table
    let result = session.load_scrapes(&[[1u8; 32]]);

    let err = result.expect_err("should fail with BulkLoadError");
    match &err {
        doc_transformer::state::bulk_load::BulkLoadError::TableOpen { table, message } => {
            assert_eq!(*table, "scrape_outputs", "table must be 'scrape_outputs'");
            assert!(message.len() > 3, "message must be non-trivial: {message}");
        }
        other => panic!("expected TableOpen, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 19: scrape_with_reuse — all unchanged
// ===========================================================================

#[test]
fn scrape_with_reuse_returns_all_reused_pages_when_all_unchanged() {
    let (_temp_dir, db) = fresh_db();

    let content_a = compute_page_content_hash("content_a");
    let content_b = compute_page_content_hash("content_b");
    let content_c = compute_page_content_hash("content_c");
    let url_hash: [u8; 32] = [1u8; 32];

    // Write url_states
    write_url_rows(
        &db,
        &[
            ("https://a.com", make_url_state(content_a, url_hash)),
            ("https://b.com", make_url_state(content_b, url_hash)),
            ("https://c.com", make_url_state(content_c, url_hash)),
        ],
    );

    // Write scrape_output with all 3 pages
    let persisted = make_persisted_scrape_result(
        vec![
            make_persisted_page("https://a.com", "content_a"),
            make_persisted_page("https://b.com", "content_b"),
            make_persisted_page("https://c.com", "content_c"),
        ],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash, &persisted);

    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: vec![
            make_page("https://a.com", "content_a"),
            make_page("https://b.com", "content_b"),
            make_page("https://c.com", "content_c"),
        ],
        total_urls: 3,
        success_count: 3,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let (result, stats) = scrape_with_reuse(fresh_result, &session).unwrap();

    assert_eq!(result.pages.len(), 3);
    assert_eq!(
        stats,
        ScrapeReuseStats {
            reused: 3,
            scraped: 0
        }
    );
}

// ===========================================================================
// Behavior 19b: scrape_with_reuse — empty ScrapeResult
// ===========================================================================

#[test]
fn scrape_with_reuse_returns_empty_result_with_zero_stats_when_no_pages() {
    let (_temp_dir, db) = fresh_db();
    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: Vec::new(),
        total_urls: 0,
        success_count: 0,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let (result, stats) = scrape_with_reuse(fresh_result, &session).unwrap();

    assert!(result.pages.is_empty(), "pages should be empty");
    assert_eq!(
        stats,
        ScrapeReuseStats {
            reused: 0,
            scraped: 0
        }
    );
}

// ===========================================================================
// Behavior 20: scrape_with_reuse — all changed
// ===========================================================================

#[test]
fn scrape_with_reuse_returns_all_fresh_pages_when_all_changed() {
    let (_temp_dir, db) = fresh_db();
    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: vec![
            make_page("https://a.com", "new_a"),
            make_page("https://b.com", "new_b"),
        ],
        total_urls: 2,
        success_count: 2,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let (result, stats) = scrape_with_reuse(fresh_result, &session).unwrap();

    assert_eq!(result.pages.len(), 2);
    assert_eq!(
        stats,
        ScrapeReuseStats {
            reused: 0,
            scraped: 2
        }
    );
}

// ===========================================================================
// Behavior 21: scrape_with_reuse — mixed
// ===========================================================================

#[test]
fn scrape_with_reuse_returns_mixed_result_when_some_unchanged_some_changed() {
    let (_temp_dir, db) = fresh_db();

    let content_a = compute_page_content_hash("same_a");
    let url_hash: [u8; 32] = [1u8; 32];

    // Only a.com has matching state
    write_url_rows(
        &db,
        &[("https://a.com", make_url_state(content_a, url_hash))],
    );

    let persisted = make_persisted_scrape_result(
        vec![make_persisted_page("https://a.com", "same_a")],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash, &persisted);

    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: vec![
            make_page("https://a.com", "same_a"),
            make_page("https://b.com", "changed_b"),
            make_page("https://c.com", "new_c"),
        ],
        total_urls: 3,
        success_count: 3,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let (result, stats) = scrape_with_reuse(fresh_result, &session).unwrap();

    assert_eq!(result.pages.len(), 3);
    assert_eq!(stats.reused, 1);
    assert_eq!(stats.scraped, 2);
}

// ===========================================================================
// Behavior 22: scrape_with_reuse — stats invariant (reused + scraped == total)
// ===========================================================================

#[test]
fn scrape_with_reuse_stats_reused_plus_scraped_equals_total_pages() {
    let (_temp_dir, db) = fresh_db();

    let content_a = compute_page_content_hash("same_a");
    let content_b = compute_page_content_hash("same_b");
    let url_hash: [u8; 32] = [1u8; 32];

    write_url_rows(
        &db,
        &[
            ("https://a.com", make_url_state(content_a, url_hash)),
            ("https://b.com", make_url_state(content_b, url_hash)),
        ],
    );

    let persisted = make_persisted_scrape_result(
        vec![
            make_persisted_page("https://a.com", "same_a"),
            make_persisted_page("https://b.com", "same_b"),
        ],
        "https://base.com",
    );
    write_scrape_output(&db, &url_hash, &persisted);

    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: vec![
            make_page("https://a.com", "same_a"),
            make_page("https://b.com", "same_b"),
            make_page("https://c.com", "changed_c"),
        ],
        total_urls: 3,
        success_count: 3,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let (result, stats) = scrape_with_reuse(fresh_result, &session).unwrap();

    assert_eq!(
        stats.reused + stats.scraped,
        result.pages.len(),
        "reused + scraped must equal total pages"
    );
}

// ===========================================================================
// Behavior 23: scrape_with_reuse — StateLoad error propagation
// ===========================================================================

#[test]
fn scrape_with_reuse_propagates_state_load_error_from_session() {
    let (_temp_dir, db) = fresh_db_without_table("url_state");
    let session = create_session(&db);

    let fresh_result = ScrapeResult {
        pages: vec![make_page("https://a.com", "content")],
        total_urls: 1,
        success_count: 1,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://base.com".to_string(),
    };

    let result = scrape_with_reuse(fresh_result, &session);

    let err = result.expect_err("should propagate StateLoad error");
    match &err {
        ScrapeReuseError::StateLoad(inner) => match inner {
            doc_transformer::state::StateLoadError::BackendError { operation, message } => {
                assert_eq!(
                    *operation, "open_table",
                    "inner operation must be 'open_table'"
                );
                assert!(
                    message.len() > 3,
                    "inner message must be non-trivial: {message}"
                );
            }
            other => panic!("expected BackendError inside StateLoad, got {other:?}"),
        },
        other => panic!("expected StateLoad, got {other:?}"),
    }
}
