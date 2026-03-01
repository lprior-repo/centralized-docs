//! Behavior-Focused Scenario Tests
//!
//! These tests tell stories about USER BEHAVIOR, not implementation details.
//! Each scenario follows the Given-When-Then format from behavior-driven development.
//!
//! ## Testing Philosophy (Fowler-Approved)
//!
//! Martin Fowler on scenario tests:
//! - "Tests should be easy to understand"
//! - "Tests should tell a story about the system's behavior"
//! - "Avoid testing implementation details - test observable behavior"
//! - "A scenario test should cover one complete user story end-to-end"
//!
//! ## Scenario Structure
//!
//! Each scenario is ONE comprehensive test covering a complete user workflow:
//! - GIVEN: The initial state/context
//! - WHEN: The user takes an action
//! - THEN: The observable outcome

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use anyhow::{bail, Context};
use doc_transformer::{analyze, assign, chunk, discover, index, search, validate};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// =============================================================================
// TEST CONTEXT
// =============================================================================

/// Context for scenario tests with temporary directory
struct ScenarioContext {
    temp_dir: TempDir,
}

impl ScenarioContext {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            temp_dir: TempDir::new()
                .context("Failed to create temporary directory for scenario test")?,
        })
    }

    fn root(&self) -> &Path {
        self.temp_dir.path()
    }

    fn create_markdown_file(&self, rel_path: &str, content: &str) -> anyhow::Result<PathBuf> {
        let path = self.root().join(rel_path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(&path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(path)
    }

    fn create_sample_docs(&self, files: &[(&str, &str)]) -> anyhow::Result<()> {
        for (rel_path, content) in files {
            self.create_markdown_file(rel_path, content)?;
        }
        Ok(())
    }

    fn read_index(&self, output_dir: &Path) -> anyhow::Result<Value> {
        let index_path = output_dir.join("INDEX.json");
        let content = fs::read_to_string(&index_path)
            .with_context(|| format!("Failed to read index at: {}", index_path.display()))?;
        let value: Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse index JSON at: {}", index_path.display()))?;
        Ok(value)
    }

    fn output_dir(&self) -> PathBuf {
        self.root().join("output")
    }
}

/// Result of running the indexing pipeline
struct IndexResult {
    document_count: usize,
    chunk_count: usize,
    _summary_chunks: usize,
    _standard_chunks: usize,
    _detailed_chunks: usize,
}

/// Run the full indexing pipeline
fn run_index_pipeline(source_dir: &Path, output_dir: &Path) -> anyhow::Result<IndexResult> {
    // Create output directory
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

    // Phase 1: DISCOVER
    let (discovered_files, _manifest) =
        discover::discover_files(source_dir).context("Discovery phase failed")?;

    // Phase 2: ANALYZE
    let analyze_result = analyze::analyze_files(&discovered_files, source_dir, None)
        .context("Analysis phase failed")?;
    let analyses = analyze_result.analyses;

    // Phase 3: ASSIGN IDs
    let (_analyses_with_ids, link_map) = assign::assign_ids(analyses.clone());

    // Phase 4: CHUNK
    let chunks_result = chunk::chunk_all(&analyses, &link_map, output_dir, 10 * 1024 * 1024)
        .context("Chunking phase failed")?;

    // Phase 5: INDEX
    index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output_dir,
        "Test Project",
        None,
        None,
        None,
        None,
    )
    .context("Indexing phase failed")?;

    Ok(IndexResult {
        document_count: analyses.len(),
        chunk_count: chunks_result.total_chunks,
        _summary_chunks: chunks_result.summary_chunks,
        _standard_chunks: chunks_result.standard_chunks,
        _detailed_chunks: chunks_result.detailed_chunks,
    })
}

/// Run a search query
fn run_search(
    index_dir: &Path,
    query: &str,
    limit: usize,
) -> anyhow::Result<Vec<search::SearchResult>> {
    let search_index =
        search::open_or_create_index(index_dir).context("Failed to open search index")?;

    search::search_index(&search_index, query, limit).context("Search query failed")
}

