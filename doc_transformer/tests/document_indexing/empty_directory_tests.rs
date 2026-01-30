//! Comprehensive tests for empty directory handling
//!
//! BEAD: file-discovery-no-markdown (P2)
//!
//! Tests the behavior when indexing an empty directory (or directory with only ignored files):
//! 1. Empty directory should complete successfully (not error)
//! 2. INDEX.json should have document_count: 0
//! 3. INDEX.json should have chunk_count: 0
//! 4. llms.txt should be created (even if empty/minimal)
//! 5. COMPASS.md should be created (even if minimal)
//! 6. Output directory structure should still be created (chunks/, not docs/)
//! 7. Re-indexing empty directory should be idempotent
//! 8. INDEX.json structure is valid even when empty
//! 9. All output files are readable and valid
//! 10. Empty directory produces valid (empty) keywords map
//! 11. Empty directory with only ignored files behaves like empty
//! 12. chunks directory is created, docs is not created when empty

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![allow(clippy::needless_borrows_for_generic_args)] // PathBuf AsRef<Path]

// Use common test fixtures
use crate::common::*;
use anyhow::{Context, Result};
use doc_transformer::llms::LlmsConfig;
use doc_transformer::{analyze, assign, chunk, discover, index, llms};
use serde_json::Value;
use std::fs;
use std::path::Path;

// =============================================================================
// EXTENDED PIPELINE RUNNER (for llms.txt and COMPASS.md generation)
// =============================================================================

/// Extended pipeline result including discovered file count
struct ExtendedIndexResult {
    inner: IndexResult,
    discovered_files_count: usize,
}

/// Run the full indexing pipeline with llms.txt and COMPASS.md generation
///
/// This extends the common `run_full_pipeline` by also generating llms.txt and COMPASS.md
/// which are tested specifically in this file.
fn run_indexing_pipeline(source_dir: &Path, output_dir: &Path) -> Result<ExtendedIndexResult> {
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
    let discovered_count = discovered_files.len();

    // Phase 2: ANALYZE
    let analyses = analyze::analyze_files(&discovered_files, source_dir, None)
        .context("Analysis phase failed")?;

    // Phase 3: ASSIGN IDs
    let (_analyses_with_ids, link_map) = assign::assign_ids(analyses.clone());

    // Phase 4: CHUNK
    let chunks_result =
        chunk::chunk_all(&analyses, &link_map, output_dir).context("Chunking phase failed")?;

    // Phase 5: INDEX
    index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output_dir,
        "Test Project",
        None, // max_related_chunks
        None, // hnsw_m
        None, // hnsw_ef_construction
    )
    .context("Indexing phase failed")?;

    // Phase 6: Generate llms.txt
    let llms_config = LlmsConfig {
        project_name: "Test Project".to_string(),
        project_description: "Test documentation".to_string(),
        ..Default::default()
    };
    llms::generate_llms_txt(&analyses, &link_map, &llms_config, output_dir)
        .context("Failed to generate llms.txt")?;

    // Phase 7: Generate COMPASS.md (using llms.txt as source)
    let llms_path = output_dir.join("llms.txt");
    let llms_content = fs::read_to_string(&llms_path)
        .context("Failed to read llms.txt for COMPASS.md generation")?;
    let compass_path = output_dir.join("COMPASS.md");
    fs::write(&compass_path, llms_content).context("Failed to write COMPASS.md")?;

    let index_path = output_dir.join("INDEX.json");

    Ok(ExtendedIndexResult {
        inner: IndexResult {
            document_count: analyses.len(),
            chunk_count: chunks_result.total_chunks,
            summary_chunks: chunks_result.summary_chunks,
            standard_chunks: chunks_result.standard_chunks,
            detailed_chunks: chunks_result.detailed_chunks,
            index_path,
            output_dir: output_dir.to_path_buf(),
        },
        discovered_files_count: discovered_count,
    })
}

// =============================================================================
// TEST CASES
// =============================================================================

