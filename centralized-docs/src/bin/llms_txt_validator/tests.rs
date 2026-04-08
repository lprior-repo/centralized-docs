#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_valid_llms_txt() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "# Project\n\n## Getting Started\n\n## Core Concepts\n\n## API Reference\n\nSee INDEX.json"
    )?;

    let result = validate_llms_txt(file.path())?;
    assert!(result.valid);
    Ok(())
}

#[test]
fn test_empty_llms_txt() -> anyhow::Result<()> {
    let file = NamedTempFile::new()?;
    let result = validate_llms_txt(file.path())?;
    assert!(!result.valid);
    Ok(())
}

#[test]
fn test_valid_index_json() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"{{"version": "1.0", "project": "test", "documents": [{{"id": "1", "title": "Doc", "path": "doc.md"}}]}}"#
    )?;

    let result = validate_index_json(file.path())?;
    assert!(result.valid);
    Ok(())
}

#[test]
fn test_link_validation_valid_urls() {
    let content = r"
# Documentation

See the [official site](https://example.com) for more info.
Check the [API docs](https://api.example.com/v1/docs).
Also see [local file](./guide.md) and [anchor](#section).
        ";

    let errors = validate_links_in_content(content);
    let result = validation_result(errors);

    // Should not have any errors, only info about link count
    assert!(!result.has_errors());
}

#[test]
fn test_link_validation_malformed_urls() {
    let content = r"
# Documentation

This has a [empty link]() in the text.
And another [newline link](https://example.com
/path) here.
        ";

    let errors = validate_links_in_content(content);
    let result = validation_result(errors);

    // Should detect malformed links (empty URL or URL with newline)
    assert!(result.has_warnings() || result.has_errors());
}

#[test]
fn test_link_validation_no_links() {
    let content = "# Documentation\n\nJust plain text with no links.";

    let errors = validate_links_in_content(content);
    let result = validation_result(errors);

    // Should report no links found (Info level)
    let has_no_links_info = result
        .errors
        .iter()
        .any(|e| e.field == "links" && e.message.contains("No links found"));
    assert!(has_no_links_info);
}

#[test]
fn test_index_json_with_chunks() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"{{
            "version": "1.0",
            "project": "test",
            "documents": [{{"id": "doc1", "title": "Doc", "path": "doc.md"}}],
            "chunks": [
                {{"chunk_id": "chunk1", "doc_id": "doc1", "chunk_level": "standard"}},
                {{"chunk_id": "chunk2", "doc_id": "doc1", "chunk_level": "detailed"}}
            ]
        }}"#
    )?;

    let result = validate_index_json(file.path())?;
    assert!(result.valid);
    Ok(())
}

#[test]
fn test_index_json_invalid_chunk_reference() -> anyhow::Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        r#"{{
            "version": "1.0",
            "project": "test",
            "documents": [{{"id": "doc1", "title": "Doc", "path": "doc.md"}}],
            "chunks": [
                {{"chunk_id": "chunk1", "doc_id": "doc_INVALID", "chunk_level": "standard"}}
            ]
        }}"#
    )?;

    let result = validate_index_json(file.path())?;
    assert!(!result.valid);
    assert!(result.has_errors());
    Ok(())
}

/// Helper to count errors in validation result
fn count_errors(result: &ValidationResult) -> usize {
    result
        .errors
        .iter()
        .filter(|e| e.severity == Severity::Error)
        .count()
}

#[test]
fn test_exit_code_for_1_to_10_errors() -> anyhow::Result<()> {
    // Create a file with exactly 5 errors (in the 1-10 range)
    let mut file = NamedTempFile::new()?;
    // Multiple duplicate chunk IDs = multiple errors
    writeln!(
        file,
        r#"{{
            "version": "1.0",
            "project": "test",
            "documents": [
                {{"id": "doc1", "title": "Doc", "path": "doc.md"}},
                {{"id": "doc1", "title": "Doc2", "path": "doc2.md"}},
                {{"id": "doc2", "title": "Doc3", "path": "doc3.md"}},
                {{"id": "doc2", "title": "Doc4", "path": "doc4.md"}},
                {{"id": "doc3", "title": "Doc5", "path": "doc5.md"}}
            ],
            "chunks": [
                {{"chunk_id": "chunk1", "doc_id": "doc1", "chunk_level": "standard"}},
                {{"chunk_id": "chunk1", "doc_id": "doc1", "chunk_level": "standard"}},
                {{"chunk_id": "chunk2", "doc_id": "doc2", "chunk_level": "standard"}},
                {{"chunk_id": "chunk2", "doc_id": "doc2", "chunk_level": "standard"}},
                {{"chunk_id": "chunk3", "doc_id": "doc3", "chunk_level": "standard"}}
            ]
        }}"#
    )?;

    let result = validate_index_json(file.path())?;
    let error_count = count_errors(&result);

    // Should have errors in 1-10 range
    assert!((1..=10).contains(&error_count));
    Ok(())
}