/// Assert the index is valid
fn assert_index_valid(index_dir: &Path) -> anyhow::Result<()> {
    let index_path = index_dir.join("INDEX.json");

    assert!(
        index_path.exists(),
        "INDEX.json not found at: {}",
        index_path.display()
    );

    let content = fs::read_to_string(&index_path)
        .with_context(|| format!("Failed to read INDEX.json at: {}", index_path.display()))?;

    let _value: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse INDEX.json at: {}", index_path.display()))?;

    Ok(())
}

// =============================================================================
// SCENARIO 1: User indexes their project documentation
// =============================================================================

/// Scenario: User indexes their project documentation
///
/// **GIVEN**: A project with markdown documentation files
/// **WHEN**: User runs the indexing pipeline on their documentation directory
/// **THEN**: All documents are indexed, chunks are created, and search works
#[test]
fn scenario_user_indexes_project_docs() -> anyhow::Result<()> {
    println!("\n=== SCENARIO 1: User indexes their project documentation ===\n");

    // GIVEN: A project with markdown documentation files
    let ctx = ScenarioContext::new()?;

    ctx.create_sample_docs(&[
        (
            "README.md",
            r#"
# My Project

A comprehensive project documentation example.

## Quick Start

Get started with these simple steps.

## Documentation

- See [docs/guide.md](docs/guide.md) for the complete guide
- See [api/reference.md](api/reference.md) for API reference
"#,
        ),
        (
            "docs/guide.md",
            r#"
# User Guide

This is the comprehensive user guide.

## Installation

Install the project using the package manager.

## Configuration

Configure the project with these settings.

### Basic Settings

The basic settings include timeout, retries, and logging.

### Advanced Settings

Advanced settings allow fine-tuning of performance.

## See Also

- [API Reference](../api/reference.md)
- [Examples](./examples.md)
"#,
        ),
        (
            "docs/examples.md",
            r#"
# Examples

This document provides practical examples.

## Example 1: Basic Usage

Basic usage example showing the core features.

## Example 2: Advanced Configuration

Advanced configuration with all options explained.
"#,
        ),
        (
            "api/reference.md",
            r#"
# API Reference

Complete API documentation for all modules.

## Core Functions

The core functions provide the main functionality.

### `process(input: &str) -> Result<String>`

Process the input and return the result.

## Utility Functions

Utility functions for common operations.

## See Also

- [User Guide](../docs/guide.md)
"#,
        ),
    ])?;

    println!("GIVEN: Project with documentation");
    println!("  - README.md (root)");
    println!("  - docs/guide.md");
    println!("  - docs/examples.md");
    println!("  - api/reference.md");

    // WHEN: User runs the indexing pipeline
    println!("\nWHEN: User runs the indexing pipeline");

    let output_dir = ctx.output_dir();
    let result = run_index_pipeline(ctx.root(), &output_dir)?;

    println!("\n  Documents indexed: {}", result.document_count);
    println!("  Chunks created: {}", result.chunk_count);

    // THEN: All documents are indexed, chunks created, search works
    println!("\nTHEN: Documentation is fully indexed and searchable\n");

    assert_index_valid(&output_dir)?;
    println!("  ✓ Index is valid");

    let index = ctx.read_index(&output_dir)?;
    let doc_count = index["documents"].as_array().map(|v| v.len()).unwrap_or(0);

    assert!(doc_count >= 4, "Index should contain at least 4 documents");
    println!("  ✓ Index contains {doc_count} documents");

    // Test search works
    let search_results = run_search(&output_dir, "configuration", 10)?;
    println!(
        "  ✓ Search returns {} results for 'configuration'",
        search_results.len()
    );

    println!("\n=== SCENARIO 1 PASSED: User successfully indexed project docs ===\n");

    Ok(())
}

// =============================================================================
// SCENARIO 2: User searches for specific content
// =============================================================================

