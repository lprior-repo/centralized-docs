//! Integration tests for `StateReadSession::load_transforms`.
//!
//! Covers Behaviors 18–23 from the test plan.

use super::common::*;
use doc_transformer::state::bulk_load::BulkLoadError;

// ===========================================================================
// Behavior 18: load_transforms returns all matching entries
// ===========================================================================

#[test]
fn load_transforms_returns_all_entries_when_all_hashes_exist() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_transform(&db, &h1, &sample_transform_result(10, 10));
    insert_transform(&db, &h2, &sample_transform_result(20, 20));

    let session = create_session(&db);
    let map = session.load_transforms(&[h1, h2]).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map[&h1].archived().unwrap().success_count, 10);
    assert_eq!(map[&h2].archived().unwrap().success_count, 20);
}

// ===========================================================================
// Behavior 19: load_transforms omits missing hashes
// ===========================================================================

#[test]
fn load_transforms_omits_missing_hashes_when_some_not_found() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h_missing = hash_from_byte(0xFF);

    insert_transform(&db, &h1, &sample_transform_result(5, 5));

    let session = create_session(&db);
    let map = session.load_transforms(&[h1, h_missing]).unwrap();

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&h1));
    assert!(!map.contains_key(&h_missing));
    assert_eq!(map[&h1].archived().unwrap().success_count, 5);
}

// ===========================================================================
// Behavior 20: load_transforms returns empty map on empty input
// ===========================================================================

#[test]
fn load_transforms_returns_empty_map_when_input_slice_empty() {
    let (_temp_dir, db) = open_db_with_tables();
    let session = create_session(&db);
    let map = session.load_transforms(&[]).unwrap();
    assert_eq!(map.len(), 0);
}

// ===========================================================================
// Behavior 21: load_transforms deduplicates input hashes
// ===========================================================================

#[test]
fn load_transforms_deduplicates_when_input_has_duplicate_hashes() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_transform(&db, &h1, &sample_transform_result(7, 7));

    let session = create_session(&db);
    let map = session.load_transforms(&[h1, h1]).unwrap();

    assert_eq!(map.len(), 1);
    assert_eq!(map[&h1].archived().unwrap().success_count, 7);
}

// ===========================================================================
// Behavior 22: load_transforms returns CorruptPayload on bad bytes
// ===========================================================================

#[test]
fn load_transforms_returns_corrupt_payload_when_bytes_invalid() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_garbage(&db, doc_transformer::state::transform_outputs_table(), &h1);

    let session = create_session(&db);
    let result = session.load_transforms(&[h1]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "transform_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h1));
        }
        other => panic!("expected CorruptPayload, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 23: load_transforms fails fast on mixed valid+corrupt
// ===========================================================================

#[test]
fn load_transforms_fails_fast_when_mix_of_valid_and_corrupt_entries() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_transform(&db, &h1, &sample_transform_result(1, 1));
    insert_garbage(&db, doc_transformer::state::transform_outputs_table(), &h2);

    let session = create_session(&db);
    let result = session.load_transforms(&[h1, h2]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "transform_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h2));
        }
        other => panic!("expected CorruptPayload for h2, got {other:?}"),
    }
}