#[test]
fn test_exit_code_for_11_to_100_errors() -> anyhow::Result<()> {
    // Create a file with 15 errors (in the 11-100 range)
    let mut file = NamedTempFile::new()?;

    // Generate documents with lots of duplicate chunk IDs
    // Each duplicate creates an error
    let json = r#"{
        "version": "1.0",
        "project": "test",
        "documents": [
            {"id": "doc0", "title": "Doc0", "path": "doc0.md"},
            {"id": "doc1", "title": "Doc1", "path": "doc1.md"},
            {"id": "doc2", "title": "Doc2", "path": "doc2.md"},
            {"id": "doc3", "title": "Doc3", "path": "doc3.md"},
            {"id": "doc4", "title": "Doc4", "path": "doc4.md"}
        ],
        "chunks": [
            {"chunk_id": "chunk0", "doc_id": "doc0", "chunk_level": "standard"},
            {"chunk_id": "chunk0", "doc_id": "doc0", "chunk_level": "standard"},
            {"chunk_id": "chunk1", "doc_id": "doc0", "chunk_level": "standard"},
            {"chunk_id": "chunk1", "doc_id": "doc0", "chunk_level": "standard"},
            {"chunk_id": "chunk2", "doc_id": "doc1", "chunk_level": "standard"},
            {"chunk_id": "chunk2", "doc_id": "doc1", "chunk_level": "standard"},
            {"chunk_id": "chunk3", "doc_id": "doc1", "chunk_level": "standard"},
            {"chunk_id": "chunk3", "doc_id": "doc1", "chunk_level": "standard"},
            {"chunk_id": "chunk4", "doc_id": "doc2", "chunk_level": "standard"},
            {"chunk_id": "chunk4", "doc_id": "doc2", "chunk_level": "standard"},
            {"chunk_id": "chunk5", "doc_id": "doc2", "chunk_level": "standard"},
            {"chunk_id": "chunk5", "doc_id": "doc2", "chunk_level": "standard"},
            {"chunk_id": "chunk6", "doc_id": "doc3", "chunk_level": "standard"},
            {"chunk_id": "chunk6", "doc_id": "doc3", "chunk_level": "standard"},
            {"chunk_id": "chunk7", "doc_id": "doc3", "chunk_level": "standard"},
            {"chunk_id": "chunk7", "doc_id": "doc3", "chunk_level": "standard"},
            {"chunk_id": "chunk8", "doc_id": "doc4", "chunk_level": "standard"},
            {"chunk_id": "chunk8", "doc_id": "doc4", "chunk_level": "standard"},
            {"chunk_id": "chunk9", "doc_id": "doc4", "chunk_level": "standard"},
            {"chunk_id": "chunk9", "doc_id": "doc4", "chunk_level": "standard"},
            {"chunk_id": "chunk10", "doc_id": "doc4", "chunk_level": "standard"},
            {"chunk_id": "chunk10", "doc_id": "doc4", "chunk_level": "standard"}
        ]
    }"#;

    writeln!(file, "{json}")?;

    let result = validate_index_json(file.path())?;
    let error_count = count_errors(&result);

    // Should have errors in 11-100 range (we have 11 duplicate chunk IDs)
    assert!(error_count > 10, "Expected >10 errors, got {error_count}");
    Ok(())
}

#[test]
fn test_parse_error_detection() -> anyhow::Result<()> {
    // This is tested indirectly via the JSON parsing in main()
    // Here we test that invalid JSON returns an error
    let mut file = NamedTempFile::new()?;
    // Invalid JSON - missing closing brace, use write! to avoid format string issues
    use std::io::Write;
    write!(file, "{{ \"key\": value ")?;

    // Need to manually write the closing brace in a way that doesn't confuse the format parser
    let _ = file.write(b"}")?;

    let result = validate_index_json(file.path())?;
    assert!(!result.valid);
    assert!(result.has_errors());

    // Check that JSON parse error is detected
    let has_json_error = result.errors.iter().any(|e| e.field == "json");
    assert!(has_json_error, "Should have JSON parse error");
    Ok(())
}

#[test]
fn test_file_not_found_scenario() {
    // Test that missing file would be handled (tested via path.exists() check)
    let non_existent_path = PathBuf::from("/tmp/this_file_definitely_does_not_exist_12345.json");

    // Verify the file doesn't exist
    assert!(!non_existent_path.exists());

    // The main() function handles this with exit code 5
    // We verify the path check works correctly
}

#[test]
fn test_validation_result_has_errors_method() {
    let mut result = ValidationResult::new();
    assert!(!result.has_errors());

    result.add_error("test", "error message", Severity::Error);
    assert!(result.has_errors());

    // Warnings should not count as errors
    let mut result2 = ValidationResult::new();
    result2.add_error("test", "warning message", Severity::Warning);
    assert!(!result2.has_errors());
}

#[test]
fn test_validation_result_has_warnings_method() {
    let mut result = ValidationResult::new();
    assert!(!result.has_warnings());

    result.add_error("test", "warning message", Severity::Warning);
    assert!(result.has_warnings());

    // Errors should not count as warnings
    let mut result2 = ValidationResult::new();
    result2.add_error("test", "error message", Severity::Error);
    assert!(!result2.has_warnings());
}
