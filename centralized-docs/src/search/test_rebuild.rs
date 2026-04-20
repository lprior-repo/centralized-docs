use super::*;
use tempfile::TempDir;

#[test]
fn test_open_existing_index_no_dir() {
    let dir = TempDir::new().unwrap();
    let result = open_existing_index(dir.path()).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_open_existing_index_valid() {
    let dir = TempDir::new().unwrap();
    let _index = open_or_create_index(dir.path()).unwrap();

    let result = open_existing_index(dir.path()).unwrap();
    assert!(result.is_some());
}

#[test]
fn test_open_existing_index_file_instead_of_dir() {
    let dir = TempDir::new().unwrap();
    let index_file = dir.path().join(".tantivy_index");
    std::fs::write(&index_file, "not a directory").unwrap();

    let result = open_existing_index(dir.path()).unwrap();
    assert!(result.is_none());
    assert!(!index_file.exists(), "File should have been removed");
}

#[test]
fn test_open_existing_index_corrupted() {
    let dir = TempDir::new().unwrap();
    let index_dir = dir.path().join(".tantivy_index");
    std::fs::create_dir_all(&index_dir).unwrap();
    std::fs::write(index_dir.join("managed.json"), "corrupted garbage data").unwrap();

    let result = open_existing_index(dir.path()).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_rebuild_index_from_json_missing_file() {
    let dir = TempDir::new().unwrap();
    let result = rebuild_index_from_json(dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to read INDEX.json"));
}

#[test]
fn test_rebuild_index_from_json_invalid_json() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("INDEX.json"), "not valid json").unwrap();

    let result = rebuild_index_from_json(dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to parse INDEX.json"));
}

#[test]
fn test_rebuild_index_from_json_missing_documents() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("INDEX.json"), r#"{"chunks": []}"#).unwrap();

    let result = rebuild_index_from_json(dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("missing documents array"));
}

#[test]
fn test_rebuild_index_from_json_missing_chunks() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("INDEX.json"),
        r#"{"documents": [{"id": "doc1", "title": "Test"}]}"#,
    )
    .unwrap();

    let result = rebuild_index_from_json(dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("missing chunks array"));
}

#[test]
fn test_rebuild_index_from_json_empty_arrays() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("INDEX.json"),
        r#"{"documents": [], "chunks": []}"#,
    )
    .unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert!(dir.path().join(".tantivy_index").exists());

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    assert_eq!(searcher.num_docs(), 0);
}

#[test]
fn test_rebuild_index_from_json_with_documents_and_chunks() {
    let dir = TempDir::new().unwrap();

    let chunks_dir = dir.path().join("chunks");
    std::fs::create_dir_all(&chunks_dir).unwrap();
    std::fs::write(
        chunks_dir.join("doc1-0-standard.md"),
        "Chunk content about Rust programming",
    )
    .unwrap();

    let index_json = r#"{"documents": [{"id": "doc1", "title": "Rust Guide", "summary": "Learn Rust", "path": "docs/rust.md", "category": "tutorial", "tags": ["rust"], "word_count": 100, "chunk_ids": ["doc1#0"], "headings": ["Introduction"], "content": "Rust programming language"}], "chunks": [{"chunk_id": "doc1#0", "doc_id": "doc1", "doc_title": "Rust Guide", "summary": "Chunk summary", "token_count": 50, "heading": "Introduction", "chunk_level": "standard"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(index.reader().unwrap().searcher().num_docs(), 1);
}

#[test]
fn test_rebuild_index_from_json_chunk_with_frontmatter() {
    let dir = TempDir::new().unwrap();

    let chunks_dir = dir.path().join("chunks");
    std::fs::create_dir_all(&chunks_dir).unwrap();
    let chunk_content = "---\ntitle: Chunk Title\n---\nActual chunk content here";
    std::fs::write(chunks_dir.join("doc1-0-standard.md"), chunk_content).unwrap();

    let index_json = r#"{"documents": [{"id": "doc1", "title": "Doc", "summary": "Summary", "path": "docs/doc.md", "category": "concept", "tags": [], "word_count": 50, "chunk_ids": ["doc1#0"], "headings": [], "content": "Content"}], "chunks": [{"chunk_id": "doc1#0", "doc_id": "doc1", "doc_title": "Doc", "summary": "Chunk summary", "token_count": 30, "heading": null, "chunk_level": "standard"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(index.reader().unwrap().searcher().num_docs(), 1);
}

#[test]
fn test_rebuild_index_from_json_chunk_file_missing() {
    let dir = TempDir::new().unwrap();

    let index_json = r#"{"documents": [{"id": "doc1", "title": "Doc", "summary": "Summary", "path": "docs/doc.md", "category": "concept", "tags": [], "word_count": 50, "chunk_ids": ["doc1#0"], "headings": [], "content": "Content"}], "chunks": [{"chunk_id": "doc1#0", "doc_id": "doc1", "doc_title": "Doc", "summary": "Chunk summary", "token_count": 30, "heading": null, "chunk_level": "standard"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(
        index.reader().unwrap().searcher().num_docs(),
        0,
        "Missing chunk files should be skipped"
    );
}

#[test]
fn test_rebuild_index_from_json_summary_chunk_level() {
    let dir = TempDir::new().unwrap();

    let chunks_dir = dir.path().join("chunks");
    std::fs::create_dir_all(&chunks_dir).unwrap();
    std::fs::write(
        chunks_dir.join("doc1-0-summary.md"),
        "Summary chunk content",
    )
    .unwrap();

    let index_json = r#"{"documents": [], "chunks": [{"chunk_id": "doc1#0", "doc_id": "doc1", "doc_title": "Doc", "summary": "Sum", "token_count": 20, "heading": null, "chunk_level": "summary"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(index.reader().unwrap().searcher().num_docs(), 1);
}

#[test]
fn test_rebuild_index_from_json_detailed_chunk_level() {
    let dir = TempDir::new().unwrap();

    let chunks_dir = dir.path().join("chunks");
    std::fs::create_dir_all(&chunks_dir).unwrap();
    std::fs::write(
        chunks_dir.join("doc1-0-detailed.md"),
        "Detailed chunk content",
    )
    .unwrap();

    let index_json = r#"{"documents": [], "chunks": [{"chunk_id": "doc1#0", "doc_id": "doc1", "doc_title": "Doc", "summary": "Det", "token_count": 20, "heading": null, "chunk_level": "detailed"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(index.reader().unwrap().searcher().num_docs(), 1);
}

#[test]
fn test_rebuild_index_from_json_doc_with_minimal_fields() {
    let dir = TempDir::new().unwrap();

    let chunks_dir = dir.path().join("chunks");
    std::fs::create_dir_all(&chunks_dir).unwrap();
    std::fs::write(chunks_dir.join("id-standard.md"), "chunk text").unwrap();

    let index_json = r#"{"documents": [{"id": "minimal"}], "chunks": [{"chunk_id": "id", "doc_id": "", "doc_title": "", "summary": "", "token_count": 0, "heading": null, "chunk_level": "standard"}]}"#;

    std::fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

    let index = rebuild_index_from_json(dir.path()).unwrap();
    assert_eq!(index.reader().unwrap().searcher().num_docs(), 1);
}
