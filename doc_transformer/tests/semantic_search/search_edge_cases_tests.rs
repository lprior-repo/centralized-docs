//! Core search behavior tests
//!
//! Tests actual user-facing search behavior:
//! 1. Search returns relevant results
//! 2. Search with no results
//! 3. Unicode search
//! 4. Large limit handling

#![allow(clippy::unwrap_used)]

use std::path::Path;
use tempfile::TempDir;

/// Helper to create a test index with known content
fn create_test_index(dir: &Path) -> anyhow::Result<()> {
    let docs = vec![
        doc_transformer::index::IndexDocument {
            id: "test/unicode-doc".to_string(),
            title: "Unicode Characters: cafe naive resume".to_string(),
            summary: "Document with unicode: cafe naive resume facade".to_string(),
            path: "test/unicode-doc.md".to_string(),
            category: "tutorial".to_string(),
            content: String::new(),
            word_count: 50,
            tags: vec!["unicode".to_string(), "encoding".to_string()],
            chunk_ids: vec!["chunk-1".to_string()],
            headings: vec!["Unicode Characters".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/exact-match".to_string(),
            title: "Exact Match Test".to_string(),
            summary: "This is an exact match test for phrase search".to_string(),
            path: "test/exact-match.md".to_string(),
            category: "ref".to_string(),
            content: String::new(),
            word_count: 30,
            tags: vec!["test".to_string()],
            chunk_ids: vec!["chunk-2".to_string()],
            headings: vec!["Exact Match".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "tutorial/programming".to_string(),
            title: "Programming Tutorial".to_string(),
            summary: "Learn programming with rust and python".to_string(),
            path: "tutorial/programming.md".to_string(),
            category: "tutorial".to_string(),
            content: String::new(),
            word_count: 200,
            tags: vec![
                "programming".to_string(),
                "rust".to_string(),
                "python".to_string(),
            ],
            chunk_ids: vec!["chunk-3".to_string()],
            headings: vec!["Programming".to_string()],
        },
    ];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;
    Ok(())
}

/// Helper to create an empty test index
fn create_empty_index(dir: &Path) -> anyhow::Result<()> {
    let index = doc_transformer::search::open_or_create_index(dir)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &[])?;
    writer.commit()?;
    Ok(())
}

// ============================================================================
// Search returns relevant results
// ============================================================================

#[test]
fn test_search_finds_documents_by_title() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "Exact Match Test", 10);

    assert!(result.is_ok(), "Search by exact title should succeed");
    let results = result.unwrap();
    assert!(
        results.iter().any(|r| r.id == "test/exact-match"),
        "Should find document by title"
    );
}

#[test]
fn test_search_results_sorted_by_score() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust programming", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();

    if results.len() > 1 {
        // Verify scores are in descending order (highest first)
        for i in 1..results.len() {
            assert!(
                results[i - 1].score >= results[i].score,
                "Results should be sorted by score descending"
            );
        }
    }
}

#[test]
fn test_search_results_sorted_by_recalculated_score() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    let docs = vec![
        doc_transformer::index::IndexDocument {
            id: "test/high-tf-low-avg".to_string(),
            title: "Rust Rust Rust Rust Rust".to_string(),
            summary: "Rust Rust".to_string(),
            path: "test/high-tf-low-avg.md".to_string(),
            category: "tutorial".to_string(),
            content: String::new(),
            word_count: 1,
            tags: vec![],
            chunk_ids: vec![],
            headings: vec![],
        },
        doc_transformer::index::IndexDocument {
            id: "test/low-tf-high-avg".to_string(),
            title: "Rust".to_string(),
            summary: "Rust".to_string(),
            path: "test/low-tf-high-avg.md".to_string(),
            category: "tutorial".to_string(),
            content: String::new(),
            word_count: 10_000,
            tags: vec![],
            chunk_ids: vec![],
            headings: vec![],
        },
    ];

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    doc_transformer::search::index_documents(&mut writer, &docs).unwrap();
    writer.commit().unwrap();

    let result = doc_transformer::search::search_index(&index, "rust", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(results.len() >= 2, "Should return both documents");

    assert!(
        results[0].score >= results[1].score,
        "Results should be sorted by recalculated score"
    );
    assert_eq!(
        results[0].id, "test/high-tf-low-avg",
        "Higher TF with shorter length should rank higher under standard BM25"
    );
}

#[test]
fn test_search_boolean_or_operator() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust OR python", 10);

    assert!(result.is_ok(), "Boolean OR query should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results with OR operator");
}

// ============================================================================
// Search with no results
// ============================================================================

#[test]
fn test_search_no_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "nonexistent_term_xyz123", 10);

    assert!(
        result.is_ok(),
        "Search with no matches should return Ok with empty results"
    );
    let results = result.unwrap();
    assert!(
        results.is_empty(),
        "Should return empty results for non-matching query"
    );
}

// ============================================================================
// Unicode search
// ============================================================================

#[test]
fn test_search_unicode_basic() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "cafe", 10);

    assert!(result.is_ok(), "Unicode query should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find unicode document");
    assert!(
        results.iter().any(|r| r.id == "test/unicode-doc"),
        "Should find the unicode document"
    );
}

#[test]
fn test_search_matches_terms_in_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    let docs = vec![doc_transformer::index::IndexDocument {
        id: "test/path-only".to_string(),
        title: "Path Only Document".to_string(),
        summary: "No keyword in title or summary".to_string(),
        path: "docs/specialpathmd".to_string(),
        category: "tutorial".to_string(),
        content: String::new(),
        word_count: 40,
        tags: vec![],
        chunk_ids: vec![],
        headings: vec![],
    }];

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    doc_transformer::search::index_documents(&mut writer, &docs).unwrap();
    writer.commit().unwrap();

    // Search for a term that only appears in the path, not in title or summary
    // Using a path without hyphens to avoid Tantivy query parser interpreting
    // hyphens as NOT operators
    let result = doc_transformer::search::search_index(&index, "specialpathmd", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results from path matches");
    assert!(
        results
            .iter()
            .any(|r| r.id == "test/path-only" && r.score.value() > 0.0),
        "Path-only match should return a positive score"
    );
}

// ============================================================================
// Large limit handling
// ============================================================================

#[test]
fn test_search_large_limit() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 1000);

    assert!(result.is_ok(), "Search with large limit should succeed");
    let results = result.unwrap();
    // We only have 3 docs, so we should get at most 3
    assert!(results.len() <= 3, "Should not exceed available documents");
}

// ============================================================================
// Empty index handling
// ============================================================================

#[test]
fn test_search_empty_index() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_empty_index(temp_dir.path()).expect("Failed to create empty index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 10);

    assert!(result.is_ok(), "Search on empty index should succeed");
    let results = result.unwrap();
    assert!(
        results.is_empty(),
        "Should return no results from empty index"
    );
}

// ============================================================================
// Empty query validation
// ============================================================================

#[test]
fn test_search_empty_query_returns_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Empty queries should fail validation
    let empty_queries = vec!["", "   ", "\t", "\n"];

    for query in empty_queries {
        let result = doc_transformer::search::search_index(&index, query, 10);
        assert!(
            result.is_err(),
            "Empty query '{query:?}' should fail validation"
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("empty"),
            "Error should mention empty query, got: {error_msg}"
        );
    }
}