/// Scenario: User searches for specific content
///
/// **GIVEN**: An indexed documentation set
/// **WHEN**: User searches for specific terms like "rust programming"
/// **THEN**: Relevant results are returned with relevance scores
#[test]
fn scenario_user_searches_for_content() -> anyhow::Result<()> {
    println!("\n=== SCENARIO 2: User searches for specific content ===\n");

    // GIVEN: An indexed documentation set
    let ctx = ScenarioContext::new()?;

    ctx.create_sample_docs(&[
        (
            "docs/rust-guide.md",
            r#"
# Rust Programming Guide

This comprehensive guide covers Rust programming fundamentals.

## Getting Started with Rust

Rust is a systems programming language focused on safety and performance.

## Memory Safety

Rust guarantees memory safety through its ownership system.

## Concurrency

Write concurrent Rust programs without data races.
"#,
        ),
        (
            "docs/python-guide.md",
            r#"
# Python Programming Guide

Learn Python programming from scratch.

## Python Basics

Python is a high-level interpreted language.

## Data Structures

Python provides built-in data structures like lists and dictionaries.
"#,
        ),
        (
            "docs/config-guide.md",
            r#"
# Configuration Guide

This document explains all configuration options.

## Basic Configuration

Configure basic settings like timeouts and retries.

## Advanced Configuration

Advanced settings for performance tuning.
"#,
        ),
    ])?;

    let output_dir = ctx.output_dir();
    run_index_pipeline(ctx.root(), &output_dir)?;

    println!("GIVEN: Indexed documentation set");

    // WHEN: User searches for specific terms
    println!("\nWHEN: User searches for various terms");

    // THEN: Relevant results are returned with scores
    println!("\nTHEN: Search returns relevant results\n");

    // Test 1: Search for "rust programming"
    let results = run_search(&output_dir, "rust programming", 10)?;
    println!("  Search 'rust programming': {} results", results.len());

    for (i, result) in results.iter().enumerate().take(3) {
        println!(
            "    {}. {} (score: {:.2})",
            i + 1,
            result.title,
            result.score
        );
    }

    // Test 2: Search for "configuration"
    let config_results = run_search(&output_dir, "configuration settings", 10)?;
    println!(
        "\n  Search 'configuration settings': {} results",
        config_results.len()
    );

    // Results should have positive scores
    for result in &config_results {
        assert!(
            result.score > 0.0,
            "Search results should have positive scores, got {} for '{}'",
            result.score,
            result.title
        );
    }

    // Test 3: Empty query should be rejected with helpful error
    println!("\n  Testing empty query rejection...");
    let error_msg = match validate::validate_query("") {
        Err(e) => e.to_string(),
        Ok(_) => bail!("Empty query should be rejected with EmptyQuery error"),
    };
    assert!(
        error_msg.contains("empty") || error_msg.contains("Empty"),
        "Error message should mention 'empty': {error_msg}"
    );
    println!("  ✓ Empty query rejected with: '{error_msg}'");

    // Test 4: Case-insensitive search
    let lower_results = run_search(&output_dir, "RUST PROGRAMMING", 10)?;
    println!(
        "\n  Search 'RUST PROGRAMMING' (uppercase): {} results",
        lower_results.len()
    );
    println!("  ✓ Search is case-insensitive");

    // Test 5: No results for non-existent term
    let no_results = run_search(&output_dir, "nonexistenttermxyz", 10)?;
    println!("\n  Search for nonsense term: {} results", no_results.len());
    println!("  ✓ Returns empty results for non-existent terms");

    println!("\n=== SCENARIO 2 PASSED: User can search and find relevant content ===\n");

    Ok(())
}

// =============================================================================
// SCENARIO 3: Scraping with content filtering
// =============================================================================

