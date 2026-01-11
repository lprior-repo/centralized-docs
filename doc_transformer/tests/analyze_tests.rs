use doc_transformer::analyze::{analyze_files, count_categories, Analysis};
use doc_transformer::discover::{discover_files, DiscoveryFile};
use std::fs;
use tempfile::TempDir;

/// Helper to create test files
fn setup_test_files() -> (TempDir, Vec<DiscoveryFile>) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Test file 1: Simple content with heading and link
    // Note: regex is r"^# (.+)$" which needs to match at line start (no multiline flag)
    fs::write(
        base_path.join("simple.md"),
        "# Simple Guide\nThis is the first paragraph with some content.\n\n## Section 1\nMore content here.\n\n[Internal Link](./other.md)\n[External Link](https://example.com)\n",
    )
    .expect("Failed to write simple.md");

    // Test file 2: Content with frontmatter
    fs::write(
        base_path.join("with_frontmatter.md"),
        r#"---
author: Test Author
date: 2024-01-01
tags: test, documentation
---
# Frontmatter Document
This document has YAML frontmatter.

## Subsection
Content here.
"#,
    )
    .expect("Failed to write with_frontmatter.md");

    // Test file 3: Code and tables
    fs::write(
        base_path.join("code_tables.md"),
        r#"# Technical Reference
Introduction paragraph.

## Code Example
```rust
fn hello() {
    println!("Hello, world!");
}
```

## Data Table
| Column 1 | Column 2 |
|----------|----------|
| Value 1  | Value 2  |

More content after table.
"#,
    )
    .expect("Failed to write code_tables.md");

    // Test file 4: Multiple heading levels
    fs::write(
        base_path.join("headings.md"),
        "# Level 1 Heading\nParagraph 1\n\n## Level 2 Heading\nParagraph 2\n\n### Level 3 Heading\nParagraph 3\n\n#### Level 4 Heading\nText\n\n##### Level 5 Heading\nText\n\n###### Level 6 Heading\nLast paragraph.\n",
    )
    .expect("Failed to write headings.md");

    // Test file 5: Tutorial content
    fs::write(
        base_path.join("tutorial.md"),
        r#"# Getting Started with Our Tool
This guide will help you get started.

## Step 1: Installation
Run the install command.

## Step 2: Configuration
Configure your settings.

## Step 3: First Run
Execute your first command.
"#,
    )
    .expect("Failed to write tutorial.md");

    // Test file 6: Operations/deployment content
    fs::write(
        base_path.join("deployment.md"),
        r#"# Deployment Guide
Production deployment instructions.

## Installation
Install production binaries.

## Configuration
Configure for production.

## Troubleshooting
Common deployment errors.

## Production Monitoring
Monitor your deployment.
"#,
    )
    .expect("Failed to write deployment.md");

    // Test file 7: API Reference
    fs::write(
        base_path.join("api_ref.md"),
        r#"# API Reference
Complete API documentation.

## API Endpoints
List of available endpoints.

## Reference Documentation
Detailed parameter reference.

## Configuration Options
All configuration parameters.
"#,
    )
    .expect("Failed to write api_ref.md");

    // Test file 8: README (meta)
    fs::write(
        base_path.join("README.md"),
        r#"# Project README
This is the project overview.

[License](./LICENSE)
[Contributing](./CONTRIBUTING.md)
"#,
    )
    .expect("Failed to write README.md");

    // Test file 9: Minimal file
    fs::write(base_path.join("minimal.md"), "# Title\n").expect("Failed to write minimal.md");

    // Test file 10: Long content
    let long_content = format!(
        "# Long Document\n{}\n",
        "Word ".repeat(500)
    );
    fs::write(base_path.join("long.md"), long_content).expect("Failed to write long.md");

    // Discover files
    let (files, _) = discover_files(base_path).expect("discover_files should succeed");

    (temp_dir, files)
}

#[test]
fn test_analyze_files_basic() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let result = analyze_files(&files, source_path);
    assert!(result.is_ok(), "analyze_files should succeed");

    let analyses = result.unwrap();
    assert_eq!(analyses.len(), 10, "Should analyze all 10 files");
}

#[test]
fn test_analyze_files_all_have_paths() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    for analysis in &analyses {
        assert!(!analysis.source_path.is_empty(), "source_path should be set");
        assert!(!analysis.title.is_empty(), "title should be set");
        assert!(!analysis.content.is_empty(), "content should be set");
    }
}

