#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Common test helpers for ctd integration tests
//!
//! This module provides reusable test utilities that all integration tests can use.
//! It encapsulates common patterns for:
//! - Creating temporary test fixtures
//! - Running the full indexing pipeline via library APIs (not CLI)
//! - Validating index structure and search results
//! - Generating sample markdown content
//! - Table-driven test support with structured test cases
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
//!     let ctx = TestContext::new()?;
//!     ctx.create_doc("intro.md", "# Introduction\n\nContent")?;
//!
//!     let result = run_full_pipeline(ctx.root(), ctx.output_dir())?;
//!     assert!(result.document_count >= 1);
//!
//!     let results = run_search(ctx.output_dir(), "introduction", 10)?;
//!     assert!(!results.is_empty());
//!
//!     Ok(())
//! }
//! ```

#![deny(clippy::panic)]
#![allow(clippy::uninlined_format_args)] // Test code with clearer format placeholders

use anyhow::{Context, Result};
use doc_transformer::{analyze, assign, chunk, discover, index, search};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// =============================================================================
// TEST CONTEXT (Encapsulated Test Environment)
// =============================================================================

/// Encapsulated test environment with temporary directory and helper methods
///
/// TestContext manages a temporary directory that is automatically cleaned up
/// when dropped. It provides convenient methods for creating test files and
/// accessing standard paths.
///
/// ## Example
///
/// ```ignore
/// let ctx = TestContext::new()?;
/// ctx.create_doc("test.md", "# Test")?;
/// let output = ctx.output_dir();
/// // Files are automatically cleaned up when ctx is dropped
/// ```
#[derive(Debug)]
pub struct TestContext {
    /// Temporary directory (automatically cleaned up on drop)
    temp_dir: TempDir,
}

impl TestContext {
    /// Create a new test context with a temporary directory
    ///
    /// Returns an error if the temporary directory cannot be created.
    pub fn new() -> Result<Self> {
        TempDir::new()
            .context("Failed to create temporary directory for test context")
            .map(|temp_dir| Self { temp_dir })
    }

