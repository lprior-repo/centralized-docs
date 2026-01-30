//! Common test helpers for doc_transformer integration tests
//!
//! This module provides reusable test utilities that all integration tests can use.
//! It encapsulates common patterns for:
//! - Creating temporary test fixtures
//! - Running the full indexing pipeline via library APIs (not CLI)
//! - Validating index structure and search results
//! - Generating sample markdown content
//!
//! ## Design Principles
//!
//! - **Library APIs only**: All helpers use direct library calls, not CLI spawning
//! - **Fast and reliable**: No subprocess overhead, deterministic results
//! - **Pure functional**: No panics, proper Result propagation
//! - **Auto-cleanup**: Uses tempfile for automatic cleanup
//!
//! ## Example Usage
//!
//! ```ignore
//! use doc_transformer::tests::common::*;
//!
//! #[test]
//! fn test_my_feature() -> anyhow::Result<()> {
//!     let fixture = fixture_dir()?;
//!     create_sample_docs(&fixture, &[
//!         ("intro.md", "# Introduction\n\nContent"),
//!     ])?;
//!
//!     let index_result = run_index(&fixture)?;
//!     assert_index_valid(&fixture)?;
//!
//!     let results = run_search(&fixture, "introduction")?;
//!     assert!(!results.is_empty());
//!
//!     Ok(())
//! }
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use anyhow::{Context, Result};
use doc_transformer::{analyze, assign, chunk, discover, index, search};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// =============================================================================
// FIXTURE MANAGEMENT
// =============================================================================

/// Creates a temporary directory for test fixtures
///
/// The directory is automatically cleaned up when the TempDir is dropped.
/// Returns a TempDir that can be used to construct paths via `.path()`.
///
/// ## Example
///
/// ```ignore
/// let fixture = fixture_dir()?;
/// let test_file = fixture.path().join("test.md");
/// fs::write(&test_file, "# Test")?;
/// // fixture is automatically cleaned up when dropped
/// ```
pub fn fixture_dir() -> Result<TempDir> {
    TempDir::new()
        .context("Failed to create temporary directory for test fixtures")
}

