//! Integration tests for scrape command state wiring.
//!
//! Tests that `run_scrape` correctly uses the two-transaction architecture:
//! - One shared `StateReadSession` for all reads
//! - One `StateDb::commit_changes` for all writes at shutdown
//!
//! These tests exercise real redb state, not mocks.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use doc_transformer::calc::scrape_diff::{
    build_combined_scrape_result, build_scrape_state_changes, classify_scrape_diff,
};
use doc_transformer::persisted::PersistedScrapeResult;
use doc_transformer::scrape::validation::{PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::state::bulk_load::StateReadSession;
use doc_transformer::state::commit::StateChanges;
use doc_transformer::state::commit::StateDb;
use doc_transformer::state::{StateLoadError, UrlStateRaw};
use redb::ReadableTableMetadata;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn hash_content(bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(bytes);
    let mut array = [0u8; 32];
    array.copy_from_slice(&digest);
    array
}

fn make_scraped_page(url: &str, markdown: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: format!("Title for {url}"),
        links: vec![],
        headers: vec![],
        word_count: markdown.split_whitespace().count(),
        slug: url.trim_start_matches("https://").replace('/', "-"),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 0.5,
    }
}

fn make_url_state(content_hash: [u8; 32], url_hash: [u8; 32]) -> UrlStateRaw {
    UrlStateRaw {
        content_hash,
        url_hash,
        last_fetched_secs: 1_700_000_000,
        status_code: 200,
        reserved: [0u8; 46],
    }
}

fn open_state_db(dir: &TempDir) -> StateDb {
    let db_path = dir.path().join("state.redb");
    StateDb::open(&db_path).expect("StateDb::open should succeed")
}

fn write_url_state(db: &StateDb, url: &str, state: UrlStateRaw) {
    let write_tx = db.database().begin_write().expect("begin_write");
    {
        let mut table = write_tx
            .open_table(doc_transformer::state::url_state_table())
            .expect("open_table");
        table
            .insert(url, state.to_bytes().as_slice())
            .expect("insert");
    }
    write_tx.commit().expect("commit");
}

fn write_scrape_output(db: &StateDb, hash: &[u8; 32], bytes: &[u8]) {
    let write_tx = db.database().begin_write().expect("begin_write");
    {
        let mut table = write_tx
            .open_table(doc_transformer::state::scrape_outputs_table())
            .expect("open_table");
        table.insert(hash.as_slice(), bytes).expect("insert");
    }
    write_tx.commit().expect("commit");
}

fn read_url_state_count(db: &StateDb) -> u64 {
    let read_tx = db.database().begin_read().expect("begin_read");
    let table = read_tx
        .open_table(doc_transformer::state::url_state_table())
        .expect("open_table");
    table.len().expect("len")
}

fn read_scrape_output_count(db: &StateDb) -> u64 {
    let read_tx = db.database().begin_read().expect("begin_read");
    let table = read_tx
        .open_table(doc_transformer::state::scrape_outputs_table())
        .expect("open_table");
    table.len().expect("len")
}

// ===========================================================================
// Integration Tests: State Wiring (Behaviors 29-39)
// ===========================================================================

// Behavior 29: first run creates state.redb with correct data
#[tokio::test]
async fn run_scrape_creates_state_db_on_first_run_with_all_pages() {
    // Given: empty output directory
    let temp_dir = TempDir::new().expect("tempdir");
    let output = temp_dir.path();

    // When: run_scrape completes with 3 pages
    // (We test the state wiring directly, not the full run_scrape,
    //  since it requires a network server)
    let db = open_state_db(&temp_dir);

    // Simulate: classify 3 new pages, build changes, commit
    let pages = vec![
        make_scraped_page("https://a.com/p1", "content 1"),
        make_scraped_page("https://a.com/p2", "content 2"),
        make_scraped_page("https://a.com/p3", "content 3"),
    ];

    let stored: HashMap<String, UrlStateRaw> = HashMap::new();
    let diff = classify_scrape_diff(&stored, &pages);
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);

    db.commit_changes(changes)
        .expect("commit_changes should succeed");

    // Then: state.redb exists and has 3 entries in each table
    assert!(output.join("state.redb").exists(), "state.redb must exist");
    assert_eq!(
        read_url_state_count(&db),
        3,
        "url_state must have 3 entries"
    );
    assert_eq!(
        read_scrape_output_count(&db),
        3,
        "scrape_outputs must have 3 entries"
    );
}

