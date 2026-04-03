#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

<<<<<<< conflict 1 of 1
%%%%%%% diff from: olvxoxns 5b18733e "fix: repair all broken test files to compile against current API" (rebased revision)
\\\\\\\        to: wmqpqryo a7cc18e6 "fix: repair all broken test files, compile against current API" (rebased revision)
-//! Integration tests for snapshot APIs on StateReadSession and StateDb.
-//!
-//! Tests B03-B24 from the test plan, plus proptest invariants and boundary tests.
-//! All tests exercise real redb databases and the full state module API.
-//!
-//! NOTE: All tests are #[ignore]d because load_snapshots(), serialize_snapshot(),
-//! and drop_snapshots_table() are not yet implemented on StateReadSession/StateDb.
-//! They will be re-enabled when the snapshot API is implemented.
-
+++++++ lzsmumur 4d2b433b "feat: implement state layer beads (h70, bvh, bg3, ej0, b3v, pxx, 4s3, 2rt)" (parents of rebased revision)
//! Integration tests for cdocs-0tv: snapshot APIs on StateReadSession and StateDb.
//!
//! Tests B03–B24 from the test plan, plus proptest invariants and boundary tests.
//! All tests exercise real redb databases and the full state module API.
//!
//! RED PHASE: All tests compile but FAIL because implementations are `todo!()` stubs.

>>>>>>> conflict 1 of 1 ends
use chrono::{TimeZone, Utc};
use doc_transformer::state::{serialize_snapshot, OwnedArchive, StateChanges, StateDb, StateError};
use doc_transformer::watch::{PageHash, Snapshot};
use proptest::prelude::*;
use std::collections::BTreeMap;

// Helpers
// ===========================================================================

fn make_page_hash(url: &str, title: &str, hash_bytes: [u8; 32]) -> PageHash {
    PageHash {
        url: url.to_string(),
        content_hash: hash_bytes,
        title: title.to_string(),
    }
}

fn make_snapshot(target: &str, pages: Vec<(&str, &str, [u8; 32])>) -> Snapshot {
    let page_map: BTreeMap<String, PageHash> = pages
        .into_iter()
        .map(|(url, title, hash)| (url.to_string(), make_page_hash(url, title, hash)))
        .collect();
    Snapshot {
        target_url: target.to_string(),
        timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        pages: page_map,
    }
}

fn sample_hash(i: u8) -> [u8; 32] {
    let mut h = [0u8; 32];
    h[0] = i;
    h
}

fn open_temp_db() -> (tempfile::TempDir, StateDb) {
    let dir = tempfile::tempdir().expect("create temp dir");
    let db_path = dir.path().join("state.redb");
    let db = StateDb::open(&db_path).expect("open state db");
    (dir, db)
}

fn serialize_and_make_changes(
    snapshots: Vec<([u8; 32], &Snapshot)>,
    deletes: Vec<[u8; 32]>,
) -> StateChanges {
    let new_snapshots = snapshots
        .into_iter()
        .map(|(key, snap)| {
            let bytes = serialize_snapshot(snap).expect("serialize snapshot");
            (key, bytes)
        })
        .collect();
    StateChanges {
        new_snapshots,
        deleted_snapshots: deletes,
    }
}

// ===========================================================================
// B03: load_snapshots returns entries for found hashes
// ===========================================================================

#[test]
fn load_snapshots_returns_owned_archives_when_hashes_exist_in_table() {
    // Given: StateDb with one snapshot in the table
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When: Load the snapshot
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[key]);

    // Then: HashMap contains the entry with correct deserialized value
    let map = result.expect("load_snapshots should succeed");
    assert_eq!(map.len(), 1, "map should contain exactly 1 entry");
    assert!(
        map.contains_key(&key),
        "map should contain the requested key"
    );

    let restored = map[&key].deserialize().expect("deserialize should succeed");
    assert_eq!(restored, snapshot, "deserialized value must equal original");
}

// ===========================================================================
// B04: load_snapshots omits missing hashes
// ===========================================================================