/// Test 1: Empty directory should complete successfully (not error)
#[test]
fn test_empty_directory_completes_successfully() -> Result<()> {
    let ctx = TestContext::new()?;

    // Run the full pipeline on an empty directory
    let result = run_indexing_pipeline(ctx.root(), &ctx.output_dir());

    assert!(result.is_ok(), "Empty directory indexing should succeed");
    let pipeline_result = result?;

    assert_eq!(
        pipeline_result.inner.document_count, 0,
        "Empty directory should have 0 documents"
    );
    assert_eq!(
        pipeline_result.inner.chunk_count, 0,
        "Empty directory should have 0 chunks"
    );
    assert_eq!(
        pipeline_result.discovered_files_count, 0,
        "Empty directory should discover 0 files"
    );

    Ok(())
}

/// Test 2: INDEX.json should have document_count: 0
#[test]
fn test_index_json_has_zero_document_count() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let index_path = &ctx.output_dir().join("INDEX.json");
    assert!(index_path.exists(), "INDEX.json should be created");

    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let index_json: Value =
        serde_json::from_str(&index_content).context("Failed to parse INDEX.json")?;

    // Check stats.doc_count
    let doc_count = index_json
        .get("stats")
        .and_then(|s| s.get("doc_count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(999);

    assert_eq!(doc_count, 0, "INDEX.json stats.doc_count should be 0");

    // Verify documents array is empty
    let documents = index_json.get("documents").and_then(|v| v.as_array());

    match documents {
        Some(docs) => {
            assert_eq!(docs.len(), 0, "INDEX.json documents array should be empty");
        }
        None => {
            return Err(anyhow::anyhow!("INDEX.json missing 'documents' array"));
        }
    }

    Ok(())
}

/// Test 3: INDEX.json should have chunk_count: 0
#[test]
fn test_index_json_has_zero_chunk_count() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let index_path = &ctx.output_dir().join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let index_json: Value =
        serde_json::from_str(&index_content).context("Failed to parse INDEX.json")?;

    // Check stats.chunk_count
    let chunk_count = index_json
        .get("stats")
        .and_then(|s| s.get("chunk_count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(999);

    assert_eq!(chunk_count, 0, "INDEX.json stats.chunk_count should be 0");

    // Verify chunks array is empty
    let chunks = index_json.get("chunks").and_then(|v| v.as_array());

    match chunks {
        Some(chunk_array) => {
            assert_eq!(
                chunk_array.len(),
                0,
                "INDEX.json chunks array should be empty"
            );
        }
        None => {
            return Err(anyhow::anyhow!("INDEX.json missing 'chunks' array"));
        }
    }

    Ok(())
}

/// Test 4: llms.txt should be created (even if empty/minimal)
#[test]
fn test_llms_txt_created_for_empty_directory() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let llms_path = &ctx.output_dir().join("llms.txt");
    assert!(
        llms_path.exists(),
        "llms.txt should be created for empty directory"
    );

    let llms_content = fs::read_to_string(&llms_path).context("Failed to read llms.txt")?;

    // Verify llms.txt has required structure
    assert!(
        llms_content.contains("Test Project"),
        "llms.txt should contain project name"
    );
    assert!(
        llms_content.contains("0"),
        "llms.txt should mention 0 documents"
    );

    // Verify YAML frontmatter exists
    assert!(
        llms_content.contains("---"),
        "llms.txt should have YAML frontmatter"
    );
    assert!(
        llms_content.contains("version:"),
        "llms.txt should have version in frontmatter"
    );
    assert!(
        llms_content.contains("documents: 0"),
        "llms.txt frontmatter should show 0 documents"
    );

    Ok(())
}

/// Test 5: COMPASS.md should be created (even if minimal)
#[test]
fn test_compass_md_created_for_empty_directory() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let compass_path = &ctx.output_dir().join("COMPASS.md");
    assert!(
        compass_path.exists(),
        "COMPASS.md should be created for empty directory"
    );

    let compass_content = fs::read_to_string(&compass_path).context("Failed to read COMPASS.md")?;

    // Verify COMPASS.md has required structure
    assert!(
        compass_content.contains("Documentation Compass"),
        "COMPASS.md should have title"
    );
    assert!(
        compass_content.contains("0 documents"),
        "COMPASS.md should mention 0 documents"
    );
    assert!(
        compass_content.contains("---"),
        "COMPASS.md should have YAML frontmatter"
    );

    Ok(())
}

