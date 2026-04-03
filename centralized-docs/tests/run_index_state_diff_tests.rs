//! Integration tests for cdocs-9nr: Wire startup state open and file diff into run_index.
//!
//! RED PHASE tests exercise the NEW state-diff behavior components:
//! - Converting FileStateRaw → StoredHashes (replicating the pure function logic)
//! - Opening StateDb and bulk-loading file states
//! - Computing file diffs via compute_file_diff
//! - Computing config hashes via compute_config_hash
//! - Error variant display output correctness
//!
//! NOTE: `run_index` itself lives in `cmd::index` which is part of the binary crate,
//! not the library crate. We test the COMPONENTS it wires together. The `run_index`
//! integration is tested via the `ctd` binary (e2e tests) or inline unit tests in
//! `cmd/index.rs`.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use doc_transformer::cache::{content_hash, ContentHash};
use doc_transformer::diff::{compute_config_hash, compute_file_diff, DiffError, StoredHashes};
use doc_transformer::discover::DiscoveryFile;
use doc_transformer::state::bulk_load::StateReadSession;
use doc_transformer::state::commit::{CommitError, StateDb};
use doc_transformer::state::{FileStateRaw, StateLoadError};
use tempfile::TempDir;

// ============================================================================
// Helpers
// ============================================================================

/// Replicate the pure conversion from `cmd/index.rs: file_states_to_stored_hashes`.
/// This is the exact same logic — projects content_hash and config_hash from each
/// FileStateRaw into StoredHashes.
fn file_states_to_stored_hashes(
    file_states: &HashMap<String, FileStateRaw>,
) -> HashMap<String, StoredHashes> {
    file_states
        .iter()
        .map(|(path, raw)| {
            (
                path.clone(),
                StoredHashes {
                    content_hash: raw.content_hash.into(),
                    config_hash: raw.config_hash.into(),
                },
            )
        })
        .collect()
}

/// Create a FileStateRaw with the given content_hash and config_hash, all other hashes zeroed.
fn file_state_with_hashes(content_hash: [u8; 32], config_hash: [u8; 32]) -> FileStateRaw {
    FileStateRaw {
        content_hash,
        config_hash,
        analysis_hash: [0u8; 32],
        transform_hash: [0u8; 32],
        chunk_hash: [0u8; 32],
        last_processed_secs: 0,
        reserved: [0u8; 32],
    }
}

/// Create a source directory with the given markdown files.
fn create_source_dir(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("create source temp dir");
    for (name, content) in files {
        let path = dir.path().join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dir");
        }
        fs::write(&path, content).expect("write file");
    }
    dir
}

/// Write file-state rows to a StateDb for seeding test data.
fn seed_file_state_rows(db: &StateDb, rows: &[(&str, FileStateRaw)]) {
    let db_ref = db.database();
    let write_tx = db_ref.begin_write().expect("begin write");
    {
        let mut table = write_tx
            .open_table(doc_transformer::state::file_state_table())
            .expect("open file_state table");
        for (key, state) in rows {
            table
                .insert(*key, state.to_bytes().as_slice())
                .expect("insert row");
        }
    }
    write_tx.commit().expect("commit write");
}

/// Write a raw (potentially malformed) byte value to the file_state table.
fn write_raw_file_state_row(db: &StateDb, key: &str, value: &[u8]) {
    let db_ref = db.database();
    let write_tx = db_ref.begin_write().expect("begin write");
    {
        let mut table = write_tx
            .open_table(doc_transformer::state::file_state_table())
            .expect("open file_state table");
        table.insert(key, value).expect("insert raw row");
    }
    write_tx.commit().expect("commit write");
}

/// Count file_state rows in the database.
fn count_file_state_rows(db: &StateDb) -> u64 {
    use redb::ReadableTableMetadata;
    let db_ref = db.database();
    let read_txn = db_ref.begin_read().expect("begin read");
    let table = read_txn
        .open_table(doc_transformer::state::file_state_table())
        .expect("open file_state table");
    table.len().expect("table len")
}

// ============================================================================
// Layer 1: Unit Tests — file_states_to_stored_hashes (pure function)
// ============================================================================

#[test]
fn file_states_to_stored_hashes_returns_map_with_identical_keys_when_input_nonempty() {
    // Given: a HashMap with 3 entries
    let mut file_states = HashMap::new();
    file_states.insert("a.rs".to_string(), FileStateRaw::zeroed());
    file_states.insert("b.md".to_string(), FileStateRaw::zeroed());
    file_states.insert("c.txt".to_string(), FileStateRaw::zeroed());

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then: same keys, same count
    assert_eq!(result.len(), 3, "output must have same number of entries");
    assert!(result.contains_key("a.rs"), "must contain key 'a.rs'");
    assert!(result.contains_key("b.md"), "must contain key 'b.md'");
    assert!(result.contains_key("c.txt"), "must contain key 'c.txt'");
}

