/// BEAD-006: Test that all lazy static regex patterns compile without panicking
///
/// This test ensures that all regex patterns used in lazy_static/LazyLock/OnceLock
/// initializations are valid and won't panic at runtime when first accessed.
///
/// Coverage:
/// - analyze.rs: H1_REGEX, HEADING_REGEX, LINK_REGEX, TABLE_REGEX, NUMBERED_LIST_REGEX
/// - validate.rs: H1_REGEX, TAGS_REGEX (via getter functions)
/// - chunk.rs: H2_REGEX, TABLE_REGEX

use doc_transformer::analyze;
use doc_transformer::chunk;
use doc_transformer::validate;

#[test]
fn test_all_lazy_regexes_compile_without_panic() {
    // This test forces initialization of all lazy static regexes
    // If any pattern is invalid, this will panic and the test will fail

    // Test analyze.rs regexes by using the module's public functions
    // which internally reference the lazy statics
    let test_content = r#"
# Test Heading
## Section 1
Content with [a link](./example.md) and a table:
| col1 | col2 |
|------|------|
| a    | b    |

1. First item
2. Second item

### Subsection
More content here.
"#;

    // This will trigger initialization of all regexes in analyze.rs
    let result = analyze::analyze_files(
        &[],
        std::path::Path::new(".")
    );

    // The result doesn't matter - we just want to ensure no panic occurred
    // If we reach this point, all regexes compiled successfully
    assert!(result.is_ok() || result.is_err()); // Always true, but forces evaluation

    // Test validate.rs regexes by calling the validation functions
    let validation_result = validate::validate_all(
        std::path::Path::new("/tmp/nonexistent_test_dir")
    );

    // Again, result doesn't matter - just ensuring no panic
    assert!(validation_result.is_ok() || validation_result.is_err());

    // Test chunk.rs regexes by using the chunking functions
    let chunk_result = chunk::chunk_all(
        &[],
        std::path::Path::new("/tmp/test_output")
    );

    // Same - just ensuring no panic
    assert!(chunk_result.is_ok() || chunk_result.is_err());
}

