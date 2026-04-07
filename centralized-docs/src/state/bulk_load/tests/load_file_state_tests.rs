//! Tests for StateReadSession::load_file_states.

use super::*;

// B1: StateReadSession::new borrows database reference

#[test]
fn session_new_holds_database_reference_when_constructed() {
    let (_dir, db) = fresh_db();
    write_file_rows(&db, &[("src/main.rs", file_state(0xAA))]);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(result.len(), 1, "session should see exactly 1 row");
    assert_eq!(
        result["src/main.rs"],
        file_state(0xAA),
        "decoded value must match what was written"
    );
}

// B4: load_file_states returns complete map for all well-formed rows

#[test]
fn load_file_states_returns_all_rows_when_table_has_valid_entries() {
    let (_dir, db) = fresh_db();
    let rows = [
        ("src/main.rs", file_state(0xAA)),
        ("src/lib.rs", file_state(0xBB)),
        ("README.md", file_state(0xCC)),
    ];
    write_file_rows(&db, &rows);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result["src/main.rs"], file_state(0xAA));
    assert_eq!(result["src/lib.rs"], file_state(0xBB));
    assert_eq!(result["README.md"], file_state(0xCC));
}

// B5: load_file_states returns empty map for empty table

#[test]
fn load_file_states_returns_empty_hashmap_when_table_is_empty() {
    let (_dir, db) = fresh_db();
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(result.len(), 0);
    assert!(result.is_empty());
    assert_eq!(result, HashMap::new());
}

// B6: load_file_states returns MalformedRow for value 1 byte short

#[test]
fn load_file_states_returns_malformed_row_error_when_value_is_one_byte_short() {
    let (_dir, db) = fresh_db();
    write_raw_file_row(&db, "bad_row.dat", &[0u8; 199]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 199,
                expected: 200,
            } if key == "bad_row.dat"
        ),
        "expected MalformedRow {{ key: 'bad_row.dat', actual: 199, expected: 200 }}, got {err:?}"
    );
}

// B7: load_file_states returns MalformedRow for value 1 byte over

#[test]
fn load_file_states_returns_malformed_row_error_when_value_is_one_byte_over() {
    let (_dir, db) = fresh_db();
    write_raw_file_row(&db, "oversized.bin", &[0u8; 201]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 201,
                expected: 200,
            } if key == "oversized.bin"
        ),
        "expected MalformedRow {{ key: 'oversized.bin', actual: 201, expected: 200 }}, got {err:?}"
    );
}

// B8: load_file_states returns MalformedRow for 0-byte value

#[test]
fn load_file_states_returns_malformed_row_error_when_value_is_0_bytes() {
    let (_dir, db) = fresh_db();
    write_raw_file_row(&db, "empty.dat", &[]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 0,
                expected: 200,
            } if key == "empty.dat"
        ),
        "expected MalformedRow {{ key: 'empty.dat', actual: 0, expected: 200 }}, got {err:?}"
    );
}

// B9: load_file_states aborts on first malformed row — no partial map

#[test]
fn load_file_states_aborts_on_first_malformed_row_without_partial_map() {
    let (_dir, db) = fresh_db();
    write_file_rows(
        &db,
        &[
            ("good1.rs", file_state(0xAA)),
            ("good2.rs", file_state(0xBB)),
        ],
    );
    write_raw_file_row(&db, "broken.rs", &[0u8; 100]);
    write_file_rows(&db, &[("good3.rs", file_state(0xDD))]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 100,
                expected: 200,
            } if key == "broken.rs"
        ),
        "expected MalformedRow for 'broken.rs', got {err:?}"
    );
}

// B11: load_file_states returns BackendError when table cannot be opened

#[test]
fn load_file_states_returns_backend_error_when_table_cannot_be_opened() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("no_tables.redb");
    let db = Database::create(&db_path).unwrap();

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::BackendError {
                operation: "open_table",
                message: _,
            }
        ),
        "expected BackendError {{ operation: 'open_table' }}, got {err:?}"
    );
    if let StateLoadError::BackendError { message, .. } = &err {
        assert!(
            !message.is_empty(),
            "BackendError message must not be empty"
        );
    }
}

// B12: load_file_states decoded values are bitwise-identical

#[test]
fn load_file_states_decoded_values_are_bitwise_identical_to_written_bytes() {
    let (_dir, db) = fresh_db();
    let original = FileStateRaw {
        content_hash: {
            let mut h = [0u8; 32];
            h[0] = 0xDE;
            h[1] = 0xAD;
            h[2] = 0xBE;
            h[3] = 0xEF;
            h
        },
        config_hash: [0x00; 32],
        analysis_hash: [0x00; 32],
        transform_hash: [0x00; 32],
        chunk_hash: [0x00; 32],
        last_processed_secs: 0x1234_5678_9ABC_DEF0,
        reserved: [0x00; 32],
    };
    write_file_rows(&db, &[("exact_test.rs", original)]);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    let decoded = &result["exact_test.rs"];
    assert_eq!(decoded.content_hash[0..4], [0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(decoded.last_processed_secs, 0x1234_5678_9ABC_DEF0);
    assert_eq!(*decoded, original, "full struct must be bitwise identical");
}

// B14: load_file_states does not read url_state table

#[test]
fn load_file_states_ignores_url_state_table_rows() {
    let (_dir, db) = fresh_db();
    write_file_rows(
        &db,
        &[
            ("file1.rs", file_state(0xAA)),
            ("file2.rs", file_state(0xBB)),
        ],
    );
    write_url_rows(
        &db,
        &[
            ("https://a.com", url_state(0x11)),
            ("https://b.com", url_state(0x22)),
            ("https://c.com", url_state(0x33)),
        ],
    );

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(result.len(), 2, "should only see file_state rows");
    assert!(result.contains_key("file1.rs"));
    assert!(result.contains_key("file2.rs"));
}

// B15: load_file_states HashMap keys are exact UTF-8 round-trips

#[test]
fn load_file_states_preserves_key_strings_exactly() {
    let (_dir, db) = fresh_db();
    write_file_rows(
        &db,
        &[
            ("src/üñíçödé/päth.rs", file_state(0xA1)),
            ("simple.txt", file_state(0xA2)),
            ("path/with spaces/and-dashes.md", file_state(0xA3)),
        ],
    );

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(result.len(), 3);
    assert!(result.contains_key("src/üñíçödé/päth.rs"));
    assert!(result.contains_key("simple.txt"));
    assert!(result.contains_key("path/with spaces/and-dashes.md"));
}

// Idempotency: load_file_states can be called multiple times

#[test]
fn load_file_states_is_idempotent_across_multiple_calls() {
    let (_dir, db) = fresh_db();
    write_file_rows(&db, &[("stable.rs", file_state(0x42))]);

    let session = StateReadSession::new(&db).unwrap();
    let first = session.load_file_states().unwrap();
    let second = session.load_file_states().unwrap();

    assert_eq!(
        first, second,
        "repeated calls must return identical results"
    );
    assert_eq!(first.len(), 1);
}