#[test]
fn file_states_to_stored_hashes_returns_empty_map_when_input_empty() {
    // Given
    let file_states = HashMap::<String, FileStateRaw>::new();

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then
    assert_eq!(result.len(), 0, "empty input must produce empty output");
    assert_eq!(result, HashMap::new());
}

#[test]
fn file_states_to_stored_hashes_projects_bitwise_identical_content_and_config_hashes() {
    // Given: one entry with known hashes
    let mut file_states = HashMap::new();
    file_states.insert(
        "key".to_string(),
        file_state_with_hashes([0xAA; 32], [0xBB; 32]),
    );

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then: content_hash and config_hash are bitwise identical
    let stored = &result["key"];
    assert_eq!(
        stored.content_hash,
        ContentHash::from([0xAA; 32]),
        "content_hash must be bitwise identical"
    );
    assert_eq!(
        stored.config_hash,
        ContentHash::from([0xBB; 32]),
        "config_hash must be bitwise identical"
    );
}

#[test]
fn file_states_to_stored_hashes_preserves_single_entry() {
    // Given
    let mut file_states = HashMap::new();
    file_states.insert("single.md".to_string(), FileStateRaw::zeroed());

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("single.md"));
    // Zeroed FileStateRaw → zeroed hashes
    assert_eq!(
        result["single.md"].content_hash,
        ContentHash::from([0u8; 32])
    );
    assert_eq!(
        result["single.md"].config_hash,
        ContentHash::from([0u8; 32])
    );
}

#[test]
fn file_states_to_stored_hashes_handles_large_input() {
    // Given: 100 entries
    let mut file_states = HashMap::new();
    for i in 0..100u8 {
        let mut hash = [0u8; 32];
        hash[0] = i;
        file_states.insert(format!("file_{i}.md"), file_state_with_hashes(hash, hash));
    }

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then: all 100 entries preserved
    assert_eq!(result.len(), 100);
    for i in 0..100u8 {
        let key = format!("file_{i}.md");
        assert!(result.contains_key(&key), "must contain key: {key}");
    }
}

#[test]
fn file_states_to_stored_hashes_preserves_distinct_hashes_per_entry() {
    // Given: 3 entries with distinct hashes
    let hash_a = {
        let mut h = [0u8; 32];
        h[0] = 0x0A;
        h
    };
    let hash_b = {
        let mut h = [0u8; 32];
        h[0] = 0x0B;
        h
    };
    let hash_c = {
        let mut h = [0u8; 32];
        h[0] = 0x0C;
        h
    };
    let config_a = {
        let mut h = [0u8; 32];
        h[1] = 0xCA;
        h
    };
    let config_b = {
        let mut h = [0u8; 32];
        h[1] = 0xCB;
        h
    };
    let config_c = {
        let mut h = [0u8; 32];
        h[1] = 0xCC;
        h
    };

    let mut file_states = HashMap::new();
    file_states.insert("a.md".to_string(), file_state_with_hashes(hash_a, config_a));
    file_states.insert("b.md".to_string(), file_state_with_hashes(hash_b, config_b));
    file_states.insert("c.md".to_string(), file_state_with_hashes(hash_c, config_c));

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then: each entry has its own distinct hashes
    assert_eq!(result["a.md"].content_hash, ContentHash::from(hash_a));
    assert_eq!(result["a.md"].config_hash, ContentHash::from(config_a));
    assert_eq!(result["b.md"].content_hash, ContentHash::from(hash_b));
    assert_eq!(result["b.md"].config_hash, ContentHash::from(config_b));
    assert_eq!(result["c.md"].content_hash, ContentHash::from(hash_c));
    assert_eq!(result["c.md"].config_hash, ContentHash::from(config_c));
}

#[test]
fn file_states_to_stored_hashes_output_keys_are_byte_identical_strings() {
    // Given: keys with special characters
    let mut file_states = HashMap::new();
    file_states.insert(
        "path/with spaces/and-dashes.md".to_string(),
        FileStateRaw::zeroed(),
    );
    file_states.insert("src/üñíçödé.md".to_string(), FileStateRaw::zeroed());

    // When
    let result = file_states_to_stored_hashes(&file_states);

    // Then
    assert!(result.contains_key("path/with spaces/and-dashes.md"));
    assert!(result.contains_key("src/üñíçödé.md"));
}

// ============================================================================
// Layer 2: Integration — StateDb + compute_file_diff end-to-end
// ============================================================================

#[test]
fn state_db_opens_at_output_dir_and_tables_are_initialized() {
    // Given: an output directory
    let output = TempDir::new().expect("create output temp dir");
    let state_db_path = output.path().join("state.redb");

    // When: StateDb::open creates the database
    let result = StateDb::open(&state_db_path);

    // Then: opens successfully with initialized tables
    let db = result.expect("StateDb::open should succeed");
    let session =
        StateReadSession::new(db.database()).expect("StateReadSession::new should succeed");
    let file_states = session
        .load_file_states()
        .expect("load_file_states should succeed on initialized db");
    assert_eq!(
        file_states.len(),
        0,
        "fresh db should have 0 file_state rows"
    );
}

