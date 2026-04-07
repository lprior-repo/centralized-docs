//! Additional write tests (atomic mixed mutations, in-memory operations).

use super::*;
use crate::state::DurabilityConfig;

#[test]
fn commit_changes_applies_mixed_mutations_atomically_in_single_transaction() {
    let (state_db, _temp_dir) = create_temp_state_db();
    // Pre-populate with old data
    let old_hash = [0x99u8; 32];
    let mut setup = make_minimal_valid_state_changes();
    setup.updated_files = vec![("old.md".to_string(), FileStateRaw::zeroed())];
    setup.new_analyses = vec![(old_hash, vec![0])];
    state_db.commit_changes(setup).expect("setup commit");
    // Mixed batch: upsert new, delete old, add payloads
    let hash_new = [0xA1u8; 32];
    let hash_t = [0xA2u8; 32];
    let hash_c = [0xA3u8; 32];
    let hash_s = [0xA4u8; 32];
    let hash_snap = [0xA5u8; 32];
    let new_file = FileStateRaw {
        content_hash: [1; 32],
        config_hash: [2; 32],
        analysis_hash: hash_new,
        transform_hash: hash_t,
        chunk_hash: hash_c,
        last_processed_secs: 42,
        reserved: [0u8; 32],
    };
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("new.md".to_string(), new_file)];
    changes.deleted_files = vec!["old.md".to_string()];
    changes.new_analyses = vec![(hash_new, vec![10, 20]), (old_hash, vec![99])];
    changes.new_transforms = vec![(hash_t, vec![30])];
    changes.new_chunks = vec![(hash_c, vec![40])];
    changes.new_scrapes = vec![(hash_s, vec![50])];
    changes.new_snapshots = vec![(hash_snap, vec![60])];
    state_db.commit_changes(changes).expect("mixed commit");
    let db = state_db.database();
    // Old deleted
    assert!(read_string_table(db, file_state_table(), "old.md").is_none());
    // New present
    assert!(read_string_table(db, file_state_table(), "new.md").is_some());
    // All payloads present
    assert_eq!(
        read_hash_table(db, analysis_outputs_table(), &hash_new),
        Some(vec![10, 20])
    );
    assert_eq!(
        read_hash_table(db, transform_outputs_table(), &hash_t),
        Some(vec![30])
    );
    assert_eq!(
        read_hash_table(db, chunk_outputs_table(), &hash_c),
        Some(vec![40])
    );
    assert_eq!(
        read_hash_table(db, scrape_outputs_table(), &hash_s),
        Some(vec![50])
    );
    assert_eq!(
        read_hash_table(db, snapshots_table(), &hash_snap),
        Some(vec![60])
    );
}

#[test]
fn state_db_open_in_memory_succeeds() {
    let state_db = StateDb::open_in_memory().expect("in-memory open should succeed");
    assert_eq!(state_db.durability_config(), DurabilityConfig::Default);
    let _session = state_db.begin_read().expect("begin_read should succeed");
}

#[test]
fn state_db_builder_open_in_memory_with_paranoid_succeeds() {
    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open_in_memory()
        .expect("in-memory open should succeed");
    assert_eq!(state_db.durability_config(), DurabilityConfig::Paranoid);
}

#[test]
fn state_db_in_memory_commit_and_read() {
    let state_db = StateDb::open_in_memory().expect("in-memory open should succeed");
    let hash_key = [1u8; 32];
    let payload = vec![10, 20, 30];
    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("commit should succeed");
    assert_eq!(
        read_hash_table(state_db.database(), analysis_outputs_table(), &hash_key),
        Some(payload)
    );
}

#[test]
fn state_db_in_memory_paranoid_commit_and_read() {
    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open_in_memory()
        .expect("in-memory open should succeed");
    let hash_key = [2u8; 32];
    let payload = vec![99, 88, 77];
    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![(hash_key, payload.clone())];
    state_db
        .commit_changes(changes)
        .expect("paranoid commit should succeed");
    assert_eq!(
        read_hash_table(state_db.database(), analysis_outputs_table(), &hash_key),
        Some(payload)
    );
}
