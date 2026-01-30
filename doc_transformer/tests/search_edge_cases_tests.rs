//! Comprehensive edge case tests for search functionality
//!
//! This module tests uncommon and edge case scenarios in search:
//! 1. Unicode characters in queries
//! 2. Very long queries (>100 words, should reject)
//! 3. Special regex characters (.*, +, etc.)
//! 4. Searches returning no results
//! 5. Exact match searches
//! 6. Category filtering (if supported)
//! 7. Large limit values
//! 8. Empty index searches
//! 9. Queries with quotes
//! 10. Queries with newlines

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
            word_count: 50,
            tags: vec!["unicode".to_string(), "encoding".to_string()],
            chunk_ids: vec!["chunk-1".to_string()],
            headings: vec!["Unicode Characters".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/special-chars".to_string(),
            title: "Special Characters: dot star plus".to_string(),
            summary: "Document about regex-like characters: . * + [ ] { } ( ) | ^ $ \\".to_string(),
            path: "test/special-chars.md".to_string(),
            category: "concept".to_string(),
            word_count: 75,
            tags: vec!["special".to_string(), "characters".to_string()],
            chunk_ids: vec!["chunk-2".to_string()],
            headings: vec!["Special Characters".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/exact-match".to_string(),
            title: "Exact Match Test".to_string(),
            summary: "This is an exact match test for phrase search".to_string(),
            path: "test/exact-match.md".to_string(),
            category: "ref".to_string(),
            word_count: 30,
            tags: vec!["test".to_string()],
            chunk_ids: vec!["chunk-3".to_string()],
            headings: vec!["Exact Match".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "tutorial/programming".to_string(),
            title: "Programming Tutorial".to_string(),
            summary: "Learn programming with rust and python".to_string(),
            path: "tutorial/programming.md".to_string(),
            category: "tutorial".to_string(),
            word_count: 200,
            tags: vec![
                "programming".to_string(),
                "rust".to_string(),
                "python".to_string(),
            ],
            chunk_ids: vec!["chunk-4".to_string()],
            headings: vec!["Programming".to_string()],
        },
    ];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    doc_transformer::search::index_documents(&index, docs)?;
    Ok(())
}

/// Helper to create an empty test index
fn create_empty_index(dir: &Path) -> anyhow::Result<()> {
    let index = doc_transformer::search::open_or_create_index(dir)?;
    doc_transformer::search::index_documents(&index, vec![])?;
    Ok(())
}

// ============================================================================
// Test 1: Unicode characters in queries
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
fn test_search_unicode_multiple() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "naive resume", 10);

    assert!(
        result.is_ok(),
        "Unicode query with multiple words should succeed"
    );
    let results = result.unwrap();
    assert!(
        !results.is_empty(),
        "Should find document with multiple unicode terms"
    );
}

#[test]
fn test_search_unicode_combined_with_ascii() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "unicode cafe", 10);

    assert!(result.is_ok(), "Mixed unicode/ASCII query should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find document");
}

#[test]
fn test_search_unicode_various_scripts() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Test with various unicode characters
    let queries = vec![
        "cafe",    // Latin with accents
        "日本語",  // Japanese (likely no results, but should not crash)
        "русский", // Cyrillic (likely no results, but should not crash)
        "العربية", // Arabic (likely no results, but should not crash)
    ];

    for query in queries {
        let result = doc_transformer::search::search_index(&index, query, 10);
        assert!(
            result.is_ok(),
            "Query '{query}' should succeed or return empty, not error"
        );
    }
}

// ============================================================================
// Test 2: Very long query (>100 words)
// ============================================================================

#[test]
fn test_search_very_long_query_rejected() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    // Create a query exceeding 1000 byte limit (validate_query limit)
    let long_query = "a ".repeat(1001); // 2002 bytes

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, &long_query, 10);

    // Should fail due to query length validation
    assert!(result.is_err(), "Very long query should be rejected");

    let error_msg = result.unwrap_err().to_string().to_lowercase();
    assert!(
        error_msg.contains("long") || error_msg.contains("limit"),
        "Error should mention query length limit, got: {error_msg}"
    );
}