#[test]
fn state_db_can_be_reopened_after_close() {
    // Given: create and drop a StateDb
    let output = TempDir::new().expect("create output temp dir");
    let state_db_path = output.path().join("state.redb");
    {
        let db = StateDb::open(&state_db_path).expect("first open");
        let _session = StateReadSession::new(db.database()).expect("session");
    }

    // When: reopen the same database
    let db2 = StateDb::open(&state_db_path).expect("second open");
    let session2 = StateReadSession::new(db2.database()).expect("second session");
    let file_states = session2.load_file_states().expect("load");
    assert_eq!(file_states.len(), 0);
}

#[test]
fn state_db_seeded_rows_visible_after_reopen() {
    // Given: seed some rows
    let output = TempDir::new().expect("temp dir");
    let state_db_path = output.path().join("state.redb");
    let db = StateDb::open(&state_db_path).expect("open");
    seed_file_state_rows(
        &db,
        &[
            ("intro.md", file_state_with_hashes([0xAA; 32], [0xBB; 32])),
            ("guide.md", file_state_with_hashes([0xCC; 32], [0xDD; 32])),
        ],
    );
    drop(db);

    // When: reopen and load
    let db2 = StateDb::open(&state_db_path).expect("reopen");
    let session = StateReadSession::new(db2.database()).expect("session");
    let file_states = session.load_file_states().expect("load");

    // Then: both rows present with exact hashes
    assert_eq!(file_states.len(), 2);
    assert_eq!(file_states["intro.md"].content_hash, [0xAA; 32]);
    assert_eq!(file_states["intro.md"].config_hash, [0xBB; 32]);
    assert_eq!(file_states["guide.md"].content_hash, [0xCC; 32]);
    assert_eq!(file_states["guide.md"].config_hash, [0xDD; 32]);
}

#[test]
fn state_db_seeded_rows_convert_to_stored_hashes_correctly() {
    // Given: seed rows, load them, convert
    let output = TempDir::new().expect("temp dir");
    let state_db_path = output.path().join("state.redb");
    let db = StateDb::open(&state_db_path).expect("open");
    seed_file_state_rows(
        &db,
        &[
            ("a.md", file_state_with_hashes([0x11; 32], [0x22; 32])),
            ("b.md", file_state_with_hashes([0x33; 32], [0x44; 32])),
        ],
    );
    drop(db);

    let db2 = StateDb::open(&state_db_path).expect("reopen");
    let session = StateReadSession::new(db2.database()).expect("session");
    let file_states = session.load_file_states().expect("load");
    let stored_hashes = file_states_to_stored_hashes(&file_states);

    // Then: conversion is correct
    assert_eq!(stored_hashes.len(), 2);
    assert_eq!(
        stored_hashes["a.md"].content_hash,
        ContentHash::from([0x11; 32])
    );
    assert_eq!(
        stored_hashes["a.md"].config_hash,
        ContentHash::from([0x22; 32])
    );
    assert_eq!(
        stored_hashes["b.md"].content_hash,
        ContentHash::from([0x33; 32])
    );
    assert_eq!(
        stored_hashes["b.md"].config_hash,
        ContentHash::from([0x44; 32])
    );
}

#[test]
fn state_db_reopenable_after_malformed_row_causes_error() {
    // Given: state.db with a malformed row
    let output = TempDir::new().expect("temp dir");
    let state_db_path = output.path().join("state.redb");
    let db = StateDb::open(&state_db_path).expect("open");
    write_raw_file_state_row(&db, "corrupt.md", &[0u8; 199]);
    drop(db);

    // When: try to load (should error)
    let db2 = StateDb::open(&state_db_path).expect("reopen after malformed write");
    let session = StateReadSession::new(db2.database()).expect("session");
    let result = session.load_file_states();

    // Then: load fails with MalformedRow
    let err = result.expect_err("should fail for malformed row");
    assert!(
        matches!(err, StateLoadError::MalformedRow { ref key, actual: 199, expected: 200 } if key == "corrupt.md"),
        "expected MalformedRow for 'corrupt.md', got: {err:?}"
    );
    drop(session);

    // And: db is still reopenable (no stale locks)
    drop(db2);
    let db3 = StateDb::open(&state_db_path).expect("third open");
    let session3 = StateReadSession::new(db3.database()).expect("third session");
    // load_file_states should still fail for the same reason
    let err2 = session3.load_file_states().expect_err("should still fail");
    assert!(matches!(err2, StateLoadError::MalformedRow { .. }));
}

// ============================================================================
// Layer 2b: compute_file_diff integration tests
// ============================================================================

