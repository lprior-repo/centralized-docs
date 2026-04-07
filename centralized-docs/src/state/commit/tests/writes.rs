//! Write persistence tests (data survives commit, deletes, dedup).

use super::*;

#[test]
fn commit_changes_persists_updated_files_to_file_state_table() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_a = [1u8; 32];
    let state = make_file_state_raw(hash_a, [0u8; 32], [0u8; 32]);
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("src/main.rs".to_string(), state)];
    changes.new_analyses = vec![(hash_a, vec![10, 20, 30])];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_string_table(db, file_state_table(), "src/main.rs");
    assert!(stored.is_some(), "file_state entry should exist");
}

#[test]
fn commit_changes_deletes_files_and_skips_nonexistent() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![("to_delete.rs".to_string(), FileStateRaw::zeroed())];
    state_db.commit_changes(setup).expect("setup commit");
    let mut changes = make_minimal_valid_state_changes();
    changes.deleted_files = vec!["to_delete.rs".to_string(), "nonexistent.rs".to_string()];
    state_db.commit_changes(changes).expect("delete commit");
    let db = state_db.database();
    assert!(read_string_table(db, file_state_table(), "to_delete.rs").is_none());
}

#[test]
fn commit_changes_persists_new_analyses_to_analysis_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [1u8; 32];
    let payload = vec![10, 20, 30];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_analyses = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, analysis_outputs_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_persists_new_transforms() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [2u8; 32];
    let payload = vec![40, 50];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_transforms = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, transform_outputs_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_persists_new_chunks() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [3u8; 32];
    let payload = vec![60];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_chunks = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, chunk_outputs_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_persists_updated_urls_to_url_state() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let url_state = make_url_state_raw([0u8; 32]);
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_urls = vec![("https://example.com".to_string(), url_state)];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_string_table(db, url_state_table(), "https://example.com");
    assert!(stored.is_some());
}

#[test]
fn commit_changes_deletes_urls_and_skips_nonexistent() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_urls = vec![("https://del.com".to_string(), UrlStateRaw::zeroed())];
    state_db.commit_changes(setup).expect("setup");
    let mut changes = make_minimal_valid_state_changes();
    changes.deleted_urls = vec![
        "https://del.com".to_string(),
        "https://nope.com".to_string(),
    ];
    state_db.commit_changes(changes).expect("delete");
    let db = state_db.database();
    assert!(read_string_table(db, url_state_table(), "https://del.com").is_none());
}

#[test]
fn commit_changes_persists_new_scrapes() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [4u8; 32];
    let payload = vec![70, 80];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_scrapes = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, scrape_outputs_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_persists_new_snapshots() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [5u8; 32];
    let payload = vec![90];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_snapshots = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, snapshots_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_deletes_snapshots_and_skips_nonexistent() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [6u8; 32];
    let mut setup = make_minimal_valid_state_changes();
    setup.new_snapshots = vec![(hash_key, vec![99])];
    state_db.commit_changes(setup).expect("setup");
    let mut changes = make_minimal_valid_state_changes();
    changes.deleted_snapshots = vec![hash_key, [7u8; 32]];
    state_db.commit_changes(changes).expect("delete");
    let db = state_db.database();
    assert!(read_hash_table(db, snapshots_table(), &hash_key).is_none());
}

#[test]
fn commit_changes_deduplicates_payload_entries_last_write_wins() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [1u8; 32];
    let mut changes = make_minimal_valid_state_changes();
    changes.new_analyses = vec![
        (hash_key, vec![1]),
        (hash_key, vec![2]),
        (hash_key, vec![3]), // last write wins
    ];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, analysis_outputs_table(), &hash_key);
    assert_eq!(stored, Some(vec![3]), "last-write-wins dedup");
}

#[test]
fn commit_changes_succeeds_with_noop_empty_batch() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let changes = StateChanges::empty();
    state_db
        .commit_changes(changes)
        .expect("noop commit should succeed");
}

#[test]
fn commit_changes_accepts_zero_byte_payload_in_analyses() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [1u8; 32];
    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![(hash_key, vec![])];
    state_db
        .commit_changes(changes)
        .expect("zero-byte payload should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, analysis_outputs_table(), &hash_key);
    assert_eq!(stored, Some(vec![]));
}

#[test]
fn commit_changes_succeeds_with_only_analyses_populated() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_a = [1u8; 32];
    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![(hash_a, vec![10])];
    state_db
        .commit_changes(changes)
        .expect("partial population should succeed");
    let db = state_db.database();
    assert_eq!(
        read_hash_table(db, analysis_outputs_table(), &hash_a),
        Some(vec![10])
    );
    assert_eq!(count_table_entries(db, "transform_outputs"), 0);
    assert_eq!(count_table_entries(db, "chunk_outputs"), 0);
}

#[test]
fn commit_changes_skips_unchanged_rows_without_rewriting() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_key = [1u8; 32];
    let payload = vec![10, 20, 30];
    let mut setup = make_minimal_valid_state_changes();
    setup.new_analyses = vec![(hash_key, payload.clone())];
    state_db.commit_changes(setup).expect("first commit");
    // Re-commit same data
    let mut changes = make_minimal_valid_state_changes();
    changes.new_analyses = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("second commit (skip)");
    let db = state_db.database();
    let stored = read_hash_table(db, analysis_outputs_table(), &hash_key);
    assert_eq!(stored, Some(payload));
}

#[test]
fn commit_changes_rolls_back_all_writes_when_validation_fails() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("valid.rs".to_string(), FileStateRaw::zeroed())];
    changes.new_analyses = vec![([0u8; 32], vec![1, 2, 3])]; // invalid
    let result = state_db.commit_changes(changes);
    assert!(matches!(result, Err(CommitError::ZeroHashKey { .. })));
    let db = state_db.database();
    assert!(
        read_string_table(db, file_state_table(), "valid.rs").is_none(),
        "no writes should be visible after validation failure"
    );
}

#[test]
fn commit_changes_handles_long_source_path_approaching_redb_key_limit() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let long_path: String = "a".repeat(4096);
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(long_path.clone(), FileStateRaw::zeroed())];
    let result = state_db.commit_changes(changes);
    match result {
        Ok(()) => {
            let db = state_db.database();
            assert!(read_string_table(db, file_state_table(), &long_path).is_some());
        }
        Err(CommitError::WriteFailed {
            table: "file_state",
            ..
        }) => {}
        Err(e) => panic!("unexpected error: {e}"),
    }
}
