//! Comprehensive tests for single file indexing (P1 file-discovery-single-file)
//!
//! Test cases:
//! 1. Index a single .md file should work
//! 2. Index a single .txt file should work
//! 3. Index a single .rst file should work
//! 4. Index a single unsupported file (.json) should fail gracefully
//! 5. Index a non-existent single file should fail
//! 6. Index a single file from subdirectory should work
//! 7. Verify INDEX.json contains exactly 1 document
//! 8. Verify chunks are generated for single file

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test 1: Index a single .md file should work
#[test]
fn test_index_single_md_file_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single markdown file
    let md_file = source_dir.join("test.md");
    let content = r#"# Test Document

This is a test document for single file indexing.

## Section 1

Some content here.

## Section 2

More content.
"#;
    fs::write(&md_file, content).expect("Failed to write test file");

    // Discover files
    let (files, manifest) = doc_transformer::discover::discover_files(&md_file, None)
        .expect("discover_files should succeed for single .md file");

    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

    // Analyze files
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    assert_eq!(analyze_result.len(), 1, "Should analyze exactly 1 file");
    assert_eq!(analyze_result[0].title, "Test Document");
    assert_eq!(analyze_result[0].source_path, "test.md");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);
    assert_eq!(analyses.len(), 1);
    assert_eq!(link_map.len(), 1);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform files
    let transform_result =
        doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
            .expect("transform_all should succeed");
    assert_eq!(transform_result.success_count, 1);
    assert_eq!(transform_result.total_count, 1);
    assert_eq!(transform_result.error_count, 0);

    // Verify transformed file exists
    let docs_dir = output_dir.join("docs");
    assert!(docs_dir.exists(), "docs directory should exist");
    let transformed_files = fs::read_dir(&docs_dir)
        .expect("Failed to read docs directory")
        .count();
    assert_eq!(transformed_files, 1, "Should have 1 transformed file");

    // Chunk files
    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");
    assert!(chunks_result.total_chunks > 0, "Should generate chunks");
    assert_eq!(chunks_result.document_count, 1);

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "test_project",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json exists and has 1 document
    let index_path = output_dir.join("INDEX.json");
    assert!(index_path.exists(), "INDEX.json should exist");

    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );

    let stats = index["stats"].as_object().expect("stats should exist");
    let doc_count = stats["doc_count"].as_u64().expect("doc_count should exist");
    assert_eq!(doc_count, 1, "stats.doc_count should be 1");

    // Verify chunks are in index
    let chunks = index["chunks"]
        .as_array()
        .expect("chunks should be an array");
    assert!(!chunks.is_empty(), "INDEX.json should contain chunks");
}

/// Test 2: Index a single .txt file should work
#[test]
fn test_index_single_txt_file_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single text file
    let txt_file = source_dir.join("readme.txt");
    let content = r#"README File

This is a plain text file that should be indexed.

It has multiple paragraphs.

And should work with the indexing pipeline.
"#;
    fs::write(&txt_file, content).expect("Failed to write test file");

    // Discover files
    let (files, manifest) = doc_transformer::discover::discover_files(&txt_file, None)
        .expect("discover_files should succeed for single .txt file");

    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

    // Analyze files
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    assert_eq!(analyze_result.len(), 1, "Should analyze exactly 1 file");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform and chunk
    let transform_result =
        doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
            .expect("transform_all should succeed");
    assert_eq!(transform_result.success_count, 1);

    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");
    assert!(chunks_result.total_chunks > 0);
    assert_eq!(chunks_result.document_count, 1);

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "test_project",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );
}

/// Test 3: Index a single .rst file should work
#[test]
fn test_index_single_rst_file_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single reStructuredText file
    let rst_file = source_dir.join("docs.rst");
    let content = r#"Documentation
=============

This is a reStructuredText file.

Subsection
----------

