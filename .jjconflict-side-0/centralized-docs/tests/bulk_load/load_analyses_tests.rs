//! Integration tests for `StateReadSession::load_analyses`.
//!
//! Covers Behaviors 7–17 from the test plan.

use super::common::*;
use doc_transformer::state::bulk_load::BulkLoadError;

// ===========================================================================
// Behavior 7: load_analyses returns all matching entries when all hashes exist
// ===========================================================================

#[test]
fn load_analyses_returns_all_entries_when_all_hashes_exist() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);
    let h3 = hash_from_byte(3);

    insert_analysis(&db, &h1, &sample_analysis("a.md", 10));
    insert_analysis(&db, &h2, &sample_analysis("b.md", 20));
    insert_analysis(&db, &h3, &sample_analysis("c.md", 30));

    let session = create_session(&db);
    let map = session.load_analyses(&[h1, h2, h3]).unwrap();

    assert_eq!(map.len(), 3);
    assert_eq!(
        map[&h1].archived().unwrap().analyses[0]
            .source_path
            .as_ref(),
        "a.md"
    );
    assert_eq!(map[&h1].archived().unwrap().analyses[0].word_count, 10);
    assert_eq!(
        map[&h2].archived().unwrap().analyses[0]
            .source_path
            .as_ref(),
        "b.md"
    );
    assert_eq!(
        map[&h3].archived().unwrap().analyses[0]
            .source_path
            .as_ref(),
        "c.md"
    );
}

// ===========================================================================
// Behavior 8: load_analyses omits missing hashes
// ===========================================================================

#[test]
fn load_analyses_omits_missing_hashes_when_some_not_found() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_analysis(&db, &h1, &sample_analysis("present.md", 5));

    let session = create_session(&db);
    let map = session.load_analyses(&[h1, h2]).unwrap();

    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&h1));
    assert!(!map.contains_key(&h2));
    assert_eq!(
        map[&h1].archived().unwrap().analyses[0]
            .source_path
            .as_ref(),
        "present.md"
    );
}

// ===========================================================================
// Behavior 9: load_analyses returns empty map when no hashes match
// ===========================================================================

#[test]
fn load_analyses_returns_empty_map_when_no_hashes_match() {
    let (_temp_dir, db) = open_db_with_tables();
    let h_unknown = hash_from_byte(0xFF);

    let session = create_session(&db);
    let map = session.load_analyses(&[h_unknown]).unwrap();

    assert_eq!(map.len(), 0);
}

// ===========================================================================
// Behavior 10: load_analyses returns empty map on empty input
// ===========================================================================

#[test]
fn load_analyses_returns_empty_map_when_input_slice_empty() {
    let (_temp_dir, db) = open_db_with_tables();
    // Pre-populate to show it's not about empty table
    insert_analysis(&db, &hash_from_byte(1), &sample_analysis("x.md", 1));

    let session = create_session(&db);
    let map = session.load_analyses(&[]).unwrap();

    assert_eq!(map.len(), 0);
}

// ===========================================================================
// Behavior 11: load_analyses deduplicates input hashes
// ===========================================================================

#[test]
fn load_analyses_deduplicates_when_input_has_duplicate_hashes() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_analysis(&db, &h1, &sample_analysis("dedup.md", 7));

    let session = create_session(&db);
    let map = session.load_analyses(&[h1, h1, h1]).unwrap();

    assert_eq!(map.len(), 1);
    assert_eq!(
        map[&h1].archived().unwrap().analyses[0]
            .source_path
            .as_ref(),
        "dedup.md"
    );
}

// ===========================================================================
// Behavior 12: load_analyses returns TableOpen error when table missing
// ===========================================================================

#[test]
fn load_analyses_returns_table_open_error_when_table_missing() {
    let (_temp_dir, db) = open_db_without_table("analysis_outputs");
    let h1 = hash_from_byte(1);

    let session = create_session(&db);
    let result = session.load_analyses(&[h1]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::TableOpen { table, message } => {
            assert_eq!(*table, "analysis_outputs");
            assert!(!message.is_empty());
        }
        other => panic!("expected TableOpen, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 14: load_analyses returns CorruptPayload on bad bytes
// ===========================================================================

#[test]
fn load_analyses_returns_corrupt_payload_when_bytes_invalid() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_garbage(&db, doc_transformer::state::analysis_outputs_table(), &h1);

    let session = create_session(&db);
    let result = session.load_analyses(&[h1]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload {
            table,
            key_hex,
            message,
        } => {
            assert_eq!(*table, "analysis_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h1));
            assert!(!message.is_empty());
        }
        other => panic!("expected CorruptPayload, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 15: load_analyses preserves key identity
// ===========================================================================

#[test]
fn load_analyses_preserves_key_identity_when_loading_entries() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1: [u8; 32] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];

    insert_analysis(&db, &h1, &sample_analysis("key_test.md", 1));

    let session = create_session(&db);
    let map = session.load_analyses(&[h1]).unwrap();

    assert_eq!(map.len(), 1);
    let output_key = map.keys().next().unwrap();
    assert_eq!(*output_key, h1);
}

// ===========================================================================
// Behavior 16: load_analyses fails fast when mix of valid and corrupt entries
// ===========================================================================

#[test]
fn load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);
    let h2 = hash_from_byte(2);

    insert_analysis(&db, &h1, &sample_analysis("valid.md", 5));
    insert_garbage(&db, doc_transformer::state::analysis_outputs_table(), &h2);

    let session = create_session(&db);
    let result = session.load_analyses(&[h1, h2]);

    let err = result.unwrap_err();
    match &err {
        BulkLoadError::CorruptPayload { table, key_hex, .. } => {
            assert_eq!(*table, "analysis_outputs");
            assert_eq!(key_hex, &hex_encode_32(&h2));
        }
        other => panic!("expected CorruptPayload for h2, got {other:?}"),
    }
}

// ===========================================================================
// Behavior 17: load_analyses returns empty map when input empty AND table missing
// ===========================================================================

#[test]
fn load_analyses_returns_empty_map_when_input_empty_and_table_missing() {
    let (_temp_dir, db) = open_db_without_table("analysis_outputs");

    let session = create_session(&db);
    let result = session.load_analyses(&[]);

    assert!(
        result.is_ok(),
        "empty input should succeed even when table missing"
    );
    assert_eq!(result.unwrap().len(), 0);
}
