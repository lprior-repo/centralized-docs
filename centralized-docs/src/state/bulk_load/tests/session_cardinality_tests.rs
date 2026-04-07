//! Snapshot isolation, cardinality, and mixed-table tests.

use super::*;

// =======================================================================
// B13: load_file_states uses the borrowed transaction — snapshot isolation
// =======================================================================

#[test]
fn load_file_states_uses_borrowed_transaction_without_opening_new_one() {
    let (_dir, db) = fresh_db();

    // Step 1: Write row A
    write_file_rows(
        &db,
        &[(
            "first.rs",
            FileStateRaw {
                content_hash: [0x11; 32],
                ..FileStateRaw::zeroed()
            },
        )],
    );

    // Step 2: Open a read transaction
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(file_state_table()).unwrap();
    assert_eq!(table.len().unwrap(), 1);

    // Step 3: Write row B (AFTER read_txn opened)
    write_file_rows(
        &db,
        &[(
            "second.rs",
            FileStateRaw {
                content_hash: [0x22; 32],
                ..FileStateRaw::zeroed()
            },
        )],
    );

    // Step 4: Verify old read_txn sees 1 row (snapshot isolation)
    assert_eq!(
        table.len().unwrap(),
        1,
        "original read_txn should still see exactly 1 row"
    );

    // New session gets a fresh transaction seeing both rows
    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();
    assert_eq!(
        result.len(),
        2,
        "new session should see both committed rows"
    );
    assert!(result.contains_key("first.rs"));
    assert!(result.contains_key("second.rs"));
}

// =======================================================================
// B26: load_url_states snapshot isolation verified
// =======================================================================

#[test]
fn load_url_states_uses_borrowed_transaction_without_opening_new_one() {
    let (_dir, db) = fresh_db();

    write_url_rows(
        &db,
        &[(
            "https://first.com",
            UrlStateRaw {
                content_hash: [0x11; 32],
                ..UrlStateRaw::zeroed()
            },
        )],
    );

    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(url_state_table()).unwrap();
    assert_eq!(table.len().unwrap(), 1);

    write_url_rows(
        &db,
        &[(
            "https://second.com",
            UrlStateRaw {
                content_hash: [0x22; 32],
                ..UrlStateRaw::zeroed()
            },
        )],
    );

    assert_eq!(
        table.len().unwrap(),
        1,
        "original read_txn should still see exactly 1 row"
    );

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains_key("https://first.com"));
    assert!(result.contains_key("https://second.com"));
}

// =======================================================================
// Both loaders work independently on the same database
// =======================================================================

#[test]
fn both_loaders_work_independently_on_same_database() {
    let (_dir, db) = fresh_db();
    write_file_rows(
        &db,
        &[
            ("file_a.rs", file_state(0x01)),
            ("file_b.rs", file_state(0x02)),
        ],
    );
    write_url_rows(
        &db,
        &[
            ("https://url_a.com", url_state(0x10)),
            ("https://url_b.com", url_state(0x20)),
        ],
    );

    let session = StateReadSession::new(&db).unwrap();
    let files = session.load_file_states().unwrap();
    let urls = session.load_url_states().unwrap();

    assert_eq!(files.len(), 2);
    assert_eq!(urls.len(), 2);
    assert!(files.contains_key("file_a.rs"));
    assert!(urls.contains_key("https://url_a.com"));
}

// =======================================================================
// Cardinality: load_file_states map size matches row count for N rows
// =======================================================================

#[rstest]
#[case::zero(0)]
#[case::one(1)]
#[case::five(5)]
#[case::twenty(20)]
fn load_file_states_map_size_equals_row_count_for_n_rows(#[case] n: usize) {
    let (_dir, db) = fresh_db();
    let rows: Vec<(&str, FileStateRaw)> = (0..n)
        .map(|i| {
            let key = format!("file_{i}.rs");
            let state = file_state(u8::try_from(i).unwrap_or(u8::MAX));
            (Box::leak(key.into_boxed_str()) as &str, state)
        })
        .collect();

    write_file_rows(&db, &rows);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_file_states().unwrap();

    assert_eq!(
        result.len(),
        n,
        "for {n} rows, expected {n} entries, got {}",
        result.len()
    );
}

// =======================================================================
// Cardinality: load_url_states map size matches row count for N rows
// =======================================================================

#[rstest]
#[case::zero(0)]
#[case::one(1)]
#[case::five(5)]
#[case::twenty(20)]
fn load_url_states_map_size_equals_row_count_for_n_rows(#[case] n: usize) {
    let (_dir, db) = fresh_db();
    let rows: Vec<(&str, UrlStateRaw)> = (0..n)
        .map(|i| {
            let key = format!("https://example.com/page_{i}");
            let state = url_state(u8::try_from(i).unwrap_or(u8::MAX));
            (Box::leak(key.into_boxed_str()) as &str, state)
        })
        .collect();

    write_url_rows(&db, &rows);

    let session = StateReadSession::new(&db).unwrap();
    let result = session.load_url_states().unwrap();

    assert_eq!(
        result.len(),
        n,
        "for {n} rows, expected {n} entries, got {}",
        result.len()
    );
}