With some content.
"#;
    fs::write(&rst_file, content).expect("Failed to write test file");

    // Discover files
    let (files, manifest) = doc_transformer::discover::discover_files(&rst_file, None)
        .expect("discover_files should succeed for single .rst file");

    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

    // Analyze files
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    assert_eq!(analyze_result.len(), 1, "Should analyze exactly 1 file");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform and chunk
    let transform_result =
        doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
            .expect("transform_all should succeed");
    assert_eq!(transform_result.success_count, 1);

    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");
    assert!(chunks_result.total_chunks > 0);
    assert_eq!(chunks_result.document_count, 1);

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "test_project",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );
}

/// Test 4: Index a single unsupported file (.json) should fail gracefully
#[test]
fn test_index_single_unsupported_json_fails_gracefully() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();

    // Create a single JSON file (unsupported extension)
    let json_file = source_dir.join("data.json");
    let content = r#"{"key": "value", "number": 123}"#;
    fs::write(&json_file, content).expect("Failed to write test file");

    // Discover files - should succeed but find no supported files
    let (files, manifest) = doc_transformer::discover::discover_files(&json_file, None)
        .expect("discover_files should succeed even for unsupported file");

    assert_eq!(
        files.len(),
        0,
        "Should discover 0 files for unsupported extension"
    );
    assert_eq!(manifest.total_files, 0, "Manifest should show 0 files");

    // Analyze files with empty input - returns empty Vec successfully
    // This is correct behavior: unsupported files are filtered at discovery,
    // not at analysis. Analysis only errors when files exist but fail to read.
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None);

    // With 0 input files, analyze returns an empty Vec (not an error)
    // The error case is when input_count > 0 but all analyses fail
    assert!(
        analyze_result.is_ok(),
        "analyze_files should succeed with 0 files (returns empty Vec)"
    );
    let analyses = analyze_result.unwrap().analyses;
    assert_eq!(
        analyses.len(),
        0,
        "Should have 0 analyses for 0 input files"
    );

    // The full pipeline would detect 0 documents and fail during validation
    // since we can't create a meaningful index from nothing
}

/// Test 5: Index a non-existent single file should fail
#[test]
fn test_index_nonexistent_single_file_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let nonexistent_file = temp_dir.path().join("does_not_exist.md");

    // Discover files - should fail with clear error
    let result = doc_transformer::discover::discover_files(&nonexistent_file, None);

    assert!(
        result.is_err(),
        "discover_files should error for non-existent file"
    );
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("not found") || error_msg.contains("No such file"),
            "Error should mention 'not found': {error_msg}"
        );
    }
}

/// Test 6: Index a single file from subdirectory should work
#[test]
fn test_index_single_file_from_subdirectory_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create subdirectory
    let subdir = source_dir.join("subdir");
    fs::create_dir(&subdir).expect("Failed to create subdirectory");

    // Create a file in the subdirectory
    let md_file = subdir.join("test.md");
    let content = r#"# Test Document in Subdirectory

This file is in a subdirectory.

## Content

Some content here.
"#;
    fs::write(&md_file, content).expect("Failed to write test file");

    // Discover files using the full path to the file
    let (files, manifest) = doc_transformer::discover::discover_files(&md_file, None)
        .expect("discover_files should succeed for file in subdirectory");

    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");
    // The discovered file should use just the filename
    assert_eq!(files[0].source_path, "test.md");

    // Use the parent directory as the analysis base path
    let analysis_base = PathBuf::from(&manifest.source_dir);

    // Analyze files
    let analyze_result = doc_transformer::analyze::analyze_files(&files, &analysis_base, None)
        .expect("analyze_files should succeed");

    assert_eq!(analyze_result.len(), 1, "Should analyze exactly 1 file");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform and chunk
    let transform_result =
        doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
            .expect("transform_all should succeed");
    assert_eq!(transform_result.success_count, 1);

    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");
    assert!(chunks_result.total_chunks > 0);
    assert_eq!(chunks_result.document_count, 1);

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "test_project",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );
}