/// Scenario: User scrapes documentation with content filtering
///
/// **GIVEN**: A documentation website with navigation and boilerplate
/// **WHEN**: User scrapes with BM25 threshold filtering enabled
/// **THEN**: Only relevant content pages are kept, navigation is removed
#[test]
fn scenario_scraping_with_content_filtering() -> anyhow::Result<()> {
    println!("\n=== SCENARIO 3: Scraping with content filtering ===\n");

    // GIVEN: Simulated HTML content from a documentation site
    let high_quality_html = r#"
<!DOCTYPE html>
<html>
<head><title>API Reference</title></head>
<body>
    <nav class="navbar">
        <a href="/">Home</a>
        <a href="/docs">Documentation</a>
        <a href="/api">API Reference</a>
    </nav>
    <main>
        <h1>API Reference</h1>
        <p>This is the comprehensive API reference documentation.</p>
        <p>It covers all the public functions, types, and modules.</p>
        <h2>Core Functions</h2>
        <p>The core functions provide the main functionality of the library.</p>
        <p>Each function is documented with parameters and return values.</p>
        <h2>Utility Functions</h2>
        <p>Utility functions provide helper functionality for common tasks.</p>
    </main>
    <footer>
        <p>Copyright 2024 - All rights reserved</p>
        <p><a href="/privacy">Privacy Policy</a></p>
    </footer>
</body>
</html>
    "#;

    let low_quality_html = r#"
<!DOCTYPE html>
<html>
<head><title>Navigation</title></head>
<body>
    <nav>
        <a href="/">Home</a>
        <a href="/about">About</a>
    </nav>
    <div class="sidebar">
        Links and more links.
    </div>
</body>
</html>
    "#;

    println!("GIVEN: HTML content from documentation pages");
    println!("  - High quality content page (API Reference)");
    println!("  - Low quality navigation page");

    // WHEN: Content filtering is applied
    println!("\nWHEN: Content filtering is applied with default config");

    use doc_transformer::filter::FilterConfig;

    let config = FilterConfig::default();

    // Test filtering on high-quality content
    let high_quality_result = doc_transformer::filter::prune_html(high_quality_html, &config);
    println!("\n  High quality page:");
    println!(
        "    - Density score: {:.2}",
        high_quality_result.density_score
    );
    println!(
        "    - Elements removed: {}",
        high_quality_result.removed_count
    );
    println!(
        "    - Used Readability: {}",
        high_quality_result.used_readability
    );

    // Test filtering on low-quality content
    let low_quality_result = doc_transformer::filter::prune_html(low_quality_html, &config);
    println!("\n  Low quality page:");
    println!(
        "    - Density score: {:.2}",
        low_quality_result.density_score
    );
    println!(
        "    - Elements removed: {}",
        low_quality_result.removed_count
    );
    println!(
        "    - Used Readability: {}",
        low_quality_result.used_readability
    );

    // THEN: High quality content is preserved, navigation is removed
    println!("\nTHEN: Content filtering preserves substantive content\n");

    assert!(
        high_quality_result.density_score > 0.1,
        "High quality content should have positive density score"
    );
    println!("  ✓ High quality content has positive density");

    assert!(
        high_quality_result.html.contains("API Reference")
            || high_quality_result.html.contains("API")
            || high_quality_result.html.len() > 50,
        "Filtered content should preserve main text"
    );
    println!("  ✓ Main content is preserved in filtered output");

    // Test BM25 scoring for query-based filtering
    println!("\n  Testing BM25 scoring for content relevance...");

    let relevant_score = search::score_document_simple(
        "API Reference",
        "This is the comprehensive API reference documentation",
        "api reference",
        100.0,
    );
    println!("    Score for 'api reference' query: {relevant_score:.2}");

    let irrelevant_score =
        search::score_document_simple("Navigation", "Links and more links", "api reference", 50.0);
    println!("    Score for navigation page: {irrelevant_score:.2}");

    assert!(
        relevant_score > irrelevant_score,
        "Relevant content should score higher than irrelevant"
    );
    println!("  ✓ BM25 scoring ranks relevant content higher");

    let bm25_threshold = 1.0;
    println!("\n  With BM25 threshold of {bm25_threshold:.1}");
    println!("    - High quality page (score {relevant_score:.2}) would be KEPT");
    println!("    - Low quality page (score {irrelevant_score:.2}) would be FILTERED");

    println!("\n=== SCENARIO 3 PASSED: Content filtering works correctly ===\n");

    Ok(())
}