/// Creates realistic test documentation in a directory
///
/// Given a list of (relative_path, content) pairs, creates the files
/// in the fixture directory, creating any necessary parent directories.
///
/// ## Arguments
///
/// * `base_dir` - Base directory (e.g., from `fixture_dir()?.path()`)
/// * `files` - Slice of (relative_path, content) tuples
///
/// ## Example
///
/// ```ignore
/// create_sample_docs(fixture.path(), &[
///     ("README.md", "# Project\n\nDescription"),
///     ("docs/guide.md", "# Guide\n\n## Step 1\n..."),
/// ])?;
/// ```
pub fn create_sample_docs<P: AsRef<Path>>(
    base_dir: P,
    files: &[(&str, &str)],
) -> Result<()> {
    let base = base_dir.as_ref();

    for (rel_path, content) in files {
        let full_path = base.join(rel_path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Write the file
        fs::write(&full_path, content)
            .with_context(|| format!("Failed to write file: {}", full_path.display()))?;
    }

    Ok(())
}

/// Creates a single markdown file with generated content
///
/// Convenience wrapper for creating a single file without needing
/// to call `create_sample_docs` with a single-element array.
///
/// ## Arguments
///
/// * `base_dir` - Base directory
/// * `rel_path` - Relative path for the file
/// * `content` - File content
pub fn create_single_doc<P: AsRef<Path>>(
    base_dir: P,
    rel_path: &str,
    content: &str,
) -> Result<PathBuf> {
    let full_path = base_dir.as_ref().join(rel_path);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(&full_path, content)
        .with_context(|| format!("Failed to write file: {}", full_path.display()))?;

    Ok(full_path)
}

/// Generates large markdown content for stress testing
///
/// Creates a document with `word_count` approximate words, divided into
/// sections with headings. Useful for testing chunking behavior with large files.
///
/// ## Arguments
///
/// * `title` - Document title
/// * `word_count` - Approximate number of words to generate
///
/// ## Returns
///
/// String containing generated markdown
pub fn generate_large_markdown(title: &str, word_count: usize) -> String {
    let mut content = format!("# {}\n\nThis is a large document for testing.\n\n", title);

    let section_template = "## Section {}\n\nContent for section {} with details.\n\n";
    let paragraph_template = "This is paragraph {} with relevant information. ";

    let mut words_generated = 0_usize;
    let mut section_num = 1_usize;
    let mut para_num = 1_usize;

    while words_generated < word_count {
        // Start a new section every ~500 words
        if words_generated % 500 == 0 {
            content.push_str(&section_template.replace("{}", &section_num.to_string()));
            section_num = section_num.saturating_add(1);
        }

        // Add paragraphs
        content.push_str(&paragraph_template.replace("{}", &para_num.to_string()));
        para_num = para_num.saturating_add(1);

        // Approximate words added
        words_generated = words_generated.saturating_add(8);
    }

    content
}

// =============================================================================
// PIPELINE EXECUTION
// =============================================================================

/// Result of running the full indexing pipeline
///
/// Contains summary statistics about the indexed documents and chunks.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Public API for test consumers
pub struct IndexResult {
    /// Number of documents indexed
    pub document_count: usize,
    /// Total number of chunks created
    pub chunk_count: usize,
    /// Number of summary-level chunks
    pub summary_chunks: usize,
    /// Number of standard-level chunks
    pub standard_chunks: usize,
    /// Number of detailed-level chunks
    pub detailed_chunks: usize,
    /// Path to the INDEX.json file
    pub index_path: PathBuf,
    /// Path to the output directory
    pub output_dir: PathBuf,
}

/// Runs the full indexing pipeline via library API
///
/// Executes: DISCOVER → ANALYZE → ASSIGN → CHUNK → INDEX
///
/// This uses the library APIs directly, not the CLI. This makes tests:
/// - Faster (no subprocess overhead)
/// - More reliable (no shell escaping issues)
/// - Easier to debug (direct error messages)
///
/// ## Arguments
///
/// * `source_dir` - Directory containing markdown files to index
/// * `output_dir` - Directory where index will be written (created if needed)
/// * `project_name` - Name for the project (used in INDEX.json)
///
/// ## Returns
///
/// IndexResult with statistics and paths
///
/// ## Example
///
/// ```ignore
/// let fixture = fixture_dir()?;
/// create_sample_docs(fixture.path(), &[("test.md", "# Test")])?;
///
/// let output = fixture.path().join("output");
/// let result = run_index(fixture.path(), &output, "Test Project")?;
///
/// assert_eq!(result.document_count, 1);
/// ```
pub fn run_index<P: AsRef<Path>>(
    source_dir: P,
    output_dir: P,
    project_name: &str,
) -> Result<IndexResult> {
    let source = source_dir.as_ref();
    let output = output_dir.as_ref();

    // Create output directory
    fs::create_dir_all(output)
        .with_context(|| format!("Failed to create output directory: {}", output.display()))?;

    // Phase 1: DISCOVER
    let (discovered_files, _manifest) =
        discover::discover_files(source).context("Discovery phase failed")?;

    // Phase 2: ANALYZE
    let analyses =
        analyze::analyze_files(&discovered_files, source, None).context("Analysis phase failed")?;

    // Phase 3: ASSIGN IDs
    let (_analyses_with_ids, link_map) = assign::assign_ids(analyses.clone());

    // Phase 4: CHUNK
    let chunks_result =
        chunk::chunk_all(&analyses, &link_map, output).context("Chunking phase failed")?;

    // Phase 5: INDEX
    index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output,
        project_name,
        None, // max_related_chunks
        None, // hnsw_m
        None, // hnsw_ef_construction
    )
    .context("Indexing phase failed")?;

    let index_path = output.join("INDEX.json");

    Ok(IndexResult {
        document_count: analyses.len(),
        chunk_count: chunks_result.total_chunks,
        summary_chunks: chunks_result.summary_chunks,
        standard_chunks: chunks_result.standard_chunks,
        detailed_chunks: chunks_result.detailed_chunks,
        index_path,
        output_dir: output.to_path_buf(),
    })
}