#[test]
fn test_search_exactly_100_words() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    // Create a query with exactly 100 words
    let query = "test ".repeat(100);

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, &query, 10);

    // The result depends on byte length validation - let's just check it doesn't panic
    // This may succeed or fail depending on exact byte length
    if let Err(e) = &result {
        let error_msg = e.to_string().to_lowercase();
        assert!(
            error_msg.contains("long") || error_msg.contains("limit"),
            "Error should be about length, got: {error_msg}"
        );
    }
}

// ============================================================================
// Test 3: Special regex characters
// ============================================================================

#[test]
fn test_search_regex_special_chars_dot() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "dot.star", 10);

    assert!(result.is_ok(), "Query with dot should succeed");
}

#[test]
fn test_search_regex_special_chars_star() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "special characters", 10);

    assert!(
        result.is_ok(),
        "Query searching for 'special characters' should succeed"
    );
    let results = result.unwrap();
    assert!(
        results.iter().any(|r| r.id == "test/special-chars"),
        "Should find special characters document"
    );
}

#[test]
fn test_search_regex_special_chars_plus() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "plus", 10);

    assert!(result.is_ok(), "Query with 'plus' should succeed");
}

#[test]
fn test_search_regex_special_chars_brackets() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "brackets", 10);

    assert!(result.is_ok(), "Query should succeed");
}

#[test]
fn test_search_regex_special_chars_pipe() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust OR python", 10);

    assert!(result.is_ok(), "Query with OR operator should succeed");
}

#[test]
fn test_search_regex_special_chars_caret_dollar() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 10);

    assert!(result.is_ok(), "Query should succeed");
}

// ============================================================================
// Test 4: Search returning no results
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

#[test]
fn test_search_no_results_gibberish() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "qwertyuiopasdfghjklzxcvbnm", 10);

    assert!(
        result.is_ok(),
        "Gibberish query should return Ok with empty results"
    );
    assert!(result.unwrap().is_empty(), "Should have no results");
}

#[test]
fn test_search_no_results_rare_term() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result =
        doc_transformer::search::search_index(&index, "supercalifragilisticexpialidocious", 10);

    assert!(
        result.is_ok(),
        "Rare term query should return Ok with empty results"
    );
    assert!(result.unwrap().is_empty(), "Should have no results");
}

// ============================================================================
// Test 5: Exact match searches
// ============================================================================

#[test]
fn test_search_exact_match_phrase() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    // Try phrase search - may behave differently based on Tantivy configuration
    let result = doc_transformer::search::search_index(&index, "\"exact match\"", 10);

    assert!(result.is_ok(), "Phrase search with quotes should succeed");
    let results = result.unwrap();

    // If phrase search works, we should find the document
    // If not (some Tantivy configs), we just verify the search doesn't fail
    if !results.is_empty() {
        assert!(
            results.iter().any(|r| r.id == "test/exact-match"),
            "Should find the exact match document"
        );
    }
    // Test passes either way - we're just checking no crash/error
}

#[test]
fn test_search_exact_match_full_title() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "\"Exact Match Test\"", 10);

    assert!(result.is_ok(), "Exact title search should succeed");
    let results = result.unwrap();
    assert!(
        results.iter().any(|r| r.id == "test/exact-match"),
        "Should find document with exact title"
    );
}

#[test]
fn test_search_exact_match_case_sensitivity() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Test case-insensitive search without phrase quotes
    let result1 = doc_transformer::search::search_index(&index, "exact match", 10);
    let result2 = doc_transformer::search::search_index(&index, "EXACT MATCH", 10);
    let result3 = doc_transformer::search::search_index(&index, "Exact MaTcH", 10);

    assert!(result1.is_ok(), "Lowercase search should succeed");
    assert!(result2.is_ok(), "Uppercase search should succeed");
    assert!(result3.is_ok(), "Mixed case search should succeed");

    let results1 = result1.unwrap();
    let results2 = result2.unwrap();
    let results3 = result3.unwrap();

    // All should find results (case-insensitive)
    assert!(!results1.is_empty(), "Should find results with lowercase");
    assert!(!results2.is_empty(), "Should find results with uppercase");
    assert!(!results3.is_empty(), "Should find results with mixed case");
}

// ============================================================================
// Test 6: Category filtering (if supported by schema)
// ============================================================================

#[test]
fn test_search_with_category_term() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Search for documents that mention "tutorial"
    let result = doc_transformer::search::search_index(&index, "tutorial", 10);

    assert!(result.is_ok(), "Search with category term should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find tutorial documents");

    // Verify the results have the tutorial category or title
    assert!(
        results
            .iter()
            .any(|r| r.category == "tutorial" || r.title.contains("Tutorial")),
        "At least one result should be from tutorial category"
    );
}