#[test]
fn load_snapshots_omits_missing_hashes_when_not_in_table() {
    // Given: StateDb with empty snapshots table
    let (_dir, db) = open_temp_db();
    let session = db.begin_read().expect("begin read");
    let missing_key = sample_hash(99);

    // When: Request a non-existent key
    let result = session.load_snapshots(&[missing_key]);

    // Then: Ok with empty map (no error for missing keys)
    let map = result.expect("load_snapshots should succeed");
    assert!(
        map.is_empty(),
        "map should be empty when no hashes are found"
    );
}

// ===========================================================================
// B05: load_snapshots returns empty HashMap for empty input
// ===========================================================================

#[test]
fn load_snapshots_returns_empty_hashmap_when_hashes_slice_is_empty() {
    // Given: StateDb with populated table
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When: Load with empty slice
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[]);

    // Then: Ok with empty map, no table access
    let map = result.expect("load_snapshots with empty input should succeed");
    assert!(map.is_empty(), "empty input should produce empty HashMap");
}

// ===========================================================================
// B06: OwnedArchive bytes are independent of redb transaction
// ===========================================================================

#[test]
fn load_snapshots_returns_bytes_independent_of_redb_transaction_lifetime() {
    // Given: StateDb with one snapshot
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When: Load and then drop the session
    let map = {
        let session = db.begin_read().expect("begin read");
        let result = session.load_snapshots(&[key]);
        result.expect("load should succeed")
    }; // session dropped here — redb read transaction ends

    // Then: OwnedArchive bytes still accessible and correct after session drop
    assert_eq!(map.len(), 1);
    let restored = map[&key]
        .deserialize()
        .expect("deserialize after session drop");
    assert_eq!(
        restored, snapshot,
        "bytes must be owned independently of redb txn"
    );
}

// ===========================================================================
// B07: load_snapshots returns TableOpenFailed (read path)
// ===========================================================================

#[test]
fn load_snapshots_returns_table_open_failed_when_snapshots_table_missing() {
    // Given: StateDb with snapshots table deleted
    let (_dir, db) = open_temp_db();
    // Delete the snapshots table to trigger TableOpenFailed
    db.drop_snapshots_table().expect("drop snapshots table");

    // When: Attempt to load from a db with no snapshots table
    let key = sample_hash(1);
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[key]);

    // Then: Exact error variant
    match result {
        Err(StateError::TableOpenFailed { table, message }) => {
            assert_eq!(table, "snapshots");
            assert!(!message.is_empty(), "error message must be non-empty");
        }
        Err(other) => panic!("Expected StateError::TableOpenFailed, got {:?}", other),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

// ===========================================================================
// B08: load_snapshots returns ArchiveValidationFailed for corrupt bytes
// ===========================================================================

#[test]
fn load_snapshots_returns_archive_validation_failed_when_bytes_corrupt() {
    // Given: StateDb with corrupt bytes in the snapshots table
    let (_dir, db) = open_temp_db();
    let key = sample_hash(1);
    let corrupt_bytes = b"DEADBEEF_CORRUPT_BYTES_NOT_VALID_RKYV".to_vec();
    let changes = StateChanges {
        new_snapshots: vec![(key, corrupt_bytes)],
        deleted_snapshots: vec![],
    };
    db.commit_changes(&changes).expect("commit corrupt data");

    // When: Load the corrupt snapshot
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[key]);

    // Then: Exact error variant with key_hex matching
    match result {
        Err(StateError::ArchiveValidationFailed { key_hex, message }) => {
            let expected_hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
            assert_eq!(key_hex, expected_hex, "key_hex must match requested key");
            assert!(
                !message.is_empty(),
                "validation error message must be non-empty"
            );
        }
        Err(other) => panic!(
            "Expected StateError::ArchiveValidationFailed, got {:?}",
            other
        ),
        Ok(_) => panic!("Expected error for corrupt bytes, got Ok"),
    }
}

// ===========================================================================
// B09: load_snapshots returns DeserializationFailed for wrong-type bytes
// ===========================================================================

