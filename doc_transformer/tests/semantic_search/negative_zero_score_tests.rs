//! Tests for negative zero score filtering (doc-tx-5dr)
//!
//! This module tests that search_index() never returns results with
//! non-positive scores (<= 0.0), which can appear as "-0.00" in output.

use std::path::Path;
use tempfile::TempDir;

fn create_test_index(dir: &Path) -> anyhow::Result<()> {
    let docs = vec![
        doc_transformer::index::IndexDocument {
            id: "test/doc1".to_string(),
            title: "Test Document".to_string(),
            summary: "A test document with content".to_string(),
            path: "test/doc1.md".to_string(),
            category: "test".to_string(),
            content: String::new(),
            word_count: 10,
            tags: vec![],
            chunk_ids: vec![],
            headings: vec![],
        },
        doc_transformer::index::IndexDocument {
            id: "test/doc2".to_string(),
            title: "Another Document".to_string(),
            summary: "Completely different content here".to_string(),
            path: "test/doc2.md".to_string(),
            category: "test".to_string(),
            content: String::new(),
            word_count: 15,
            tags: vec![],
            chunk_ids: vec![],
            headings: vec![],
        },
    ];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;
    Ok(())
}

#[test]
fn test_non_matching_query_returns_empty_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let results = doc_transformer::search::search_index(&index, "SELECT * FROM users;--", 10);

    assert!(results.is_ok(), "Query should succeed");
    let results = results.unwrap();
    assert!(
        results.is_empty(),
        "Non-matching query should return empty results, not -0.00 scores"
    );
}

#[test]
fn test_non_matching_special_chars_returns_empty_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let results = doc_transformer::search::search_index(&index, "xyzzy nonexistent term", 10);

    assert!(results.is_ok(), "Query should succeed");
    let results = results.unwrap();
    assert!(
        results.is_empty(),
        "Query with no matching terms should return empty results"
    );
}

#[test]
fn test_all_results_have_positive_scores() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let results = doc_transformer::search::search_index(&index, "test", 10);

    assert!(results.is_ok(), "Query should succeed");
    let results = results.unwrap();

    for result in &results {
        assert!(
            result.score.value() > 0.0,
            "Result {} has score {} which is not positive",
            result.id,
            result.score.value()
        );
        assert_ne!(
            result.score.value(),
            -0.0,
            "Result {} has negative zero score",
            result.id
        );
    }
}

#[test]
fn test_query_with_partial_match_filters_non_matching() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let results = doc_transformer::search::search_index(&index, "test nonexistent", 10);

    assert!(results.is_ok(), "Query should succeed");
    let results = results.unwrap();

    for result in &results {
        assert!(
            result.score.value() > 0.0,
            "Result {} has score {} which is not positive (partial query: 'test nonexistent')",
            result.id,
            result.score.value()
        );
    }
}