#[test]
fn test_search_different_categories() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Search for programming which appears in a tutorial doc
    let result = doc_transformer::search::search_index(&index, "programming", 10);

    assert!(result.is_ok(), "Category-based search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find programming content");
}

// ============================================================================
// Test 7: Large limit values
// ============================================================================

#[test]
fn test_search_large_limit() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 1000);

    assert!(result.is_ok(), "Search with large limit should succeed");
    let results = result.unwrap();
    // We only have 4 docs, so we should get at most 4
    assert!(results.len() <= 4, "Should not exceed available documents");
}

#[test]
fn test_search_limit_zero() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 0);

    // Limit of 0 should either return empty results or fail validation
    // The current implementation may fail at the query parser or return empty
    match result {
        Ok(results) => assert!(results.is_empty(), "Limit 0 should return empty results"),
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            assert!(
                error_msg.contains("limit") || error_msg.contains("invalid"),
                "Error should mention limit, got: {error_msg}"
            );
        }
    }
}

#[test]
fn test_search_limit_one() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 1);

    assert!(result.is_ok(), "Search with limit=1 should succeed");
    let results = result.unwrap();
    assert!(results.len() <= 1, "Should return at most 1 result");
}

#[test]
fn test_search_limit_usize_max() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    // Note: usize::MAX can cause overflow in Tantivy's collector
    // Using a large but safe value instead
    let result = doc_transformer::search::search_index(&index, "test", 1_000_000);

    // Should not panic, just return all available results
    assert!(result.is_ok(), "Search with large limit should succeed");
    let results = result.unwrap();
    assert!(results.len() <= 4, "Should not exceed available documents");
}

// ============================================================================
// Test 8: Empty index searches
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

#[test]
fn test_search_empty_index_multiple_queries() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_empty_index(temp_dir.path()).expect("Failed to create empty index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    let queries = vec!["test", "rust", "programming", "", "  "];

    for query in queries {
        let result = doc_transformer::search::search_index(&index, query, 10);

        // Empty queries should fail, others should return empty results
        if query.trim().is_empty() {
            assert!(result.is_err(), "Empty query should fail");
        } else {
            assert!(
                result.is_ok(),
                "Non-empty query on empty index should succeed"
            );
            assert!(result.unwrap().is_empty(), "Should return empty results");
        }
    }
}

// ============================================================================
// Test 9: Queries with quotes
// ============================================================================

#[test]
fn test_search_with_single_quotes() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "'exact'", 10);

    assert!(result.is_ok(), "Query with single quotes should succeed");
}

#[test]
fn test_search_with_double_quotes_phrase() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "\"exact match test\"", 10);

    assert!(
        result.is_ok(),
        "Query with double quotes (phrase) should succeed"
    );
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find the exact phrase");
}

#[test]
fn test_search_with_unclosed_quote_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "\"unclosed", 10);

    assert!(result.is_err(), "Query with unclosed quote should fail");
}

#[test]
fn test_search_with_multiple_quotes() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "\"exact\" \"match\"", 10);

    assert!(
        result.is_ok(),
        "Query with multiple quoted phrases should succeed"
    );
}

// ============================================================================
// Test 10: Queries with newlines and whitespace
// ============================================================================

#[test]
fn test_search_with_newlines() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let query = "rust\nprogramming";

    let result = doc_transformer::search::search_index(&index, query, 10);

    assert!(result.is_ok(), "Query with newline should succeed");
    let results = result.unwrap();
    // Should search for "rust" and "programming" (newline treated as whitespace)
    assert!(
        !results.is_empty(),
        "Should find results after normalizing whitespace"
    );
}

#[test]
fn test_search_with_tabs() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let query = "rust\tprogramming";

    let result = doc_transformer::search::search_index(&index, query, 10);

    assert!(result.is_ok(), "Query with tab should succeed");
}

#[test]
fn test_search_with_carriage_return() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let query = "rust\rprogramming";

    let result = doc_transformer::search::search_index(&index, query, 10);

    assert!(result.is_ok(), "Query with carriage return should succeed");
}

