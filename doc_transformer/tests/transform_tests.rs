use doc_transformer::analyze::analyze_files;
use doc_transformer::assign::IdMapping;
use doc_transformer::discover::discover_files;
use doc_transformer::transform::transform_all;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

/// Helper to create test files for transform testing
fn setup_test_files() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Test file 1: Document with internal links
    fs::write(
        base_path.join("doc1.md"),
        "# First Document\nThis is the first document.\n\nSee [second document](./doc2.md) for more info.\n\nAlso check [third doc](./doc3.md).\n",
    )
    .expect("Failed to write doc1.md");

    // Test file 2: Referenced document
    fs::write(
        base_path.join("doc2.md"),
        "# Second Document\nThis is the second document.\n\nRefer back to [first](./doc1.md).\n",
    )
    .expect("Failed to write doc2.md");

    // Test file 3: Document with external links (should not be changed)
    fs::write(
        base_path.join("doc3.md"),
        "# Third Document\nExternal link: [Example](https://example.com)\n\nAnchor: [Top](#heading)\n\nEmail: [Contact](mailto:test@example.com)\n",
    )
    .expect("Failed to write doc3.md");

    temp_dir
}

#[test]
fn test_link_rewriting_no_extra_space() {
    let temp_dir = setup_test_files();
    let base_path = temp_dir.path();
    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Discover and analyze files
    let (files, _manifest) = discover_files(base_path).expect("Failed to discover files");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze files");

    // Create link map
    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    // Transform files
    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");

    assert!(result.success_count > 0, "No files were transformed successfully");
    assert_eq!(result.error_count, 0, "Transform had errors");

    // Read transformed doc1.md
    let doc1_content = fs::read_to_string(output_dir.join("docs/test-doc1.md"))
        .expect("Failed to read transformed doc1.md");

    // CRITICAL TEST: Verify no extra space in rewritten links
    // Bug was: [text](./ filename.md)
    // Fixed: [text](./filename.md)
    assert!(!doc1_content.contains("](./ "),
        "Link rewriting produced invalid markdown with extra space after './' - Bug not fixed!");

    // Verify correct link format
    assert!(doc1_content.contains("](./test-doc2.md)"),
        "Expected rewritten link to doc2 not found");
    assert!(doc1_content.contains("](./test-doc3.md)"),
        "Expected rewritten link to doc3 not found");

    // Verify links are valid markdown (no spaces in URL portion)
    for line in doc1_content.lines() {
        if line.contains("](./") {
            // Extract the link portion
            if let Some(start) = line.find("](./") {
                let after = &line[start + 2..]; // Skip "]("
                if let Some(end) = after.find(')') {
                    let url_portion = &after[..end];
                    // Verify no spaces in the URL portion
                    assert!(!url_portion.contains(' '),
                        "Invalid markdown link with space in URL: {}", url_portion);
                }
            }
        }
    }
}

#[test]
fn test_external_links_preserved() {
    let temp_dir = setup_test_files();
    let base_path = temp_dir.path();
    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let (files, _manifest) = discover_files(base_path).expect("Failed to discover files");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze files");

    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");
    assert_eq!(result.error_count, 0);

    let doc3_content = fs::read_to_string(output_dir.join("docs/test-doc3.md"))
        .expect("Failed to read transformed doc3.md");

    // External links should remain unchanged
    assert!(doc3_content.contains("https://example.com"), "External HTTP link was modified");
    assert!(doc3_content.contains("mailto:test@example.com"), "Mailto link was modified");
    assert!(doc3_content.contains("](#heading)"), "Anchor link was modified");
}

#[test]
fn test_multiple_links_in_single_line() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Document with multiple links on one line
    fs::write(
        base_path.join("multi.md"),
        "# Multi Links\nCheck [doc1](./doc1.md) and [doc2](./doc2.md) and [doc3](./doc3.md) together.\n",
    )
    .expect("Failed to write multi.md");

    fs::write(base_path.join("doc1.md"), "# Doc1\n").expect("Failed to write doc1.md");
    fs::write(base_path.join("doc2.md"), "# Doc2\n").expect("Failed to write doc2.md");
    fs::write(base_path.join("doc3.md"), "# Doc3\n").expect("Failed to write doc3.md");

    let (files, _manifest) = discover_files(base_path).expect("Failed to discover");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze");

    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");
    assert_eq!(result.error_count, 0);

    let multi_content = fs::read_to_string(output_dir.join("docs/test-multi.md"))
        .expect("Failed to read transformed multi.md");

    // ALL links should be rewritten correctly without spaces
    assert!(!multi_content.contains("](./ "), "Found invalid link with extra space");
    assert!(multi_content.contains("](./test-doc1.md)"), "Link 1 not rewritten correctly");
    assert!(multi_content.contains("](./test-doc2.md)"), "Link 2 not rewritten correctly");
    assert!(multi_content.contains("](./test-doc3.md)"), "Link 3 not rewritten correctly");
}

#[test]
fn test_transform_adds_frontmatter() {
    let temp_dir = setup_test_files();
    let base_path = temp_dir.path();
    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let (files, _manifest) = discover_files(base_path).expect("Failed to discover files");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze files");

    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");
    assert_eq!(result.error_count, 0);

    let doc1_content = fs::read_to_string(output_dir.join("docs/test-doc1.md"))
        .expect("Failed to read transformed doc1.md");

    // Should have frontmatter
    assert!(doc1_content.starts_with("---"), "Missing frontmatter start");
    assert!(doc1_content.contains("id:"), "Missing id in frontmatter");
    assert!(doc1_content.contains("title:"), "Missing title in frontmatter");
    assert!(doc1_content.contains("category:"), "Missing category in frontmatter");
}

#[test]
fn test_transform_adds_context_block() {
    let temp_dir = setup_test_files();
    let base_path = temp_dir.path();
    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let (files, _manifest) = discover_files(base_path).expect("Failed to discover files");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze files");

    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");
    assert_eq!(result.error_count, 0);

    let doc1_content = fs::read_to_string(output_dir.join("docs/test-doc1.md"))
        .expect("Failed to read transformed doc1.md");

    // Should have context block
    assert!(doc1_content.contains("> **Context**:"), "Missing context block");
}

#[test]
fn test_transform_adds_see_also() {
    let temp_dir = setup_test_files();
    let base_path = temp_dir.path();
    let output_dir = base_path.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let (files, _manifest) = discover_files(base_path).expect("Failed to discover files");
    let analyses = analyze_files(&files, base_path).expect("Failed to analyze files");

    let mut link_map = HashMap::new();
    for analysis in &analyses {
        let filename = analysis.source_path.split('/').last().unwrap_or("unknown");
        let mapping = IdMapping {
            id: format!("test-{}", filename.replace(".md", "")),
            filename: format!("test-{}", filename),
            subcategory: "general".to_string(),
            slug: filename.replace(".md", ""),
        };
        link_map.insert(analysis.source_path.clone(), mapping);
    }

    let result = transform_all(&analyses, &link_map, &output_dir).expect("Transform failed");
    assert_eq!(result.error_count, 0);

    let doc1_content = fs::read_to_string(output_dir.join("docs/test-doc1.md"))
        .expect("Failed to read transformed doc1.md");

    // Should have See Also section
    assert!(doc1_content.contains("## See Also"), "Missing See Also section");
    assert!(doc1_content.contains("COMPASS.md"), "Missing link to COMPASS");
}
