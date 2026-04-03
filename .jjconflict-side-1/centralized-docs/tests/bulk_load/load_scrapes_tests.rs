//! Integration tests for `StateReadSession::load_scrapes`.
//!
//! Covers Behaviors 29–33 from the test plan.

use super::common::*;
use doc_transformer::state::bulk_load::BulkLoadError;

// ===========================================================================
// Behavior 29: load_scrapes returns all matching entries
// ===========================================================================

#[test]
fn load_scrapes_returns_all_entries_when_all_hashes_exist() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_scrape(&db, &h1, &sample_scrape_result("https://example.com"));

    let session = create_session(&db);
    let map = session.load_scrapes(&[h1]).unwrap();

    assert_eq!(map.len(), 1);
    let archived = map[&h1].archived().unwrap();
    assert_eq!(archived.pages.len(), 1);
}

// ===========================================================================
// Behavior 30: load_scrapes omits missing hashes
// ===========================================================================

#[test]
fn load_scrapes_omits_missing_hashes_when_some_not_found() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h_missing = hash_from_byte(0xFF);

    insert_scrape(&db, &h1, &sample_scrape_result("https://present.com"));

    let session = create_session(&db);
    let map = session.load_scrapes(&[h1, h_missing]).unwrap();

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&h1));
    assert!(!map.contains_key(&h_missing));
}

// ===========================================================================
// Behavior 31: load_scrapes returns empty map on empty input
// ===========================================================================

#[test]
fn load_scrapes_returns_empty_map_when_input_slice_empty() {
    let (_temp_dir, db) = open_db_with_tables();
    let session = create_session(&db);
    let map = session.load_scrapes(&[]).unwrap();
    assert_eq!(map.len(), 0);
}

// ===========================================================================
// Behavior 32: load_scrapes returns CorruptPayload on bad bytes
// ===========================================================================

#[test]
fn load_scrapes_returns_corrupt_payload_when_bytes_invalid() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_garbage(&db, doc_transformer::state::scrape_outputs_table(), &h1);

    let session = create_session(&db);
    let result = session.load_scrapes(&[h1]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "scrape_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h1));
        }
        other => panic!("expected CorruptPayload, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 33: load_scrapes fails fast on mixed valid+corrupt
// ===========================================================================

#[test]
fn load_scrapes_fails_fast_when_mix_of_valid_and_corrupt_entries() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_scrape(&db, &h1, &sample_scrape_result("https://good.com"));
    insert_garbage(&db, doc_transformer::state::scrape_outputs_table(), &h2);

    let session = create_session(&db);
    let result = session.load_scrapes(&[h1, h2]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "scrape_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h2));
        }
        other => panic!("expected CorruptPayload for h2, got {other:?}"),
    }
}