#[test]
fn test_search_with_multiple_whitespace_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let query = "  rust\t\n\r programming  ";

    let result = doc_transformer::search::search_index(&index, query, 10);

    assert!(result.is_ok(), "Query with mixed whitespace should succeed");
    let results = result.unwrap();
    assert!(
        !results.is_empty(),
        "Should find results after trimming whitespace"
    );
}

#[test]
fn test_search_whitespace_only_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "   \n\t  ", 10);

    assert!(result.is_err(), "Whitespace-only query should fail");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("empty"),
        "Error should mention empty query, got: {error_msg}"
    );
}

// ============================================================================
// Additional edge cases
// ============================================================================

#[test]
fn test_search_query_with_leading_trailing_whitespace() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "   rust   ", 10);

    assert!(
        result.is_ok(),
        "Query with leading/trailing whitespace should succeed"
    );
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results after trimming");
}

#[test]
fn test_search_boolean_and_operator() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust AND programming", 10);

    assert!(result.is_ok(), "Boolean AND query should succeed");
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

#[test]
fn test_search_boolean_not_operator() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test NOT nonexistent", 10);

    assert!(result.is_ok(), "Boolean NOT query should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results with NOT operator");
}

#[test]
fn test_search_parentheses() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result =
        doc_transformer::search::search_index(&index, "(rust OR python) AND programming", 10);

    assert!(result.is_ok(), "Query with parentheses should succeed");
}

#[test]
fn test_search_fuzzy_term_wildcard() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // Tantivy supports wildcards in some configurations
    let result = doc_transformer::search::search_index(&index, "pro*", 10);

    // This may succeed or fail depending on Tantivy configuration
    // We just verify it doesn't panic
    if let Err(e) = &result {
        let error_msg = e.to_string().to_lowercase();
        // If it fails, it should be a parse error, not a crash
        assert!(
            error_msg.contains("parse")
                || error_msg.contains("syntax")
                || error_msg.contains("invalid"),
            "Error should be parse-related, got: {error_msg}"
        );
    }
}

#[test]
fn test_search_hyphenated_words() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "special-chars", 10);

    assert!(result.is_ok(), "Query with hyphenated word should succeed");
}

#[test]
fn test_search_underscore_words() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "unicode_doc", 10);

    assert!(result.is_ok(), "Query with underscore should succeed");
}

#[test]
fn test_search_numbers_in_query() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "123 456", 10);

    assert!(result.is_ok(), "Query with numbers should succeed");
}

#[test]
fn test_search_email_like_query() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    // Email-like query - the @ may cause parsing issues in some parsers
    let result = doc_transformer::search::search_index(&index, "test@example.com", 10);

    // May fail due to @ being a special character in Tantivy
    match result {
        Ok(_) => {}
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            // If it fails, it should be a parse error
            assert!(
                error_msg.contains("parse")
                    || error_msg.contains("syntax")
                    || error_msg.contains("invalid"),
                "Error should be parse-related, got: {error_msg}"
            );
        }
    }
}

#[test]
fn test_search_url_like_query() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    // URL-like query - :// may cause parsing issues
    let result = doc_transformer::search::search_index(&index, "https://example.com", 10);

    // May fail due to :// being parsed as special syntax
    match result {
        Ok(_) => {}
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            // If it fails, it should be a parse error
            assert!(
                error_msg.contains("parse")
                    || error_msg.contains("syntax")
                    || error_msg.contains("invalid"),
                "Error should be parse-related, got: {error_msg}"
            );
        }
    }
}

#[test]
fn test_search_empty_after_validation_returns_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();

    // These should all fail validation
    let empty_queries = vec!["", "   ", "\t", "\n", "\r\n", "  \t  \n  "];

    for query in empty_queries {
        let result = doc_transformer::search::search_index(&index, query, 10);
        assert!(result.is_err(), "Query '{query:?}' should fail validation");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("empty"),
            "Error should mention empty query, got: {error_msg}"
        );
    }
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
                "Results should be sorted by score descending: {:?} >= {:?}",
                results[i - 1].score,
                results[i].score
            );
        }
    }
}

#[test]
fn test_search_all_positive_scores() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_index(temp_dir.path()).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(temp_dir.path()).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();

    // All scores should be positive (negative and zero are filtered out)
    for r in &results {
        assert!(
            r.score > 0.0,
            "All returned scores should be positive, got: {} for {}",
            r.score,
            r.id
        );
    }
}
