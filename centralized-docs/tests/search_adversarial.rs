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
    writer.commit()?;

    test_fn(&index);
    Ok(())
}

/// Test: Non-whitespace control characters in search queries are rejected by validation
/// \x0c (form feed) is excluded because it is whitespace and gets trimmed before
/// the control character check, so it passes validation as a normal query.
/// Expected: Rejected with a QueryParseError (no panic)
#[test]
fn test_search_control_characters_rejected() {
    run_search_test(|index| {
        let control_chars = vec![
            "test\x01", // SOH
            "test\x08", // Backspace
            "test\x1b", // Escape
            "test\x1f", // Unit separator
            "test\x7f", // Delete
        ];

        for query in control_chars {
            let result = search::search_index(index, query, 10);
            assert!(
                result.is_err(),
                "Query with non-whitespace control character should be rejected, got: {result:?}",
            );
        }
    })
    .expect("search test setup should succeed");
}

/// Test: Whitespace control characters (form feed) get trimmed and search succeeds
/// Expected: Treated as a normal query after trimming (no panic)
#[test]
fn test_search_whitespace_control_characters_trimmed() {
    run_search_test(|index| {
        let result = search::search_index(index, "test\x0c", 10);
        assert!(
            result.is_ok(),
            "Query with form feed (whitespace) should succeed after trimming, got: {result:?}",
        );
    })
    .expect("search test setup should succeed");
}

/// Test: Query with extremely long word sequences
/// Expected: Accepted and returns results (all under 1024-byte limit)
#[test]
fn test_search_extremely_long_word_sequences() {
    run_search_test(|index| {
        let long_queries = vec![
            "a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string(),
            "word ".repeat(100),
            "a b c d e f g h i j k l m n o p q r s t u v w x y z a b c d e f g h i j k l m n o p q r s t u v w x y z".to_string(),
        ];

        for query in long_queries {
            let result = search::search_index(index, &query, 10);
            assert!(
                result.is_ok(),
                "Long query ({} bytes) should succeed, got: {result:?}",
                query.len(),
            );
        }
    }).expect("search test setup should succeed");
}