#[test]
fn compute_file_diff_classifies_all_as_new_when_stored_hashes_empty() {
    // Given: 2 discovered files, empty stored hashes
    let source = create_source_dir(&[("a.md", "aaa"), ("b.md", "bbb")]);
    let files = vec![
        DiscoveryFile {
            source_path: "a.md".to_string(),
            size_bytes: 3,
        },
        DiscoveryFile {
            source_path: "b.md".to_string(),
            size_bytes: 3,
        },
    ];
    let stored_hashes = HashMap::new();

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let diff = result.expect("compute_file_diff should succeed");
    assert_eq!(diff.unchanged.len(), 0, "no files should be unchanged");
    assert_eq!(diff.changed.len(), 0, "no files should be changed");
    assert_eq!(diff.new.len(), 2, "both files should be new");
    assert!(diff.new.contains("a.md"));
    assert!(diff.new.contains("b.md"));
    assert_eq!(diff.deleted.len(), 0, "no files should be deleted");
}

#[test]
fn compute_file_diff_classifies_as_unchanged_when_hashes_match() {
    // Given
    let source = create_source_dir(&[("same.md", "same content")]);
    let content_hash_val = content_hash(b"same content");
    let config_hash_val = compute_config_hash(None);
    let mut stored_hashes = HashMap::new();
    stored_hashes.insert(
        "same.md".to_string(),
        StoredHashes {
            content_hash: content_hash_val,
            config_hash: config_hash_val,
        },
    );
    let files = vec![DiscoveryFile {
        source_path: "same.md".to_string(),
        size_bytes: 12,
    }];

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let diff = result.expect("compute_file_diff should succeed");
    assert_eq!(diff.unchanged.len(), 1);
    assert!(diff.unchanged.contains("same.md"));
    assert_eq!(diff.changed.len(), 0);
    assert_eq!(diff.new.len(), 0);
    assert_eq!(diff.deleted.len(), 0);
}

#[test]
fn compute_file_diff_classifies_as_changed_when_content_differs() {
    // Given
    let source = create_source_dir(&[("changed.md", "new content")]);
    let old_hash = content_hash(b"old content");
    let config_hash_val = compute_config_hash(None);
    let mut stored_hashes = HashMap::new();
    stored_hashes.insert(
        "changed.md".to_string(),
        StoredHashes {
            content_hash: old_hash,
            config_hash: config_hash_val,
        },
    );
    let files = vec![DiscoveryFile {
        source_path: "changed.md".to_string(),
        size_bytes: 11,
    }];

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let diff = result.expect("compute_file_diff should succeed");
    assert_eq!(diff.changed.len(), 1);
    assert!(diff.changed.contains("changed.md"));
    assert_eq!(diff.unchanged.len(), 0);
    assert_eq!(diff.new.len(), 0);
    assert_eq!(diff.deleted.len(), 0);
}

#[test]
fn compute_file_diff_classifies_as_deleted_when_not_discovered() {
    // Given
    let source = create_source_dir(&[("present.md", "here")]);
    let content_hash_val = content_hash(b"here");
    let config_hash_val = compute_config_hash(None);
    let mut stored_hashes = HashMap::new();
    stored_hashes.insert(
        "present.md".to_string(),
        StoredHashes {
            content_hash: content_hash_val,
            config_hash: config_hash_val,
        },
    );
    stored_hashes.insert(
        "gone.md".to_string(),
        StoredHashes {
            content_hash: ContentHash::from([0u8; 32]),
            config_hash: config_hash_val,
        },
    );
    let files = vec![DiscoveryFile {
        source_path: "present.md".to_string(),
        size_bytes: 4,
    }];

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let diff = result.expect("compute_file_diff should succeed");
    assert!(diff.unchanged.contains("present.md"));
    assert!(diff.deleted.contains("gone.md"));
    assert_eq!(diff.deleted.len(), 1);
}

#[test]
fn compute_file_diff_handles_mixed_unchanged_changed_new_deleted() {
    // Given
    let source = create_source_dir(&[
        ("same.md", "unchanged content"),
        ("modified.md", "new content"),
        ("brand_new.md", "brand new"),
    ]);

    let unchanged_hash = content_hash(b"unchanged content");
    let old_modified_hash = content_hash(b"old modified content");
    let deleted_hash = ContentHash::from([0u8; 32]);
    let config_hash_val = compute_config_hash(None);

    let mut stored_hashes = HashMap::new();
    stored_hashes.insert(
        "same.md".to_string(),
        StoredHashes {
            content_hash: unchanged_hash,
            config_hash: config_hash_val,
        },
    );
    stored_hashes.insert(
        "modified.md".to_string(),
        StoredHashes {
            content_hash: old_modified_hash,
            config_hash: config_hash_val,
        },
    );
    stored_hashes.insert(
        "deleted.md".to_string(),
        StoredHashes {
            content_hash: deleted_hash,
            config_hash: config_hash_val,
        },
    );

    let files = vec![
        DiscoveryFile {
            source_path: "same.md".to_string(),
            size_bytes: 18,
        },
        DiscoveryFile {
            source_path: "modified.md".to_string(),
            size_bytes: 11,
        },
        DiscoveryFile {
            source_path: "brand_new.md".to_string(),
            size_bytes: 9,
        },
    ];

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let diff = result.expect("compute_file_diff should succeed");
    assert_eq!(diff.unchanged.len(), 1, "1 unchanged");
    assert!(diff.unchanged.contains("same.md"));
    assert_eq!(diff.changed.len(), 1, "1 changed");
    assert!(diff.changed.contains("modified.md"));
    assert_eq!(diff.new.len(), 1, "1 new");
    assert!(diff.new.contains("brand_new.md"));
    assert_eq!(diff.deleted.len(), 1, "1 deleted");
    assert!(diff.deleted.contains("deleted.md"));
}