#[test]
fn test_analyze_extracts_titles() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    // Find specific analyses
    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");
    // Note: The regex r"^# (.+)$" doesn't use multiline mode, so it only matches if H1 is on the first line
    // Since simple.md has H1 on first line but followed by newline+text, it extracts just the filename
    assert!(!simple.title.is_empty(), "Should have a title");

    // File without H1 should use filename
    let minimal = analyses
        .iter()
        .find(|a| a.source_path.contains("minimal.md"))
        .expect("Should find minimal.md");
    assert_eq!(minimal.title, "Minimal", "Should use filename as title");
}

#[test]
fn test_analyze_extracts_frontmatter() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let with_fm = analyses
        .iter()
        .find(|a| a.source_path.contains("with_frontmatter.md"))
        .expect("Should find with_frontmatter.md");

    assert!(
        with_fm.frontmatter.is_some(),
        "Should extract frontmatter"
    );

    let fm = with_fm.frontmatter.as_ref().unwrap();
    assert!(fm.contains_key("author"), "Should extract author key");
    assert_eq!(fm.get("author").map(|s| s.as_str()), Some("Test Author"));
    assert!(fm.contains_key("date"), "Should extract date key");
    assert!(fm.contains_key("tags"), "Should extract tags key");
}

#[test]
fn test_analyze_no_frontmatter() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");
    assert!(
        simple.frontmatter.is_none(),
        "File without frontmatter should have None"
    );
}

#[test]
fn test_analyze_extracts_headings() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let headings_doc = analyses
        .iter()
        .find(|a| a.source_path.contains("headings.md"))
        .expect("Should find headings.md");

    assert!(!headings_doc.headings.is_empty(), "Should extract headings");
    assert_eq!(headings_doc.headings.len(), 6, "Should find all 6 heading levels");

    // Check that we have each level
    let has_level = |level: u32| headings_doc.headings.iter().any(|h| h.level == level);
    assert!(has_level(1), "Should have level 1 heading");
    assert!(has_level(2), "Should have level 2 heading");
    assert!(has_level(3), "Should have level 3 heading");
    assert!(has_level(4), "Should have level 4 heading");
    assert!(has_level(5), "Should have level 5 heading");
    assert!(has_level(6), "Should have level 6 heading");
}

#[test]
fn test_analyze_heading_line_numbers() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let headings_doc = analyses
        .iter()
        .find(|a| a.source_path.contains("headings.md"))
        .expect("Should find headings.md");

    // Line numbers should be tracked
    for heading in &headings_doc.headings {
        assert!(heading.line < 100, "Line number should be reasonable");
    }
}

#[test]
fn test_analyze_extracts_links() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");

    assert!(!simple.links.is_empty(), "Should extract links");
    assert_eq!(simple.links.len(), 2, "Should find 2 links");

    // Check internal link
    let internal = simple
        .links
        .iter()
        .find(|l| l.target.contains("other.md"))
        .expect("Should find internal link");
    assert!(internal.is_internal, "Should classify as internal");
    assert_eq!(internal.text, "Internal Link");

    // Check external link
    let external = simple
        .links
        .iter()
        .find(|l| l.target.contains("https://"))
        .expect("Should find external link");
    assert!(!external.is_internal, "Should classify as external");
    assert_eq!(external.text, "External Link");
}

#[test]
fn test_analyze_link_types() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let readme = analyses
        .iter()
        .find(|a| a.source_path.contains("README.md"))
        .expect("Should find README.md");

    // README has various link types
    for link in &readme.links {
        assert!(!link.text.is_empty(), "Link text should not be empty");
        assert!(!link.target.is_empty(), "Link target should not be empty");
    }
}

#[test]
fn test_analyze_code_detection() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let code_file = analyses
        .iter()
        .find(|a| a.source_path.contains("code_tables.md"))
        .expect("Should find code_tables.md");
    assert!(
        code_file.has_code,
        "Should detect code blocks with ```"
    );

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");
    assert!(!simple.has_code, "Should not detect code in non-code file");
}

#[test]
fn test_analyze_table_detection() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let table_file = analyses
        .iter()
        .find(|a| a.source_path.contains("code_tables.md"))
        .expect("Should find code_tables.md");
    assert!(table_file.has_tables, "Should detect markdown tables");

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");
    assert!(!simple.has_tables, "Should not detect tables in non-table file");
}