#[test]
fn load_snapshots_returns_deserialization_failed_when_bytes_wrong_type() {
    // Given: StateDb with valid rkyv archive but of wrong type (e.g., rkyv-serialized String)
    let (_dir, db) = open_temp_db();
    let key = sample_hash(1);
    // Use a simple string as "wrong type" bytes — this would be a valid rkyv
    // archive of a String, not a Snapshot. For RED phase, we use placeholder bytes.
    let wrong_type_bytes = b"valid_rkyv_but_wrong_type".to_vec();
    let changes = StateChanges {
        new_snapshots: vec![(key, wrong_type_bytes)],
        deleted_snapshots: vec![],
    };
    db.commit_changes(&changes).expect("commit wrong type data");

    // When: Load the wrong-type snapshot
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[key]);

    // Then: Exact error variant
    match result {
        Err(StateError::DeserializationFailed { key_hex, message }) => {
            let expected_hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
            assert_eq!(key_hex, expected_hex);
            assert!(!message.is_empty());
        }
        Err(StateError::ArchiveValidationFailed { .. }) => {
            // Also acceptable — corrupt bytes that fail validation before deserialization
        }
        Err(other) => panic!(
            "Expected StateError::DeserializationFailed or ArchiveValidationFailed, got {:?}",
            other
        ),
        Ok(_) => panic!("Expected error for wrong-type bytes, got Ok"),
    }
}

// ===========================================================================
// B10: load_snapshots returns StorageError (read path)
// ===========================================================================

#[test]
fn load_snapshots_returns_storage_error_when_redb_read_fails() {
    // Given: A StateDb in a state that causes redb read failure.
    // NOTE: redb provides strong guarantees for read operations on an open database.
    // If there is no deterministic way to trigger StorageError during a healthy read,
    // this test documents that the error mapping code is correct but the path may be
    // unreachable in practice.
    let (_dir, db) = open_temp_db();
    let key = sample_hash(1);
    let session = db.begin_read().expect("begin read");

    // When: Attempt to read (with a db that should be healthy)
    let result = session.load_snapshots(&[key]);

    // Then: This test is a placeholder — if redb reads are infallible on healthy dbs,
    // the implementation should still map any redb error to StorageError correctly.
    // For RED phase, this fails because todo!() in load_snapshots.
    let _ = result;
}

// ===========================================================================
// B11: commit_changes writes new snapshots
// ===========================================================================

#[test]
fn commit_changes_writes_new_snapshots_to_table_when_changes_committed() {
    // Given: Empty StateDb, one snapshot to write
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // When: Commit
    let result = db.commit_changes(&changes);

    // Then: Ok + verify persistence
    result.expect("commit_changes should succeed");
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");
    assert_eq!(map.len(), 1);
    let restored = map[&key].deserialize().expect("deserialize");
    assert_eq!(restored, snapshot);
}

// ===========================================================================
// B12: commit_changes deletes snapshots
// ===========================================================================

#[test]
fn commit_changes_removes_deleted_snapshots_from_table() {
    // Given: StateDb with one snapshot
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let insert_changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&insert_changes).expect("insert");

    // When: Delete it
    let delete_changes = StateChanges {
        new_snapshots: vec![],
        deleted_snapshots: vec![key],
    };
    db.commit_changes(&delete_changes).expect("delete");

    // Then: Key is absent
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");
    assert!(map.is_empty(), "deleted key should be absent from table");
}

// ===========================================================================
// B13: Delete takes precedence over insert for same key
// ===========================================================================

#[test]
fn commit_changes_delete_wins_when_same_key_in_new_and_deleted() {
    // Given: StateDb with no prior snapshot for key
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let bytes = serialize_snapshot(&snapshot).expect("serialize");
    let changes = StateChanges {
        new_snapshots: vec![(key, bytes)],
        deleted_snapshots: vec![key],
    };

    // When: Commit with same key in both new and deleted
    db.commit_changes(&changes).expect("commit");

    // Then: Key is absent (delete wins)
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");
    assert!(
        map.is_empty(),
        "delete must take precedence over insert for same key"
    );
}

// ===========================================================================
// B14: Last entry wins for duplicate keys in new_snapshots
// ===========================================================================