#[test]
fn compute_file_diff_partition_completeness_union_covers_all_paths() {
    // Given
    let source = create_source_dir(&[("shared.md", "shared"), ("new_file.md", "new")]);
    let shared_hash = content_hash(b"shared");
    let config_hash_val = compute_config_hash(None);
    let mut stored_hashes = HashMap::new();
    stored_hashes.insert(
        "shared.md".to_string(),
        StoredHashes {
            content_hash: shared_hash,
            config_hash: config_hash_val,
        },
    );
    stored_hashes.insert(
        "deleted.md".to_string(),
        StoredHashes {
            content_hash: ContentHash::from([0u8; 32]),
            config_hash: config_hash_val,
        },
    );

    let files = vec![
        DiscoveryFile {
            source_path: "shared.md".to_string(),
            size_bytes: 6,
        },
        DiscoveryFile {
            source_path: "new_file.md".to_string(),
            size_bytes: 3,
        },
    ];

    // When
    let diff =
        compute_file_diff(&files, source.path(), None, &stored_hashes).expect("should succeed");

    // Then: union of all buckets = union of discovered + stored paths
    let mut all_paths: HashSet<String> = HashSet::new();
    all_paths.extend(diff.unchanged.iter().cloned());
    all_paths.extend(diff.changed.iter().cloned());
    all_paths.extend(diff.new.iter().cloned());
    all_paths.extend(diff.deleted.iter().cloned());

    assert!(
        all_paths.contains("shared.md"),
        "shared must appear in a bucket"
    );
    assert!(
        all_paths.contains("new_file.md"),
        "new file must appear in a bucket"
    );
    assert!(
        all_paths.contains("deleted.md"),
        "deleted must appear in a bucket"
    );
    assert_eq!(
        all_paths.len(),
        3,
        "exactly 3 unique paths across all buckets"
    );
}

#[test]
fn compute_file_diff_is_deterministic_for_same_inputs() {
    // Given
    let source = create_source_dir(&[("file.md", "content")]);
    let files = vec![DiscoveryFile {
        source_path: "file.md".to_string(),
        size_bytes: 7,
    }];
    let content_hash_val = content_hash(b"content");
    let config_hash_val = compute_config_hash(None);
    let mut stored = HashMap::new();
    stored.insert(
        "file.md".to_string(),
        StoredHashes {
            content_hash: content_hash_val,
            config_hash: config_hash_val,
        },
    );

    // When: call twice
    let diff1 = compute_file_diff(&files, source.path(), None, &stored).expect("diff1");
    let diff2 = compute_file_diff(&files, source.path(), None, &stored).expect("diff2");

    // Then
    assert_eq!(
        diff1, diff2,
        "deterministic: same inputs must produce same diff"
    );
}

// ============================================================================
// Layer 2c: Error variant tests
// ============================================================================

#[test]
fn compute_file_diff_returns_source_dir_not_found_when_dir_missing() {
    // Given
    let missing = PathBuf::from("/nonexistent_cdocs_test_dir_xyz");
    let files = vec![DiscoveryFile {
        source_path: "a.md".to_string(),
        size_bytes: 1,
    }];
    let stored_hashes = HashMap::new();

    // When
    let result = compute_file_diff(&files, &missing, None, &stored_hashes);

    // Then
    let err = result.expect_err("should fail for missing source dir");
    assert!(
        matches!(err, DiffError::SourceDirNotFound(_)),
        "expected SourceDirNotFound, got: {err}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("source directory does not exist"),
        "error message should mention source directory: {msg}"
    );
}

#[test]
fn compute_file_diff_returns_path_traversal_when_malicious_path() {
    // Given
    let source = create_source_dir(&[("safe.md", "content")]);
    let files = vec![DiscoveryFile {
        source_path: "../../etc/passwd".to_string(),
        size_bytes: 0,
    }];
    let stored_hashes = HashMap::new();

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    // Then
    let err = result.expect_err("should reject path traversal");
    assert!(
        matches!(err, DiffError::PathTraversal { ref path } if path == "../../etc/passwd"),
        "expected PathTraversal with correct path, got: {err}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("path traversal detected"),
        "error message should mention path traversal: {msg}"
    );
}