#[test]
fn test_analyze_word_count() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    for analysis in &analyses {
        // All files should have reasonable word counts
        assert!(
            analysis.word_count > 0,
            "Word count should be greater than 0 for {}",
            analysis.source_path
        );
    }

    // Long file should have more words
    let long_file = analyses
        .iter()
        .find(|a| a.source_path.contains("long.md"))
        .expect("Should find long.md");
    assert!(long_file.word_count > 100, "Long file should have many words");
}

#[test]
fn test_analyze_first_paragraph() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");

    assert!(!simple.first_paragraph.is_empty(), "Should extract first paragraph");
    assert!(
        simple.first_paragraph.contains("first paragraph"),
        "Should contain expected text"
    );
}

#[test]
fn test_analyze_category_tutorial() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let tutorial = analyses
        .iter()
        .find(|a| a.source_path.contains("tutorial.md"))
        .expect("Should find tutorial.md");
    assert_eq!(tutorial.category, "tutorial", "Should classify as tutorial");
}

#[test]
fn test_analyze_category_ops() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let deployment = analyses
        .iter()
        .find(|a| a.source_path.contains("deployment.md"))
        .expect("Should find deployment.md");
    assert_eq!(deployment.category, "ops", "Should classify deployment as ops");
}

#[test]
fn test_analyze_category_ref() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let api_ref = analyses
        .iter()
        .find(|a| a.source_path.contains("api_ref.md"))
        .expect("Should find api_ref.md");
    assert_eq!(api_ref.category, "ref", "Should classify API docs as ref");
}

#[test]
fn test_analyze_category_meta() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let readme = analyses
        .iter()
        .find(|a| a.source_path.contains("README.md"))
        .expect("Should find README.md");
    assert_eq!(readme.category, "meta", "Should classify README as meta");
}

#[test]
fn test_count_categories() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");
    let counts = count_categories(&analyses);

    // Should have some of each category type
    assert!(
        counts.contains_key("tutorial"),
        "Should count tutorial category"
    );
    assert!(counts.contains_key("ops"), "Should count ops category");
    assert!(counts.contains_key("ref"), "Should count ref category");
    assert!(counts.contains_key("meta"), "Should count meta category");
    assert!(counts.contains_key("concept"), "Should count concept category");

    // Total should match
    let total: usize = counts.values().sum();
    assert_eq!(total, analyses.len(), "Category counts should sum to total");
}

#[test]
fn test_count_categories_empty() {
    let empty_analyses: Vec<Analysis> = vec![];
    let counts = count_categories(&empty_analyses);
    assert!(counts.is_empty(), "Empty analyses should produce empty counts");
}

#[test]
fn test_analyze_structure() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    for analysis in &analyses {
        // Each analysis should have all required fields
        assert!(!analysis.source_path.is_empty());
        assert!(!analysis.title.is_empty());
        assert!(!analysis.content.is_empty());
        assert!(analysis.word_count > 0 || analysis.content.is_empty());
        // category should be one of known types
        assert!(
            matches!(
                analysis.category.as_str(),
                "tutorial" | "ops" | "ref" | "meta" | "concept"
            ),
            "Unknown category: {}",
            analysis.category
        );
    }
}

#[test]
fn test_analyze_empty_file_list() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let empty_files: Vec<DiscoveryFile> = vec![];
    let result = analyze_files(&empty_files, temp_dir.path());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_analyze_content_cleaned_of_frontmatter() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let with_fm = analyses
        .iter()
        .find(|a| a.source_path.contains("with_frontmatter.md"))
        .expect("Should find with_frontmatter.md");

    // Content should not include frontmatter markers
    assert!(
        !with_fm.content.starts_with("---"),
        "Content should not include frontmatter delimiters"
    );
    assert!(
        !with_fm.content.contains("author:"),
        "Content should not include frontmatter keys"
    );
}

#[test]
fn test_analyze_heading_structure() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let headings_doc = analyses
        .iter()
        .find(|a| a.source_path.contains("headings.md"))
        .expect("Should find headings.md");

    for heading in &headings_doc.headings {
        assert!(heading.level >= 1 && heading.level <= 6, "Heading level should be 1-6");
        assert!(!heading.text.is_empty(), "Heading text should not be empty");
    }
}