/// Test 6: Output directory structure should still be created
#[test]
fn test_output_directory_structure_created() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    // Verify main output files exist
    assert!(&ctx.output_dir().exists(), "Output directory should exist");
    assert!(
        &ctx.output_dir().join("INDEX.json").exists(),
        "INDEX.json should exist"
    );
    assert!(
        &ctx.output_dir().join("llms.txt").exists(),
        "llms.txt should exist"
    );
    assert!(
        &ctx.output_dir().join("COMPASS.md").exists(),
        "COMPASS.md should exist"
    );

    // Verify chunks directory exists (even if empty)
    let chunks_dir = &ctx.output_dir().join("chunks");
    assert!(chunks_dir.exists(), "chunks directory should be created");

    // Note: docs directory is only created when there are documents to write
    // For empty input, docs directory does not exist

    Ok(())
}

/// Test 7: Re-indexing empty directory should be idempotent
#[test]
fn test_reindexing_empty_directory_is_idempotent() -> Result<()> {
    let ctx = TestContext::new()?;

    // First indexing run
    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let index_path = &ctx.output_dir().join("INDEX.json");
    let first_content =
        fs::read_to_string(&index_path).context("Failed to read INDEX.json after first run")?;

    let llms_path = &ctx.output_dir().join("llms.txt");
    let _first_llms =
        fs::read_to_string(&llms_path).context("Failed to read llms.txt after first run")?;

    let compass_path = &ctx.output_dir().join("COMPASS.md");
    let _first_compass =
        fs::read_to_string(&compass_path).context("Failed to read COMPASS.md after first run")?;

    // Second indexing run (should be idempotent)
    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let second_content =
        fs::read_to_string(&index_path).context("Failed to read INDEX.json after second run")?;

    let _second_llms =
        fs::read_to_string(&llms_path).context("Failed to read llms.txt after second run")?;

    let _second_compass =
        fs::read_to_string(&compass_path).context("Failed to read COMPASS.md after second run")?;

    // Verify the outputs are consistent (ignoring timestamps)
    let first_json: Value =
        serde_json::from_str(&first_content).context("Failed to parse first INDEX.json")?;
    let second_json: Value =
        serde_json::from_str(&second_content).context("Failed to parse second INDEX.json")?;

    // Check that structure is the same (ignoring timestamps)
    let first_doc_count = first_json
        .get("stats")
        .and_then(|s| s.get("doc_count"))
        .and_then(|v| v.as_u64());
    let second_doc_count = second_json
        .get("stats")
        .and_then(|s| s.get("doc_count"))
        .and_then(|v| v.as_u64());

    assert_eq!(
        first_doc_count, second_doc_count,
        "Document count should be consistent across runs"
    );

    let first_chunk_count = first_json
        .get("stats")
        .and_then(|s| s.get("chunk_count"))
        .and_then(|v| v.as_u64());
    let second_chunk_count = second_json
        .get("stats")
        .and_then(|s| s.get("chunk_count"))
        .and_then(|v| v.as_u64());

    assert_eq!(
        first_chunk_count, second_chunk_count,
        "Chunk count should be consistent across runs"
    );

    // Both should show 0
    assert_eq!(
        first_doc_count,
        Some(0),
        "First run should have 0 documents"
    );
    assert_eq!(first_chunk_count, Some(0), "First run should have 0 chunks");

    Ok(())
}

