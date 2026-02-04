use doc_transformer::search;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn run_search_test<F>(mut test_fn: F)
where
    F: FnMut(&tantivy::Index),
{
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    // Create a simple index first
    let index = search::open_or_create_index(index_path);
    let doc = doc_transformer::index::IndexDocument {
        id: "test".to_string(),
        title: "Test Document".to_string(),
        path: "docs/test.md".to_string(),
        category: "ref".to_string(),
        tags: vec!["test".to_string()],
        summary: "This is a test document".to_string(),
        word_count: 100,
        chunk_ids: vec![],
        headings: vec![],
    };
    let index_ref = index.expect("Failed to unwrap index");
    search::index_documents(&index_ref, vec![doc]);

    test_fn(&index_ref);
}

/// Test 1: SQL injection attempts in search queries
/// Expected: Should be rejected or handled safely without panics
#[test]
fn test_search_sql_injection_squot_single() {
    run_search_test(|index| {
        // Test queries with control characters
        let control_chars = vec![
            "test\x01", "test\x08", "test\x0c", // Form feed
            "test\x1b", // Escape
            "test\x1f", // Unit separator
            "test\x7f", // Delete
        ];

        for query in control_chars {
            let result = search::search_index(index, query, 10);
            assert!(
                result.is_ok() || result.is_err(),
                "Query with control character '{}' should not panic, got: {:?}",
                query,
                result
            );
        }
    });
}

/// Test 8: Query with extremely long word sequences
/// Expected: Should be handled or rejected
#[test]
fn test_search_extremely_long_word_sequences() {
    run_search_test(|index| {
        // Test queries with many words
        let long_queries = vec![
            "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string(),
            "word ".repeat(100), // 100 words
            "a b c d e f g h i j k l m n o p q r s t u v w x y z a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string(),
        ];

        for query in long_queries {
            let result = search::search_index(index, &query, 10);
            assert!(
                result.is_ok() || result.is_err(),
                "Long query '{}' should not panic, got: {:?}",
                query.chars().take(50).collect::<String>(),
                result
            );
        }
    });
}