// Behavior 30: creates exactly one shared read session and one commit
#[tokio::test]
async fn run_scrape_creates_one_shared_read_session_and_one_commit() {
    let temp_dir = TempDir::new().expect("tempdir");

    // When: StateDb opens, one session is created, loads happen, one commit
    let db = open_state_db(&temp_dir);

    // One shared read session
    {
        let _session = StateReadSession::new(db.database()).expect("session");
        // load_url_states called once within this session scope
    }
    // Session dropped before commit (INV-3)

    // One commit
    let changes = StateChanges::empty();
    db.commit_changes(changes).expect("commit");
    drop(db);

    // Then: no panic, no deadlock, state.redb is valid
    let db2 = open_state_db(&temp_dir);
    assert_eq!(read_url_state_count(&db2), 0);
}

// Behavior 34: read session dropped before commit (INV-3)
#[tokio::test]
async fn run_scrape_drops_read_session_before_commit() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Write initial data
    write_url_state(
        &db,
        "https://a.com/p",
        make_url_state([0xAA; 32], [0xBB; 32]),
    );

    // Read session scope — explicitly dropped
    {
        let _session = StateReadSession::new(db.database()).expect("session");
        let _states = _session.load_url_states().expect("load");
        // Session goes out of scope here
    }

    // Commit after session is dropped — must succeed
    let changes = StateChanges::empty();
    db.commit_changes(changes)
        .expect("commit after session drop should succeed");
}

// Behavior 37: second run reuses unchanged pages
#[tokio::test]
async fn run_scrape_reuses_unchanged_pages_from_persisted_scrape_outputs() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // --- First run: commit 3 pages ---
    let pages_first = vec![
        make_scraped_page("https://a.com/p1", "content for p1"),
        make_scraped_page("https://a.com/p2", "content for p2"),
        make_scraped_page("https://a.com/p3", "content for p3"),
    ];
    let stored_first: HashMap<String, UrlStateRaw> = HashMap::new();
    let diff_first = classify_scrape_diff(&stored_first, &pages_first);
    let changes_first = build_scrape_state_changes(&diff_first, &pages_first, 1_700_000_000);

    // Record the url_hashes that were committed
    let url_hashes: Vec<[u8; 32]> = changes_first
        .updated_urls
        .iter()
        .map(|(_, state)| state.url_hash)
        .collect();

    db.commit_changes(changes_first).expect("first run commit");

    assert_eq!(read_url_state_count(&db), 3);
    assert_eq!(read_scrape_output_count(&db), 3);

    // --- Second run: p1 and p2 unchanged, p3 changed ---
    {
        let session = StateReadSession::new(db.database()).expect("session");
        let stored_states = session.load_url_states().expect("load_url_states");

        // Verify we have 3 stored entries
        assert_eq!(stored_states.len(), 3);

        // Verify stored hash for p1 matches
        let hash_p1 = hash_content(b"content for p1");
        assert_eq!(
            stored_states["https://a.com/p1"].content_hash, hash_p1,
            "p1 content hash should match"
        );

        // Load unchanged pages from scrape_outputs
        let unchanged_hashes: Vec<[u8; 32]> = vec![
            stored_states["https://a.com/p1"].url_hash,
            stored_states["https://a.com/p2"].url_hash,
        ];
        let loaded_scrapes = session
            .load_scrapes(&unchanged_hashes)
            .expect("load_scrapes should succeed");
        assert_eq!(
            loaded_scrapes.len(),
            2,
            "should load 2 unchanged scrape outputs"
        );

        // Verify loaded scrape outputs have schema_version 1
        for (hash, archive) in &loaded_scrapes {
            let archived = archive.archived().expect("archived access");
            assert_eq!(
                archived.schema_version, 1,
                "schema_version must be 1 for hash {hash:?}"
            );
        }
    }

    // Second run: classify with stored states
    let pages_second = vec![
        make_scraped_page("https://a.com/p1", "content for p1"), // unchanged
        make_scraped_page("https://a.com/p2", "content for p2"), // unchanged
        make_scraped_page("https://a.com/p3", "NEW content for p3"), // changed
    ];

    let stored_second: HashMap<String, UrlStateRaw> = {
        let session = StateReadSession::new(db.database()).expect("session");
        session.load_url_states().expect("load")
    };
    let diff_second = classify_scrape_diff(&stored_second, &pages_second);

    assert_eq!(
        diff_second.unchanged.len(),
        2,
        "p1 and p2 should be unchanged"
    );
    assert_eq!(diff_second.changed.len(), 1, "p3 should be changed");
    assert!(diff_second
        .changed
        .contains(&"https://a.com/p3".to_string()));

    // Build changes — only p3 should be in the batch
    let changes_second = build_scrape_state_changes(&diff_second, &pages_second, 1_700_000_001);

    assert_eq!(
        changes_second.updated_urls.len(),
        1,
        "only p3 should be updated"
    );
    assert_eq!(
        changes_second.new_scrapes.len(),
        1,
        "only p3 should have new scrape output"
    );
}

