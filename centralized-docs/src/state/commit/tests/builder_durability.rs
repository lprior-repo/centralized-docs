//! Builder, durability, in-memory database, and `db_path` accessor tests.

use super::*;
use crate::state::DurabilityConfig;

#[test]
fn durability_config_satisfies_debug_clone_copy_partial_eq() {
    fn assert_traits<T: std::fmt::Debug + Clone + Copy + PartialEq + Eq>() {}
    assert_traits::<DurabilityConfig>();
}

#[test]
fn durability_config_default_returns_default_variant() {
    assert_eq!(DurabilityConfig::default(), DurabilityConfig::Default);
}

#[test]
fn state_db_builder_new_has_64mib_cache_and_default_durability() {
    let builder = StateDbBuilder::new();
    assert_eq!(builder.cache_size, 67_108_864);
    assert_eq!(builder.durability, DurabilityConfig::Default);
}

#[test]
fn state_db_builder_default_equals_new() {
    let a = StateDbBuilder::new();
    let b = StateDbBuilder::default();
    assert_eq!(a.cache_size, b.cache_size);
    assert_eq!(a.durability, b.durability);
}

#[test]
fn state_db_builder_cache_size_returns_updated_builder() {
    assert_eq!(
        StateDbBuilder::new()
            .cache_size(128 * 1024 * 1024)
            .cache_size,
        134_217_728
    );
}

#[test]
fn state_db_builder_cache_size_zero_is_accepted() {
    assert_eq!(StateDbBuilder::new().cache_size(0).cache_size, 0);
}

#[test]
fn state_db_builder_durability_returns_updated_builder() {
    assert_eq!(
        StateDbBuilder::new()
            .durability(DurabilityConfig::Paranoid)
            .durability,
        DurabilityConfig::Paranoid
    );
}

#[test]
fn state_db_builder_open_creates_parent_directories_when_absent() {
    let temp_dir = TempDir::new().unwrap();
    let nested_path = temp_dir.path().join("does_not_exist_xxx/sub/state.redb");
    let state_db = StateDbBuilder::new()
        .open(&nested_path)
        .expect("builder open should succeed");
    assert!(temp_dir.path().join("does_not_exist_xxx/sub").is_dir());
    let _session = state_db.begin_read().expect("begin_read should succeed");
}

#[test]
fn state_db_builder_open_returns_state_db_with_configured_durability() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("configured.redb");
    let state_db = StateDbBuilder::new()
        .cache_size(32 * 1024 * 1024)
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("builder open should succeed");
    assert_eq!(state_db.durability_config(), DurabilityConfig::Paranoid);
}

#[test]
fn state_db_builder_open_returns_database_open_when_path_invalid() {
    let path = std::path::Path::new("/nonexistent_root_xyz_cdocs/deeply/nested/state.redb");
    assert!(matches!(
        StateDbBuilder::new().open(path).expect_err("should fail"),
        CommitError::DatabaseOpen { .. }
    ));
}

#[test]
fn state_db_builder_open_preserves_fallback_open_then_create_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("fallback.redb");
    std::fs::write(
        &db_path,
        b"THIS IS NOT A VALID REDB DATABASE - CORRUPT DATA!!",
    )
    .unwrap();
    assert!(matches!(
        StateDbBuilder::new().open(&db_path),
        Err(CommitError::DatabaseOpen { .. })
    ));
}

#[test]
fn state_db_open_creates_parent_directories_when_missing() {
    let temp_dir = TempDir::new().unwrap();
    let nested_path = temp_dir.path().join("deeply/nested/dir/state.redb");
    StateDb::open(&nested_path).expect("open should succeed");
    assert!(temp_dir.path().join("deeply/nested/dir").is_dir());
}

#[test]
fn state_db_open_handles_filename_only_path_without_create_dir() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let state_db = StateDb::open(&db_path).expect("open should succeed");
    let _session = state_db.begin_read().expect("begin_read should succeed");
}

#[test]
fn state_db_open_succeeds_with_unicode_and_spaces_in_path() {
    let temp_dir = TempDir::new().unwrap();
    let unicode_path = temp_dir.path().join("path with spaces/数据库/state.redb");
    let state_db = StateDb::open(&unicode_path).expect("open with unicode path should succeed");
    let _session = state_db.begin_read().expect("begin_read should succeed");
}