#[test]
fn compute_file_diff_returns_file_read_when_file_unreadable() {
    // Given: a file that exists but is unreadable
    let source = create_source_dir(&[("secret.md", "secret content")]);
    let secret_path = source.path().join("secret.md");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&secret_path, PermissionsExt::from_mode(0o000)).expect("chmod 000");
    }

    let files = vec![DiscoveryFile {
        source_path: "secret.md".to_string(),
        size_bytes: 14,
    }];
    let stored_hashes = HashMap::new();

    // When
    let result = compute_file_diff(&files, source.path(), None, &stored_hashes);

    #[cfg(unix)]
    {
        let err = result.expect_err("should fail to read file");
        assert!(
            matches!(err, DiffError::FileRead { ref path, .. } if path == "secret.md"),
            "expected FileRead for 'secret.md', got: {err}"
        );
        let msg = err.to_string();
        assert!(
            msg.contains("failed to read file"),
            "error message should mention file read: {msg}"
        );

        // Cleanup for TempDir
        let _ = fs::set_permissions(&secret_path, PermissionsExt::from_mode(0o644));
    }
    #[cfg(not(unix))]
    {
        let _ = result;
    }
}

#[test]
fn commit_error_database_open_display_contains_path_and_reason() {
    let err = CommitError::DatabaseOpen {
        path: "/tmp/state.redb".to_string(),
        reason: "permission denied".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("/tmp/state.redb"),
        "DatabaseOpen message must contain path: {msg}"
    );
    assert!(
        msg.contains("permission denied"),
        "DatabaseOpen message must contain reason: {msg}"
    );
}

#[test]
fn commit_error_table_init_display_contains_reason() {
    let err = CommitError::TableInit {
        reason: "corrupted header".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("corrupted header"),
        "TableInit message must contain reason: {msg}"
    );
}

#[test]
fn commit_error_read_transaction_display_contains_reason() {
    let err = CommitError::ReadTransaction {
        reason: "lock conflict".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("lock conflict"),
        "ReadTransaction message must contain reason: {msg}"
    );
}

#[test]
fn state_load_error_malformed_row_display_contains_key_and_sizes() {
    let err = StateLoadError::MalformedRow {
        key: "corrupt.md".to_string(),
        actual: 199,
        expected: 200,
    };
    let msg = err.to_string();
    assert!(
        msg.contains("corrupt.md"),
        "MalformedRow message must contain key: {msg}"
    );
    assert!(
        msg.contains("199"),
        "MalformedRow message must contain actual size: {msg}"
    );
    assert!(
        msg.contains("200"),
        "MalformedRow message must contain expected size: {msg}"
    );
}

#[test]
fn state_load_error_utf8_key_error_display_contains_bytes() {
    let err = StateLoadError::Utf8KeyError {
        bytes_lossy: "��invalid".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("non-UTF-8"),
        "Utf8KeyError message must mention non-UTF-8: {msg}"
    );
    assert!(
        msg.contains("��invalid"),
        "Utf8KeyError message must contain lossy bytes: {msg}"
    );
}

#[test]
fn state_load_error_backend_error_display_contains_operation() {
    let err = StateLoadError::BackendError {
        operation: "open_table",
        message: "table not found".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("open_table"),
        "BackendError message must contain operation: {msg}"
    );
}

#[test]
fn diff_error_source_dir_not_found_display_contains_path() {
    let err = DiffError::SourceDirNotFound("/no/such/dir".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("/no/such/dir"),
        "SourceDirNotFound must contain path: {msg}"
    );
    assert!(
        msg.contains("source directory does not exist"),
        "SourceDirNotFound must describe the issue: {msg}"
    );
}

#[test]
fn diff_error_file_read_display_contains_path() {
    let err = DiffError::FileRead {
        path: "unreadable.md".to_string(),
        source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied"),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("unreadable.md"),
        "FileRead must contain path: {msg}"
    );
    assert!(
        msg.contains("failed to read file"),
        "FileRead must mention read failure: {msg}"
    );
}

#[test]
fn diff_error_path_traversal_display_contains_path() {
    let err = DiffError::PathTraversal {
        path: "../../etc/passwd".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("../../etc/passwd"),
        "PathTraversal must contain path: {msg}"
    );
    assert!(
        msg.contains("path traversal detected"),
        "PathTraversal must mention traversal: {msg}"
    );
}

// ============================================================================
// Layer 2d: Full state-db-to-diff pipeline (simulating run_index STEP 1.5)
// ============================================================================