/// Test 7: Verify INDEX.json contains exactly 1 document for single file
#[test]
fn test_index_json_contains_exactly_one_document() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single markdown file
    let md_file = source_dir.join("single.md");
    let content = r#"# Single Document

This is the only document being indexed.

## Section A

Content A.

## Section B

Content B.
"#;
    fs::write(&md_file, content).expect("Failed to write test file");

    // Full pipeline: discover
    let (files, _manifest) = doc_transformer::discover::discover_files(&md_file, None)
        .expect("discover_files should succeed");

    // Analyze
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform
    doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
        .expect("transform_all should succeed");

    // Chunk
    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "single_file_test",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json structure
    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    // Check document count
    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );

    // Check stats
    let stats = index["stats"].as_object().expect("stats should exist");
    assert_eq!(
        stats["doc_count"].as_u64().expect("doc_count should exist"),
        1
    );
    assert!(
        stats["chunk_count"]
            .as_u64()
            .expect("chunk_count should exist")
            > 0
    );

    // Verify document properties
    let doc = &docs[0];
    assert!(doc["id"].is_string(), "Document should have an id");
    assert!(doc["title"].is_string(), "Document should have a title");
    assert!(doc["path"].is_string(), "Document should have a path");
    assert!(
        doc["category"].is_string(),
        "Document should have a category"
    );
    assert!(doc["tags"].is_array(), "Document should have tags");
    assert!(doc["summary"].is_string(), "Document should have a summary");
    assert!(
        doc["word_count"].is_number(),
        "Document should have word_count"
    );
    assert!(
        doc["chunk_ids"].is_array(),
        "Document should have chunk_ids"
    );
    assert!(doc["headings"].is_array(), "Document should have headings");

    // Verify chunks exist and reference the document
    let chunks = index["chunks"]
        .as_array()
        .expect("chunks should be an array");
    assert!(!chunks.is_empty(), "Should have chunks");

    // All chunks should reference the same document
    for chunk in chunks {
        let doc_id = doc["id"].as_str().expect("doc id should be a string");
        let chunk_doc_id = chunk["doc_id"]
            .as_str()
            .expect("chunk doc_id should be a string");
        assert_eq!(
            chunk_doc_id, doc_id,
            "All chunks should reference the single document"
        );
    }
}

/// Test 8: Verify chunks are generated for single file
#[test]
fn test_chunks_generated_for_single_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single markdown file with enough content to generate multiple chunks
    let md_file = source_dir.join("chunkable.md");
    let content = r#"# Document for Chunking

This document has multiple sections to test chunking.

## Introduction

This is the introduction paragraph with some initial content.

## Main Content

Here is the main content of the document. It contains multiple paragraphs
that should be split into chunks for proper semantic indexing.

### Subsection 1

Additional content under a subsection to test heading hierarchy.

### Subsection 2

More content to ensure we have enough material for meaningful chunking.

## Advanced Topics

This section covers advanced topics related to the document subject.

The content should be substantial enough to create separate chunks.

## Conclusion

