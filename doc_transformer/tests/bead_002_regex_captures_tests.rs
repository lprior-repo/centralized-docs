// BEAD-002 Tests: Ensure no panics on regex captures
// These tests verify that regex capture group access never panics

use doc_transformer::analyze::{analyze_files};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_extract_title_no_panic_on_empty_capture() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Edge case: H1 with only whitespace
    fs::write(base_path.join("empty_h1.md"), "# \n\nContent here")
        .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("Should not panic");

    assert_eq!(analyses.len(), 1);
    assert!(!analyses[0].title.is_empty(), "Should have fallback title");
}

#[test]
fn test_extract_headings_no_panic_on_malformed() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Edge cases for headings
    fs::write(
        base_path.join("malformed_headings.md"),
        "# Valid H1\n## \n###No space\n####   Multiple   Spaces  \n##### \t\tTabs\n"
    )
    .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("Should not panic");

    assert_eq!(analyses.len(), 1);
    // Should extract valid headings, skip or handle malformed ones gracefully
    assert!(!analyses[0].headings.is_empty(), "Should extract at least some headings");
}

#[test]
fn test_extract_links_no_panic_on_edge_cases() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Edge cases for links
    fs::write(
        base_path.join("edge_links.md"),
        "# Links Test\n[](empty-target.md)\n[Empty text]()\n[Normal](./doc.md)\n"
    )
    .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("Should not panic");

    assert_eq!(analyses.len(), 1);
    // Should handle edge cases without panicking
    for link in &analyses[0].links {
        // text and target may be empty, but should not panic
        assert!(link.text.len() >= 0);
        assert!(link.target.len() >= 0);
    }
}

#[test]
fn test_headings_with_all_edge_cases() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Comprehensive edge cases
    let content = r#"# Title
## Also empty
###No space after hashes
####
#####
######Too many hashes but valid
Content here
"#;

    fs::write(base_path.join("edge_headings.md"), content)
        .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let result = analyze_files(&files, base_path);

    // Should not panic - this is the key requirement
    assert!(result.is_ok(), "Should not panic on edge case headings");

    let analyses = result.unwrap();
    assert_eq!(analyses.len(), 1);

    // Verify all extracted headings have valid structure
    for heading in &analyses[0].headings {
        assert!(heading.level >= 1 && heading.level <= 6);
        // text may be empty for malformed headings, but shouldn't panic
    }
}

#[test]
fn test_transform_fix_headings_no_panic() {
    use doc_transformer::transform::transform_all;
    use doc_transformer::assign::assign_ids;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // File with extreme heading edge cases
    let content = r#"# Main Title
####### Too many levels (7)
###No space
## Normal heading
#### Skipped level (should be H3)
"#;

    fs::write(base_path.join("headings_transform.md"), content)
        .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("analyze should succeed");
    let (analyses_updated, link_map) = assign_ids(analyses).expect("assign_ids should succeed");

    // This should not panic when processing headings
    let result = transform_all(&analyses_updated, &link_map, &output_dir);
    assert!(result.is_ok(), "Transform should not panic on edge case headings");
}

#[test]
fn test_unicode_in_headings_no_panic() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Unicode edge cases
    let content = "# 日本語タイトル\n## Émojis 🎉\n### Мультибайт\n";

    fs::write(base_path.join("unicode.md"), content)
        .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let result = analyze_files(&files, base_path);

    assert!(result.is_ok(), "Should handle unicode without panicking");

    let analyses = result.unwrap();
    assert_eq!(analyses.len(), 1);
    assert!(!analyses[0].title.is_empty());
}