#[test]
fn full_step_1_5_pipeline_first_run_all_files_new() {
    // Simulate what run_index does in STEP 1.5 for a first run

    // Given: 3 markdown files on disk, empty state db
    let source = create_source_dir(&[
        ("intro.md", "# Intro\n\nHello."),
        ("guide.md", "# Guide\n\nWorld."),
        ("api.md", "# API\n\nDocs."),
    ]);
    let output = TempDir::new().expect("output temp dir");

    // STEP 1.5a: Open StateDb
    let state_db_path = output.path().join("state.redb");
    let state_db = StateDb::open(&state_db_path).expect("StateDb::open should succeed");

    // STEP 1.5b-c: Begin read session, load file states
    let session = StateReadSession::new(state_db.database()).expect("begin read session");
    let file_states = session.load_file_states().expect("load file states");

    // STEP 1.5d: Convert to stored hashes
    let stored_hashes = file_states_to_stored_hashes(&file_states);

    // STEP 1.5e: Compute config hash
    let _config_hash = compute_config_hash(None);

    // STEP 1.5f: Compute file diff
    let files = vec![
        DiscoveryFile {
            source_path: "intro.md".to_string(),
            size_bytes: 17,
        },
        DiscoveryFile {
            source_path: "guide.md".to_string(),
            size_bytes: 17,
        },
        DiscoveryFile {
            source_path: "api.md".to_string(),
            size_bytes: 16,
        },
    ];
    let source_dir = source.path().to_path_buf();
    let file_diff = compute_file_diff(&files, &source_dir, None, &stored_hashes)
        .expect("compute_file_diff should succeed");

    // STEP 1.5g: Print diff statistics (verify the counts)
    assert_eq!(file_diff.unchanged.len(), 0, "first run: no unchanged");
    assert_eq!(file_diff.changed.len(), 0, "first run: no changed");
    assert_eq!(file_diff.new.len(), 3, "first run: all 3 files are new");
    assert_eq!(file_diff.deleted.len(), 0, "first run: no deleted");
}

#[test]
fn full_step_1_5_pipeline_second_run_with_pre_seeded_state() {
    // Simulate what run_index does for a second run with pre-existing state

    // Given: 2 files on disk matching state, 1 deleted, 1 new, 1 changed
    let source = create_source_dir(&[
        ("unchanged.md", "same content"),
        ("changed.md", "modified content"),
        ("new_file.md", "brand new content"),
    ]);
    let output = TempDir::new().expect("output temp dir");
    let state_db_path = output.path().join("state.redb");

    // Seed state with previous indexing results
    let db = StateDb::open(&state_db_path).expect("open for seeding");
    let unchanged_hash = content_hash(b"same content");
    let old_changed_hash = content_hash(b"original content");
    let deleted_hash = content_hash(b"deleted file content");
    let config_hash_val = compute_config_hash(None);

    seed_file_state_rows(
        &db,
        &[
            (
                "unchanged.md",
                file_state_with_hashes(unchanged_hash.into(), config_hash_val.into()),
            ),
            (
                "changed.md",
                file_state_with_hashes(old_changed_hash.into(), config_hash_val.into()),
            ),
            (
                "deleted.md",
                file_state_with_hashes(deleted_hash.into(), config_hash_val.into()),
            ),
        ],
    );
    drop(db); // Release before re-opening

    // STEP 1.5: Re-open and compute diff
    let db2 = StateDb::open(&state_db_path).expect("reopen");
    let session = StateReadSession::new(db2.database()).expect("session");
    let file_states = session.load_file_states().expect("load");
    let stored_hashes = file_states_to_stored_hashes(&file_states);

    let files = vec![
        DiscoveryFile {
            source_path: "unchanged.md".to_string(),
            size_bytes: 12,
        },
        DiscoveryFile {
            source_path: "changed.md".to_string(),
            size_bytes: 16,
        },
        DiscoveryFile {
            source_path: "new_file.md".to_string(),
            size_bytes: 17,
        },
    ];

    let file_diff = compute_file_diff(&files, source.path(), None, &stored_hashes)
        .expect("compute_file_diff should succeed");

    // Verify diff counts
    assert_eq!(file_diff.unchanged.len(), 1, "1 unchanged");
    assert!(file_diff.unchanged.contains("unchanged.md"));
    assert_eq!(file_diff.changed.len(), 1, "1 changed");
    assert!(file_diff.changed.contains("changed.md"));
    assert_eq!(file_diff.new.len(), 1, "1 new");
    assert!(file_diff.new.contains("new_file.md"));
    assert_eq!(file_diff.deleted.len(), 1, "1 deleted");
    assert!(file_diff.deleted.contains("deleted.md"));
}

#[test]
fn full_step_1_5_no_writes_to_state_db() {
    // Verify that the STEP 1.5 pipeline does NOT write to the state db

    // Given
    let source = create_source_dir(&[("test.md", "content")]);
    let output = TempDir::new().expect("output temp dir");
    let state_db_path = output.path().join("state.redb");

    // Open, read, convert, diff — no writes
    let db = StateDb::open(&state_db_path).expect("open");
    let session = StateReadSession::new(db.database()).expect("session");
    let file_states = session.load_file_states().expect("load");
    let stored_hashes = file_states_to_stored_hashes(&file_states);
    let _config_hash = compute_config_hash(None);
    let files = vec![DiscoveryFile {
        source_path: "test.md".to_string(),
        size_bytes: 7,
    }];
    let _diff = compute_file_diff(&files, source.path(), None, &stored_hashes).expect("diff");
    drop(session);
    drop(db);

    // Then: state db has 0 file_state rows
    let db2 = StateDb::open(&state_db_path).expect("reopen");
    let row_count = count_file_state_rows(&db2);
    assert_eq!(
        row_count, 0,
        "file_state table should have 0 rows (read-only in this bead)"
    );
}

// ============================================================================
// Layer 3: Proptests
// ============================================================================

mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn arb_file_state_raw() -> impl Strategy<Value = FileStateRaw> {
        (
            prop::array::uniform32(any::<u8>()),
            prop::array::uniform32(any::<u8>()),
            prop::array::uniform32(any::<u8>()),
            prop::array::uniform32(any::<u8>()),
            prop::array::uniform32(any::<u8>()),
            any::<u64>(),
            prop::array::uniform32(any::<u8>()),
        )
            .prop_map(
                |(content, config, analysis, transform, chunk, ts, reserved)| FileStateRaw {
                    content_hash: content,
                    config_hash: config,
                    analysis_hash: analysis,
                    transform_hash: transform,
                    chunk_hash: chunk,
                    last_processed_secs: ts,
                    reserved,
                },
            )
    }

    fn arb_file_states(max_size: usize) -> impl Strategy<Value = HashMap<String, FileStateRaw>> {
        prop::collection::hash_map(
            "[a-zA-Z0-9_/.-]{1,30}".prop_map(|s| format!("{s}.md")),
            arb_file_state_raw(),
            0..max_size,
        )
    }

    proptest! {
        #[test]
        fn proptest_file_states_to_stored_hashes_preserves_all_keys(
            file_states in arb_file_states(20)
        ) {
            let result = file_states_to_stored_hashes(&file_states);
            prop_assert_eq!(result.len(), file_states.len(),
                "output must have same number of entries as input");
            for key in file_states.keys() {
                prop_assert!(result.contains_key(key),
                    "output must contain key '{}' present in input", key);
            }
        }

        #[test]
        fn proptest_file_states_to_stored_hashes_bitwise_field_identity(
            file_states in arb_file_states(20)
        ) {
            let result = file_states_to_stored_hashes(&file_states);
            for (key, raw) in &file_states {
                let stored = &result[key];
                prop_assert_eq!(
                    stored.content_hash,
                    ContentHash::from(raw.content_hash),
                    "content_hash must be bitwise identical for key '{}'", key
                );
                prop_assert_eq!(
                    stored.config_hash,
                    ContentHash::from(raw.config_hash),
                    "config_hash must be bitwise identical for key '{}'", key
                );
            }
        }

        #[test]
        fn proptest_compute_file_diff_partition_disjoint(
            file_count in 1usize..5,
            stored_extra in 0usize..3,
        ) {
            let source = TempDir::new().expect("temp dir");
            let mut discovery_files = Vec::new();
            let config_hash_val = compute_config_hash(None);
            let mut stored_hashes = HashMap::new();

            for i in 0..file_count {
                let name = format!("file_{i}.md");
                let content = format!("content {i}");
                fs::write(source.path().join(&name), &content).expect("write");
                discovery_files.push(DiscoveryFile {
                    source_path: name.clone(),
                    size_bytes: content.len() as u64,
                });
                // Store matching hash for deterministic unchanged
                stored_hashes.insert(name, StoredHashes {
                    content_hash: content_hash(content.as_bytes()),
                    config_hash: config_hash_val,
                });
            }

            // Add extra stored entries (not discovered = deleted)
            for i in 0..stored_extra {
                let name = format!("deleted_{i}.md");
                stored_hashes.insert(name, StoredHashes {
                    content_hash: ContentHash::from([0u8; 32]),
                    config_hash: config_hash_val,
                });
            }

            let result = compute_file_diff(&discovery_files, source.path(), None, &stored_hashes);
            let diff = match result {
                Ok(d) => d,
                Err(_) => return Ok(()),
            };

            // Check disjointness
            let all_keys: Vec<&String> = diff.unchanged.iter()
                .chain(diff.changed.iter())
                .chain(diff.new.iter())
                .chain(diff.deleted.iter())
                .collect();

            let unique_keys: HashSet<&&String> = all_keys.iter().collect();
            prop_assert_eq!(all_keys.len(), unique_keys.len(),
                "no path should appear in multiple buckets");
        }

        #[test]
        fn proptest_compute_file_diff_deterministic(
            file_count in 1usize..5,
        ) {
            let source = TempDir::new().expect("temp dir");
            let mut discovery_files = Vec::new();
            let mut stored_hashes = HashMap::new();
            let config_hash_val = compute_config_hash(None);

            for i in 0..file_count {
                let name = format!("det_file_{i}.md");
                let content = format!("deterministic content {i}");
                fs::write(source.path().join(&name), &content).expect("write");
                discovery_files.push(DiscoveryFile {
                    source_path: name.clone(),
                    size_bytes: content.len() as u64,
                });
                stored_hashes.insert(name, StoredHashes {
                    content_hash: content_hash(content.as_bytes()),
                    config_hash: config_hash_val,
                });
            }

            let diff1 = compute_file_diff(&discovery_files, source.path(), None, &stored_hashes)
                .expect("diff1");
            let diff2 = compute_file_diff(&discovery_files, source.path(), None, &stored_hashes)
                .expect("diff2");

            prop_assert_eq!(diff1, diff2, "deterministic: same inputs must produce same diff");
        }
    }
}
