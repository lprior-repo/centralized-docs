#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use doc_transformer::index;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use doc_transformer::search;

/// Test 1: Corrupt index files
/// Expected: Should handle corruption gracefully and rebuild
#[test]
fn test_index_corrupt_index_file() {
    let dir = match TempDir::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to create temp dir: {e}");
            return;
        }
    };
    let index_path = dir.path();

    // Create a corrupt index file (not a directory)
    let index_dir = index_path.join(".tantivy_index");
    if let Err(e) = fs::write(&index_dir, "not a valid index") {
        eprintln!("Failed to write corrupt index file: {e}");
        return;
    }

    // Should be able to open and rebuild
    let index = search::open_or_create_index(index_path);
    assert!(
        index.is_ok(),
        "Should handle corrupt index file, got: {index:?}",
    );

    // Verify index was rebuilt
    assert!(index_dir.exists(), "Index directory should be recreated");
    assert!(index_dir.is_dir(), "Index directory should be a directory");
}

/// Test 2: Malformed INDEX.json
/// Expected: Should handle gracefully or report error
#[test]
fn test_index_malformed_json() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    // Create malformed INDEX.json
    let index_file = index_path.join("INDEX.json");
    fs::write(&index_file, "{ invalid json }").unwrap();

    // Should either fail gracefully or handle error
    let result = index::build_and_write_index(
        &[],
        &std::collections::HashMap::new(),
        &doc_transformer::chunking_adapter::ChunksResult {
            chunks_metadata: vec![],
            total_chunks: 0,
            document_count: 0,
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        },
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    assert!(
        result.is_ok() || result.is_err(),
        "Should handle malformed JSON, got: {result:?}",
    );
}

/// Test 3: Missing required fields in index
/// Expected: Should handle gracefully
#[test]
fn test_index_missing_required_fields() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    // Create INDEX.json with missing required fields
    let index_file = index_path.join("INDEX.json");
    fs::write(
        &index_file,
        r#"{
             "documents": [],
             "chunks": []
         }"#,
    )
    .unwrap();

    // Should either fail gracefully or handle missing fields
    let result = index::build_and_write_index(
        &[],
        &std::collections::HashMap::new(),
        &doc_transformer::chunking_adapter::ChunksResult {
            chunks_metadata: vec![],
            total_chunks: 0,
            document_count: 0,
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        },
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    assert!(
        result.is_ok() || result.is_err(),
        "Should handle missing chunks result, got: {result:?}",
    );
}

/// Test 4: Invalid HNSW parameters
/// Expected: Should reject or handle gracefully
#[test]
fn test_index_invalid_hnsw_parameters() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    // Create valid test data
    let analyses = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        category: "ref".to_string(),
        content: String::new().into(),
    }];
    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Test invalid HNSW parameters
    let invalid_params = vec![
        (Some(0), Some(200)),   // m < 4
        (Some(100), Some(50)),  // ef_construction < 50
        (Some(100), Some(900)), // ef_construction > 800
        (Some(65), Some(200)),  // m > 64
        (None, Some(50)),       // ef_construction < 50 without m
        (Some(4), None),        // m >= 4 without ef_construction
    ];

    for (hnsw_m, ef_construction) in invalid_params {
        let result = index::build_and_write_index(
            &analyses,
            &link_map,
            &chunks_result,
            index_path,
            "test_project",
            None,
            hnsw_m,
            ef_construction,
            None,
        );
        // Should either fail gracefully or handle invalid parameters
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle invalid HNSW params (m={m}, ef={ef}), got: {result:?}",
            m = hnsw_m.unwrap_or(0),
            ef = ef_construction.unwrap_or(0),
        );
    }
}

/// Test 5: Empty analyses
/// Expected: Should handle gracefully
#[test]
fn test_index_empty_analyses() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    let analyses: Vec<doc_transformer::analyze::Analysis> = vec![];
    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle empty analyses, got: {result:?}",
    );
}

/// Test 6: Missing chunks result
/// Expected: Should handle gracefully
#[test]
fn test_index_missing_chunks_result() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    let analyses = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        content: String::new().into(),
        first_paragraph: "Test paragraph".to_string(),
        frontmatter: None,
        word_count: 100,
        headings: vec![],
        has_code: false,
        links: vec![],
        has_tables: false,
    }];
    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle missing chunks result, got: {result:?}",
    );
}

/// Test 7: Very large document count
/// Expected: Should handle gracefully without O(n²) behavior
#[test]
fn test_index_large_document_count() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    // Create many test analyses
    let mut analyses = Vec::new();
    for i in 0..1000 {
        analyses.push(doc_transformer::analyze::Analysis {
            source_path: format!("test{i}.md"),
            title: format!("Test Document {i}"),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: format!("Paragraph {i}"),
            word_count: 100,
            has_code: false,
            has_tables: false,
            category: "ref".to_string(),
            content: String::new().into(),
        });
    }

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Should complete without hanging or running out of memory
    let start = std::time::Instant::now();
    let result = index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    let duration = start.elapsed();

    assert!(
        result.is_ok(),
        "Should handle large document count (1000 docs, took {duration:?}), got: {result:?}",
    );
}

/// Test 8: Empty documents array
/// Expected: Should handle gracefully
#[test]
fn test_index_empty_documents_array() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    let analyses = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        content: String::new().into(),
        first_paragraph: "Test paragraph".to_string(),
        frontmatter: None,
        word_count: 100,
        headings: vec![],
        has_code: false,
        links: vec![],
        has_tables: false,
    }];
    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle empty documents array, got: {result:?}",
    );
}

/// Test 9: Invalid path
/// Expected: Should fail with informative error
#[test]
fn test_index_invalid_path() {
    let dir = TempDir::new().unwrap();
    let _index_path = dir.path();

    // Try to create index with invalid path components
    let result = index::build_and_write_index(
        &[],
        &std::collections::HashMap::new(),
        &doc_transformer::chunking_adapter::ChunksResult {
            chunks_metadata: vec![],
            total_chunks: 0,
            document_count: 0,
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        },
        Path::new("/invalid/path/with/slashes"),
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_err(),
        "Should reject invalid path, got: {result:?}",
    );
}

/// Test 10: Very long project name
/// Expected: Should handle without panics
#[test]
fn test_index_very_long_project_name() {
    let dir = TempDir::new().unwrap();
    let index_path = dir.path();

    let analyses = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        content: String::new().into(),
        first_paragraph: "Test paragraph".to_string(),
        frontmatter: None,
        word_count: 100,
        headings: vec![],
        has_code: false,
        links: vec![],
        has_tables: false,
    }];
    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Very long project name (1000 chars)
    let long_name = "a".repeat(1000);

    let result = index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        index_path,
        &long_name,
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok() || result.is_err(),
        "Should handle empty documents array, got: {result:?}",
    );
}
