//! Paranoid durability precondition tests.
//!
//! Validates that DurabilityConfig::Paranoid enforces extra safety checks:
//! zero hash rejection, empty string rejection, duplicate detection,
//! oversized payload rejection, missing reference detection, rollback
//! on failure, and read session blocking.

use super::*;
use crate::state::commit::MAX_VALUE_SIZE;
use crate::state::DurabilityConfig;

#[test]
fn commit_changes_with_paranoid_rejects_zero_hash_key() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_zero.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![([0u8; 32], vec![1, 2, 3])];

    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject zero hash");
    assert!(
        matches!(
            err,
            CommitError::ZeroHashKey {
                table: "analysis_outputs",
                index: 0
            }
        ),
        "expected ZeroHashKey(analysis_outputs, 0), got: {err}"
    );
}

#[test]
fn commit_changes_with_paranoid_rejects_empty_string_key() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_empty.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.updated_files = vec![(String::new(), FileStateRaw::zeroed())];

    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject empty key");
    assert!(
        matches!(
            err,
            CommitError::EmptyStringKey {
                table: "file_state",
                index: 0
            }
        ),
        "expected EmptyStringKey(file_state, 0), got: {err}"
    );
}

#[test]
fn commit_changes_with_paranoid_rejects_duplicate_keys() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_dup.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.updated_files = vec![
        ("dup.md".to_string(), FileStateRaw::zeroed()),
        ("dup.md".to_string(), FileStateRaw::zeroed()),
    ];

    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject duplicate");
    assert!(
        matches!(
            err,
            CommitError::DuplicateStateKey { table: "file_state", ref key }
                if key == "dup.md"
        ),
        "expected DuplicateStateKey(file_state, dup.md), got: {err}"
    );
}

#[test]
fn commit_changes_with_paranoid_rejects_oversized_payload() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_oversize.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "analysis_outputs",
                size: 52428801,
                max: 52428800,
            }
        ),
        "expected PayloadTooLarge(analysis_outputs), got: {err}"
    );
}

#[test]
fn commit_changes_with_paranoid_rejects_missing_reference() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_missing_ref.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.updated_files = vec![(
        "src/main.rs".to_string(),
        make_file_state_raw([1u8; 32], [0u8; 32], [0u8; 32]),
    )];
    // new_analyses is empty — [1u8; 32] not found

    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject missing ref");
    assert!(
        matches!(
            err,
            CommitError::MissingReference {
                table: "file_state",
                field: "analysis_hash",
                payload_table: "analysis_outputs",
                ..
            }
        ),
        "expected MissingReference(analysis_hash), got: {err}"
    );
}

#[test]
fn commit_changes_with_paranoid_rolls_back_on_validation_failure() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_rollback.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let mut changes = StateChanges::empty();
    changes.updated_files = vec![("valid.rs".to_string(), FileStateRaw::zeroed())];
    changes.new_analyses = vec![([0u8; 32], vec![1, 2, 3])];

    let result = state_db.commit_changes(changes);
    assert!(
        matches!(result, Err(CommitError::ZeroHashKey { .. })),
        "should fail with ZeroHashKey: {result:?}"
    );

    let db = state_db.database();
    assert!(
        read_string_table(db, file_state_table(), "valid.rs").is_none(),
        "no writes should be visible after validation failure"
    );
}

#[test]
fn commit_changes_with_paranoid_rejects_when_read_session_active() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_read_session.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    let _session = state_db.begin_read().expect("begin_read should succeed");

    let changes = StateChanges::empty();
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject with active session");

    assert!(
        matches!(err, CommitError::WriteTransaction { .. }),
        "should be WriteTransaction: {err}"
    );
    let msg = format!("{err}");
    assert!(
        msg.contains("read session"),
        "error should mention read session: {msg}"
    );
}

#[test]
fn read_session_drop_enables_commit_with_paranoid_durability() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_session_drop.redb");

    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");

    {
        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    let changes = StateChanges::empty();
    let result = state_db.commit_changes(changes);
    assert!(
        result.is_ok(),
        "commit should succeed after session dropped: {result:?}"
    );
}
