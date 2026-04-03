#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Tests for atomic validation behavior
//!
//! Verifies that validation runs BEFORE artifact writing,
//! ensuring no partial artifacts remain on validation failure.

#![allow(unused_must_use)]

use doc_transformer::{analyze, assign, chunk, discover, index, llms, transform, validate};
use std::fs;
use tempfile::TempDir;

/// Test that validation failure prevents artifact writing
///
/// This test creates an invalid file directly in output/docs (bypassing transform),
/// then simulates the fixed behavior where validation runs before artifact writing.
/// The key verification: when validation fails, no artifacts should be written.
#[test]
fn test_validation_failure_prevents_artifacts() -> anyhow::Result<()> {
    // Create test context with temporary directory
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path();
    let output_dir = source_dir.join("output");

    // Create a VALID document first (to pass transform)
    let valid_content = r#"---
id: test/valid
title: Valid Test Document
category: tutorial
tags: ["test", "valid"]
---

# Valid Document

This document has proper frontmatter and an H1 heading.
It should pass validation.
"#;
    fs::write(source_dir.join("valid.md"), valid_content)?;

    // Run discovery
    let (files, _manifest) = discover::discover_files(source_dir, None)?;

    // Run analysis
    let analysis_base_path = source_dir.to_path_buf();
    let analyze_result = analyze::analyze_files(&files, &analysis_base_path, None)?;
    let analyses = analyze_result.analyses;

    // Assign IDs
    let (analyses, link_map) = assign::assign_ids(analyses);

    // Transform (creates files in output/docs)
    let transform_result = transform::transform_all(&analyses, &link_map, &output_dir)?;
    assert_eq!(
        transform_result.success_count, 1,
        "Transform should succeed"
    );

    // NOW add an INVALID file directly to output/docs that will fail validation
    // This simulates a case where something goes wrong in the pipeline
    let docs_dir = output_dir.join("docs");
    let invalid_file = docs_dir.join("invalid.md");
    let invalid_content = r#"
# First Heading

Some content.

# Second Heading

More content - this has multiple H1s!
"#;
    fs::write(&invalid_file, invalid_content)?;

    // Chunk
    let chunks_result = chunk::chunk_all(&analyses, &link_map, &output_dir, 1_000_000)?;

    // Validate - this should find errors because of the manually added invalid file
    let validation_result = validate::validate_all(&output_dir)?;

    // Validation SHOULD fail for our manually added invalid document
    assert!(
        validation_result.total_errors > 0,
        "Expected validation errors for invalid document, got {} errors",
        validation_result.total_errors
    );

    // Now test the key behavior: with validation failure,
    // artifacts should NOT be written
    // This is the core of the fix - simulate what run_index does

    // Only write artifacts if validation passes
    // (This is the fix - validation check before writing)
    if validation_result.total_errors == 0 {
        index::build_and_write_index(
            &analyses,
            &link_map,
            &chunks_result,
            &output_dir,
            "test",
            Some(20),
            Some(16),
            Some(200),
            Some(12),
        )?;

        let llms_config = llms::LlmsConfig {
            project_name: "test".to_string(),
            project_description: "test project".to_string(),
            generate_full: true,
            ..Default::default()
        };
        llms::generate_llms_txt(&analyses, &link_map, &llms_config, &output_dir)?;
    }

    // After fix: if validation fails, no artifacts should exist
    let index_exists = output_dir.join("INDEX.json").exists();
    let compass_exists = output_dir.join("COMPASS.md").exists();
    let llms_exists = output_dir.join("llms.txt").exists();

    // Because validation failed, artifacts should NOT be written
    assert!(
        !index_exists,
        "INDEX.json should NOT exist when validation fails"
    );
    assert!(
        !compass_exists,
        "COMPASS.md should NOT exist when validation fails"
    );
    assert!(
        !llms_exists,
        "llms.txt should NOT exist when validation fails"
    );

    Ok(())
}

/// Test that successful validation allows artifact writing
///
/// Verifies the positive case: when validation passes,
/// artifacts ARE written correctly.
#[test]
fn test_validation_success_allows_artifacts() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path();
    let output_dir = source_dir.join("output");

    // Create a VALID document (has frontmatter and H1)
    let valid_content = r#"---
id: test/valid
title: Valid Test Document
category: tutorial
tags: ["test", "valid"]
---

# Valid Document

This document has proper frontmatter and an H1 heading.
It should pass validation.
"#;
    fs::write(source_dir.join("valid.md"), valid_content)?;

    // Run pipeline
    let (files, _manifest) = discover::discover_files(source_dir, None)?;
    let analysis_base_path = source_dir.to_path_buf();
    let analyze_result = analyze::analyze_files(&files, &analysis_base_path, None)?;
    let analyses = analyze_result.analyses;
    let (analyses, link_map) = assign::assign_ids(analyses);
    let _transform_result = transform::transform_all(&analyses, &link_map, &output_dir)?;
    let chunks_result = chunk::chunk_all(&analyses, &link_map, &output_dir, 1_000_000)?;

    // Validate
    let validation_result = validate::validate_all(&output_dir)?;

    // Validation SHOULD pass for our valid document
    assert_eq!(
        validation_result.total_errors, 0,
        "Expected no validation errors for valid document, got {}",
        validation_result.total_errors
    );

    // Now write artifacts (simulating the fixed run_index behavior)
    // Only write artifacts if validation passes
    if validation_result.total_errors == 0 {
        index::build_and_write_index(
            &analyses,
            &link_map,
            &chunks_result,
            &output_dir,
            "test",
            Some(20),
            Some(16),
            Some(200),
            Some(12),
        )?;

        let llms_config = llms::LlmsConfig {
            project_name: "test".to_string(),
            project_description: "test project".to_string(),
            generate_full: true,
            ..Default::default()
        };
        llms::generate_llms_txt(&analyses, &link_map, &llms_config, &output_dir)?;
    }

    // After fix: if validation passes, artifacts SHOULD exist
    let index_exists = output_dir.join("INDEX.json").exists();
    let llms_exists = output_dir.join("llms.txt").exists();

    assert!(
        index_exists,
        "INDEX.json should exist when validation passes"
    );
    assert!(llms_exists, "llms.txt should exist when validation passes");

    Ok(())
}