    /// Get the root path of the temporary directory
    pub fn root(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get the standard output directory path (root/output)
    pub fn output_dir(&self) -> PathBuf {
        self.root().join("output")
    }

    /// Get the standard chunks directory path (root/output/chunks)
    pub fn chunks_dir(&self) -> PathBuf {
        self.output_dir().join("chunks")
    }

    /// Get the INDEX.json path (root/output/INDEX.json)
    pub fn index_path(&self) -> PathBuf {
        self.output_dir().join("INDEX.json")
    }

    /// Create a markdown file at the given relative path
    ///
    /// Parent directories are created automatically.
    ///
    /// ## Arguments
    ///
    /// * `rel_path` - Relative path from root (e.g., "docs/guide.md")
    /// * `content` - File content as a string
    pub fn create_doc(&self, rel_path: &str, content: &str) -> Result<PathBuf> {
        let path = self.root().join(rel_path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(&path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(path)
    }

    /// Create multiple markdown files from a slice of (path, content) pairs
    ///
    /// ## Example
    ///
    /// ```ignore
    /// ctx.create_docs(&[
    ///     ("README.md", "# Readme"),
    ///     ("docs/guide.md", "# Guide"),
    /// ])?;
    /// ```
    pub fn create_docs(&self, files: &[(&str, &str)]) -> Result<()> {
        for (rel_path, content) in files {
            self.create_doc(rel_path, content)?;
        }
        Ok(())
    }

    /// Read a file's content as a string
    ///
    /// ## Arguments
    ///
    /// * `rel_path` - Relative path from root
    pub fn read_file(&self, rel_path: &str) -> Result<String> {
        let path = self.root().join(rel_path);
        fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display()))
    }

    /// Check if a file exists at the given relative path
    pub fn file_exists(&self, rel_path: &str) -> bool {
        self.root().join(rel_path).exists()
    }

    /// Read and parse the INDEX.json file
    pub fn read_index(&self) -> Result<Value> {
        let index_path = self.index_path();
        let content = fs::read_to_string(&index_path)
            .with_context(|| format!("Failed to read INDEX.json at: {}", index_path.display()))?;
        let value: Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse INDEX.json at: {}", index_path.display()))?;
        Ok(value)
    }

    /// List all markdown files in the test directory recursively
    pub fn list_markdown_files(&self) -> Result<Vec<PathBuf>> {
        use walkdir::WalkDir;

        let files = WalkDir::new(self.root())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                // Exclude common ignore directories
                !path.components().any(|c| {
                    matches!(
                        c.as_os_str().to_str(),
                        Some("node_modules" | ".git" | "target")
                    )
                })
            })
            .filter(|e| {
                let path = e.path();
                path.is_file()
                    && path.extension().is_some_and(|ext| {
                        matches!(ext.to_str(), Some("md" | "mdx" | "rst" | "txt"))
                    })
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        Ok(files)
    }
}

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
    TempDir::new().context("Failed to create temporary directory for test fixtures")
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
pub fn create_sample_docs<P: AsRef<Path>>(base_dir: P, files: &[(&str, &str)]) -> Result<()> {
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

// =============================================================================
// TABLE-DRIVEN TEST SUPPORT
// =============================================================================

/// Test case specification for table-driven pipeline tests
///
/// Use this struct to define test cases that can be iterated over
/// in table-driven tests.
///
/// ## Example
///
/// ```ignore
/// let cases = vec![
///     PipelineTestCase {
///         name: "empty_directory",
///         description: "Pipeline should handle empty input",
///         files: vec![],
///         expected_min_documents: 0,
///         should_succeed: true,
///     },
///     // ... more cases
/// ];
///
/// for case in cases {
///     // Run test with case
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PipelineTestCase {
    /// Unique name for this test case
    pub name: &'static str,
    /// Human-readable description of what is being tested
    pub description: &'static str,
    /// Files to create: (relative_path, content) pairs
    pub files: Vec<(&'static str, &'static str)>,
    /// Minimum expected document count
    pub expected_min_documents: usize,
    /// Minimum expected chunk count
    pub expected_min_chunks: usize,
    /// Whether the pipeline should succeed
    pub should_succeed: bool,
}

impl PipelineTestCase {
    /// Create a new simple test case
    ///
    /// ## Arguments
    ///
    /// * `name` - Unique test case identifier
    /// * `files` - Files to create for this test
    pub fn simple(name: &'static str, files: Vec<(&'static str, &'static str)>) -> Self {
        let doc_count = files.len();
        Self {
            name,
            description: "",
            files,
            expected_min_documents: doc_count,
            expected_min_chunks: doc_count,
            should_succeed: true,
        }
    }

    /// Create a test case with custom expectations
    pub fn with_expectations(
        name: &'static str,
        files: Vec<(&'static str, &'static str)>,
        min_docs: usize,
        min_chunks: usize,
    ) -> Self {
        Self {
            name,
            description: "",
            files,
            expected_min_documents: min_docs,
            expected_min_chunks: min_chunks,
            should_succeed: true,
        }
    }
}

/// Standard test cases for pipeline testing
///
/// Returns a collection of commonly-used test cases covering:
/// - Empty input
/// - Single file
/// - Multiple files
/// - Unicode content
/// - Large files
/// - Malformed markdown
/// - Special characters
pub fn standard_pipeline_cases() -> Vec<PipelineTestCase> {
    vec![
        PipelineTestCase {
            name: "empty_directory",
            description: "Pipeline should handle empty input gracefully",
            files: vec![],
            expected_min_documents: 0,
            expected_min_chunks: 0,
            should_succeed: true,
        },
        PipelineTestCase {
            name: "single_minimal_file",
            description: "Single markdown file with minimal content",
            files: vec![("README.md", "# Test\n\nContent here.")],
            expected_min_documents: 1,
            expected_min_chunks: 1,
            should_succeed: true,
        },
        PipelineTestCase {
            name: "unicode_content",
            description: "Document with international characters and emoji",
            files: vec![(
                "international.md",
                r#"# Documentation 文档 📚

## German 🇩🇪

Dies ist eine Dokumentation mit Umlauten: äöü ÄÖÜ ß.

## Japanese 🇯🇵

これは日本語のドキュメントです。

## Math Symbols

π ≈ 3.14159, e ≈ 2.71828
"#,
            )],
            expected_min_documents: 1,
            expected_min_chunks: 1,
            should_succeed: true,
        },
        PipelineTestCase {
            name: "malformed_no_h1",
            description: "Document without H1 heading",
            files: vec![("no-h1.md", "## Section One\n\nContent without H1.")],
            expected_min_documents: 1,
            expected_min_chunks: 1,
            should_succeed: true,
        },
    ]
}

// =============================================================================
// CONTENT GENERATION
// =============================================================================

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
///
/// ## Example
///
/// ```ignore
/// let large_doc = generate_large_markdown("Stress Test", 5000);
/// ctx.create_doc("large.md", &large_doc)?;
/// ```
pub fn generate_large_markdown(title: &str, word_count: usize) -> String {
    let mut content = format!("# {title}\n\nThis is a large document for testing.\n\n");

    let section_template = "## Section {}\n\nContent for section {} with details.\n\n";
    let paragraph_template = "This is paragraph {} with relevant information. ";

    let mut words_generated = 0_usize;
    let mut section_num = 1_usize;
    let mut para_num = 1_usize;

    while words_generated < word_count {
        // Start a new section every ~500 words
        if words_generated.is_multiple_of(500) {
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

/// Generate markdown with specific features for testing
///
/// ## Arguments
///
/// * `features` - Features to include (code, tables, lists, frontmatter, etc.)
pub fn generate_markdown_with(features: &str) -> String {
    let mut content = String::from("# Test Document\n\n");

    if features.contains("frontmatter") {
        content.insert_str(
            0,
            r#"---
title: Test Document
category: test
tags: example,test
---

"#,
        );
    }

    if features.contains("code") {
        content.push_str("## Code Examples\n\n");
        content.push_str("```rust\nfn main() {\n    println!(\"Hello!\");\n}\n```\n\n");
    }

    if features.contains("tables") {
        content.push_str("## Table\n\n");
        content.push_str("| Column 1 | Column 2 |\n");
        content.push_str("|----------|----------|\n");
        content.push_str("| Value 1  | Value 2  |\n\n");
    }

    if features.contains("lists") {
        content.push_str("## Lists\n\n");
        content.push_str("- Item 1\n- Item 2\n  - Nested item\n\n");
    }

    if features.contains("links") {
        content.push_str("## Links\n\n");
        content.push_str("[External](https://example.com)\n\n");
        content.push_str("[Internal](./other.md)\n\n");
    }

    content.push_str("Content continues here.\n");
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
/// Executes: DISCOVER -> ANALYZE -> ASSIGN -> CHUNK -> INDEX
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
        discover::discover_files(source, None).context("Discovery phase failed")?;

    // Phase 2: ANALYZE
    let analyze_result =
        analyze::analyze_files(&discovered_files, source, None).context("Analysis phase failed")?;
    let analyses = analyze_result.analyses;

    // Phase 3: ASSIGN IDs
    let (_analyses_with_ids, link_map) = assign::assign_ids(analyses.clone());

    // Phase 4: CHUNK
    let chunks_result = chunk::chunk_all(&analyses, &link_map, output, 10 * 1024 * 1024)
        .context("Chunking phase failed")?;

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
        None, // max_chunk_keywords
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

/// Runs the full pipeline with TestContext
///
/// Convenience method that uses TestContext paths directly.
///
/// ## Example
///
/// ```ignore
/// let ctx = TestContext::new()?;
/// ctx.create_doc("test.md", "# Test")?;
/// let result = run_full_pipeline(ctx.root(), ctx.output_dir())?;
/// ```
pub fn run_full_pipeline(source_dir: &Path, output_dir: &Path) -> Result<IndexResult> {
    run_index(source_dir, output_dir, "Test Project")
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

impl SearchResult {
    /// Check if this result matches the given title
    pub fn has_title(&self, title: &str) -> bool {
        self.title.contains(title)
    }

    /// Check if the result contains the term in summary or title
    pub fn contains(&self, term: &str) -> bool {
        let term_lower = term.to_lowercase();
        self.title.to_lowercase().contains(&term_lower)
            || self.summary.to_lowercase().contains(&term_lower)
    }
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
    let index =
        search::open_or_create_index(index_dir.as_ref()).context("Failed to open search index")?;

    let raw_results = search::search_index(&index, query, limit).context("Search query failed")?;

    // Convert to our SearchResult wrapper
    let results = raw_results
        .into_iter()
        .map(|r| SearchResult {
            id: r.id,
            title: r.title,
            summary: r.summary,
            category: r.category,
            score: r.score.value(),
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

    let value: Value = serde_json::from_str(&content).with_context(|| {
        format!(
            "Failed to parse INDEX.json as JSON: {}",
            index_path.display()
        )
    })?;

    // Check required top-level keys
    let obj = value.as_object().ok_or_else(|| {
        anyhow::anyhow!(
            "INDEX.json is not a JSON object at: {}",
            index_path.display()
        )
    })?;

    for key in &[
        "version",
        "project",
        "updated",
        "stats",
        "documents",
        "chunks",
    ] {
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

    let value: Value = serde_json::from_str(&content).context("Failed to parse INDEX.json")?;

    if let Some(docs) = value.get("documents").and_then(|v| v.as_array()) {
        for doc in docs {
            if doc
                .get("title")
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

/// Asserts that chunks directory exists and contains expected files
///
/// ## Arguments
///
/// * `output_dir` - Output directory containing chunks
/// * `min_chunks` - Minimum expected chunk files
pub fn assert_chunks_valid<P: AsRef<Path>>(output_dir: P, min_chunks: usize) -> Result<()> {
    let chunks_dir = output_dir.as_ref().join("chunks");

    assert!(
        chunks_dir.exists(),
        "Chunks directory not found at: {}",
        chunks_dir.display()
    );

    let entries = fs::read_dir(&chunks_dir)
        .with_context(|| format!("Failed to read chunks directory: {}", chunks_dir.display()))?;

    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .count();

    assert!(
        count >= min_chunks,
        "Expected at least {} chunk files, found {}",
        min_chunks,
        count
    );

    Ok(())
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

/// Sample markdown with code blocks and multiple languages
pub const SAMPLE_WITH_CODE: &str = r#"# Code Examples

This document contains code blocks in multiple languages.

## Rust Example

```rust
fn main() {
    println!("Hello, Rust!");
    let numbers = vec
![1, 2, 3];
    for n in numbers {
        println!("{}", n);
    }
}
```

## Python Example

```python
def main():
    print("Hello, Python!")
    numbers = [1, 2, 3]
    for n in numbers:
        print(n)

if __name__ == "__main__":
    main()
```

## JavaScript Example

```javascript
function main() {
    console.log("Hello, JavaScript!")
    const numbers = [1, 2, 3]
    numbers.forEach(n => console.log(n))
}

main()
```
"#;

/// Sample markdown with tables
pub const SAMPLE_WITH_TABLES: &str = r#"# Feature Comparison

This document contains various table examples.

## Simple Table

| Name | Type | Description |
|------|------|-------------|
| Foo  | int  | A foo value |
| Bar  | str  | A bar value |

## Aligned Table

| Left | Center | Right |
|:-----|:------:|------:|
| L1   | C1     | R1    |
| L2   | C2     | R2    |

## Complex Table

| Feature | Status | Notes |
|---------|--------|-------|
| Basic   | ✓      | Done  |
| Advanced | ○     | WIP   |
| Future  | ✗      | TBD   |
"#;

/// Sample markdown with nested lists
pub const SAMPLE_WITH_LISTS: &str = r#"# Lists Examples

## Unordered List

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
    - Deep nested 2.2.1
- Item 3

## Ordered List

1. First step
2. Second step
   2.1. Sub-step a
   2.2. Sub-step b
3. Third step

## Task List

- [x] Completed task
- [ ] Pending task
- [x] Another completed
- [ ] Another pending

## Definition List (via HTML)

<dl>
<dt>Term 1</dt>
<dd>Description 1</dd>
<dt>Term 2</dt>
<dd>Description 2</dd>
</dl>
"#;

/// Sample markdown with links (internal and external)
pub const SAMPLE_WITH_LINKS: &str = r#"# Link Examples

## External Links

- [Example](https://example.com)
- [Rust Language](https://www.rust-lang.org/)
- [Documentation](https://docs.rs/)

## Internal Links

- [Getting Started](./getting-started.md)
- [API Reference](./api.md)
- [Examples](../examples/)

## Reference-style Links

This is a [reference-style link][ref].

[ref]: https://example.com/reference

## Auto Links

<https://example.com>
<user@example.com>

## Inline Links

See [the guide](guide.md "Optional title") for details.
"#;

/// Sample markdown with blockquotes
pub const SAMPLE_WITH_QUOTES: &str = r#"# Quote Examples

## Simple Blockquote

> This is a simple blockquote.
> It spans multiple lines.

## Nested Blockquote

> Level 1 quote
>
> > Level 2 nested quote
> >
> > > Level 3 deeply nested

## Blockquote with formatting

> **Bold text** in a quote
>
> *Italic text* too
>
> `code` also works

## Attributed quote (convention)

> The only way to do great work is to love what you do.
>
> — Steve Jobs
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
    fn test_test_context_creation() {
        let ctx = TestContext::new().expect("Failed to create TestContext");
        assert!(ctx.root().exists());
        assert!(ctx.root().is_dir());
    }

    #[test]
    fn test_test_context_create_doc() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        ctx.create_doc("test.md", "# Test")
            .expect("Failed to create doc");

        assert!(ctx.file_exists("test.md"));
        assert!(!ctx.file_exists("nonexistent.md"));
    }

    #[test]
    fn test_test_context_create_docs() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        ctx.create_docs(&[("README.md", "# Readme"), ("docs/guide.md", "# Guide")])
            .expect("Failed to create docs");

        assert!(ctx.file_exists("README.md"));
        assert!(ctx.file_exists("docs/guide.md"));
    }

    #[test]
    fn test_test_context_read_file() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        ctx.create_doc("test.md", "# Test Content")
            .expect("Failed to create doc");

        let content = ctx.read_file("test.md").expect("Failed to read file");
        assert_eq!(content, "# Test Content");
    }

    #[test]
    fn test_full_pipeline_creates_index() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        create_sample_docs(
            ctx.root(),
            &[("guide.md", SAMPLE_MARKDOWN), ("README.md", SAMPLE_MINIMAL)],
        )
        .expect("Failed to create sample docs");

        let result = run_index_simple(ctx.root(), "Test Project").expect("Failed to run index");

        assert_eq!(result.document_count, 2);
        assert!(result.chunk_count > 0);

        // Verify index is valid
        assert_index_valid(&result.output_dir, 2).expect("Index validation failed");
    }

    #[test]
    fn test_assert_document_exists() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        create_single_doc(ctx.root(), "guide.md", SAMPLE_MARKDOWN).expect("Failed to create doc");

        run_index_simple(ctx.root(), "Test").expect("Failed to run index");

        let doc = assert_document_exists(ctx.output_dir(), "Getting Started Guide")
            .expect("Document not found");

        assert_eq!(
            doc.get("title").unwrap().as_str().unwrap(),
            "Getting Started Guide"
        );
    }

    #[test]
    fn test_standard_pipeline_cases_exist() {
        let cases = standard_pipeline_cases();
        assert!(!cases.is_empty(), "Should have standard test cases");
        assert!(cases.iter().any(|c| c.name == "empty_directory"));
        assert!(cases.iter().any(|c| c.name == "unicode_content"));
    }

    #[test]
    fn test_generate_markdown_with_features() {
        let content = generate_markdown_with("code tables lists links");

        assert!(content.contains("## Code Examples"));
        assert!(content.contains("```rust"));
        assert!(content.contains("| Column 1"));
        assert!(content.contains("- Item 1"));
        assert!(content.contains("[External]"));
    }

    #[test]
    fn test_pipeline_test_case_builder() {
        let case = PipelineTestCase::simple("test_case", vec![("test.md", "# Test")]);

        assert_eq!(case.name, "test_case");
        assert_eq!(case.expected_min_documents, 1);
        assert_eq!(case.expected_min_chunks, 1);
        assert!(case.should_succeed);
    }

    #[test]
    fn test_assert_chunks_valid() {
        let ctx = TestContext::new().expect("Failed to create TestContext");

        ctx.create_doc("test.md", SAMPLE_MARKDOWN)
            .expect("Failed to create doc");

        run_index_simple(ctx.root(), "Test").expect("Failed to run index");

        assert_chunks_valid(ctx.output_dir(), 1).expect("Chunks validation failed");
    }
}
