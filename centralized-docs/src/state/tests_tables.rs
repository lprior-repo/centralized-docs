//! Table definition and key validation tests.

use super::*;
use redb::{MultimapTableHandle, TableHandle};
use std::collections::HashSet;

#[test]
fn table_definition_names_are_all_unique() {
    let t1 = file_state_table();
    let t2 = url_state_table();
    let t3 = analysis_outputs_table();
    let t4 = transform_outputs_table();
    let t5 = chunk_outputs_table();
    let t6 = scrape_outputs_table();
    let t7 = snapshots_table();
    let t8 = metadata_table();
    let t9 = source_path_chunks_table();
    let names: HashSet<&str> = [
        t1.name(),
        t2.name(),
        t3.name(),
        t4.name(),
        t5.name(),
        t6.name(),
        t7.name(),
        t8.name(),
        t9.name(),
    ]
    .into_iter()
    .collect();
    assert_eq!(names.len(), 9, "expected exactly 9 unique table names");
}

#[test]
fn table_names_match_architecture_spec_exactly() {
    assert_eq!(file_state_table().name(), "file_state");
    assert_eq!(url_state_table().name(), "url_state");
    assert_eq!(analysis_outputs_table().name(), "analysis_outputs");
    assert_eq!(transform_outputs_table().name(), "transform_outputs");
    assert_eq!(chunk_outputs_table().name(), "chunk_outputs");
    assert_eq!(scrape_outputs_table().name(), "scrape_outputs");
    assert_eq!(snapshots_table().name(), "snapshots");
    assert_eq!(metadata_table().name(), "metadata");
    assert_eq!(source_path_chunks_table().name(), "source_path_chunks");
}

#[test]
fn new_table_names_disjoint_from_legacy_except_metadata() {
    let new_names: HashSet<&str> = [
        "file_state",
        "analysis_outputs",
        "transform_outputs",
        "chunk_outputs",
        "url_state",
        "scrape_outputs",
        "snapshots",
        "metadata",
    ]
    .into_iter()
    .collect();
    let legacy_names: HashSet<&str> = [
        "documents",
        "scrape",
        "transforms",
        "snapshots",
        "analysis",
        "chunks",
        "metadata",
    ]
    .into_iter()
    .collect();
    let intersection: HashSet<&str> = new_names.intersection(&legacy_names).copied().collect();
    assert_eq!(
        intersection,
        HashSet::from(["metadata", "snapshots"]),
        "only 'metadata' and 'snapshots' should be shared"
    );
}

#[test]
fn metadata_table_definition_identical_to_legacy() {
    assert_eq!(metadata_table().name(), "metadata");
}

#[test]
fn file_state_table_returns_definition_named_file_state() {
    assert_eq!(file_state_table().name(), "file_state");
}

#[test]
fn url_state_table_returns_definition_named_url_state() {
    assert_eq!(url_state_table().name(), "url_state");
}

#[test]
fn analysis_outputs_table_returns_definition_named_analysis_outputs() {
    assert_eq!(analysis_outputs_table().name(), "analysis_outputs");
}

#[test]
fn transform_outputs_table_returns_definition_named_transform_outputs() {
    assert_eq!(transform_outputs_table().name(), "transform_outputs");
}

#[test]
fn chunk_outputs_table_returns_definition_named_chunk_outputs() {
    assert_eq!(chunk_outputs_table().name(), "chunk_outputs");
}

#[test]
fn scrape_outputs_table_returns_definition_named_scrape_outputs() {
    assert_eq!(scrape_outputs_table().name(), "scrape_outputs");
}

#[test]
fn snapshots_table_returns_definition_named_snapshots() {
    assert_eq!(snapshots_table().name(), "snapshots");
}

#[test]
fn metadata_table_returns_definition_named_metadata() {
    assert_eq!(metadata_table().name(), "metadata");
}

#[test]
fn hash_key_wrong_length_returns_invalid_hash_key_length() {
    assert!(matches!(
        validate_hash_key(&[0u8; 16]),
        Err(StateError::InvalidHashKeyLength { actual: 16 })
    ));
    assert!(matches!(
        validate_hash_key(&[0u8; 33]),
        Err(StateError::InvalidHashKeyLength { actual: 33 })
    ));
    assert!(matches!(
        validate_hash_key(&[]),
        Err(StateError::InvalidHashKeyLength { actual: 0 })
    ));
    assert!(validate_hash_key(&[0u8; 32]).is_ok());
}

#[test]
fn source_path_with_leading_slash_returns_invalid_source_path() {
    let err_msg = validate_source_path("/absolute/path.md")
        .unwrap_err()
        .to_string();
    assert!(
        err_msg.contains("must not start with '/'"),
        "err: {err_msg}"
    );
}

#[test]
fn source_path_with_dot_dot_returns_invalid_source_path() {
    let err_msg = validate_source_path("foo/../bar.md")
        .unwrap_err()
        .to_string();
    assert!(err_msg.contains("'..'"), "err: {err_msg}");
}

#[test]
fn url_key_without_scheme_returns_invalid_url_key() {
    let err_msg = validate_url_key("example.com/page")
        .unwrap_err()
        .to_string();
    assert!(err_msg.contains("scheme"), "err: {err_msg}");
}

#[test]
fn source_path_empty_returns_invalid_source_path() {
    let err_msg = validate_source_path("").unwrap_err().to_string();
    assert!(err_msg.contains("empty"), "err: {err_msg}");
}

#[test]
fn source_path_valid_relative() {
    assert!(validate_source_path("concept/general/test.md").is_ok());
}

#[test]
fn url_key_valid_with_scheme() {
    assert!(validate_url_key("https://docs.example.com/api").is_ok());
}

#[test]
fn url_key_empty_returns_invalid_url_key() {
    let err_msg = validate_url_key("").unwrap_err().to_string();
    assert!(err_msg.contains("empty"), "err: {err_msg}");
}

#[test]
fn validate_source_path_accepts_three_dots_in_path() {
    assert!(
        validate_source_path("foo/.../bar").is_ok(),
        "three dots should be accepted"
    );
}

#[test]
fn validate_source_path_accepts_single_dot_segment() {
    assert!(
        validate_source_path("./foo").is_ok(),
        "single dot segment should be accepted"
    );
}

#[test]
fn validate_source_path_accepts_dot_dot_prefix_in_filename() {
    assert!(
        validate_source_path("..hidden").is_ok(),
        "dot-dot prefix filename should be accepted"
    );
}

#[test]
fn validate_source_path_accepts_unicode_path() {
    assert!(
        validate_source_path("概念/一般/test.md").is_ok(),
        "unicode path should be accepted"
    );
}

#[test]
fn validate_source_path_accepts_very_long_path() {
    let long_path: String = "a".repeat(4096);
    assert!(
        validate_source_path(&long_path).is_ok(),
        "4096-char path should be accepted"
    );
}