/// Runs indexing with a simple helper that creates output_dir if needed
///
/// Convenience wrapper around `run_index` that creates the output directory
/// automatically. Use this when you want the output in a subdirectory of
/// the fixture.
///
/// ## Arguments
///
/// * `source_dir` - Directory containing markdown files
/// * `project_name` - Name for the project
///
/// ## Returns
///
/// IndexResult with statistics (output is in `source_dir/output`)
pub fn run_index_simple<P: AsRef<Path>>(source_dir: P, project_name: &str) -> Result<IndexResult> {
    let source = source_dir.as_ref();
    let output = source.join("output");
    run_index(source, &output, project_name)
}

// =============================================================================
// SEARCH
// =============================================================================

/// Search result wrapper for easier testing
///
/// Wraps the library's SearchResult with additional helper methods.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Public API for test consumers
pub struct SearchResult {
    /// Document ID
    pub id: String,
    /// Document title
    pub title: String,
    /// Document summary
    pub summary: String,
    /// Document category
    pub category: String,
    /// BM25 score (higher = more relevant)
    pub score: f32,
    /// Path to the document
    pub path: String,
}

/// Runs a search query against the indexed content
///
/// Opens the Tantivy index and searches for documents matching the query.
/// Returns results as `SearchResult` structs for easier assertions.
///
/// ## Arguments
///
/// * `index_dir` - Directory containing the index (e.g., output_dir from run_index)
/// * `query` - Search query string
/// * `limit` - Maximum number of results to return
///
/// ## Returns
///
/// Vector of search results sorted by relevance
///
/// ## Example
///
/// ```ignore
/// let results = run_search(&output_dir, "rust programming", 10)?;
/// assert!(!results.is_empty());
/// assert!(results[0].score > results[1].score); // First is most relevant
/// ```
#[allow(dead_code)] // Public API for test consumers
pub fn run_search<P: AsRef<Path>>(
    index_dir: P,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    let index = search::open_or_create_index(index_dir.as_ref())
        .context("Failed to open search index")?;

    let raw_results = search::search_index(&index, query, limit)
        .context("Search query failed")?;

    // Convert to our SearchResult wrapper
    let results = raw_results
        .into_iter()
        .map(|r| SearchResult {
            id: r.id,
            title: r.title,
            summary: r.summary,
            category: r.category,
            score: r.score,
            path: r.path,
        })
        .collect();

    Ok(results)
}

/// Runs a simple search with default limit of 10 results
///
/// Convenience wrapper around `run_search` with a reasonable default limit.
#[allow(dead_code)] // Public API for test consumers
pub fn run_search_simple<P: AsRef<Path>>(index_dir: P, query: &str) -> Result<Vec<SearchResult>> {
    run_search(index_dir, query, 10)
}

// =============================================================================
// ASSERTIONS
// =============================================================================

/// Verifies that INDEX.json exists and has valid structure
///
/// Checks:
/// - File exists
/// - Valid JSON
/// - Has required top-level keys (version, project, documents, etc.)
/// - Documents array has expected structure
/// - Chunks array has expected structure
///
/// ## Arguments
///
/// * `index_dir` - Directory containing INDEX.json
/// * `min_documents` - Minimum expected document count (0 to skip check)
///
/// ## Example
///
/// ```ignore
/// assert_index_valid(&output_dir, 1)?;
/// // Panic with descriptive message if index is invalid
/// ```
pub fn assert_index_valid<P: AsRef<Path>>(index_dir: P, min_documents: usize) -> Result<()> {
    let index_path = index_dir.as_ref().join("INDEX.json");

    // Check file exists
    assert!(
        index_path.exists(),
        "INDEX.json not found at: {}",
        index_path.display()
    );

    // Read and parse JSON
    let content = fs::read_to_string(&index_path)
        .with_context(|| format!("Failed to read INDEX.json at: {}", index_path.display()))?;

    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse INDEX.json as JSON: {}", index_path.display()))?;

    // Check required top-level keys
    let obj = value.as_object().ok_or_else(|| {
        anyhow::anyhow!("INDEX.json is not a JSON object at: {}", index_path.display())
    })?;

    for key in &["version", "project", "updated", "stats", "documents", "chunks"] {
        assert!(
            obj.contains_key(*key),
            "INDEX.json missing required key: '{}' at: {}",
            key,
            index_path.display()
        );
    }

    // Check documents array structure
    if let Some(docs) = obj.get("documents").and_then(|v| v.as_array()) {
        if min_documents > 0 {
            assert!(
                docs.len() >= min_documents,
                "Expected at least {} documents, found {} in INDEX.json at: {}",
                min_documents,
                docs.len(),
                index_path.display()
            );
        }

        // Verify each document has required fields
        for (i, doc) in docs.iter().enumerate() {
            let doc_obj = doc.as_object().ok_or_else(|| {
                anyhow::anyhow!("Document at index {} is not an object in INDEX.json", i)
            })?;

            for key in &["id", "title", "path", "category", "summary"] {
                assert!(
                    doc_obj.contains_key(*key),
                    "Document {} missing required key: '{}' in INDEX.json",
                    i,
                    key
                );
            }
        }
    }

    // Check stats structure
    if let Some(stats) = obj.get("stats").and_then(|v| v.as_object()) {
        assert!(
            stats.contains_key("doc_count"),
            "INDEX.json stats missing 'doc_count' key"
        );
        assert!(
            stats.contains_key("chunk_count"),
            "INDEX.json stats missing 'chunk_count' key"
        );
    }

    Ok(())
}

