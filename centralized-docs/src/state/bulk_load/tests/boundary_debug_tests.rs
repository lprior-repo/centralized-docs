//! Boundary, debug-print, and equality tests.

use super::*;

// Boundary tests

#[test]
fn load_file_states_handles_single_row_with_all_zero_state() {
    let (_dir, db) = fresh_db();
    write_file_rows(&db, &[("zero.rs", FileStateRaw::zeroed())]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["zero.rs"], FileStateRaw::zeroed());
}

#[test]
fn load_url_states_handles_single_row_with_all_zero_state() {
    let (_dir, db) = fresh_db();
    write_url_rows(&db, &[("https://zero.com", UrlStateRaw::zeroed())]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["https://zero.com"], UrlStateRaw::zeroed());
}

#[test]
fn load_file_states_handles_row_with_max_timestamp() {
    let (_dir, db) = fresh_db();
    let state = FileStateRaw {
        content_hash: [0xFF; 32],
        config_hash: [0xFF; 32],
        analysis_hash: [0xFF; 32],
        transform_hash: [0xFF; 32],
        chunk_hash: [0xFF; 32],
        last_processed_secs: u64::MAX,
        reserved: [0xFF; 32],
    };
    write_file_rows(&db, &[("max.rs", state)]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["max.rs"].last_processed_secs, u64::MAX);
}

#[test]
fn load_url_states_handles_row_with_max_fields() {
    let (_dir, db) = fresh_db();
    let state = UrlStateRaw {
        content_hash: [0xFF; 32],
        url_hash: [0xFF; 32],
        last_fetched_secs: u64::MAX,
        status_code: u16::MAX,
        reserved: [0xFF; 46],
    };
    write_url_rows(&db, &[("https://max.com", state)]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result["https://max.com"].last_fetched_secs, u64::MAX);
    assert_eq!(result["https://max.com"].status_code, u16::MAX);
}

#[test]
fn load_file_states_preserves_distinct_keys_with_identical_values() {
    let (_dir, db) = fresh_db();
    let shared_state = file_state(0x42);
    write_file_rows(
        &db,
        &[
            ("path/a.rs", shared_state),
            ("path/b.rs", shared_state),
            ("path/c.rs", shared_state),
        ],
    );
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result["path/a.rs"], shared_state);
    assert_eq!(result["path/b.rs"], shared_state);
    assert_eq!(result["path/c.rs"], shared_state);
}

#[test]
fn load_url_states_preserves_distinct_keys_with_identical_values() {
    let (_dir, db) = fresh_db();
    let shared_state = url_state(0x42);
    write_url_rows(
        &db,
        &[
            ("https://a.com", shared_state),
            ("https://b.com", shared_state),
            ("https://c.com", shared_state),
        ],
    );
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result["https://a.com"], shared_state);
    assert_eq!(result["https://b.com"], shared_state);
    assert_eq!(result["https://c.com"], shared_state);
}

#[test]
fn load_file_states_returns_malformed_row_for_value_exactly_100_bytes() {
    let (_dir, db) = fresh_db();
    write_raw_file_row(&db, "half.dat", &[0u8; 100]);
    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(&err, StateLoadError::MalformedRow { key, actual: 100, expected: 200 } if key == "half.dat"),
        "expected MalformedRow {{ actual: 100 }}, got {err:?}"
    );
}

#[test]
fn load_url_states_returns_malformed_row_for_value_exactly_60_bytes() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://half.example.com", &[0u8; 60]);
    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(&err, StateLoadError::MalformedRow { key, actual: 60, expected: 120 } if key == "https://half.example.com"),
        "expected MalformedRow {{ actual: 60 }}, got {err:?}"
    );
}

#[test]
fn load_file_states_returns_malformed_row_for_value_exactly_1_byte() {
    let (_dir, db) = fresh_db();
    write_raw_file_row(&db, "one.dat", &[0xFF]);
    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_file_states().unwrap_err();
    assert!(
        matches!(&err, StateLoadError::MalformedRow { key, actual: 1, expected: 200 } if key == "one.dat"),
        "expected MalformedRow {{ actual: 1 }}, got {err:?}"
    );
}

#[test]
fn load_url_states_returns_malformed_row_for_value_exactly_1_byte() {
    let (_dir, db) = fresh_db();
    write_raw_url_row(&db, "https://one.example.com", &[0xAB]);
    let session = StateReadSession::new(&db).unwrap();
    let err = session.load_url_states().unwrap_err();
    assert!(
        matches!(&err, StateLoadError::MalformedRow { key, actual: 1, expected: 120 } if key == "https://one.example.com"),
        "expected MalformedRow {{ actual: 1 }}, got {err:?}"
    );
}

#[test]
fn load_file_states_with_10_rows_preserves_each_key() {
    let (_dir, db) = fresh_db();
    let rows: Vec<(&str, FileStateRaw)> = (0..10)
        .map(|i| {
            let key = format!("file_{i}.rs");
            let state = file_state(i);
            (Box::leak(key.into_boxed_str()) as &str, state)
        })
        .collect();
    write_file_rows(&db, &rows);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(result.len(), 10);
    assert_eq!(result["file_0.rs"], file_state(0));
    assert_eq!(result["file_5.rs"], file_state(5));
    assert_eq!(result["file_9.rs"], file_state(9));
}

#[test]
fn load_url_states_with_10_rows_preserves_each_key() {
    let (_dir, db) = fresh_db();
    let rows: Vec<(&str, UrlStateRaw)> = (0..10)
        .map(|i| {
            let key = format!("https://example.com/page_{i}");
            let state = url_state(i);
            (Box::leak(key.into_boxed_str()) as &str, state)
        })
        .collect();
    write_url_rows(&db, &rows);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 10);
    assert_eq!(result["https://example.com/page_0"], url_state(0));
    assert_eq!(result["https://example.com/page_5"], url_state(5));
    assert_eq!(result["https://example.com/page_9"], url_state(9));
}

#[test]
fn session_new_opens_readable_transaction_on_valid_db() {
    let (_dir, db) = fresh_db();
    write_file_rows(&db, &[("probe.rs", file_state(0x01))]);
    let session = StateReadSession::new(&db).expect("session should open");
    let result = session.load_file_states().expect("should load");
    assert_eq!(result.len(), 1);
}

#[test]
fn both_loaders_return_empty_when_no_data_written() {
    let (_dir, db) = fresh_db();
    let session = StateReadSession::new(&db).unwrap();
    assert_eq!(session.load_file_states().unwrap().len(), 0);
    assert_eq!(session.load_url_states().unwrap().len(), 0);
}

#[test]
fn file_state_key_with_deeply_nested_path_roundtrips() {
    let (_dir, db) = fresh_db();
    let deep_key = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/deep.rs";
    write_file_rows(&db, &[(deep_key, file_state(0x01))]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(result.len(), 1);
    assert!(result.contains_key(deep_key));
}

#[test]
fn url_state_key_with_long_query_string_roundtrips() {
    let (_dir, db) = fresh_db();
    let long_key =
        "https://example.com/api/v2/resource?param1=value1&param2=value2&param3=value3&page=42";
    write_url_rows(&db, &[(long_key, url_state(0x02))]);
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 1);
    assert!(result.contains_key(long_key));
}

// See debug_equality_tests.rs for debug-print and equality tests.
