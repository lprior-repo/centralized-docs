#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! Test to verify Tantivy index is properly populated

use tempfile::TempDir;

#[test]
fn test_tantivy_index_is_populated() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory to simulate test_output
    let temp_dir = TempDir::new()?;
    let test_output = temp_dir.path();
    let tantivy_index_dir = test_output.join(".tantivy_index");

    // Create some test documents to index
    let docs = vec![
        doc_transformer::index::IndexDocument {
            id: "test/doc1".to_string(),
            title: "Test Document 1".to_string(),
            summary: "This is a test summary for document 1".to_string(),
            path: "docs/test1.md".to_string(),
            category: "test".to_string(),
            content: String::new().into(),
            word_count: 100,
            tags: vec!["test".to_string()],
            chunk_ids: vec!["chunk1".to_string()],
            headings: vec!["Test 1".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/doc2".to_string(),
            title: "Test Document 2".to_string(),
            summary: "This is a test summary for document 2".to_string(),
            path: "docs/test2.md".to_string(),
            category: "test".to_string(),
            content: String::new().into(),
            word_count: 150,
            tags: vec!["test".to_string()],
            chunk_ids: vec!["chunk2".to_string()],
            headings: vec!["Test 2".to_string()],
        },
    ];

    // Create INDEX.json to simulate existing documents
    let index_json = test_output.join("INDEX.json");
    let index_content = serde_json::json!({
        "documents": docs.iter().map(|d| serde_json::json!({
            "id": d.id,
            "title": d.title,
            "summary": d.summary,
            "path": d.path,
            "category": d.category,
            "word_count": d.word_count,
            "tags": d.tags,
            "chunk_ids": d.chunk_ids,
            "headings": d.headings,
        })).collect::<Vec<_>>()
    });
    std::fs::write(&index_json, serde_json::to_string_pretty(&index_content)?)?;

    // Index the documents
    let index = doc_transformer::search::open_or_create_index(test_output)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;

    // Verify Tantivy index directory exists
    assert!(
        tantivy_index_dir.exists(),
        ".tantivy_index directory should exist"
    );

    // Verify INDEX.json exists
    assert!(index_json.exists(), "INDEX.json should exist");

    // Read INDEX.json to get expected document count
    let index_content = std::fs::read_to_string(&index_json)?;
    let index_value: serde_json::Value = serde_json::from_str(&index_content)?;
    let doc_count = index_value["documents"]
        .as_array()
        .map(|arr| arr.len())
        .unwrap_or(0);

    assert!(
        doc_count > 0,
        "INDEX.json should contain at least one document"
    );

    // Verify Tantivy index has segment files (not just metadata)
    let entries = std::fs::read_dir(&tantivy_index_dir)?;
    let mut has_segment_files = false;

    for entry in entries {
        let path = entry?.path();
        if let Some(filename) = path.file_name() {
            let name = filename.to_string_lossy();
            // Check for segment files (not meta.json, lock files, etc.)
            if name.ends_with(".store")
                || name.ends_with(".fast")
                || name.ends_with(".idx")
                || name.ends_with(".term")
                || name.ends_with(".pos")
                || name.ends_with(".fieldnorm")
            {
                has_segment_files = true;
                break;
            }
        }
    }

    assert!(
        has_segment_files,
        "Tantivy index should contain segment files (*.store, *.fast, etc.)"
    );

    Ok(())
}

#[test]
fn test_search_returns_results() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory to simulate test_output
    let temp_dir = TempDir::new()?;
    let test_output = temp_dir.path();

    // Create some test documents to index
    let docs = vec![doc_transformer::index::IndexDocument {
        id: "test/doc1".to_string(),
        title: "Test Document About Rust".to_string(),
        summary: "This document discusses rust programming".to_string(),
        path: "docs/rust.md".to_string(),
        category: "programming".to_string(),
        content: String::new().into(),
        word_count: 100,
        tags: vec!["rust".to_string()],
        chunk_ids: vec!["chunk1".to_string()],
        headings: vec!["Rust".to_string()],
    }];

    // Index the documents
    let index = doc_transformer::search::open_or_create_index(test_output)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;

    // Try searching - just verify the search executes without error
    let results = doc_transformer::search::search_index(&index, "rust", 10)?;

    // Search should succeed and return results (may be empty if no matches)
    // The important part is that it doesn't panic or error
    assert!(
        !results.is_empty(),
        "Search for 'rust' should find the test document"
    );

    Ok(())
}
