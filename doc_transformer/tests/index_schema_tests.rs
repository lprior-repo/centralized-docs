//! Tests for INDEX.json schema compliance
//!
//! These tests verify that INDEX.json conforms to validator schema:
//! - Required fields: version, project, updated
//! - Forbidden fields: generated

use std::fs;
use tempfile::TempDir;

/// Test that INDEX.json contains all required top-level fields
#[test]
fn test_index_json_has_required_fields() {
    // Arrange: Create temporary directory and sample data
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Create minimal test data
    let analyses = vec![];
    let link_map = std::collections::HashMap::new();

    // Create minimal chunks result with all required fields
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        total_chunks: 0,
        document_count: 0,
        chunks_metadata: vec![],
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let project_name = "test-project";

    // Act: Generate INDEX.json
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output_dir,
        project_name,
        None, // max_related_chunks
        None, // hnsw_m
        None, // hnsw_ef_construction
    )
    .expect("Failed to build index");

    // Read INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse INDEX.json");

    // Assert: Required fields exist
    assert!(
        json.get("version").is_some(),
        "INDEX.json must contain 'version' field"
    );
    assert!(
        json.get("project").is_some(),
        "INDEX.json must contain 'project' field"
    );
    assert!(
        json.get("updated").is_some(),
        "INDEX.json must contain 'updated' field"
    );

    // Assert: project field has correct value
    assert_eq!(
        json["project"], project_name,
        "project field must match provided project_name"
    );

    // Assert: updated field is valid ISO 8601 timestamp
    let updated = json["updated"].as_str().expect("updated must be a string");
    chrono::DateTime::parse_from_rfc3339(updated)
        .expect("updated must be valid ISO 8601 timestamp");
}

/// Test that INDEX.json does NOT contain 'generated' field
#[test]
fn test_index_json_no_generated_field() {
    // Arrange: Create temporary directory and sample data
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Create minimal test data
    let analyses = vec![];
    let link_map = std::collections::HashMap::new();

    // Create minimal chunks result with all required fields
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        total_chunks: 0,
        document_count: 0,
        chunks_metadata: vec![],
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let project_name = "test-project";

    // Act: Generate INDEX.json
    doc_transformer::index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output_dir,
        project_name,
        None, // max_related_chunks
        None, // hnsw_m
        None, // hnsw_ef_construction
    )
    .expect("Failed to build index");

    // Read INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let content = fs::read_to_string(&index_path).expect("Failed to read INDEX.json");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse INDEX.json");

    // Assert: 'generated' field must NOT exist
    assert!(
        json.get("generated").is_none(),
        "INDEX.json must NOT contain 'generated' field (use 'updated' instead)"
    );
}