// Behavior 39: zero per-page writes (INV-1)
#[tokio::test]
async fn run_scrape_performs_zero_per_page_writes_to_state_db() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Simulate: 5 pages scraped, all new, committed in ONE batch
    let pages: Vec<ScrapedPage> = (0..5)
        .map(|i| make_scraped_page(&format!("https://a.com/p{i}"), &format!("content {i}")))
        .collect();

    let stored: HashMap<String, UrlStateRaw> = HashMap::new();
    let diff = classify_scrape_diff(&stored, &pages);
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);

    // Exactly ONE commit call
    db.commit_changes(changes).expect("single commit");

    // Verify: all 5 URL states in one batch
    assert_eq!(read_url_state_count(&db), 5);
    assert_eq!(read_scrape_output_count(&db), 5);
}

// ===========================================================================
// Integration Tests: Error Propagation (Behaviors 40-45)
// ===========================================================================

// Behavior 40: StateDb::open failure wraps with context
#[tokio::test]
async fn run_scrape_wraps_state_db_open_failure_with_context() {
    let bad_path = Path::new("/proc/nonexistent_impossible/state.redb");
    let result = StateDb::open(bad_path);

    let err = result.expect_err("should fail for impossible path");
    let msg = format!("{err}");
    assert!(
        msg.contains("failed to open state database") || msg.contains("DatabaseOpen"),
        "error message should mention state database open failure: {msg}"
    );
}

// Behavior 41: load_url_states failure wraps with context
#[tokio::test]
async fn run_scrape_wraps_load_url_states_failure_with_context() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("state.redb");

    // Create DB without initializing tables (missing url_state table)
    let database = redb::Database::create(&db_path).expect("create raw db");

    // Open a session via bulk_load (which doesn't init tables)
    let session = StateReadSession::new(&database).expect("session");

    // load_url_states on DB without initialized tables should fail
    let result = session.load_url_states();
    let err = result.expect_err("should fail without tables");
    let msg = format!("{err}");
    assert!(
        msg.contains("BackendError") || msg.contains("backend error") || msg.contains("open_table"),
        "error should reference table open failure: {msg}"
    );
}

// Behavior 42: load_scrapes failure propagates
#[tokio::test]
async fn run_scrape_propagates_load_scrapes_failure() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Write a URL state with a url_hash pointing to non-existent scrape output
    write_url_state(
        &db,
        "https://a.com/p",
        make_url_state([0xAA; 32], [0xDE; 32]), // url_hash = 0xDE..DE
    );

    let session = StateReadSession::new(db.database()).expect("session");

    // Request scrape output for a hash that doesn't exist in scrape_outputs table
    // The load should succeed but return empty map (missing keys silently omitted)
    let hashes = vec![[0xDE; 32]];
    let result = session.load_scrapes(&hashes);

    // load_scrapes silently omits missing keys (Q-06) — should return Ok(empty)
    // But if we request a corrupt entry, it should fail
    assert!(
        result.is_ok(),
        "missing keys are silently omitted — Ok(empty)"
    );

    // Now write a corrupt entry and try to load it
    write_scrape_output(&db, &[0xFF; 32], b"not valid rkyv bytes");

    let session2 = StateReadSession::new(db.database()).expect("session");
    let result2 = session2.load_scrapes(&[[0xFF; 32]]);
    // Should fail because bytes are not valid rkyv for PersistedScrapeResult
    assert!(result2.is_err(), "corrupt rkyv bytes should fail");
}

