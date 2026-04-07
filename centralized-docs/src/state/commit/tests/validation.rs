//! Validation precondition tests (zero hash, empty keys, duplicates, references, payload sizes).

use super::*;
use crate::state::commit::MAX_VALUE_SIZE;

#[test]
fn commit_changes_rejects_zero_hash_key_in_analysis_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
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
fn commit_changes_reports_index_2_for_zero_hash_in_analyses() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_analyses = vec![
        ([1u8; 32], vec![10]),
        ([2u8; 32], vec![20]),
        ([0u8; 32], vec![30]),
    ];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject zero hash");
    assert!(
        matches!(
            err,
            CommitError::ZeroHashKey {
                table: "analysis_outputs",
                index: 2
            }
        ),
        "expected ZeroHashKey(analysis_outputs, 2), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_zero_hash_key_in_transform_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_transforms = vec![([0u8; 32], vec![1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject zero hash");
    assert!(
        matches!(
            err,
            CommitError::ZeroHashKey {
                table: "transform_outputs",
                index: 0
            }
        ),
        "expected ZeroHashKey(transform_outputs, 0), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_zero_hash_key_in_chunk_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_chunks = vec![([0u8; 32], vec![1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject zero hash");
    assert!(
        matches!(
            err,
            CommitError::ZeroHashKey {
                table: "chunk_outputs",
                index: 0
            }
        ),
        "expected ZeroHashKey(chunk_outputs, 0), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_zero_hash_key_in_scrape_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_scrapes = vec![([0u8; 32], vec![1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject zero hash");
    assert!(
        matches!(
            err,
            CommitError::ZeroHashKey {
                table: "scrape_outputs",
                index: 0
            }
        ),
        "expected ZeroHashKey(scrape_outputs, 0), got: {err}"
    );
}

#[test]
fn commit_changes_accepts_zero_hash_key_in_snapshots() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_snapshots = vec![([0u8; 32], vec![1])];
    let result = state_db.commit_changes(changes);
    assert!(
        result.is_ok(),
        "zero hash in snapshots should be accepted, got: {result:?}"
    );
}

#[test]
fn commit_changes_rejects_empty_source_path_in_updated_files() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
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
fn commit_changes_rejects_empty_url_in_updated_urls() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_urls = vec![(String::new(), UrlStateRaw::zeroed())];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject empty key");
    assert!(
        matches!(
            err,
            CommitError::EmptyStringKey {
                table: "url_state",
                index: 0
            }
        ),
        "expected EmptyStringKey(url_state, 0), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_whitespace_only_source_path() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("   \t\n".to_string(), FileStateRaw::zeroed())];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject whitespace key");
    assert!(
        matches!(
            err,
            CommitError::EmptyStringKey {
                table: "file_state",
                ..
            }
        ),
        "expected EmptyStringKey(file_state), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_whitespace_only_url() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_urls = vec![("  \t ".to_string(), UrlStateRaw::zeroed())];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject whitespace key");
    assert!(
        matches!(
            err,
            CommitError::EmptyStringKey {
                table: "url_state",
                ..
            }
        ),
        "expected EmptyStringKey(url_state), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_duplicate_source_path_in_updated_files() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![
        ("dup.rs".to_string(), FileStateRaw::zeroed()),
        ("dup.rs".to_string(), FileStateRaw::zeroed()),
    ];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject duplicate");
    assert!(
        matches!(err, CommitError::DuplicateStateKey { table: "file_state", ref key } if key == "dup.rs"),
        "expected DuplicateStateKey(file_state, dup.rs), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_duplicate_url_in_updated_urls() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_urls = vec![
        ("https://dup.com".to_string(), UrlStateRaw::zeroed()),
        ("https://dup.com".to_string(), UrlStateRaw::zeroed()),
    ];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject duplicate");
    assert!(
        matches!(
            err,
            CommitError::DuplicateStateKey {
                table: "url_state",
                ..
            }
        ),
        "expected DuplicateStateKey(url_state), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_missing_analysis_hash_reference() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(
        "test.rs".to_string(),
        make_file_state_raw([1u8; 32], [0u8; 32], [0u8; 32]),
    )];
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
fn commit_changes_rejects_missing_transform_hash_reference() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(
        "test.rs".to_string(),
        make_file_state_raw([0u8; 32], [1u8; 32], [0u8; 32]),
    )];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject missing ref");
    assert!(
        matches!(
            err,
            CommitError::MissingReference {
                field: "transform_hash",
                ..
            }
        ),
        "expected MissingReference(transform_hash), got: {err}"
    );
}
