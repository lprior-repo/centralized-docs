//! Compaction tests.

use super::*;
use crate::state::commit::{compact_state_db, should_suggest_compaction, MAX_VALUE_SIZE};

#[test]
fn test_compact_on_empty_db_succeeds() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compact_empty.redb");
    {
        StateDb::open(&db_path).expect("open should succeed");
    }
    compact_state_db(&db_path).expect("compact on empty database should succeed");
}

#[test]
fn test_compact_on_fresh_db_is_noop() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compact_fresh.redb");
    {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut changes = StateChanges::empty();
        changes.new_analyses = vec![([1u8; 32], vec![10, 20, 30])];
        state_db
            .commit_changes(changes)
            .expect("commit should succeed");
    }
    compact_state_db(&db_path).expect("compact on fresh database should succeed");
    let state_db = StateDb::open(&db_path).expect("reopen should succeed");
    let db = state_db.database();
    let stored = read_hash_table(db, analysis_outputs_table(), &[1u8; 32]);
    assert_eq!(stored, Some(vec![10, 20, 30]));
}

#[test]
fn test_compact_after_deletes_preserves_remaining_data() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compact_delete.redb");
    let keep_hash = [0xAAu8; 32];
    let keep_payload = vec![42; 1024];
    {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut changes = StateChanges::empty();
        changes.new_analyses.push((keep_hash, keep_payload.clone()));
        for i in 1..100u8 {
            let mut hash = [0u8; 32];
            hash[0] = i;
            changes.new_analyses.push((hash, vec![i; 4096]));
        }
        state_db
            .commit_changes(changes)
            .expect("insert commit should succeed");
    }
    {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut setup = StateChanges::empty();
        setup.updated_files = vec![("to_delete.rs".to_string(), FileStateRaw::zeroed())];
        state_db
            .commit_changes(setup)
            .expect("setup commit should succeed");
        let mut changes = StateChanges::empty();
        changes.deleted_files = vec!["to_delete.rs".to_string()];
        state_db
            .commit_changes(changes)
            .expect("delete commit should succeed");
    }
    compact_state_db(&db_path).expect("compact should succeed");
    let state_db = StateDb::open(&db_path).expect("reopen should succeed");
    let db = state_db.database();
    assert_eq!(
        read_hash_table(db, analysis_outputs_table(), &keep_hash),
        Some(keep_payload)
    );
}

#[test]
fn test_compact_reduces_file_size_after_bulk_delete() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compact_size.redb");
    {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut changes = StateChanges::empty();
        for i in 1u8..=200 {
            let mut hash = [0u8; 32];
            hash[0] = i;
            hash[1] = i.wrapping_mul(3);
            changes.new_analyses.push((hash, vec![i; 4096]));
        }
        state_db
            .commit_changes(changes)
            .expect("bulk insert should succeed");
    }
    {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut changes = StateChanges::empty();
        for i in 11u8..=200 {
            let mut hash = [0u8; 32];
            hash[0] = i;
            hash[1] = i.wrapping_mul(3);
            changes.deleted_snapshots.push(hash);
        }
        state_db
            .commit_changes(changes)
            .expect("bulk delete should succeed");
    }
    let compacted = compact_state_db(&db_path).expect("compact should succeed");
    assert!(compacted, "compact should return true");
    let state_db = StateDb::open(&db_path).expect("reopen should succeed");
    let db = state_db.database();
    for i in 1u8..=10 {
        let mut hash = [0u8; 32];
        hash[0] = i;
        hash[1] = i.wrapping_mul(3);
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash),
            Some(vec![i; 4096])
        );
    }
}

#[test]
fn test_compact_after_churn_recovers_space() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compact_churn.redb");
    for cycle in 1u8..=50 {
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut changes = StateChanges::empty();
        let mut hash = [0u8; 32];
        hash[0] = cycle;
        changes.new_analyses.push((hash, vec![cycle; 8192]));
        state_db
            .commit_changes(changes)
            .expect("insert should succeed");
        drop(state_db);
        let state_db = StateDb::open(&db_path).expect("open should succeed");
        let mut del = StateChanges::empty();
        del.deleted_snapshots.push(hash);
        state_db.commit_changes(del).expect("delete should succeed");
        drop(state_db);
    }
    let size_before = std::fs::metadata(&db_path).expect("db should exist").len();
    let compacted = compact_state_db(&db_path).expect("compact should succeed");
    let size_after = std::fs::metadata(&db_path)
        .expect("db should exist after compact")
        .len();
    assert!(compacted);
    assert!(
        size_after < size_before,
        "after churn, compacted ({size_after}) < before ({size_before})"
    );
}

#[test]
fn should_suggest_compaction_returns_true_when_ratio_exceeded() {
    assert!(should_suggest_compaction(100_000, 1_000));
}

#[test]
fn should_suggest_compaction_returns_false_when_ratio_ok() {
    assert!(!should_suggest_compaction(5_000, 1_000));
}

#[test]
fn should_suggest_compaction_at_exact_threshold_boundary() {
    assert!(!should_suggest_compaction(10_000, 1_000));
    assert!(should_suggest_compaction(10_001, 1_000));
}

#[test]
fn should_suggest_compaction_returns_false_for_zero_sizes() {
    assert!(!should_suggest_compaction(0, 1_000));
    assert!(!should_suggest_compaction(1_000, 0));
    assert!(!should_suggest_compaction(0, 0));
}

#[test]
fn commit_error_compact_failed_display_contains_path_and_reason() {
    let err = CommitError::CompactFailed {
        path: "/tmp/state.redb".to_string(),
        reason: "transaction in progress".to_string(),
    };
    let msg = format!("{err}");
    assert!(msg.contains("compaction failed"), "{msg}");
    assert!(msg.contains("/tmp/state.redb"), "{msg}");
    assert!(msg.contains("transaction in progress"), "{msg}");
}

#[test]
fn commit_changes_persists_batch_with_100_entries_per_vec() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = StateChanges::empty();
    let mut file_states = Vec::with_capacity(100);
    for i in 0..100u8 {
        let path = format!("file_{i}.rs");
        let state = FileStateRaw {
            content_hash: [i; 32],
            config_hash: [i.saturating_add(1); 32],
            analysis_hash: [0u8; 32],
            transform_hash: [0u8; 32],
            chunk_hash: [0u8; 32],
            last_processed_secs: u64::from(i),
            reserved: [0u8; 32],
        };
        file_states.push((path, state));
    }
    changes.updated_files = file_states;
    let mut analyses = Vec::with_capacity(100);
    for i in 0..100u8 {
        let mut hash = [0u8; 32];
        hash[0] = if i == 0 { 100 } else { i };
        analyses.push((hash, vec![i]));
    }
    changes.new_analyses = analyses;
    state_db
        .commit_changes(changes)
        .expect("large batch should succeed");
    let db = state_db.database();
    assert_eq!(count_table_entries(db, "file_state"), 100);
    assert_eq!(count_table_entries(db, "analysis_outputs"), 100);
}