/// Test 8: Empty directory with only ignored files should behave like empty
#[test]
fn test_empty_directory_with_only_ignored_files() -> Result<()> {
    let ctx = TestContext::new()?;

    // Create only ignored files (in node_modules)
    let node_modules = ctx.root().join("node_modules");
    fs::create_dir_all(&node_modules).context("Failed to create node_modules directory")?;
    fs::write(node_modules.join("package.md"), "# Package\n\nContent")
        .context("Failed to write ignored file")?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    // Should behave like empty directory
    let index_path = &ctx.output_dir().join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let index_json: Value =
        serde_json::from_str(&index_content).context("Failed to parse INDEX.json")?;

    let doc_count = index_json
        .get("stats")
        .and_then(|s| s.get("doc_count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(999);

    assert_eq!(
        doc_count, 0,
        "Directory with only ignored files should have 0 documents"
    );

    Ok(())
}

/// Test 9: INDEX.json structure is valid even when empty
#[test]
fn test_index_json_structure_valid_when_empty() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let index_path = &ctx.output_dir().join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let index_json: Value =
        serde_json::from_str(&index_content).context("Failed to parse INDEX.json")?;

    // Verify all required top-level keys exist
    let required_keys = [
        "version",
        "project",
        "updated",
        "stats",
        "documents",
        "chunks",
        "keywords",
        "graph",
    ];
    for key in &required_keys {
        assert!(
            index_json.get(*key).is_some(),
            "INDEX.json should have '{key}' key"
        );
    }

    // Verify stats structure
    let stats = index_json
        .get("stats")
        .context("INDEX.json missing 'stats'")?;

    assert!(
        stats.get("doc_count").is_some(),
        "stats should have doc_count"
    );
    assert!(
        stats.get("chunk_count").is_some(),
        "stats should have chunk_count"
    );
    assert!(stats.get("graph").is_some(), "stats should have graph");

    // Verify graph structure
    let graph = index_json
        .get("graph")
        .context("INDEX.json missing 'graph'")?;

    assert!(graph.get("nodes").is_some(), "graph should have nodes");
    assert!(graph.get("edges").is_some(), "graph should have edges");

    // Nodes and edges should be empty arrays
    let nodes = graph
        .get("nodes")
        .and_then(|v| v.as_array())
        .context("graph.nodes should be an array")?;
    assert_eq!(nodes.len(), 0, "graph.nodes should be empty");

    let edges = graph
        .get("edges")
        .and_then(|v| v.as_array())
        .context("graph.edges should be an array")?;
    assert_eq!(edges.len(), 0, "graph.edges should be empty");

    Ok(())
}

/// Test 10: All output files are readable and valid
#[test]
fn test_all_output_files_readable_for_empty_directory() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    // INDEX.json should be valid JSON
    let index_path = &ctx.output_dir().join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let _: Value =
        serde_json::from_str(&index_content).with_context(|| "INDEX.json should be valid JSON")?;

    // llms.txt should be readable text
    let llms_path = &ctx.output_dir().join("llms.txt");
    let llms_content = fs::read_to_string(&llms_path).context("Failed to read llms.txt")?;
    assert!(!llms_content.is_empty(), "llms.txt should not be empty");

    // COMPASS.md should be readable markdown
    let compass_path = &ctx.output_dir().join("COMPASS.md");
    let compass_content = fs::read_to_string(&compass_path).context("Failed to read COMPASS.md")?;
    assert!(
        !compass_content.is_empty(),
        "COMPASS.md should not be empty"
    );

    Ok(())
}

/// Test 11: Empty directory produces valid keywords map
#[test]
fn test_empty_keywords_map() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let index_path = &ctx.output_dir().join("INDEX.json");
    let index_content = fs::read_to_string(&index_path).context("Failed to read INDEX.json")?;
    let index_json: Value =
        serde_json::from_str(&index_content).context("Failed to parse INDEX.json")?;

    let keywords = index_json
        .get("keywords")
        .and_then(|v| v.as_object())
        .context("INDEX.json keywords should be an object")?;

    assert_eq!(
        keywords.len(),
        0,
        "keywords map should be empty for empty directory"
    );

    Ok(())
}

/// Test 12: Verify chunks directory is created (docs is not created when empty)
#[test]
fn test_chunks_directory_created_docs_not_created() -> Result<()> {
    let ctx = TestContext::new()?;

    run_indexing_pipeline(ctx.root(), &ctx.output_dir())?;

    let chunks_dir = &ctx.output_dir().join("chunks");
    let docs_dir = &ctx.output_dir().join("docs");

    // chunks directory should exist even when empty
    assert!(chunks_dir.exists(), "chunks directory should exist");
    assert!(chunks_dir.is_dir(), "chunks should be a directory");

    // docs directory is NOT created when there are no documents
    assert!(
        !docs_dir.exists(),
        "docs directory should not exist when empty"
    );

    // Verify chunks directory is empty
    let chunk_entries: Vec<_> = fs::read_dir(chunks_dir)
        .context("Failed to read chunks directory")?
        .filter_map(Result::ok)
        .collect();

    assert_eq!(chunk_entries.len(), 0, "chunks directory should be empty");

    Ok(())
}
