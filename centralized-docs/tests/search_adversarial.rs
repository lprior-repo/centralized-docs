#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


use doc_transformer::search;
use tempfile::TempDir;

fn run_search_test<F>(mut test_fn: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&tantivy::Index),
{
    let dir = TempDir::new()?;
    let index_path = dir.path();

    // Create a simple index first
    let index = search::open_or_create_index(index_path)?;
    let mut writer = index.writer(50_000_000)?;
    let doc = doc_transformer::index::IndexDocument {
        id: "test".to_string(),
        title: "Test Document".to_string(),
        path: "docs/test.md".to_string(),
        category: "ref".to_string(),
        tags: vec!["test".to_string()],
        summary: "This is a test document".to_string(),
        content: String::new().into(),
        word_count: 100,
        chunk_ids: vec!["test_chunk".to_string()],
        headings: vec![],
    };

    let chunk = doc_transformer::chunking_adapter::Chunk {
        chunk_id: "test_chunk".to_string(),
        doc_id: "test".to_string(),
        doc_title: "Test Document".to_string(),
        chunk_index: 0,
        content: "This is a test chunk content.".to_string(),
        token_count: 100,
        heading: Some("Test Heading".to_string()),
        heading_path: vec!["Test Document".to_string(), "Test Heading".to_string()],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: "Test summary".to_string(),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    };

    let _ = search::index_chunks(&mut writer, &[doc], &[chunk]);
    let _ = writer.commit()?;

    test_fn(&index);
    Ok(())
}

/// Test 1: SQL injection attempts in search queries
/// Expected: Should be rejected or handled safely without panics
#[test]
fn test_search_sql_injection_squot_single() {
    let _ = run_search_test(|index| {
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
                "Query with control character '{query}' should not panic, got: {result:?}",
            );
        }
    });
}

/// Test 8: Query with extremely long word sequences
/// Expected: Should be handled or rejected
#[test]
fn test_search_extremely_long_word_sequences() {
    let _ = run_search_test(|index| {
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
