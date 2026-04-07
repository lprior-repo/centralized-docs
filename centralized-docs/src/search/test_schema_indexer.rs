use super::test_support::*;
use super::*;
use tempfile::TempDir;

#[test]
fn test_open_or_create_index_new() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let index_path = dir.path();

    let _index = open_or_create_index(index_path)?;
    assert!(index_path.join(".tantivy_index").exists());

    Ok(())
}

#[test]
fn test_open_or_create_index_existing() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let index_path = dir.path();

    let _index1 = open_or_create_index(index_path)?;
    let _index2 = open_or_create_index(index_path)?;

    assert!(index_path.join(".tantivy_index").exists());

    Ok(())
}

#[test]
fn test_open_or_create_index_recovers_from_file_path() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let index_path = dir.path();
    let index_dir = index_path.join(".tantivy_index");

    std::fs::write(&index_dir, "not a directory")?;

    let _index = open_or_create_index(index_path)?;

    assert!(index_dir.exists());
    assert!(index_dir.is_dir());

    Ok(())
}

#[test]
fn test_create_schema() {
    let (schema, fields) = create_schema();

    assert!(schema.get_field("id").is_ok());
    assert!(schema.get_field("title").is_ok());
    assert!(schema.get_field("summary").is_ok());
    assert!(schema.get_field("content").is_ok());
    assert!(schema.get_field("category").is_ok());
    assert!(schema.get_field("word_count").is_ok());
    assert!(schema.get_field("path").is_ok());

    assert_eq!(fields.id, schema.get_field("id").unwrap());
    assert_eq!(fields.title, schema.get_field("title").unwrap());
    assert_eq!(fields.summary, schema.get_field("summary").unwrap());
    assert_eq!(fields.content, schema.get_field("content").unwrap());
    assert_eq!(fields.category, schema.get_field("category").unwrap());
    assert_eq!(fields.word_count, schema.get_field("word_count").unwrap());
    assert_eq!(fields.path, schema.get_field("path").unwrap());
}

#[test]
fn test_schema_fields_struct_fields() {
    let (_, fields) = create_schema();
    let _ = SchemaFields {
        id: fields.id,
        title: fields.title,
        summary: fields.summary,
        content: fields.content,
        category: fields.category,
        word_count: fields.word_count,
        path: fields.path,
    };
}

#[test]
fn test_index_documents_single() {
    let docs = vec![make_index_document(
        "doc1",
        "Rust Programming",
        "A guide to Rust",
        "Rust is a systems programming language",
        "tutorial",
    )];
    let (_dir, index) = create_test_index_with_docs(&docs);
    let results = search_index(&index, "Rust", 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_index_documents_multiple() {
    let docs = vec![
        make_index_document(
            "doc1",
            "Rust Basics",
            "Intro to Rust",
            "Rust basics tutorial",
            "tutorial",
        ),
        make_index_document(
            "doc2",
            "Python Guide",
            "Python basics",
            "Python programming guide",
            "tutorial",
        ),
        make_index_document(
            "doc3",
            "API Reference",
            "HTTP API",
            "REST API endpoints",
            "ref",
        ),
    ];
    let (_dir, index) = create_test_index_with_docs(&docs);
    let results = search_index(&index, "Rust", 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_index_documents_empty() {
    let docs: Vec<crate::index::IndexDocument> = vec![];
    let (_dir, index) = create_test_index_with_docs(&docs);
    let results = search_index(&index, "nonexistent", 10).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_index_documents_with_tags_and_headings() {
    let mut doc = make_index_document(
        "doc1",
        "Advanced Rust",
        "Advanced patterns",
        "Pattern matching and traits in Rust",
        "concept",
    );
    doc.tags = vec!["rust".to_string(), "patterns".to_string()];
    doc.headings = vec!["Introduction".to_string(), "Patterns".to_string()];
    let (_dir, index) = create_test_index_with_docs(&[doc]);
    let results = search_index(&index, "Advanced", 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_index_chunks_basic() {
    let docs = vec![make_index_document(
        "doc1",
        "Test Document",
        "Test summary",
        "Test content about programming",
        "concept",
    )];
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Test Document",
            "First chunk content about Rust",
            Some("Introduction"),
        ),
        make_chunk(
            "doc1#1-standard",
            "doc1",
            "Test Document",
            "Second chunk about programming",
            Some("Details"),
        ),
    ];

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &docs, &chunks).unwrap();
    writer.commit().unwrap();

    let results = search_index(&index, "Rust", 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_index_chunks_empty_docs_and_chunks() {
    let docs: Vec<crate::index::IndexDocument> = vec![];
    let chunks: Vec<crate::chunking_adapter::Chunk> = vec![];

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &docs, &chunks).unwrap();
    writer.commit().unwrap();
}

#[test]
fn test_index_chunks_doc_not_in_doc_map() {
    let chunks = vec![make_chunk(
        "orphan#0-standard",
        "orphan_doc",
        "Orphan Doc",
        "Orphan chunk content",
        None,
    )];

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &[], &chunks).unwrap();
    writer.commit().unwrap();
}

#[test]
fn test_index_chunks_no_heading() {
    let docs = vec![make_index_document(
        "doc1",
        "No Heading Doc",
        "Summary",
        "Content",
        "concept",
    )];
    let chunks = vec![make_chunk(
        "doc1#0-standard",
        "doc1",
        "No Heading Doc",
        "Chunk without heading",
        None,
    )];

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &docs, &chunks).unwrap();
    writer.commit().unwrap();
}

#[test]
fn test_index_chunks_summary_level() {
    let docs = vec![make_index_document(
        "doc1",
        "Summary Doc",
        "Summary",
        "Content",
        "tutorial",
    )];
    let mut chunk = make_chunk("doc1#0", "doc1", "Summary Doc", "Summary content", None);
    chunk.chunk_level = contextual_chunker::ChunkLevel::Summary;

    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_chunks(&mut writer, &docs, &[chunk]).unwrap();
    writer.commit().unwrap();
}

#[test]
fn test_open_or_create_index_recovers_from_corrupted_index_with_json() {
    let dir = TempDir::new().unwrap();
    let index_dir = dir.path().join(".tantivy_index");
    std::fs::create_dir_all(&index_dir).unwrap();

    std::fs::write(index_dir.join("managed.json"), "corrupted").unwrap();

    std::fs::write(
        dir.path().join("INDEX.json"),
        r#"{"documents": [], "chunks": []}"#,
    )
    .unwrap();

    let _index = open_or_create_index(dir.path()).unwrap();
    assert!(index_dir.exists());
    assert!(index_dir.is_dir());
}

#[test]
fn test_open_or_create_index_corrupted_no_json_falls_back() {
    let dir = TempDir::new().unwrap();
    let index_dir = dir.path().join(".tantivy_index");
    std::fs::create_dir_all(&index_dir).unwrap();
    std::fs::write(index_dir.join("managed.json"), "corrupted").unwrap();

    let _index = open_or_create_index(dir.path()).unwrap();
    assert!(index_dir.is_dir());
}
