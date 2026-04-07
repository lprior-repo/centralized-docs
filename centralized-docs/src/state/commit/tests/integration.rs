//! More validation tests: missing references, payload sizes, error displays, boundary tests.

use super::*;
use crate::state::commit::should_skip_write;
use crate::state::commit::MAX_VALUE_SIZE;

#[test]
fn commit_changes_rejects_missing_chunk_hash_reference() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![(
        "test.rs".to_string(),
        make_file_state_raw([0u8; 32], [0u8; 32], [1u8; 32]),
    )];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject missing ref");
    assert!(
        matches!(
            err,
            CommitError::MissingReference {
                field: "chunk_hash",
                ..
            }
        ),
        "expected MissingReference(chunk_hash), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_missing_url_hash_reference() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_urls = vec![(
        "https://example.com".to_string(),
        make_url_state_raw([1u8; 32]),
    )];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject missing ref");
    assert!(
        matches!(
            err,
            CommitError::MissingReference {
                field: "url_hash",
                ..
            }
        ),
        "expected MissingReference(url_hash), got: {err}"
    );
}

#[test]
fn commit_changes_accepts_zero_hashes_as_no_output() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.updated_files = vec![("test.rs".to_string(), FileStateRaw::zeroed())];
    changes.updated_urls = vec![("https://example.com".to_string(), UrlStateRaw::zeroed())];
    let result = state_db.commit_changes(changes);
    assert!(result.is_ok(), "zero hashes should be accepted: {result:?}");
}

#[test]
fn commit_changes_rejects_payload_exceeding_max_value_size_in_analysis_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "analysis_outputs",
                ..
            }
        ),
        "expected PayloadTooLarge(analysis_outputs), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_payload_exceeding_max_value_size_in_transform_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_transforms = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "transform_outputs",
                ..
            }
        ),
        "expected PayloadTooLarge(transform_outputs), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_payload_exceeding_max_value_size_in_chunk_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_chunks = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "chunk_outputs",
                ..
            }
        ),
        "expected PayloadTooLarge(chunk_outputs), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_payload_exceeding_max_value_size_in_scrape_outputs() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_scrapes = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "scrape_outputs",
                ..
            }
        ),
        "expected PayloadTooLarge(scrape_outputs), got: {err}"
    );
}

#[test]
fn commit_changes_rejects_payload_exceeding_max_value_size_in_snapshots() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = make_minimal_valid_state_changes();
    changes.new_snapshots = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];
    let err = state_db
        .commit_changes(changes)
        .expect_err("should reject oversized payload");
    assert!(
        matches!(
            err,
            CommitError::PayloadTooLarge {
                table: "snapshots",
                ..
            }
        ),
        "expected PayloadTooLarge(snapshots), got: {err}"
    );
}

#[test]
fn should_skip_write_returns_true_when_bytes_identical() {
    assert!(should_skip_write(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn should_skip_write_returns_false_when_bytes_differ() {
    assert!(!should_skip_write(&[1, 2, 3], &[1, 2, 4]));
}

#[test]
fn should_skip_write_returns_false_for_large_differing_inputs() {
    let large_a = vec![0xFFu8; 1_048_576];
    let large_b = vec![0xFEu8; 1_048_576];
    assert!(!should_skip_write(&large_a, &large_b));
    assert!(should_skip_write(&large_a, &large_a.clone()));
}

#[test]
fn commit_changes_accepts_payload_exactly_at_max_value_size_boundary() {
    let (state_db, _temp_dir) = create_temp_state_db();
    let mut changes = StateChanges::empty();
    changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE])];
    let result = state_db.commit_changes(changes);
    assert!(
        result.is_ok(),
        "payload at exactly MAX_VALUE_SIZE should be accepted: {result:?}"
    );
}

// -- Error variant display tests --

#[test]
fn commit_error_table_init_display_contains_reason() {
    let err = CommitError::TableInit {
        reason: "corrupt tables".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("corrupt tables"),
        "TableInit display should contain reason: {msg}"
    );
    assert!(
        msg.contains("initialize tables"),
        "TableInit display should mention table init: {msg}"
    );
}

#[test]
fn commit_error_read_transaction_display_contains_reason() {
    let err = CommitError::ReadTransaction {
        reason: "read tx failed".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("read tx failed"),
        "ReadTransaction display should contain reason: {msg}"
    );
}

#[test]
fn commit_error_write_transaction_display_contains_reason() {
    let err = CommitError::WriteTransaction {
        reason: "write tx failed".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("write tx failed"),
        "WriteTransaction display should contain reason: {msg}"
    );
}

#[test]
fn commit_error_commit_failed_display_contains_reason() {
    let err = CommitError::CommitFailed {
        reason: "commit aborted".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("commit aborted"),
        "CommitFailed display should contain reason: {msg}"
    );
}

#[test]
fn commit_error_read_failed_display_contains_table_and_reason() {
    let err = CommitError::ReadFailed {
        table: "analysis_outputs",
        reason: "disk error".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("analysis_outputs"),
        "ReadFailed display should contain table name: {msg}"
    );
    assert!(
        msg.contains("disk error"),
        "ReadFailed display should contain reason: {msg}"
    );
}

#[test]
fn commit_error_write_failed_display_contains_table_and_reason() {
    let err = CommitError::WriteFailed {
        table: "file_state",
        reason: "disk full".to_string(),
    };
    let msg = format!("{err}");
    assert!(
        msg.contains("file_state"),
        "Display should contain table name: {msg}"
    );
    assert!(
        msg.contains("disk full"),
        "Display should contain reason: {msg}"
    );
}