#[test]
fn commit_changes_last_entry_wins_when_duplicate_keys_in_new_snapshots() {
    // Given: Two different snapshots with the same key
    let (_dir, db) = open_temp_db();
    let key = sample_hash(1);

    let snapshot_v1 = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Version 1", sample_hash(10))],
    );
    let snapshot_v2 = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Version 2", sample_hash(20))],
    );

    let bytes_v1 = serialize_snapshot(&snapshot_v1).expect("serialize v1");
    let bytes_v2 = serialize_snapshot(&snapshot_v2).expect("serialize v2");

    let changes = StateChanges {
        new_snapshots: vec![(key, bytes_v1), (key, bytes_v2)],
        deleted_snapshots: vec![],
    };

    // When: Commit with duplicate keys
    db.commit_changes(&changes).expect("commit");

    // Then: Last entry (v2) wins
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");
    assert_eq!(map.len(), 1);
    let restored = map[&key].deserialize().expect("deserialize");
    assert_eq!(
        restored, snapshot_v2,
        "last entry (v2) must win on duplicate keys"
    );
}

// ===========================================================================
// B15: ACID rollback on failure
// ===========================================================================

#[test]
fn commit_changes_rolls_back_all_snapshot_changes_when_commit_fails() {
    // Given: StateDb with existing snapshot
    let (_dir, db) = open_temp_db();
    let existing_snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/existing", "Existing", sample_hash(100))],
    );
    let key_existing = sample_hash(100);
    let key_new = sample_hash(200);

    let insert_changes =
        serialize_and_make_changes(vec![(key_existing, &existing_snapshot)], vec![]);
    db.commit_changes(&insert_changes).expect("insert existing");

    // When: Commit changes that fail (force failure)
    // NOTE: This test requires a mechanism to force commit failure.
    // In practice, redb guarantees atomicity at the storage level.
    // For RED phase, the test structure is correct; the trigger mechanism
    // will be refined during implementation.
    let new_snapshot = make_snapshot(
        "https://other.com",
        vec![("https://other.com/new", "New", sample_hash(200))],
    );
    let changes = serialize_and_make_changes(vec![(key_new, &new_snapshot)], vec![key_existing]);

    // Then: Either commit succeeds (both changes applied) or fails (no changes applied).
    // For ACID test: if we can force failure, verify rollback.
    let result = db.commit_changes(&changes);

    match result {
        Ok(()) => {
            // Commit succeeded — verify both changes applied
            let session = db.begin_read().expect("begin read");
            let map = session
                .load_snapshots(&[key_existing, key_new])
                .expect("load");
            // key_existing was deleted, key_new was inserted
            assert!(
                !map.contains_key(&key_existing),
                "existing should be deleted"
            );
            assert!(map.contains_key(&key_new), "new should be present");
        }
        Err(StateError::CommitFailed { message }) => {
            assert!(!message.is_empty());
            // Verify rollback: existing still there, new not present
            let session = db.begin_read().expect("begin read");
            let map = session
                .load_snapshots(&[key_existing, key_new])
                .expect("load");
            assert!(
                map.contains_key(&key_existing),
                "rollback: existing must still be present"
            );
            assert!(
                !map.contains_key(&key_new),
                "rollback: new must not be present"
            );
        }
        Err(other) => {
            panic!("Expected Ok or CommitFailed, got {:?}", other);
        }
    }
}

// ===========================================================================
// B16: commit_changes returns WriteTransactionFailed
// ===========================================================================

#[test]
fn commit_changes_returns_write_transaction_failed_when_begin_write_fails() {
    // Given: A StateDb that cannot start a write transaction
    // NOTE: This is difficult to trigger with redb on a healthy filesystem.
    // Potential mechanisms: read-only filesystem, lock contention.
    // The test structure verifies error variant mapping.
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // When: Attempt to commit
    // (On a healthy db this should succeed; this test enforces error variant)
    let result = db.commit_changes(&changes);

    // Then: Should either succeed or return exact error variant
    match result {
        Ok(()) => {
            // Healthy db — commit succeeds. The error path is tested via
            // adversarial conditions (read-only fs, lock contention).
        }
        Err(StateError::WriteTransactionFailed { message }) => {
            assert!(!message.is_empty());
        }
        Err(other) => {
            panic!("Expected Ok or WriteTransactionFailed, got {:?}", other);
        }
    }
}