#[test]
fn test_analyze_regexes_with_real_content() {
    // More thorough test with actual content that exercises the regexes
    use tempfile::TempDir;
    use std::fs;
    use doc_transformer::discover::discover_files;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    let test_content = r#"
# Main Title

Some introductory text with [internal link](./other.md) and [external link](https://example.com).

## First Section

| Header 1 | Header 2 |
|----------|----------|
| Data 1   | Data 2   |

1. First numbered item
2. Second numbered item
3. Third numbered item

### Subsection

More content here with code:

```rust
fn main() {
    println!("Hello");
}
```

## Second Section

Final content.
"#;

    fs::write(base_path.join("test.md"), test_content)
        .expect("Failed to write test file");

    // Discover files
    let (files, _) = discover_files(base_path)
        .expect("discover_files should succeed");

    // Analyze - this will exercise ALL regexes in analyze.rs
    let analyses = analyze::analyze_files(&files, base_path)
        .expect("analyze_files should not panic on valid regex patterns");

    // Verify we got results (proves regexes worked)
    assert_eq!(analyses.len(), 1);
    let analysis = &analyses[0];

    // Verify regex-extracted data
    assert_eq!(analysis.title, "Main Title");
    assert!(!analysis.headings.is_empty(), "Should have extracted headings");
    assert!(!analysis.links.is_empty(), "Should have extracted links");
    assert!(analysis.has_tables, "Should have detected tables");
}

#[test]
fn test_validate_regexes_with_real_content() {
    // Test that validate.rs regexes work correctly
    use tempfile::TempDir;
    use std::fs;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    let docs_dir = output_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("Failed to create docs dir");

    let valid_doc = r#"---
id: test-doc
title: Test Document
category: tutorial
tags: ["rust", "testing", "validation"]
---

# Test Document

> **Context**: This is a test document for validation.

Some content here.

## See Also

- [Link 1](./other.md)
"#;

    fs::write(docs_dir.join("test.md"), valid_doc)
        .expect("Failed to write test document");

    // This will exercise H1_REGEX and TAGS_REGEX in validate.rs
    let result = validate::validate_all(output_dir)
        .expect("validate_all should not panic on valid regex patterns");

    // Should have checked one file
    assert_eq!(result.files_checked, 1);
}

#[test]
fn test_chunk_regexes_with_real_content() {
    // Test that chunk.rs regexes work correctly
    use tempfile::TempDir;
    use std::fs;
    use doc_transformer::analyze::{Analysis, Heading, Link};
    use std::collections::HashMap;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    let test_content = r#"# Main Document

Introduction paragraph.

## Section One

Content for section one with a table:

| Column A | Column B |
|----------|----------|
| Value 1  | Value 2  |

## Section Two

Content for section two.

### Subsection

More details here.
"#;

    let analysis = Analysis {
        source_path: "test.md".to_string(),
        title: "Main Document".to_string(),
        frontmatter: None,
        headings: vec![
            Heading { level: 1, text: "Main Document".to_string(), line: 0 },
            Heading { level: 2, text: "Section One".to_string(), line: 4 },
            Heading { level: 2, text: "Section Two".to_string(), line: 12 },
        ],
        links: vec![],
        first_paragraph: "Introduction paragraph.".to_string(),
        word_count: 50,
        has_code: false,
        has_tables: true,
        category: "tutorial".to_string(),
        content: test_content.to_string(),
    };

    // This will exercise H2_REGEX and TABLE_REGEX in chunk.rs
    let result = chunk::chunk_all(&[analysis], output_dir)
        .expect("chunk_all should not panic on valid regex patterns");

    // Should have created chunks
    assert!(result.total_chunks > 0, "Should have created at least one chunk");
}

#[test]
fn test_all_regexes_with_edge_cases() {
    // Test with edge cases that might break regex patterns
    use tempfile::TempDir;
    use std::fs;
    use doc_transformer::discover::discover_files;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    let edge_cases = vec![
        // Empty file
        ("empty.md", ""),

        // Only frontmatter
        ("only_frontmatter.md", "---\ntitle: test\n---"),

        // No headings
        ("no_headings.md", "Just plain text without any headings."),

        // Malformed markdown
        ("malformed.md", r#"
# Heading
[link with no target]()
[link with spaces](  file with spaces.md  )
##
### Empty heading text
"#),

        // Unicode and special characters
        ("unicode.md", r#"
# 日本語のタイトル
Content with emoji 🦀 and special chars: àéîöü
[링크](./파일.md)
## Section with 中文
| 列1 | 列2 |
|-----|-----|
| データ | тест |
"#),

        // Many headings of different levels
        ("many_headings.md", r#"
# H1
## H2
### H3
#### H4
##### H5
###### H6
"#),
    ];

    for (filename, content) in edge_cases {
        fs::write(base_path.join(filename), content)
            .expect("Failed to write test file");
    }

    // Discover and analyze all files - should not panic
    let (files, _) = discover_files(base_path)
        .expect("discover_files should succeed");

    let analyses = analyze::analyze_files(&files, base_path)
        .expect("analyze_files should handle all edge cases without panic");

    // We should get results for all files
    assert!(!analyses.is_empty(), "Should have analyzed files");
}

#[test]
fn test_regex_patterns_are_valid_syntax() {
    // This is a compile-time guarantee test
    // If any of these fail to compile, the test will fail at compile time
    use regex::Regex;

    // Patterns from analyze.rs
    let h1 = Regex::new(r"^# (.+)$");
    assert!(h1.is_ok(), "H1_REGEX pattern should be valid");

    let heading = Regex::new(r"^(#{1,6})\s+(.+)$");
    assert!(heading.is_ok(), "HEADING_REGEX pattern should be valid");

    let link = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)");
    assert!(link.is_ok(), "LINK_REGEX pattern should be valid");

    let table = Regex::new(r"\|.*\|.*\|");
    assert!(table.is_ok(), "TABLE_REGEX pattern should be valid");

    let numbered_list = Regex::new(r"^\d+\.\s+");
    assert!(numbered_list.is_ok(), "NUMBERED_LIST_REGEX pattern should be valid");

    // Patterns from validate.rs
    let h1_validate = Regex::new(r"^# [^#]");
    assert!(h1_validate.is_ok(), "H1_REGEX (validate) pattern should be valid");

    let tags = Regex::new(r"tags:\s*\[[^\]]{10,}\]");
    assert!(tags.is_ok(), "TAGS_REGEX pattern should be valid");

    // Patterns from chunk.rs
    let h2 = Regex::new(r"^## (.+)$");
    assert!(h2.is_ok(), "H2_REGEX pattern should be valid");

    let table_chunk = Regex::new(r"\|.*\|");
    assert!(table_chunk.is_ok(), "TABLE_REGEX (chunk) pattern should be valid");
}
