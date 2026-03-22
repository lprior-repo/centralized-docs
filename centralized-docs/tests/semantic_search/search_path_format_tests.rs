//! Tests for search result path format consistency (doc-tx-ss7)
//!
//! This module tests that search results display paths that match the actual
//! file locations in INDEX.json. Paths should use hyphens (tutorial-general-test2.md)
//! not slashes (tutorial/general/test2).

use std::path::Path;
use tempfile::TempDir;

/// Create a test index with a document that has a hierarchical ID
fn create_test_index_with_hierarchical_id(dir: &Path) -> anyhow::Result<()> {
    let docs = vec![doc_transformer::index::IndexDocument {
        id: "tutorial/general/test2".to_string(),
        title: "Test Tutorial Document".to_string(),
        summary: "A test tutorial document with hierarchical ID".to_string(),
        path: "docs/tutorial-general-test2.md".to_string(), // Correct format with hyphens
        category: "tutorial".to_string(),
        content: String::new().into(),
        word_count: 20,
        tags: vec![],
        chunk_ids: vec![],
        headings: vec![],
    }];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &docs)?;
    writer.commit()?;
    Ok(())
}

#[test]
fn test_search_result_path_uses_hyphens_not_slashes() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index_with_hierarchical_id(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let result = doc_transformer::search::search_index(&index, "tutorial", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results");

    // The document ID is "tutorial/general/test2"
    // The correct path format should be "docs/tutorial-general-test2.md" (hyphens, not slashes)
    // NOT "docs/tutorial/general/test2" (slashes, wrong format)
    let search_result = &results[0];

    assert_eq!(
        search_result.path, "docs/tutorial-general-test2.md",
        "Path should use hyphens to replace slashes, matching actual file location. \
         Expected: docs/tutorial-general-test2.md, Got: {}",
        search_result.path
    );
}

#[test]
fn test_search_result_path_includes_md_extension() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index_with_hierarchical_id(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let result = doc_transformer::search::search_index(&index, "test", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results");

    let search_result = &results[0];

    assert!(
        search_result.path.ends_with(".md"),
        "Path should include .md extension. Got: {}",
        search_result.path
    );
}

#[test]
fn test_search_result_single_segment_id() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    // Test document with single-segment ID (no subcategory)
    let docs = vec![doc_transformer::index::IndexDocument {
        id: "ref-api".to_string(),
        title: "Reference API".to_string(),
        summary: "API reference documentation".to_string(),
        path: "docs/ref-api.md".to_string(),
        category: "ref".to_string(),
        content: String::new().into(),
        word_count: 15,
        tags: vec![],
        chunk_ids: vec![],
        headings: vec![],
    }];

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    doc_transformer::search::index_documents(&mut writer, &docs).unwrap();
    writer.commit().unwrap();

    let result = doc_transformer::search::search_index(&index, "api", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results");

    let search_result = &results[0];

    assert_eq!(
        search_result.path, "docs/ref-api.md",
        "Single-segment ID should produce correct path. Expected: docs/ref-api.md, Got: {}",
        search_result.path
    );
}

#[test]
fn test_search_result_path_matches_index_json_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    let test_doc = doc_transformer::index::IndexDocument {
        id: "category/subcategory/slug".to_string(),
        title: "Test Document".to_string(),
        summary: "Test summary".to_string(),
        path: "docs/category-subcategory-slug.md".to_string(),
        category: "category".to_string(),
        content: String::new().into(),
        word_count: 10,
        tags: vec![],
        chunk_ids: vec![],
        headings: vec![],
    };

    let docs = vec![test_doc.clone()];
    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    doc_transformer::search::index_documents(&mut writer, &docs).unwrap();
    writer.commit().unwrap();

    let result = doc_transformer::search::search_index(&index, "test", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results");

    let search_result = &results[0];

    // Postcondition: result.path matches INDEX.json.documents[i].path
    assert_eq!(
        search_result.path, test_doc.path,
        "Search result path must match INDEX.json path exactly. \
         Expected (from INDEX.json): {}, Got (from search): {}",
        test_doc.path, search_result.path
    );
}

#[test]
fn test_search_result_prefers_indexed_path_over_id_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    let docs = vec![doc_transformer::index::IndexDocument {
        id: "category/subcategory/slug".to_string(),
        title: "Custom Path Document".to_string(),
        summary: "Doc with a custom path stored in the index".to_string(),
        path: "docs/custom-path-alias.md".to_string(),
        category: "category".to_string(),
        content: String::new().into(),
        word_count: 12,
        tags: vec![],
        chunk_ids: vec![],
        headings: vec![],
    }];

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    doc_transformer::search::index_documents(&mut writer, &docs).unwrap();
    writer.commit().unwrap();

    let result = doc_transformer::search::search_index(&index, "custom path", 10);

    assert!(result.is_ok(), "Search should succeed");
    let results = result.unwrap();
    assert!(!results.is_empty(), "Should find results");

    let search_result = &results[0];
    assert_eq!(
        search_result.path, "docs/custom-path-alias.md",
        "Search results should return the stored path from INDEX.json. Expected: \
         docs/custom-path-alias.md, Got: {}",
        search_result.path
    );
}