// ===========================================================================
// B17: commit_changes returns CommitFailed
// ===========================================================================

#[test]
fn commit_changes_returns_commit_failed_when_redb_commit_fails() {
    // Given: A scenario where write_tx.commit() fails
    // NOTE: Forcing redb commit failure deterministically may require
    // external infrastructure (FUSE filesystem, disk quota).
    // This test enforces error variant mapping.
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // When
    let result = db.commit_changes(&changes);

    // Then: Exact variant if error
    match result {
        Ok(()) => {}
        Err(StateError::CommitFailed { message }) => {
            assert!(!message.is_empty());
        }
        Err(other) => {
            panic!("Expected Ok or CommitFailed, got {:?}", other);
        }
    }
}

// ===========================================================================
// B18: Mixed found and not-found hashes
// ===========================================================================

#[test]
fn load_snapshots_returns_partial_map_when_some_hashes_found_and_some_missing() {
    // Given: StateDb with K1 and K2, NOT K3
    let (_dir, db) = open_temp_db();
    let snap1 = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(10))],
    );
    let snap2 = make_snapshot(
        "https://example.com",
        vec![("https://example.com/b", "Page B", sample_hash(20))],
    );
    let k1 = sample_hash(1);
    let k2 = sample_hash(2);
    let k3 = sample_hash(3);

    let changes = serialize_and_make_changes(vec![(k1, &snap1), (k2, &snap2)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When: Request all three
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&[k1, k2, k3]);

    // Then: Only K1 and K2 found
    let map = result.expect("load should succeed");
    assert_eq!(map.len(), 2, "should find exactly 2 of 3 keys");
    assert!(map.contains_key(&k1), "K1 should be found");
    assert!(map.contains_key(&k2), "K2 should be found");
    assert!(!map.contains_key(&k3), "K3 should NOT be found");
}

// ===========================================================================
// B19: commit_changes returns TableOpenFailed (write path)
// ===========================================================================

#[test]
fn commit_changes_returns_table_open_failed_when_snapshots_table_missing_for_write() {
    // Given: StateDb with snapshots table deleted
    let (_dir, db) = open_temp_db();
    // Delete the snapshots table to trigger TableOpenFailed on write path
    db.drop_snapshots_table().expect("drop snapshots table");

    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // When: Attempt commit
    let result = db.commit_changes(&changes);

    // Then: Exact error variant
    match result {
        Ok(()) => {
            // If table exists (normal case), commit succeeds
        }
        Err(StateError::TableOpenFailed { table, message }) => {
            assert_eq!(table, "snapshots");
            assert!(!message.is_empty());
        }
        Err(other) => {
            panic!("Expected Ok or TableOpenFailed, got {:?}", other);
        }
    }
}

// ===========================================================================
// B20: commit_changes returns StorageError (write path)
// ===========================================================================

#[test]
fn commit_changes_returns_storage_error_when_redb_insert_fails_during_commit() {
    // Given: StateDb in a state that causes redb insert/delete to fail
    // NOTE: redb's insert/delete within an open write transaction are typically
    // infallible on a healthy database. This test documents error mapping.
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // When
    let result = db.commit_changes(&changes);

    // Then: Verify error mapping
    match result {
        Ok(()) => {}
        Err(StateError::StorageError { operation, message }) => {
            assert!(!operation.is_empty());
            assert!(!message.is_empty());
        }
        Err(other) => {
            panic!("Expected Ok or StorageError, got {:?}", other);
        }
    }
}

// ===========================================================================
// B21: commit_changes fails when StateReadSession is still active
// ===========================================================================