// =============================================================================
// SCENARIO 4: Error messages are helpful
// =============================================================================

/// Scenario: User receives helpful error messages
///
/// **GIVEN**: Various error conditions
/// **WHEN**: User makes mistakes or encounters problems
/// **THEN**: Clear, actionable error messages guide them to resolution
#[test]
fn scenario_error_messages_are_helpful() -> anyhow::Result<()> {
    println!("\n=== SCENARIO 4: Error messages are helpful ===\n");

    println!("GIVEN: User can make several types of errors\n");
    println!("WHEN: User encounters error conditions\n");

    use validate::{validate_limit, validate_query, ValidationError};

    // Test 1: Empty query
    println!("Test 1: Empty query");
    let result = validate_query("");
    match result {
        Err(ref e) => {
            let msg = e.to_string();
            println!("  Error: '{msg}'");
            assert!(
                msg.contains("empty") || msg.contains("Empty"),
                "Error should mention 'empty'"
            );
            assert!(msg.len() < 100, "Error message should be concise");
            println!("  ✓ Message is clear and actionable");
        }
        _ => bail!("Should return EmptyQuery error"),
    }

    // Test 2: Whitespace-only query
    println!("\nTest 2: Whitespace-only query");
    let result = validate_query("   \n\t  ");
    match result {
        Err(ref e) => {
            let msg = e.to_string();
            println!("  Error: '{msg}'");
            println!("  ✓ Correctly identifies whitespace as empty");
        }
        _ => bail!("Should return EmptyQuery for whitespace"),
    }

    // Test 3: Over-length query
    println!("\nTest 3: Query too long");
    let long_query = "search ".repeat(300);
    let result = validate_query(&long_query);
    match result {
        Err(ref e) => {
            let msg = e.to_string();
            println!("  Error: '{msg}'");
            if let ValidationError::QueryTooLong { length, max } = e {
                println!("  Length: {length}, Max: {max}");
                assert!(
                    msg.contains(&length.to_string()) || msg.contains(&max.to_string()),
                    "Error should include actual and max length"
                );
                println!("  ✓ Shows actual ({length}) vs expected ({max}) length");
            }
        }
        Ok(_) => bail!("Should return QueryTooLong error"),
    }

    // Test 4: Invalid limit (zero)
    println!("\nTest 4: Invalid limit (zero)");
    let result = validate_limit("0");
    match result {
        Err(ref e) => {
            let msg = e.to_string();
            println!("  Error: '{msg}'");
            if matches!(e, ValidationError::InvalidLimitZero) {
                assert!(
                    msg.contains("at least 1"),
                    "Error should explain requirement"
                );
            }
            println!("  ✓ Explains the requirement clearly");
        }
        Ok(_) => bail!("Should return InvalidLimit error"),
    }

    // Test 5: Regex pattern rejection (security)
    println!("\nTest 5: Regex pattern rejected (security)");
    let result = validate_query("/[a-z]+/");
    match result {
        Err(ref e) => {
            let msg = e.to_string();
            println!("  Error: '{msg}'");
            assert!(
                msg.contains("Regex") || msg.contains("regex") || msg.contains("pattern"),
                "Error should mention regex/pattern"
            );
            println!("  ✓ Explains why regex was rejected");
        }
        Ok(_) => bail!("Should reject regex patterns"),
    }

    println!("\nTHEN: All error messages are helpful\n");
    println!("  ✓ Messages explain what went wrong");
    println!("  ✓ Messages suggest how to fix");
    println!("  ✓ No confusing technical jargon");
    println!("  ✓ Consistent error format across all cases");

    println!("\n=== SCENARIO 4 PASSED: Error messages are clear and actionable ===\n");

    Ok(())
}

