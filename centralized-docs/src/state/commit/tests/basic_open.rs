//! Basic StateDb::open and begin_read tests.

use super::*;
use crate::state::{file_state_table, metadata_table, source_path_chunks_table};

#[test]
fn state_db_open_returns_ok_when_path_valid() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let session = state_db.begin_read();
    assert!(
        session.is_ok(),
        "begin_read should succeed on valid StateDb"
    );
}

#[test]
fn state_db_open_returns_error_on_corrupt_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    std::fs::write(
        &db_path,
        b"THIS IS NOT A VALID REDB DATABASE - CORRUPT DATA!!",
    )
    .unwrap();
    let result = StateDb::open(&db_path);
    let err = result.expect_err("should fail on corrupt database");
    let msg = format!("{err}");
    assert!(
        msg.contains("failed to open"),
        "error should mention open failure: {msg}"
    );
    assert!(
        matches!(err, CommitError::DatabaseOpen { .. }),
        "should be DatabaseOpen variant, got: {err}"
    );
}

#[test]
fn state_db_open_returns_database_open_error_when_path_invalid() {
    let path = std::path::Path::new("/nonexistent_root_xyz_cdocs/deeply/nested/state.redb");
    let result = StateDb::open(path);
    let err = result.expect_err("should fail for nonexistent root");
    let msg = format!("{err}");
    assert!(
        msg.contains("nonexistent_root_xyz_cdocs"),
        "error should reference path: {msg}"
    );
}

#[test]
fn state_db_open_returns_database_open_error_when_path_is_empty() {
    let path = std::path::Path::new("");
    let result = StateDb::open(path);
    let err = result.expect_err("should fail for empty path");
    let msg = format!("{err}");
    assert!(
        msg.contains("failed to open"),
        "error should mention open failure: {msg}"
    );
}

#[test]
fn state_db_begin_read_returns_session_when_db_open() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let _session = state_db.begin_read().expect("begin_read should succeed");
}

#[test]
fn database_returns_reference_to_underlying_redb_database() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let db_ref = state_db.database();
    let read_txn = db_ref
        .begin_read()
        .expect("begin_read on returned &Database should succeed");
    read_txn.open_table(file_state_table()).unwrap();
    read_txn.open_table(url_state_table()).unwrap();
    read_txn.open_table(analysis_outputs_table()).unwrap();
    read_txn.open_table(transform_outputs_table()).unwrap();
    read_txn.open_table(chunk_outputs_table()).unwrap();
    read_txn.open_table(scrape_outputs_table()).unwrap();
    read_txn.open_table(snapshots_table()).unwrap();
    read_txn.open_table(metadata_table()).unwrap();
    read_txn
        .open_multimap_table(source_path_chunks_table())
        .unwrap();
}

#[test]
fn state_changes_empty_creates_batch_with_all_empty_vecs() {
    let changes = StateChanges::empty();
    assert_eq!(changes.updated_files.len(), 0);
    assert_eq!(changes.deleted_files.len(), 0);
    assert_eq!(changes.new_analyses.len(), 0);
    assert_eq!(changes.new_transforms.len(), 0);
    assert_eq!(changes.new_chunks.len(), 0);
    assert_eq!(changes.updated_urls.len(), 0);
    assert_eq!(changes.deleted_urls.len(), 0);
    assert_eq!(changes.new_scrapes.len(), 0);
    assert_eq!(changes.new_snapshots.len(), 0);
    assert_eq!(changes.deleted_snapshots.len(), 0);
}

#[test]
fn state_changes_default_equals_empty() {
    let default = StateChanges::default();
    let empty = StateChanges::empty();
    assert_eq!(default.updated_files.len(), empty.updated_files.len());
    assert_eq!(default.deleted_files.len(), empty.deleted_files.len());
    assert_eq!(default.new_analyses.len(), empty.new_analyses.len());
    assert_eq!(default.new_transforms.len(), empty.new_transforms.len());
    assert_eq!(default.new_chunks.len(), empty.new_chunks.len());
    assert_eq!(default.updated_urls.len(), empty.updated_urls.len());
    assert_eq!(default.deleted_urls.len(), empty.deleted_urls.len());
    assert_eq!(default.new_scrapes.len(), empty.new_scrapes.len());
    assert_eq!(default.new_snapshots.len(), empty.new_snapshots.len());
    assert_eq!(
        default.deleted_snapshots.len(),
        empty.deleted_snapshots.len()
    );
}