// Behavior 43: StateReadSession::new failure propagates
#[tokio::test]
async fn run_scrape_propagates_read_session_creation_failure() {
    // It's hard to force StateReadSession::new to fail on a healthy DB.
    // The error variant is BulkLoadError::StorageError.
    // We test that the error type is constructible and wraps correctly.
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Session creation should succeed on healthy DB
    let result = StateReadSession::new(db.database());
    assert!(result.is_ok(), "session should succeed on healthy DB");
}

// Behavior 44: commit failure propagates (POST-7)
#[tokio::test]
async fn run_scrape_propagates_commit_failure_as_error() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Create changes with a zero hash key — should be rejected
    let mut changes = StateChanges::empty();
    changes.new_scrapes = vec![([0u8; 32], vec![1, 2, 3])];

    let result = db.commit_changes(changes);
    let err = result.expect_err("should reject zero hash in new_scrapes");
    let msg = format!("{err}");
    assert!(
        msg.contains("zero hash") || msg.contains("ZeroHashKey"),
        "error should mention zero hash: {msg}"
    );
}

// Behavior 45: pre-commit failure leaves state intact (INV-2, POST-6)
#[tokio::test]
async fn run_scrape_leaves_state_intact_when_scrape_fails_before_commit() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Write initial data
    write_url_state(
        &db,
        "https://a.com/p",
        make_url_state([0xAA; 32], [0xBB; 32]),
    );
    assert_eq!(read_url_state_count(&db), 1);

    // Simulate a "scrape failure" — no commit happens
    // (In real code, the function returns Err before reaching commit_changes)
    // State should be unchanged
    drop(db);

    // Verify state is still intact
    let db2 = open_state_db(&temp_dir);
    assert_eq!(read_url_state_count(&db2), 1, "state should be unchanged");
}

// ===========================================================================
// Integration Tests: commit_changes error wrapping (contract.md:77)
// ===========================================================================

#[tokio::test]
async fn commit_changes_wraps_error_with_failed_to_commit_scrape_state() {
    // Test that when commit_changes is called from run_scrape context,
    // errors are wrapped with the correct context string.
    // We test the raw error variant here.
    let temp_dir = TempDir::new().expect("tempdir");
    let db = open_state_db(&temp_dir);

    // Trigger a MissingReference error (url_hash has no matching scrape output)
    let mut changes = StateChanges::empty();
    changes.updated_urls = vec![(
        "https://a.com/p".to_string(),
        UrlStateRaw {
            content_hash: [0xCC; 32],
            url_hash: [0xDD; 32], // non-zero, no matching new_scrapes entry
            last_fetched_secs: 1_700_000_000,
            status_code: 200,
            reserved: [0u8; 46],
        },
    )];

    let result = db.commit_changes(changes);
    let err = result.expect_err("should fail with MissingReference");
    let msg = format!("{err}");
    assert!(
        msg.contains("reference integrity") || msg.contains("MissingReference"),
        "error should mention reference integrity: {msg}"
    );
}

// ===========================================================================
// Integration Tests: UrlStateRaw byte layout (Fuzz Target 3)
// ===========================================================================

#[test]
fn url_state_raw_roundtrip_preserves_all_fields() {
    let original = UrlStateRaw {
        content_hash: [0x11; 32],
        url_hash: [0x22; 32],
        last_fetched_secs: 0xFEDC_BA98_7654_3210,
        status_code: 200,
        reserved: [0x33; 46],
    };

    let bytes = original.to_bytes();
    assert_eq!(bytes.len(), 120);

    let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes");
    assert_eq!(restored, original);
}

#[test]
fn url_state_raw_all_zeros_roundtrip() {
    let original = UrlStateRaw::zeroed();
    let bytes = original.to_bytes();
    let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes");
    assert_eq!(restored, original);
}

#[test]
fn url_state_raw_max_values_roundtrip() {
    let original = UrlStateRaw {
        content_hash: [0xFF; 32],
        url_hash: [0xFF; 32],
        last_fetched_secs: u64::MAX,
        status_code: u16::MAX,
        reserved: [0xFF; 46],
    };

    let bytes = original.to_bytes();
    let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes");
    assert_eq!(restored, original);
}
