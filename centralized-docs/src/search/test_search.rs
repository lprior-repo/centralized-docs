use super::test_support::*;
use super::*;
use tempfile::TempDir;

#[test]
fn test_search_index_basic_query() {
    let docs = vec![
        make_index_document(
            "doc1",
            "Rust Programming",
            "Learn Rust",
            "Rust is a systems programming language focused on safety",
            "tutorial",
        ),
        make_index_document(
            "doc2",
            "Python Guide",
            "Learn Python",
            "Python is a high-level scripting language",
            "tutorial",
        ),
    ];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let results = search_index(&index, "Rust", 10).unwrap();
    assert!(!results.is_empty());
    assert!(
        results.iter().any(|r| r.id == "doc1"),
        "doc1 should be in search results"
    );
}

#[test]
fn test_search_index_returns_empty_for_no_match() {
    let docs = vec![make_index_document(
        "doc1",
        "Rust Programming",
        "Learn Rust",
        "Rust content",
        "tutorial",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let results = search_index(&index, "xyznonexistent", 10).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_search_index_limit() {
    let docs = vec![
        make_index_document(
            "doc1",
            "Alpha Document",
            "First",
            "alpha content here",
            "concept",
        ),
        make_index_document(
            "doc2",
            "Beta Document",
            "Second",
            "beta content here",
            "concept",
        ),
        make_index_document(
            "doc3",
            "Gamma Document",
            "Third",
            "gamma content here",
            "concept",
        ),
    ];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let results = search_index(&index, "content", 2).unwrap();
    assert!(results.len() <= 2);
}

#[test]
fn test_search_index_empty_query_returns_error() {
    let docs = vec![make_index_document(
        "doc1", "Test", "Test", "content", "concept",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let result = search_index(&index, "", 10);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, SearchError::QueryParseError(_)),
        "Expected QueryParseError, got: {err:?}"
    );
}

#[test]
fn test_search_index_whitespace_only_query_returns_error() {
    let docs = vec![make_index_document(
        "doc1", "Test", "Test", "content", "concept",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let result = search_index(&index, "   ", 10);
    assert!(result.is_err());
}

#[test]
fn test_search_index_null_bytes_returns_error() {
    let docs = vec![make_index_document(
        "doc1", "Test", "Test", "content", "concept",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let result = search_index(&index, "test\0query", 10);
    assert!(result.is_err());
}

#[test]
fn test_search_index_results_sorted_by_score() {
    let docs = vec![
        make_index_document(
            "doc1",
            "Rust Programming Language",
            "All about Rust",
            "Rust programming language systems",
            "tutorial",
        ),
        make_index_document(
            "doc2",
            "Some Document",
            "Unrelated",
            " mentions rust once briefly",
            "concept",
        ),
    ];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let results = search_index(&index, "Rust programming", 10).unwrap();
    assert!(results.len() >= 2);
    for i in 0..results.len().saturating_sub(1) {
        assert!(
            results[i].score >= results[i + 1].score,
            "Results should be sorted by descending score"
        );
    }
}

#[test]
fn test_search_result_fields() {
    let docs = vec![make_index_document(
        "doc1",
        "Test Title",
        "Test Summary",
        "Test content",
        "ref",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);

    let results = search_index(&index, "Test", 10).unwrap();
    assert_eq!(results.len(), 1);
    let r = &results[0];
    assert_eq!(r.id, "doc1");
    assert_eq!(r.title, "Test Title");
    assert_eq!(r.summary, "Test Summary");
    assert_eq!(r.category, "ref");
    assert!(r.path.contains("doc1"));
}

#[test]
fn test_search_index_with_chunks() {
    let docs = vec![make_index_document(
        "doc1",
        "Test Doc",
        "Summary",
        "Original content",
        "tutorial",
    )];
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Test Doc",
            "Chunk about rust programming patterns",
            Some("Rust Patterns"),
        ),
        make_chunk(
            "doc1#1-standard",
            "doc1",
            "Test Doc",
            "Chunk about python scripting",
            Some("Python Scripts"),
        ),
    ];

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_documents(&mut writer, &docs).unwrap();
    index_chunks(&mut writer, &docs, &chunks).unwrap();
    writer.commit().unwrap();

    let results = search_index(&index, "rust patterns", 10).unwrap();
    assert!(!results.is_empty());
}

fn escape_tantivy_query(query: &str) -> String {
    super::query::escape_tantivy_query(query)
}

#[test]
fn test_escape_tantivy_query_no_special_chars() {
    assert_eq!(escape_tantivy_query("hello world"), "hello world");
}

#[test]
fn test_escape_tantivy_query_wildcard_star() {
    assert_eq!(escape_tantivy_query("test*"), "test\\*");
}

#[test]
fn test_escape_tantivy_query_wildcard_question() {
    assert_eq!(escape_tantivy_query("test?"), "test\\?");
}

#[test]
fn test_escape_tantivy_query_multiple_special() {
    assert_eq!(escape_tantivy_query("a*b?c*d"), "a\\*b\\?c\\*d");
}

#[test]
fn test_escape_tantivy_query_empty() {
    assert_eq!(escape_tantivy_query(""), "");
}

#[test]
fn test_search_error_types() {
    let _ = IndexerError::DirectoryAccessFailed("test".to_string());
    let _ = IndexerError::IndexCommitFailed("test".to_string());
    let _ = IndexerError::InvalidDocument;
    let _ = IndexerError::UncommittedChanges;
    let _ = IndexerError::Other(anyhow::anyhow!("wrapped"));

    let _ = SearchError::EmptyQuery;
    let _ = SearchError::QueryParseError("bad".to_string());
    let _ = SearchError::PostconditionViolated;
    let _ = SearchError::Other(anyhow::anyhow!("wrapped"));
}

#[test]
fn test_search_result_debug_clone() {
    let result = SearchResult {
        id: "test-id".to_string(),
        title: "Test Title".to_string(),
        summary: "Test Summary".to_string(),
        category: "tutorial".to_string(),
        score: crate::math_types::Score::zero(),
        path: "docs/test.md".to_string(),
    };

    let _cloned = result.clone();
    let debug = format!("{result:?}");
    assert!(debug.contains("test-id"));
}

#[test]
fn test_chunk_with_special_chars_in_id() {
    let docs = vec![make_index_document(
        "doc/a#1",
        "Special ID",
        "Summary",
        "content",
        "concept",
    )];
    let mut chunk = make_chunk(
        "doc/a#1-0-standard",
        "doc/a#1",
        "Special ID",
        "chunk content",
        None,
    );
    chunk.chunk_id = "doc/a#1".to_string();

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &docs, &[chunk]).unwrap();
    writer.commit().unwrap();
}
