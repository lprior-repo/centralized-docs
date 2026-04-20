//! Tests for `StateReadSession::load_url_states`.

use super::*;

// B16: load_url_states returns complete map for all well-formed rows

#[test]
fn load_url_states_returns_all_rows_when_table_has_valid_entries() {
    let (_dir, db) = fresh_db();
    let rows = [
        ("https://docs.rs/sha2", url_state(0x11)),
        ("https://example.com/guide", url_state(0x22)),
        ("https://rust-lang.org/learn", url_state(0x33)),
    ];
    write_url_rows(&db, &rows);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result["https://docs.rs/sha2"], url_state(0x11));
    assert_eq!(result["https://example.com/guide"], url_state(0x22));
    assert_eq!(result["https://rust-lang.org/learn"], url_state(0x33));
}

// B17: load_url_states returns empty map for empty table

#[test]
fn load_url_states_returns_empty_hashmap_when_table_is_empty() {
    let (_dir, db) = fresh_db();
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    assert_eq!(result.len(), 0);
    assert!(result.is_empty());
    assert_eq!(result, HashMap::new());
}

// B18: load_url_states returns MalformedRow for 119-byte value

#[test]
fn load_url_states_returns_malformed_row_error_when_value_is_one_byte_short() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://broken-short.example.com", &[0u8; 119]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 119,
                expected: 120,
            } if key == "https://broken-short.example.com"
        ),
        "expected MalformedRow {{ actual: 119, expected: 120 }}, got {err:?}"
    );
}

// B19: load_url_states returns MalformedRow for 121-byte value

#[test]
fn load_url_states_returns_malformed_row_error_when_value_is_one_byte_over() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://oversized.example.com", &[0u8; 121]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 121,
                expected: 120,
            } if key == "https://oversized.example.com"
        ),
        "expected MalformedRow {{ actual: 121, expected: 120 }}, got {err:?}"
    );
}

// B20: load_url_states returns MalformedRow for 0-byte value

#[test]
fn load_url_states_returns_malformed_row_error_when_value_is_0_bytes() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://empty.example.com", &[]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 0,
                expected: 120,
            } if key == "https://empty.example.com"
        ),
        "expected MalformedRow {{ actual: 0, expected: 120 }}, got {err:?}"
    );
}

// B21: load_url_states returns MalformedRow for 240-byte value

#[test]
fn load_url_states_returns_malformed_row_error_when_value_is_double_size() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://double.example.com", &[0u8; 240]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 240,
                expected: 120,
            } if key == "https://double.example.com"
        ),
        "expected MalformedRow {{ actual: 240, expected: 120 }}, got {err:?}"
    );
}

// B22: load_url_states aborts on first malformed row

#[test]
fn load_url_states_aborts_on_first_malformed_row_without_partial_map() {
    let (_dir, db) = fresh_db();
    write_url_rows(
        &db,
        &[
            ("https://good1.example.com", url_state(0xAA)),
            ("https://good2.example.com", url_state(0xBB)),
        ],
    );
    write_raw_url_row(&db, "https://broken.example.com", &[0u8; 60]);
    write_url_rows(&db, &[("https://good3.example.com", url_state(0xDD))]);

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 60,
                expected: 120,
            } if key == "https://broken.example.com"
        ),
        "expected MalformedRow for 'https://broken.example.com', got {err:?}"
    );
}

// B24: load_url_states returns BackendError when table cannot be opened

#[test]
fn load_url_states_returns_backend_error_when_table_cannot_be_opened() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("no_tables_url.redb");
    let db = Database::create(&db_path).unwrap();

    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
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

// B25: load_url_states decoded values are bitwise-identical

#[test]
fn load_url_states_decoded_values_are_bitwise_identical_to_written_bytes() {
    let (_dir, db) = fresh_db();
    let original = UrlStateRaw {
        content_hash: {
            let mut h = [0u8; 32];
            h[0] = 0xCA;
            h[1] = 0xFE;
            h[2] = 0xBA;
            h[3] = 0xBE;
            h
        },
        url_hash: [0x00; 32],
        last_fetched_secs: 0xFEDC_BA98_7654_3210,
        status_code: 200,
        reserved: [0x00; 46],
    };
    write_url_rows(&db, &[("https://test.example.com", original)]);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    let decoded = &result["https://test.example.com"];
    assert_eq!(decoded.content_hash[0..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(decoded.last_fetched_secs, 0xFEDC_BA98_7654_3210);
    assert_eq!(*decoded, original, "full struct must be bitwise identical");
}

// B27: load_url_states does not read file_state table

#[test]
fn load_url_states_ignores_file_state_table_rows() {
    let (_dir, db) = fresh_db();
    write_url_rows(
        &db,
        &[
            ("https://a.com", url_state(0x11)),
            ("https://b.com", url_state(0x22)),
        ],
    );
    write_file_rows(
        &db,
        &[
            ("file1.rs", file_state(0xAA)),
            ("file2.rs", file_state(0xBB)),
            ("file3.rs", file_state(0xCC)),
        ],
    );

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    assert_eq!(result.len(), 2, "should only see url_state rows");
    assert!(result.contains_key("https://a.com"));
    assert!(result.contains_key("https://b.com"));
}

// B28: load_url_states HashMap keys are exact UTF-8 round-trips

#[test]
fn load_url_states_preserves_key_strings_exactly() {
    let (_dir, db) = fresh_db();
    write_url_rows(
        &db,
        &[
            ("https://example.com/üñíçödé", url_state(0xB1)),
            ("https://simple.com/page", url_state(0xB2)),
            ("https://example.com/path with spaces", url_state(0xB3)),
        ],
    );

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    assert_eq!(result.len(), 3);
    assert!(result.contains_key("https://example.com/üñíçödé"));
    assert!(result.contains_key("https://simple.com/page"));
    assert!(result.contains_key("https://example.com/path with spaces"));
}

// Idempotency: load_url_states can be called multiple times

#[test]
fn load_url_states_is_idempotent_across_multiple_calls() {
    let (_dir, db) = fresh_db();
    write_url_rows(&db, &[("https://stable.com", url_state(0x42))]);

    let session = StateReadSession::new(&db).unwrap();
    let first = session.load_url_states().unwrap();
    let second = session.load_url_states().unwrap();

    assert_eq!(
        first, second,
        "repeated calls must return identical results"
    );
    assert_eq!(first.len(), 1);
}