/// Asserts that a search returns results
///
/// Helper for common test pattern of checking that search found something.
///
/// ## Arguments
///
/// * `index_dir` - Directory containing the search index
/// * `query` - Search query to run
/// * `min_results` - Minimum expected results
///
/// ## Example
///
/// ```ignore
/// assert_search_has_results(&output_dir, "introduction", 1)?;
/// ```
#[allow(dead_code)] // Public API for test consumers
pub fn assert_search_has_results<P: AsRef<Path>>(
    index_dir: P,
    query: &str,
    min_results: usize,
) -> Result<()> {
    let results = run_search_simple(index_dir, query)?;

    assert!(
        results.len() >= min_results,
        "Expected at least {} results for query '{}', got {}",
        min_results,
        query,
        results.len()
    );

    Ok(())
}

/// Asserts that a document with given title exists in the index
///
/// Searches the INDEX.json for a document with the specified title.
///
/// ## Arguments
///
/// * `index_dir` - Directory containing INDEX.json
/// * `title` - Expected document title
///
/// ## Returns
///
/// The document's value if found
pub fn assert_document_exists<P: AsRef<Path>>(index_dir: P, title: &str) -> Result<Value> {
    let index_path = index_dir.as_ref().join("INDEX.json");
    let content = fs::read_to_string(&index_path)
        .with_context(|| format!("Failed to read INDEX.json at: {}", index_path.display()))?;

    let value: Value = serde_json::from_str(&content)
        .context("Failed to parse INDEX.json")?;

    if let Some(docs) = value.get("documents").and_then(|v| v.as_array()) {
        for doc in docs {
            if doc.get("title")
                .and_then(|t| t.as_str())
                .map(|t| t == title)
                .unwrap_or(false)
            {
                return Ok(doc.clone());
            }
        }
    }

    anyhow::bail!(
        "Document with title '{}' not found in INDEX.json at: {}",
        title,
        index_path.display()
    );
}

// =============================================================================
// PREDEFINED SAMPLE CONTENT
// =============================================================================

/// Sample markdown document with typical documentation structure
///
/// Returns a markdown string with sections, code blocks, tables, and lists.
pub const SAMPLE_MARKDOWN: &str = r#"# Getting Started Guide

This comprehensive guide will help you get started with the project.

## Prerequisites

Before you begin, ensure you have the following installed:

- Rust 1.70 or later
- Cargo package manager
- A code editor of your choice

## Installation

Follow these steps to install:

1. Clone the repository
2. Navigate to the project directory
3. Run `cargo build --release`

## Quick Start

Here's a simple example:

```rust
fn main() {
    println!("Hello, world!");
}
```

## Configuration

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| debug | bool | false | Enable debug mode |
| port | number | 8080 | Server port |
| host | string | "0.0.0.0" | Bind address |

## Next Steps

- [API Reference](../api.md)
- [Examples](../examples.md)
- [Troubleshooting](../troubleshooting.md)
"#;

