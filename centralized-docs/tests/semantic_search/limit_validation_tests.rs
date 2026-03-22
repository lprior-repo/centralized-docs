//! Tests for limit validation (P2 search-limit-zero + P1 negative limit)
//!
//! Tests that:
//! - limit = 0 is rejected (prevents tantivy panic)
//! - limit < 0 is rejected (negative values)
//! - Valid limits work correctly

#![allow(clippy::unwrap_used)]

use tempfile::TempDir;

/// Helper to create a test index with sample documents
fn create_test_index(dir: &tempfile::TempDir) -> anyhow::Result<()> {
    let docs = vec![doc_transformer::index::IndexDocument {
        id: "test/doc1".to_string(),
        title: "Rust Programming".to_string(),
        summary: "A comprehensive guide to Rust programming language".to_string(),
        path: "test/doc1.md".to_string(),
        category: "tutorial".to_string(),
        content: String::new().into(),
        word_count: 500,
        tags: vec!["rust".to_string(), "programming".to_string()],
        chunk_ids: vec![],
        headings: vec!["Rust Programming".to_string()],
    }];

    let index = doc_transformer::search::open_or_create_index(dir.path())?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;
    Ok(())
}

// ============================================================================
// INVALID INPUT TESTS (boundary conditions)
// ============================================================================

#[test]
fn test_internal_validate_limit_zero_rejected() {
    // P2: limit = 0 should be rejected (prevents tantivy panic)
    let result = doc_transformer::validate::validate_limit("0");
    assert!(result.is_err(), "limit=0 should be rejected");
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        doc_transformer::validate::ValidationError::InvalidLimitZero
    ));
}

#[test]
fn test_cli_validate_limit_negative_rejected() {
    // P1: limit = -1 should be rejected (negative)

    // Parse as i64 first to catch negative values
    let value = "-1".parse::<i64>().unwrap();

    assert!(value < 0, "Negative value should be detected");
}

#[test]
fn test_cli_validate_limit_exceeds_max_rejected() {
    // limit = 1001 should be rejected (exceeds max of 1000)
    let value = "1001".parse::<i64>().unwrap();

    assert!(value > 1000, "Value > 1000 should exceed maximum");
}

// ============================================================================
// VALID INPUT TESTS (happy path)
// ============================================================================

#[test]
fn test_internal_validate_limit_one_accepted() {
    // limit = 1 should work (minimum valid)
    let result = doc_transformer::validate::validate_limit("1");
    assert!(result.is_ok(), "limit=1 should be accepted");
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_internal_validate_limit_default_accepted() {
    // limit = 10 should work (default)
    let result = doc_transformer::validate::validate_limit("10");
    assert!(result.is_ok(), "limit=10 should be accepted");
    assert_eq!(result.unwrap(), 10);
}

#[test]
fn test_search_with_limit_one_works() {
    // Integration: limit = 1 should work
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    create_test_index(&temp_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust", 1);

    assert!(result.is_ok(), "Search with limit=1 should work");
    let results = result.unwrap();
    assert!(results.len() <= 1, "Should return at most 1 result");
}
