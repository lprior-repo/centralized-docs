//! Integration tests for `StateReadSession::load_chunks`.
//!
//! Covers Behaviors 24–28 from the test plan.

use super::common::*;
use doc_transformer::state::bulk_load::BulkLoadError;

// ===========================================================================
// Behavior 24: load_chunks returns all matching entries
// ===========================================================================

#[test]
fn load_chunks_returns_all_entries_when_all_hashes_exist() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_chunks(&db, &h1, &sample_chunks_result(3));
    insert_chunks(&db, &h2, &sample_chunks_result(7));

    let session = create_session(&db);
    let map = session.load_chunks(&[h1, h2]).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map[&h1].archived().unwrap().chunks_metadata.len(), 3);
    assert_eq!(map[&h2].archived().unwrap().chunks_metadata.len(), 7);
}

// ===========================================================================
// Behavior 25: load_chunks omits missing hashes
// ===========================================================================

#[test]
fn load_chunks_omits_missing_hashes_when_some_not_found() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h_missing = hash_from_byte(0xFF);

    insert_chunks(&db, &h1, &sample_chunks_result(1));

    let session = create_session(&db);
    let map = session.load_chunks(&[h1, h_missing]).unwrap();

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&h1));
    assert!(!map.contains_key(&h_missing));
    assert_eq!(map[&h1].archived().unwrap().chunks_metadata.len(), 1);
}

// ===========================================================================
// Behavior 26: load_chunks returns empty map on empty input
// ===========================================================================

#[test]
fn load_chunks_returns_empty_map_when_input_slice_empty() {
    let (_temp_dir, db) = open_db_with_tables();
    let session = create_session(&db);
    let map = session.load_chunks(&[]).unwrap();
    assert_eq!(map.len(), 0);
}

// ===========================================================================
// Behavior 27: load_chunks returns CorruptPayload on bad bytes
// ===========================================================================

#[test]
fn load_chunks_returns_corrupt_payload_when_bytes_invalid() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_garbage(&db, doc_transformer::state::chunk_outputs_table(), &h1);

    let session = create_session(&db);
    let result = session.load_chunks(&[h1]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "chunk_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h1));
        }
        other => panic!("expected CorruptPayload, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 28: load_chunks fails fast on mixed valid+corrupt
// ===========================================================================

#[test]
fn load_chunks_fails_fast_when_mix_of_valid_and_corrupt_entries() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_chunks(&db, &h1, &sample_chunks_result(2));
    insert_garbage(&db, doc_transformer::state::chunk_outputs_table(), &h2);

    let session = create_session(&db);
    let result = session.load_chunks(&[h1, h2]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "chunk_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h2));
        }
        other => panic!("expected CorruptPayload for h2, got {other:?}"),
    }
}