#[test]
#[cfg(unix)]
fn state_db_open_returns_database_open_error_on_read_only_parent() {
    use std::os::unix::fs::PermissionsExt;
    let temp_dir = TempDir::new().unwrap();
    let readonly_dir = temp_dir.path().join("readonly");
    std::fs::create_dir(&readonly_dir).unwrap();
    std::fs::set_permissions(&readonly_dir, std::fs::Permissions::from_mode(0o444)).unwrap();
    let db_path = readonly_dir.join("state.redb");
    let msg = format!("{}", StateDb::open(&db_path).expect_err("should fail"));
    assert!(
        msg.contains("readonly") || msg.contains("failed to open"),
        "{msg}"
    );
    let _ = std::fs::set_permissions(&readonly_dir, std::fs::Permissions::from_mode(0o755));
}

#[test]
fn state_db_open_creates_deeply_nested_parent_directories() {
    let temp_dir = TempDir::new().unwrap();
    let deep_path = temp_dir.path().join("a/b/c/d/e/f/g/h/i/j/state.redb");
    StateDb::open(&deep_path).expect("open should succeed");
    assert!(temp_dir.path().join("a/b/c/d/e/f/g/h/i/j").is_dir());
}

#[test]
fn state_db_durability_config_returns_configured_value() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("accessor.redb");
    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");
    assert_eq!(state_db.durability_config(), DurabilityConfig::Paranoid);
}

#[test]
fn state_db_open_returns_default_durability_for_backward_compat() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compat.redb");
    let state_db = StateDb::open(&db_path).expect("open should succeed");
    assert_eq!(state_db.durability_config(), DurabilityConfig::Default);
}

#[test]
fn state_db_open_is_equivalent_to_builder_new_open() {
    let temp_dir_a = TempDir::new().unwrap();
    let temp_dir_b = TempDir::new().unwrap();
    let db_a = StateDb::open(&temp_dir_a.path().join("a.redb")).expect("open a");
    let db_b = StateDbBuilder::new()
        .open(&temp_dir_b.path().join("b.redb"))
        .expect("open b");
    assert_eq!(db_a.durability_config(), db_b.durability_config());
}

#[test]
fn commit_changes_with_default_durability_commits_successfully() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("default_commit.redb");
    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Default)
        .open(&db_path)
        .expect("open should succeed");
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
fn commit_changes_with_paranoid_durability_commits_and_data_readable() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_commit.redb");
    let state_db = StateDbBuilder::new()
        .durability(DurabilityConfig::Paranoid)
        .open(&db_path)
        .expect("open should succeed");
    let hash_key = [1u8; 32];
    let payload = vec![42, 43, 44];
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

#[test]
fn paranoid_commit_data_survives_database_reopen_cycle() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("paranoid_reopen.redb");
    let key = "test.md";
    let original = FileStateRaw {
        content_hash: [0xAB; 32],
        config_hash: [0xCD; 32],
        analysis_hash: [0u8; 32],
        transform_hash: [0u8; 32],
        chunk_hash: [0u8; 32],
        last_processed_secs: 999,
        reserved: [0u8; 32],
    };
    {
        let state_db = StateDbBuilder::new()
            .durability(DurabilityConfig::Paranoid)
            .open(&db_path)
            .expect("open should succeed");
        let mut changes = StateChanges::empty();
        changes.updated_files = vec![(key.to_string(), original)];
        state_db
            .commit_changes(changes)
            .expect("paranoid commit should succeed");
    }
    let state_db = StateDb::open(&db_path).expect("reopen should succeed");
    assert!(read_string_table(state_db.database(), file_state_table(), key).is_some());
}

#[test]
fn state_db_db_path_returns_path_for_on_disk_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("path_test.redb");
    let state_db = StateDb::open(&db_path).expect("open should succeed");
    assert_eq!(state_db.db_path(), Some(db_path.as_path()));
}

#[test]
fn state_db_db_path_returns_none_for_in_memory_database() {
    assert!(StateDb::open_in_memory()
        .expect("in-memory open")
        .db_path()
        .is_none());
}