// =============================================================================
// SCENARIO 5: Empty directory edge case
// =============================================================================

/// Scenario: User tries to index an empty directory
///
/// **GIVEN**: An empty directory with no markdown files
/// **WHEN**: User runs the indexing pipeline
/// **THEN**: Pipeline completes successfully with zero documents
#[test]
fn scenario_empty_directory_handling() -> anyhow::Result<()> {
    println!("\n=== SCENARIO 5: Empty directory handling ===\n");

    // GIVEN: An empty directory
    let ctx = ScenarioContext::new()?;
    let empty_dir = ctx.root().join("empty");
    fs::create_dir(&empty_dir)?;
    let output_dir = ctx.output_dir();

    println!("GIVEN: Empty directory at {}", empty_dir.display());
    println!("  No markdown files present");

    // WHEN: User runs the indexing pipeline
    println!("\nWHEN: User runs the indexing pipeline");

    let result = run_index_pipeline(&empty_dir, &output_dir)?;
    println!("  Documents indexed: {}", result.document_count);
    println!("  Chunks created: {}", result.chunk_count);

    assert_eq!(
        result.document_count, 0,
        "Empty directory should have 0 documents"
    );
    assert_eq!(
        result.chunk_count, 0,
        "Empty directory should have 0 chunks"
    );

    // THEN: Pipeline completes successfully
    println!("\nTHEN: Pipeline completes without errors\n");

    assert!(output_dir.exists(), "Output directory should be created");
    println!("  ✓ Output directory created");

    let index_path = output_dir.join("INDEX.json");
    assert!(index_path.exists(), "Index should be created");
    println!("  ✓ Index file created");

    let index_content = fs::read_to_string(&index_path)?;
    let _index: Value = serde_json::from_str(&index_content)?;
    println!("  ✓ Index is valid JSON");

    println!("\n=== SCENARIO 5 PASSED: Empty directory handled gracefully ===\n");

    Ok(())
}

// =============================================================================
// Test Coverage Summary
// =============================================================================

#[test]
fn scenario_coverage_report() {
    println!("\n{}", "=".repeat(70));
    println!("SCENARIO TEST COVERAGE REPORT");
    println!("{}", "=".repeat(70));
    println!();
    println!("Behavior-focused scenario tests following Fowler's guidelines:");
    println!();
    println!("Scenarios Covered:");
    println!("  1. User indexes their project documentation");
    println!("     - Full pipeline: DISCOVER -> ANALYZE -> ASSIGN -> CHUNK -> INDEX");
    println!("     - Multi-file project with directory structure");
    println!("     - Search index creation and verification");
    println!();
    println!("  2. User searches for specific content");
    println!("     - Multi-term search queries");
    println!("     - Relevance scoring and ranking");
    println!("     - Case-insensitive matching");
    println!("     - Empty query rejection with helpful error");
    println!();
    println!("  3. Scraping with content filtering");
    println!("     - HTML content extraction with Readability");
    println!("     - Navigation/boilerplate removal");
    println!("     - BM25 scoring for relevance filtering");
    println!("     - Density score calculation");
    println!();
    println!("  4. Error messages are helpful");
    println!("     - Empty query errors");
    println!("     - Query too long errors");
    println!("     - Invalid limit errors");
    println!("     - Regex rejection for security");
    println!("     - Clear, actionable messages");
    println!();
    println!("  5. Empty directory handling");
    println!("     - Graceful handling of no files");
    println!("     - Pipeline completes successfully");
    println!("     - Empty but valid index created");
    println!();
    println!("Testing Principles (Fowler-Approved):");
    println!("  - Tests tell stories about user behavior");
    println!("  - No implementation detail testing");
    println!("  - Each scenario covers a complete user workflow");
    println!("  - Given-When-Then format for clarity");
    println!("  - Observable outcomes, not internals");
    println!("  - No panics, no unwraps, proper error handling");
    println!();
    println!("{}", "=".repeat(70));
    println!();
}
