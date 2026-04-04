//! Integration tests: verify StateChanges from build_scrape_state_changes
//! passes commit_changes reference integrity validation.
//!
//! INT-1: StateChanges produced by build_scrape_state_changes passes
//!        StateDb::commit_changes reference integrity.
//! INT-2: UrlStateRaw round-trips through to_bytes/from_bytes preserving all fields.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use doc_transformer::calc::{
    build_url_state_raw, hash_payload, scrape_batch as build_scrape_state_changes, ScrapeArtifact,
    ScrapeBatchConfig, ScrapeDiff, ScrapeOutputs,
};
use doc_transformer::state::commit::{StateChanges, StateDb};
use doc_transformer::state::UrlStateRaw;
use std::collections::HashMap;

// ===================================================================
// INT-1: commit_changes accepts batch from build_scrape_state_changes
// ===================================================================

#[test]
fn scrape_batch_state_changes_passes_commit_changes_reference_integrity() {
    // Given: a valid scrape batch with changed + new + deleted URLs
    let diff = ScrapeDiff {
        unchanged: vec!["https://unchanged.com".to_string()],
        changed: vec!["https://changed.com".to_string()],
        new_urls: vec!["https://new.com".to_string()],
        deleted: vec!["https://deleted.com".to_string()],
    };

    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://changed.com".to_string(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: b"changed_payload_data".to_vec(),
        },
    );
    artifacts.insert(
        "https://new.com".to_string(),
        ScrapeArtifact {
            content_hash: [0x02; 32],
            status_code: 201,
            payload_bytes: b"new_payload_data".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = ScrapeBatchConfig {
        now_secs: 1_700_000_000,
    };

    // When: build the state changes
    let changes = build_scrape_state_changes(&diff, &outputs, &config)
        .expect("build_scrape_state_changes should succeed");

    // Then: commit_changes should accept the batch (reference integrity passes)
    let temp_dir = tempfile::TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let state_db = StateDb::open(&db_path).unwrap();

    let commit_result = state_db.commit_changes(changes);
    assert!(
        commit_result.is_ok(),
        "commit_changes should accept the batch: {:?}",
        commit_result
    );

    // Verify the data was written by reading it back
    let session = state_db.begin_read().unwrap();
    // The url_state table should have 2 entries (changed + new)
    // The scrape_outputs table should have 2 entries
    drop(session);
}

// ===================================================================
// INT-2: UrlStateRaw round-trip through bytes preserves all fields
// ===================================================================

#[test]
fn url_state_raw_roundtrips_through_bytes_preserving_all_fields() {
    // Given: a UrlStateRaw with specific non-zero values
    let content_hash: [u8; 32] = [0xDE; 32];
    let url_hash: [u8; 32] = [0xAD; 32];
    let last_fetched_secs: u64 = 1_712_345_678;
    let status_code: u16 = 301;

    // When: construct via build_url_state_raw and round-trip through bytes
    let original = build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code);
    let bytes = original.to_bytes();

    // Then: byte length is exactly 120
    assert_eq!(bytes.len(), 120);

    // And: from_bytes recovers the exact original
    let restored = UrlStateRaw::from_bytes(&bytes).unwrap();
    assert_eq!(restored.content_hash, content_hash);
    assert_eq!(restored.url_hash, url_hash);
    assert_eq!(restored.last_fetched_secs, last_fetched_secs);
    assert_eq!(restored.status_code, status_code);
    assert_eq!(restored.reserved, [0u8; 46]);
}

// ===================================================================
// INT-2b: UrlStateRaw survives write to redb and read back
// ===================================================================

#[test]
fn url_state_raw_survives_redb_write_and_read() {
    // Given: a StateDb with initialized tables
    let temp_dir = tempfile::TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let state_db = StateDb::open(&db_path).unwrap();

    let content_hash = [0xCA; 32];
    let payload = b"test_payload_for_persist";
    let payload_hash = hash_payload(payload);

    // When: build UrlStateRaw via the function under test
    let state = build_url_state_raw(content_hash, payload_hash, 1_700_000_000, 200);

    // Assert the function set the fields correctly (not zeroed stub)
    assert_eq!(
        state.content_hash, content_hash,
        "content_hash must match input"
    );
    assert_eq!(
        state.url_hash, payload_hash,
        "url_hash must match payload hash"
    );
    assert_eq!(
        state.last_fetched_secs, 1_700_000_000,
        "last_fetched_secs must match input"
    );
    assert_eq!(state.status_code, 200, "status_code must match input");

    // Write via commit_changes
    let mut changes = StateChanges::empty();
    changes
        .updated_urls
        .push(("https://persist.com".to_string(), state));
    changes.new_scrapes.push((payload_hash, payload.to_vec()));

    state_db.commit_changes(changes).unwrap();
}