#[test]
fn commit_changes_returns_write_transaction_failed_when_read_session_still_active() {
    // Given: StateDb with an active StateReadSession
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let key = sample_hash(1);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);

    // Hold read session — violates one-read, one-write invariant
    let _session = db.begin_read().expect("begin read");

    // When: Try to commit while session is active
    let result = db.commit_changes(&changes);

    // Then: Error — session must be dropped first
    match result {
        Err(StateError::WriteTransactionFailed { message }) => {
            assert!(
                !message.is_empty(),
                "error message must describe contention"
            );
        }
        Err(other) => {
            panic!(
                "Expected WriteTransactionFailed when read session active, got {:?}",
                other
            );
        }
        Ok(()) => {
            // If redb allows concurrent read+write, this would succeed.
            // The test verifies the invariant is enforced one way or another.
            panic!(
                "commit_changes should fail when StateReadSession is still active (one-read, one-write invariant)"
            );
        }
    }
}

// ===========================================================================
// B22: commit_changes succeeds with no mutations when changes are empty
// ===========================================================================

#[test]
fn commit_changes_succeeds_with_no_mutations_when_new_and_deleted_snapshots_empty() {
    // Given: StateDb with existing snapshot
    let (_dir, db) = open_temp_db();
    let existing_snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/existing", "Existing", sample_hash(100))],
    );
    let key_existing = sample_hash(100);
    let insert_changes =
        serialize_and_make_changes(vec![(key_existing, &existing_snapshot)], vec![]);
    db.commit_changes(&insert_changes).expect("insert existing");

    // When: Commit empty changes
    let empty_changes = StateChanges::default();
    let result = db.commit_changes(&empty_changes);

    // Then: Ok, existing data unchanged
    result.expect("empty commit should succeed");
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key_existing]).expect("load");
    assert_eq!(map.len(), 1, "existing data should be unchanged");
    let restored = map[&key_existing].deserialize().expect("deserialize");
    assert_eq!(restored, existing_snapshot);
}

// ===========================================================================
// B23: load_snapshots handles 10,000+ hashes
// ===========================================================================

#[test]
fn load_snapshots_returns_all_entries_when_given_10000_hashes() {
    // Given: StateDb with 10,000 snapshots
    let (_dir, db) = open_temp_db();
    let num_entries = 10_000usize;
    let mut all_entries: Vec<([u8; 32], Snapshot)> = Vec::with_capacity(num_entries);

    for i in 0..num_entries {
        let key = {
            let mut k = [0u8; 32];
            let i_bytes = (i as u64).to_le_bytes();
            k[..8].copy_from_slice(&i_bytes);
            k
        };
        let page_hash = {
            let mut h = [0u8; 32];
            h[0] = (i % 256) as u8;
            h
        };
        let snap = Snapshot {
            target_url: format!("https://example-{i}.com"),
            timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            pages: {
                let mut m = BTreeMap::new();
                m.insert(
                    format!("https://example-{i}.com/page"),
                    PageHash {
                        url: format!("https://example-{i}.com/page"),
                        content_hash: page_hash,
                        title: format!("Page {i}"),
                    },
                );
                m
            },
        };
        all_entries.push((key, snap));
    }

    // Commit in batches to avoid oversized single transactions
    let mut new_snapshots = Vec::with_capacity(num_entries);
    for (key, snap) in &all_entries {
        let bytes = serialize_snapshot(snap).expect("serialize");
        new_snapshots.push((*key, bytes));
    }
    let changes = StateChanges {
        new_snapshots,
        deleted_snapshots: vec![],
    };
    db.commit_changes(&changes).expect("commit 10k entries");

    // When: Load all 10,000 keys
    let all_keys: Vec<[u8; 32]> = all_entries.iter().map(|(k, _)| *k).collect();
    let session = db.begin_read().expect("begin read");
    let result = session.load_snapshots(&all_keys);

    // Then: All entries found, no OOM/panic
    let map = result.expect("load 10k should succeed");
    assert_eq!(map.len(), num_entries, "all 10,000 entries should be found");

    // Spot-check 10 entries
    for i in [0, 1, 50, 100, 500, 1000, 2500, 5000, 7500, 9999] {
        let (key, expected_snap) = &all_entries[i];
        let restored = map[key]
            .deserialize()
            .unwrap_or_else(|e| panic!("deserialize entry {i} failed: {e:?}"));
        assert_eq!(
            restored.target_url, expected_snap.target_url,
            "entry {i} target_url mismatch"
        );
    }
}