/// Sample markdown document with frontmatter
#[allow(dead_code)] // Public API for test consumers
pub const SAMPLE_WITH_FRONTMATTER: &str = r#"---
title: Custom Title
category: tutorial
tags: rust,programming,guide
description: A comprehensive tutorial for beginners
---

# Tutorial Document

This document has YAML frontmatter with custom metadata.

## Content

The content starts after the frontmatter delimiter.
"#;

/// Sample markdown with internationalization content
#[allow(dead_code)] // Public API for test consumers
pub const SAMPLE_UNICODE: &str = r#"# Dokumentation (German)

Dies ist eine Dokumentation in deutscher Sprache mit Umlauten: äöü ÄÖÜ ß.

## Anleitung

Folgen Sie diesen Schritten:

1. Herunterladen
2. Installieren
3. Konfigurieren

# 文档 (Chinese)

这是一份中文文档，包含简体和繁体中文字符。

## 安装步骤

按照以下步骤进行安装：

1. 下载软件包
2. 解压缩
3. 运行安装程序

# Документация (Russian)

Это документация на русском языке с кириллицей.

## Инструкция

Выполните следующие шаги:

1. Загрузить
2. Установить
3. Настроить

## Math Symbols

Mathematical symbols: π ≈ 3.14159, e ≈ 2.71828, φ = (1 + √5) / 2

## Emoji

Emoji in documentation: 🎉 🚀 ✨ 📚 💻 🔧
"#;

/// Sample minimal markdown document
pub const SAMPLE_MINIMAL: &str = r#"# Test Document

A simple document with minimal content.
"#;

/// Sample markdown without H1 heading (malformed but should still process)
#[allow(dead_code)] // Public API for test consumers
pub const SAMPLE_NO_H1: &str = r#"## Section One

This document has no H1 heading.

### Subsection

Content continues here.
"#;

// =============================================================================
// TEST MODULES
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_dir_creation() {
        let fixture = fixture_dir().expect("Failed to create fixture dir");
        let path = fixture.path();
        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn test_create_sample_docs() {
        let fixture = fixture_dir().expect("Failed to create fixture dir");

        create_sample_docs(
            fixture.path(),
            &[
                ("test.md", "# Test\n\nContent"),
                ("nested/file.md", "## Nested\n\nContent"),
            ],
        )
        .expect("Failed to create sample docs");

        assert!(fixture.path().join("test.md").exists());
        assert!(fixture.path().join("nested/file.md").exists());
    }

    #[test]
    fn test_create_single_doc() {
        let fixture = fixture_dir().expect("Failed to create fixture dir");

        let path = create_single_doc(fixture.path(), "single.md", "# Single\n\nContent")
            .expect("Failed to create single doc");

        assert!(path.exists());
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "single.md");
    }

    #[test]
    fn test_generate_large_markdown() {
        let content = generate_large_markdown("Large Doc", 100);
        assert!(content.contains("# Large Doc"));
        assert!(content.contains("## Section"));
        // Check approximate length
        assert!(content.len() > 500);
    }

    #[test]
    fn test_full_pipeline_creates_index() {
        let fixture = fixture_dir().expect("Failed to create fixture dir");

        create_sample_docs(
            fixture.path(),
            &[("guide.md", SAMPLE_MARKDOWN), ("README.md", SAMPLE_MINIMAL)],
        )
        .expect("Failed to create sample docs");

        let result = run_index_simple(fixture.path(), "Test Project")
            .expect("Failed to run index");

        assert_eq!(result.document_count, 2);
        assert!(result.chunk_count > 0);

        // Verify index is valid
        assert_index_valid(&result.output_dir, 2)
            .expect("Index validation failed");
    }

    #[test]
    fn test_assert_document_exists() {
        let fixture = fixture_dir().expect("Failed to create fixture dir");

        create_single_doc(fixture.path(), "guide.md", SAMPLE_MARKDOWN)
            .expect("Failed to create doc");

        run_index_simple(fixture.path(), "Test")
            .expect("Failed to run index");

        let doc = assert_document_exists(fixture.path().join("output"), "Getting Started Guide")
            .expect("Document not found");

        assert_eq!(doc.get("title").unwrap().as_str().unwrap(), "Getting Started Guide");
    }
}