Finally, we have a conclusion section that wraps up the document.
"#;
    fs::write(&md_file, content).expect("Failed to write test file");

    // Full pipeline
    let (files, _manifest) = doc_transformer::discover::discover_files(&md_file, None)
        .expect("discover_files should succeed");

    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
        .expect("transform_all should succeed");

    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");

    // Verify chunks were generated
    assert!(
        chunks_result.total_chunks > 0,
        "Should generate at least 1 chunk"
    );
    assert_eq!(chunks_result.document_count, 1, "Should have 1 document");

    // Verify chunk files exist on disk
    let chunks_dir = output_dir.join("chunks");
    assert!(chunks_dir.exists(), "chunks directory should exist");

    let chunk_files: Vec<_> = fs::read_dir(&chunks_dir)
        .expect("Failed to read chunks directory")
        .filter_map(Result::ok)
        .collect();

    assert!(!chunk_files.is_empty(), "Should have chunk files on disk");
    assert_eq!(
        chunk_files.len(),
        chunks_result.total_chunks,
        "Number of chunk files should match chunks_result.total_chunks"
    );

    // Verify each chunk file has valid content
    for entry in chunk_files {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            let content = fs::read_to_string(&path).expect("Failed to read chunk file");
            // Chunk files should have frontmatter
            assert!(
                content.contains("---"),
                "Chunk file should have frontmatter"
            );
            // Chunk files should have doc_id
            assert!(content.contains("doc_id:"), "Chunk file should have doc_id");
            // Chunk files should have chunk_id
            assert!(
                content.contains("chunk_id:"),
                "Chunk file should have chunk_id"
            );
        }
    }

    // Build and verify index contains chunks
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "chunk_test",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let chunks = index["chunks"]
        .as_array()
        .expect("chunks should be an array");
    assert_eq!(
        chunks.len(),
        chunks_result.total_chunks,
        "INDEX.json should contain all chunks"
    );

    // Verify chunk metadata
    for chunk in chunks {
        assert!(chunk["chunk_id"].is_string(), "Chunk should have chunk_id");
        assert!(chunk["doc_id"].is_string(), "Chunk should have doc_id");
        assert!(
            chunk["doc_title"].is_string(),
            "Chunk should have doc_title"
        );
        assert!(
            chunk["chunk_type"].is_string(),
            "Chunk should have chunk_type"
        );
        assert!(
            chunk["token_count"].is_number(),
            "Chunk should have token_count"
        );
        assert!(
            chunk["chunk_level"].is_string(),
            "Chunk should have chunk_level"
        );
        assert!(chunk["path"].is_string(), "Chunk should have path");
    }

    // Verify hierarchical chunk distribution (if applicable)
    let stats = index["stats"].as_object().expect("stats should exist");
    let chunk_count = stats["chunk_count"]
        .as_u64()
        .expect("chunk_count should exist");
    assert!(chunk_count > 0, "Should have at least 1 chunk in stats");
}

/// Additional test: Single .mdx file should work
#[test]
fn test_index_single_mdx_file_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source_dir = temp_dir.path();
    let output_dir = temp_dir.path().join("output");

    // Create a single MDX file
    let mdx_file = source_dir.join("component.mdx");
    let content = r#"import { Component } from './library'

# Component Documentation

This is an MDX file with JSX components.

<Component prop="value" />

## Usage

```jsx
<Component prop="value" />
```
"#;
    fs::write(&mdx_file, content).expect("Failed to write test file");

    // Discover files
    let (files, manifest) = doc_transformer::discover::discover_files(&mdx_file, None)
        .expect("discover_files should succeed for single .mdx file");

    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

    // Analyze files
    let analyze_result = doc_transformer::analyze::analyze_files(&files, source_dir, None)
        .expect("analyze_files should succeed");

    assert_eq!(analyze_result.len(), 1, "Should analyze exactly 1 file");

    // Assign IDs
    let (analyses, link_map) = doc_transformer::assign::assign_ids(analyze_result.analyses);

    // Create output directory
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Transform and chunk
    let transform_result =
        doc_transformer::transform::transform_all(&analyses, &link_map, &output_dir)
            .expect("transform_all should succeed");
    assert_eq!(transform_result.success_count, 1);

    let chunks_result = doc_transformer::chunking_adapter::chunk_all(
        &analyses,
        &link_map,
        &output_dir,
        10 * 1024 * 1024,
    )
    .expect("chunk_all should succeed");
    assert!(chunks_result.total_chunks > 0);
    assert_eq!(chunks_result.document_count, 1);

    // Build index
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        &output_dir,
        "test_project",
        None,
        None,
        None,
        None,
    )
    .expect("build_and_write_index should succeed");

    // Verify INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let index: serde_json::Value =
        serde_json::from_str(&index_content).expect("Failed to parse INDEX.json");

    let docs = index["documents"]
        .as_array()
        .expect("documents should be an array");
    assert_eq!(
        docs.len(),
        1,
        "INDEX.json should contain exactly 1 document"
    );
}