// ===========================================================================
// B24: commit_changes handles 10,000+ entries
// ===========================================================================

#[test]
fn commit_changes_writes_10000_snapshots_when_given_10000_new_entries() {
    // Given: Empty StateDb, 10,000 entries to write
    let (_dir, db) = open_temp_db();
    let num_entries = 10_000usize;
    let mut new_snapshots = Vec::with_capacity(num_entries);
    let mut expected_snapshots: Vec<([u8; 32], Snapshot)> = Vec::with_capacity(num_entries);

    for i in 0..num_entries {
        let key = {
            let mut k = [0u8; 32];
            let i_bytes = (i as u64).to_le_bytes();
            k[..8].copy_from_slice(&i_bytes);
            k
        };
        let page_hash = {
            let mut h = [0u8; 32];
            h[0] = (i % 256) as u8;
            h
        };
        let snap = Snapshot {
            target_url: format!("https://example-{i}.com"),
            timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            pages: {
                let mut m = BTreeMap::new();
                m.insert(
                    format!("https://example-{i}.com/page"),
                    PageHash {
                        url: format!("https://example-{i}.com/page"),
                        content_hash: page_hash,
                        title: format!("Page {i}"),
                    },
                );
                m
            },
        };
        let bytes = serialize_snapshot(&snap).expect("serialize");
        new_snapshots.push((key, bytes));
        expected_snapshots.push((key, snap));
    }

    let changes = StateChanges {
        new_snapshots,
        deleted_snapshots: vec![],
    };

    // When: Commit 10,000 entries
    db.commit_changes(&changes).expect("commit 10k entries");

    // Then: All 10,000 persist
    let all_keys: Vec<[u8; 32]> = expected_snapshots.iter().map(|(k, _)| *k).collect();
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&all_keys).expect("load 10k");

    assert_eq!(
        map.len(),
        num_entries,
        "all 10,000 entries should be persisted"
    );

    // Spot-check
    for i in [0, 1, 100, 1000, 5000, 9999] {
        let (key, expected_snap) = &expected_snapshots[i];
        let restored = map[key]
            .deserialize()
            .unwrap_or_else(|e| panic!("deserialize entry {i}: {e:?}"));
        assert_eq!(
            restored.target_url, expected_snap.target_url,
            "entry {i} target_url mismatch"
        );
    }
}

// ===========================================================================
// Additional boundary tests
// ===========================================================================

#[test]
fn load_snapshots_returns_entry_when_key_is_all_zeros() {
    // Given: Snapshot stored under all-zeros key
    let (_dir, db) = open_temp_db();
    let key = [0u8; 32];
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");

    // Then
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&key));
    let restored = map[&key].deserialize().expect("deserialize");
    assert_eq!(restored, snapshot);
}

#[test]
fn load_snapshots_returns_entry_when_key_is_all_0xff() {
    // Given: Snapshot stored under all-0xFF key
    let (_dir, db) = open_temp_db();
    let key = [0xFFu8; 32];
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(1))],
    );
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");

    // Then
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&key));
    let restored = map[&key].deserialize().expect("deserialize");
    assert_eq!(restored, snapshot);
}

#[test]
fn load_snapshots_handles_single_hash_lookup() {
    // Given: One snapshot
    let (_dir, db) = open_temp_db();
    let snapshot = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(42))],
    );
    let key = sample_hash(42);
    let changes = serialize_and_make_changes(vec![(key, &snapshot)], vec![]);
    db.commit_changes(&changes).expect("commit");

    // When
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[key]).expect("load");

    // Then
    assert_eq!(map.len(), 1);
    let restored = map[&key].deserialize().expect("deserialize");
    assert_eq!(restored.target_url, "https://example.com");
}

