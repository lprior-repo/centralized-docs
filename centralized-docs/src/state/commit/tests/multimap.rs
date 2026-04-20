//! Multimap (`source_path_chunks`) tests.

use super::*;

#[test]
fn upsert_with_nonzero_chunk_hash_populates_multimap() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let chunk_hash = [0xAAu8; 32];
    let file_state = make_file_state_raw([0u8; 32], [0u8; 32], chunk_hash);
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("src/main.rs".to_string(), file_state)];
    changes.new_chunks = vec![(chunk_hash, vec![1, 2, 3])];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let entries = read_multimap_entries(db, "src/main.rs");
    assert_eq!(entries, vec![chunk_hash]);
}

#[test]
fn upsert_with_zero_chunk_hash_does_not_populate_multimap() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let file_state = FileStateRaw::zeroed();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("src/main.rs".to_string(), file_state)];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    let entries = read_multimap_entries(db, "src/main.rs");
    assert!(entries.is_empty());
}

#[test]
fn delete_file_removes_orphaned_chunks_and_multimap_entries() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let chunk_hash = [0xAAu8; 32];
    let file_state = make_file_state_raw([0u8; 32], [0u8; 32], chunk_hash);
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![("src/main.rs".to_string(), file_state)];
    setup.new_chunks = vec![(chunk_hash, vec![1, 2, 3])];
    state_db
        .commit_changes(setup)
        .expect("setup commit should succeed");
    let mut changes = make_minimal_valid_state_changes();
    changes.deleted_files = vec!["src/main.rs".to_string()];
    state_db
        .commit_changes(changes)
        .expect("delete commit should succeed");
    let db = state_db.database();
    assert!(read_string_table(db, file_state_table(), "src/main.rs").is_none());
    assert!(read_hash_table(db, chunk_outputs_table(), &chunk_hash).is_none());
    assert!(read_multimap_entries(db, "src/main.rs").is_empty());
}

#[test]
fn delete_nonexistent_file_does_not_error_and_multimap_clean() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.deleted_files = vec!["nonexistent.rs".to_string()];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    let db = state_db.database();
    assert!(read_multimap_entries(db, "nonexistent.rs").is_empty());
}

#[test]
fn re_upsert_updates_multimap_to_new_chunk_hash() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let chunk_v1 = [0xAAu8; 32];
    let chunk_v2 = [0xBBu8; 32];
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![(
        "src/main.rs".to_string(),
        make_file_state_raw([0u8; 32], [0u8; 32], chunk_v1),
    )];
    setup.new_chunks = vec![(chunk_v1, vec![1])];
    state_db.commit_changes(setup).expect("v1 commit");
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(
        "src/main.rs".to_string(),
        make_file_state_raw([0u8; 32], [0u8; 32], chunk_v2),
    )];
    changes.new_chunks = vec![(chunk_v2, vec![2])];
    state_db.commit_changes(changes).expect("v2 commit");
    let db = state_db.database();
    let entries = read_multimap_entries(db, "src/main.rs");
    assert!(entries.contains(&chunk_v2));
}

#[test]
fn full_lifecycle_insert_update_delete_multimap() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_a = [1u8; 32];
    let hash_b = [2u8; 32];
    // Insert A
    let mut s1 = make_minimal_valid_state_changes();
    s1.updated_files = vec![(
        "a.rs".to_string(),
        make_file_state_raw([0u8; 32], [0u8; 32], hash_a),
    )];
    s1.new_chunks = vec![(hash_a, vec![10])];
    state_db.commit_changes(s1).expect("insert a.rs");
    // Insert B
    let mut s2 = make_minimal_valid_state_changes();
    s2.updated_files = vec![(
        "b.rs".to_string(),
        make_file_state_raw([0u8; 32], [0u8; 32], hash_b),
    )];
    s2.new_chunks = vec![(hash_b, vec![20])];
    state_db.commit_changes(s2).expect("insert b.rs");
    // Delete A
    let mut da = make_minimal_valid_state_changes();
    da.deleted_files = vec!["a.rs".to_string()];
    state_db.commit_changes(da).expect("delete a.rs");
    let db = state_db.database();
    assert!(read_multimap_entries(db, "b.rs").contains(&hash_b));
    assert!(read_hash_table(db, chunk_outputs_table(), &hash_b).is_some());
    assert!(read_multimap_entries(db, "a.rs").is_empty());
    assert!(read_hash_table(db, chunk_outputs_table(), &hash_a).is_none());
}

#[test]
fn failed_validation_leaves_multimap_unchanged() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let chunk_hash = [0xAAu8; 32];
    let file_state = make_file_state_raw([0u8; 32], [0u8; 32], chunk_hash);
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("src/main.rs".to_string(), file_state)];
    changes.new_chunks = vec![(chunk_hash, vec![1])];
    changes.new_analyses = vec![([0u8; 32], vec![99])]; // Invalid
    let result = state_db.commit_changes(changes);
    assert!(matches!(result, Err(CommitError::ZeroHashKey { .. })));
    let db = state_db.database();
    assert!(read_multimap_entries(db, "src/main.rs").is_empty());
}

#[test]
fn mixed_mutations_include_multimap_operations() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let hash_old = [0x99u8; 32];
    let hash_new = [0xA1u8; 32];
    let hash_t = [0xA2u8; 32];
    let hash_c = [0xA3u8; 32];
    let hash_s = [0xA4u8; 32];
    let hash_snap = [0xA5u8; 32];
    // Pre-populate
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![(
        "old.rs".to_string(),
        make_file_state_raw(hash_old, [0u8; 32], [0u8; 32]),
    )];
    setup.new_analyses = vec![(hash_old, vec![0])];
    state_db.commit_changes(setup).expect("setup");
    // Mixed batch
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(
        "new.rs".to_string(),
        make_file_state_raw(hash_new, hash_t, hash_c),
    )];
    changes.deleted_files = vec!["old.rs".to_string()];
    changes.new_analyses = vec![(hash_new, vec![10, 20])];
    changes.new_transforms = vec![(hash_t, vec![30])];
    changes.new_chunks = vec![(hash_c, vec![40])];
    changes.new_scrapes = vec![(hash_s, vec![50])];
    changes.new_snapshots = vec![(hash_snap, vec![60])];
    state_db.commit_changes(changes).expect("mixed commit");
    let db = state_db.database();
    assert!(read_multimap_entries(db, "new.rs").contains(&hash_c));
    assert!(read_multimap_entries(db, "old.rs").is_empty());
    assert!(read_hash_table(db, chunk_outputs_table(), &hash_c).is_some());
}

#[test]
fn shared_chunk_hash_survives_partial_delete() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let shared_hash = [0xAAu8; 32];
    let state_a = make_file_state_raw([0u8; 32], [0u8; 32], shared_hash);
    let file_state_b = make_file_state_raw([0u8; 32], [0u8; 32], shared_hash);
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![
        ("file_a.rs".to_string(), state_a),
        ("file_b.rs".to_string(), file_state_b),
    ];
    setup.new_chunks = vec![(shared_hash, vec![42])];
    state_db.commit_changes(setup).expect("setup");
    let mut da = make_minimal_valid_state_changes();
    da.deleted_files = vec!["file_a.rs".to_string()];
    state_db.commit_changes(da).expect("delete file_a");
    let db = state_db.database();
    assert!(read_multimap_entries(db, "file_a.rs").is_empty());
}
