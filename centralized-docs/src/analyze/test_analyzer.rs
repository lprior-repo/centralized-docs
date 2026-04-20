use std::sync::Arc;

use super::analyzer::analyze_files;
use super::category::{analyze_single_file, generate_untitled_id};
use super::types::*;
use crate::discover::DiscoveryFile;

#[test]
fn test_generate_untitled_id_deterministic() {
    let id1 = generate_untitled_id("path/to/file.md", "content");
    let id2 = generate_untitled_id("path/to/file.md", "content");
    assert_eq!(id1, id2, "Same input should produce same hash");
}

#[test]
fn test_generate_untitled_id_different_paths() {
    let id1 = generate_untitled_id("path/a.md", "content");
    let id2 = generate_untitled_id("path/b.md", "content");
    assert_ne!(id1, id2, "Different paths should produce different hashes");
}

#[test]
fn test_generate_untitled_id_starts_with_untitled() {
    let id = generate_untitled_id("test.md", "content");
    assert!(id.starts_with("Untitled-"));
}

#[test]
fn test_analyze_single_file_basic() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let file_path = dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "# Test Document\n\nThis is a paragraph.\n\n## Section\n\nMore text.",
    )?;

    let analysis = analyze_single_file("test.md", &file_path, None)?;

    assert_eq!(analysis.title, "Test Document");
    assert_eq!(analysis.headings.len(), 2);
    assert_eq!(analysis.word_count, 11);
    assert_eq!(analysis.source_path, "test.md");
    assert!(!analysis.first_paragraph.is_empty());
    Ok(())
}

#[test]
fn test_analyze_single_file_with_frontmatter() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let file_path = dir.path().join("test.md");
    std::fs::write(
        &file_path,
        "---\ntitle: Custom Title\ncategory: tutorial\n---\n\n# Custom Title\n\nContent without heading.",
    )?;

    let analysis = analyze_single_file("test.md", &file_path, None)?;

    assert_eq!(analysis.title, "Custom Title");
    assert!(analysis.frontmatter.is_some());
    let fm = analysis.frontmatter.unwrap();
    assert_eq!(fm.get("title").unwrap(), "Custom Title");
    assert_eq!(fm.get("category").unwrap(), "tutorial");
    Ok(())
}

#[test]
fn test_analyze_single_file_title_from_filename() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let file_path = dir.path().join("my-cool-guide.md");
    std::fs::write(&file_path, "No heading here, just content.")?;

    let analysis = analyze_single_file("my-cool-guide.md", &file_path, None)?;

    assert_eq!(analysis.title, "My Cool Guide");
    Ok(())
}

#[test]
fn test_analyze_single_file_missing_file() {
    let result = analyze_single_file(
        "missing.md",
        std::path::Path::new("/nonexistent/path.md"),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn test_analyze_single_file_code_detection() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, "# Doc\n\n```\ncode block\n```\n\nMore text.")?;

    let analysis = analyze_single_file("test.md", &file_path, None)?;

    assert!(analysis.has_code);
    Ok(())
}

#[test]
fn test_analyze_files_empty() -> anyhow::Result<()> {
    let result = analyze_files(&[], std::path::Path::new("/tmp"), None)?;
    assert!(result.analyses.is_empty());
    assert!(result.failed_files.is_empty());
    assert_eq!(result.total_discovered, 0);
    Ok(())
}

#[test]
fn test_analyze_files_basic() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let file_path = dir.path().join("doc.md");
    std::fs::write(&file_path, "# My Document\n\nContent paragraph.")?;

    let files = vec![DiscoveryFile {
        source_path: "doc.md".to_string(),
        size_bytes: 10,
    }];

    let result = analyze_files(&files, dir.path(), None)?;

    assert_eq!(result.analyses.len(), 1);
    assert_eq!(result.analyses[0].title, "My Document");
    assert_eq!(result.total_discovered, 1);
    Ok(())
}

#[test]
fn test_analyze_files_with_failed_file() {
    let files = vec![DiscoveryFile {
        source_path: "nonexistent.md".to_string(),
        size_bytes: 10,
    }];

    let result = analyze_files(&files, std::path::Path::new("/nonexistent"), None);

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Failed to analyze any"));
}

#[test]
fn test_analyze_files_mixed_success_failure() -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let good_file = dir.path().join("good.md");
    std::fs::write(&good_file, "# Good Doc\n\nContent.")?;

    let files = vec![
        DiscoveryFile {
            source_path: "good.md".to_string(),
            size_bytes: 10,
        },
        DiscoveryFile {
            source_path: "bad.md".to_string(),
            size_bytes: 10,
        },
    ];

    let result = analyze_files(&files, dir.path(), None)?;

    assert_eq!(result.analyses.len(), 1);
    assert_eq!(result.failed_files.len(), 1);
    assert_eq!(result.total_discovered, 2);
    Ok(())
}

#[test]
fn test_analyze_result_len_and_empty() {
    let empty = AnalyzeResult {
        analyses: vec![],
        failed_files: vec![],
        total_discovered: 0,
    };
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);

    let nonempty = AnalyzeResult {
        analyses: vec![Analysis {
            source_path: "a.md".to_string(),
            title: "A".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: String::new(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "c".to_string(),
            content: Arc::from(""),
        }],
        failed_files: vec![],
        total_discovered: 1,
    };
    assert!(!nonempty.is_empty());
    assert_eq!(nonempty.len(), 1);
}

#[test]
fn test_analyze_result_deref() {
    let result = AnalyzeResult {
        analyses: vec![
            Analysis {
                source_path: "a.md".to_string(),
                title: "A".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "c".to_string(),
                content: Arc::from(""),
            },
            Analysis {
                source_path: "b.md".to_string(),
                title: "B".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "c".to_string(),
                content: Arc::from(""),
            },
        ],
        failed_files: vec![],
        total_discovered: 2,
    };

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].title, "A");
    assert_eq!(result[1].title, "B");
}

#[test]
fn test_count_categories() {
    let analyses = vec![
        Analysis {
            source_path: "a.md".to_string(),
            title: "A".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: String::new(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: Arc::from(""),
        },
        Analysis {
            source_path: "b.md".to_string(),
            title: "B".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: String::new(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: Arc::from(""),
        },
        Analysis {
            source_path: "c.md".to_string(),
            title: "C".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: String::new(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "ref".to_string(),
            content: Arc::from(""),
        },
    ];

    let counts = count_categories(&analyses);
    assert_eq!(counts.get("tutorial").unwrap(), &2);
    assert_eq!(counts.get("ref").unwrap(), &1);
    assert_eq!(counts.len(), 2);
}

#[test]
fn test_count_categories_empty() {
    let counts = count_categories(&[]);
    assert!(counts.is_empty());
}