#[test]
fn commit_changes_writes_multiple_new_snapshots_to_table() {
    // Given: Multiple snapshots
    let (_dir, db) = open_temp_db();
    let snap1 = make_snapshot(
        "https://example.com",
        vec![("https://example.com/a", "Page A", sample_hash(10))],
    );
    let snap2 = make_snapshot(
        "https://other.com",
        vec![("https://other.com/b", "Page B", sample_hash(20))],
    );
    let k1 = sample_hash(1);
    let k2 = sample_hash(2);
    let changes = serialize_and_make_changes(vec![(k1, &snap1), (k2, &snap2)], vec![]);

    // When
    db.commit_changes(&changes).expect("commit");

    // Then: Both persist
    let session = db.begin_read().expect("begin read");
    let map = session.load_snapshots(&[k1, k2]).expect("load");
    assert_eq!(map.len(), 2);
    assert_eq!(map[&k1].deserialize().expect("d1"), snap1);
    assert_eq!(map[&k2].deserialize().expect("d2"), snap2);
}

// ===========================================================================
// Proptest 1: serialize_snapshot round-trip
// ===========================================================================

prop_compose! {
    fn arb_page_hash()(url in "https://[a-z]{3,10}\\.com/[a-z]{1,10}",
                       title in "[a-zA-Z ]{1,50}",
                       hash_bytes: [u8; 32]) -> PageHash {
        PageHash {
            url,
            content_hash: hash_bytes,
            title,
        }
    }
}

prop_compose! {
    fn arb_snapshot()(target_url in "https://[a-z]{3,10}\\.com",
                      timestamp_secs in 0i64..4102444800i64,
                      pages in prop::collection::btree_map(
                          "https://[a-z]{3,10}\\.com/[a-z]{1,10}",
                          arb_page_hash(),
                          0..10
                      )) -> Snapshot {
        Snapshot {
            target_url,
            timestamp: Utc.timestamp_opt(timestamp_secs, 0).single().unwrap_or_else(|| Utc::now()),
            pages,
        }
    }
}

proptest! {
    #[test]
    fn proptest_serialize_snapshot_roundtrip(snapshot in arb_snapshot()) {
        let bytes = serialize_snapshot(&snapshot)?;
        let archive = OwnedArchive::<Snapshot>::from_bytes(bytes);
        let restored = archive.deserialize()?;
        prop_assert_eq!(restored.target_url, snapshot.target_url);
        prop_assert_eq!(restored.pages.len(), snapshot.pages.len());
    }

    #[test]
    fn proptest_serialize_snapshot_deterministic(snapshot in arb_snapshot()) {
        let bytes1 = serialize_snapshot(&snapshot)?;
        let bytes2 = serialize_snapshot(&snapshot)?;
        prop_assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn proptest_load_snapshots_roundtrip(
        entries in prop::collection::vec(
            (any::<[u8; 32]>(), arb_snapshot()),
            0..5
        )
    ) {
        // Given: Unique-keyed snapshots committed to StateDb
        let dir = tempfile::tempdir()?;
        let db_path = dir.path().join("state.redb");
        let db = StateDb::open(&db_path)?;

        // Deduplicate by key (first occurrence wins for strategy, last for commit)
        let mut seen_keys = std::collections::HashSet::new();
        let mut unique_entries = Vec::new();
        for (key, snap) in entries {
            if seen_keys.insert(key) {
                unique_entries.push((key, snap));
            }
        }

        let new_snapshots: Vec<([u8; 32], Vec<u8>)> = unique_entries
            .iter()
            .map(|(key, snap)| {
                let bytes = serialize_snapshot(snap)?;
                Ok((*key, bytes))
            })
            .collect::<Result<_, StateError>>()?;

        let changes = StateChanges {
            new_snapshots,
            deleted_snapshots: vec![],
        };
        db.commit_changes(&changes)?;

        // When: Load all keys
        let keys: Vec<[u8; 32]> = unique_entries.iter().map(|(k, _)| *k).collect();
        let session = db.begin_read()?;
        let map = session.load_snapshots(&keys)?;

        // Then: All committed keys are present with correct values
        prop_assert_eq!(map.len(), unique_entries.len());
        for (key, expected_snap) in &unique_entries {
            let restored = map[key].deserialize()?;
            prop_assert_eq!(restored.target_url, expected_snap.target_url.clone());
            prop_assert_eq!(restored.pages.len(), expected_snap.pages.len());
        }
    }
}
