//! Quick manual test to verify current error message behavior
use std::path::Path;
use tempfile::TempDir;

fn create_test_index(dir: &Path) -> anyhow::Result<()> {
    let docs = vec![doc_transformer::index::IndexDocument {
        id: "test/doc1".to_string(),
        title: "Test Document".to_string(),
        summary: "A test document with content".to_string(),
        path: "test/doc1.md".to_string(),
        category: "test".to_string(),
        word_count: 10,
        tags: vec![],
        chunk_ids: vec![],
    }];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    doc_transformer::search::index_documents(&index, docs)?;
    Ok(())
}

fn main() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();

    println!("Current error messages for invalid queries:\n");
    println!("1. Query with <script> tags:");
    match doc_transformer::search::search_index(&index, "test<script>alert(1)</script>", 10) {
        Ok(_) => println!("   OK"),
        Err(e) => println!("   Error: {}\n", e),
    }

    println!("2. Query with trailing operator:");
    match doc_transformer::search::search_index(&index, "test AND", 10) {
        Ok(_) => println!("   OK"),
        Err(e) => println!("   Error: {}\n", e),
    }

    println!("3. Query with unclosed quote:");
    match doc_transformer::search::search_index(&index, "\"unclosed", 10) {
        Ok(_) => println!("   OK"),
        Err(e) => println!("   Error: {}\n", e),
    }

    println!("4. Query with unbalanced parentheses:");
    match doc_transformer::search::search_index(&index, "(unbalanced", 10) {
        Ok(_) => println!("   OK"),
        Err(e) => println!("   Error: {}", e),
    }
}