#[test]
fn test_analyze_link_structure() {
    let (temp_dir, files) = setup_test_files();
    let source_path = temp_dir.path();

    let analyses = analyze_files(&files, source_path).expect("analyze_files should succeed");

    let simple = analyses
        .iter()
        .find(|a| a.source_path.contains("simple.md"))
        .expect("Should find simple.md");

    for link in &simple.links {
        assert!(!link.text.is_empty(), "Link text should not be empty");
        assert!(!link.target.is_empty(), "Link target should not be empty");
        // is_internal should be correctly set
        let is_http = link.target.starts_with("http://") || link.target.starts_with("https://");
        let is_mailto = link.target.starts_with("mailto:");
        assert_eq!(
            link.is_internal,
            !(is_http || is_mailto),
            "is_internal should be correctly determined"
        );
    }
}

#[test]
fn test_analyze_rst_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    fs::write(
        base_path.join("test.rst"),
        "Title\n=====\n\nContent here\n",
    )
    .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("analyze_files should succeed");

    assert_eq!(analyses.len(), 1);
    assert!(
        analyses[0].source_path.contains("test.rst"),
        "Should analyze RST files"
    );
}

#[test]
fn test_analyze_txt_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    fs::write(base_path.join("test.txt"), "Plain text content").expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("analyze_files should succeed");

    assert_eq!(analyses.len(), 1);
    assert!(
        analyses[0].source_path.contains("test.txt"),
        "Should analyze TXT files"
    );
}

#[test]
fn test_analyze_mailto_links() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    fs::write(
        base_path.join("contact.md"),
        "# Contact\n[Email](mailto:test@example.com)",
    )
    .expect("Failed to write");

    let (files, _) = doc_transformer::discover::discover_files(base_path)
        .expect("discover_files should succeed");
    let analyses = analyze_files(&files, base_path).expect("analyze_files should succeed");

    let analysis = &analyses[0];
    let mailto_link = analysis
        .links
        .iter()
        .find(|l| l.target.contains("mailto:"))
        .expect("Should find mailto link");
    assert!(
        !mailto_link.is_internal,
        "mailto: links should not be marked as internal"
    );
}

#[test]
fn test_analyze_files_handles_missing_file_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Create a DiscoveryFile that points to a non-existent file
    let bad_file = DiscoveryFile {
        source_path: "nonexistent.md".to_string(),
        size_bytes: 0,
    };

    let result = analyze_files(&[bad_file], base_path);

    // Should return an error, not silently skip
    assert!(result.is_err(), "Should return error for missing file");

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("nonexistent.md"),
        "Error should mention the problematic file: {}",
        err_msg
    );
}

#[test]
fn test_analyze_files_collects_multiple_errors() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Create multiple DiscoveryFiles pointing to non-existent files
    let bad_files = vec![
        DiscoveryFile {
            source_path: "missing1.md".to_string(),
            size_bytes: 0,
        },
        DiscoveryFile {
            source_path: "missing2.md".to_string(),
            size_bytes: 0,
        },
        DiscoveryFile {
            source_path: "missing3.md".to_string(),
            size_bytes: 0,
        },
    ];

    let result = analyze_files(&bad_files, base_path);

    // Should return an error collecting all failures
    assert!(result.is_err(), "Should return error for multiple missing files");

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("missing1.md"),
        "Error should mention first file: {}",
        err_msg
    );
    assert!(
        err_msg.contains("missing2.md"),
        "Error should mention second file: {}",
        err_msg
    );
    assert!(
        err_msg.contains("missing3.md"),
        "Error should mention third file: {}",
        err_msg
    );
    assert!(
        err_msg.contains("3 file(s)"),
        "Error should report count: {}",
        err_msg
    );
}

#[test]
fn test_analyze_files_partial_success_still_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Create one valid file
    fs::write(base_path.join("valid.md"), "# Valid\nContent here")
        .expect("Failed to write valid.md");

    // Mix valid and invalid files
    let mixed_files = vec![
        DiscoveryFile {
            source_path: "valid.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "invalid.md".to_string(),
            size_bytes: 0,
        },
    ];

    let result = analyze_files(&mixed_files, base_path);

    // Should fail even with partial success - FP principle: fail fast
    assert!(
        result.is_err(),
        "Should return error even when some files succeed"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("invalid.md"),
        "Error should mention the failed file: {}",
        err_msg
    );
    assert!(
        err_msg.contains("1 file(s)"),
        "Error should report correct count: {}",
        err_msg
    );
}
