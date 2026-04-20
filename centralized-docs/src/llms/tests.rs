//! Integration tests for llms module.
//!
//! Tests `generate_llms_txt`, `generate_agents_md`, and `LlmsConfig`.

use std::sync::Arc;

use crate::analyze::Analysis;
use crate::assign::IdMapping;

use super::config::LlmsConfig;
use super::generate_agents::generate_agents_md;
use super::generate_llms::generate_llms_txt;

fn make_analysis(source_path: &str, category: &str, title: &str) -> Analysis {
    Analysis {
        source_path: source_path.to_string(),
        title: title.to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "A short summary for testing".to_string(),
        word_count: 50,
        has_code: false,
        has_tables: false,
        category: category.to_string(),
        content: Arc::from("Some content here"),
    }
}

fn make_link_map(pairs: &[(&str, &str)]) -> std::collections::HashMap<String, IdMapping> {
    pairs
        .iter()
        .map(|(path, filename)| {
            let mapping = IdMapping {
                id: path.to_string(),
                filename: filename.to_string(),
                subcategory: "test".to_string(),
                slug: "test".to_string(),
            };
            (path.to_string(), mapping)
        })
        .collect()
}

#[test]
fn test_llms_config_default() {
    let config = LlmsConfig::default();
    assert_eq!(config.project_name, "Documentation");
    assert_eq!(config.max_per_category, 5);
    assert!(config.include_frontmatter);
    assert_eq!(config.spec_version, "1.0");
    assert_eq!(config.project_version, "0.1.0");
}

#[test]
fn test_generate_llms_txt_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let analyses = vec![
        make_analysis(
            "docs/getting-started/intro.md",
            "tutorial",
            "Getting Started",
        ),
        make_analysis("docs/api/ref.md", "ref", "API Reference"),
    ];
    let link_map = make_link_map(&[
        (
            "docs/getting-started/intro.md",
            "tutorial-getting-started-intro.md",
        ),
        ("docs/api/ref.md", "ref-api-ref.md"),
    ]);
    let config = LlmsConfig::default();

    generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

    let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
    assert!(content.contains("# Documentation"));
    assert!(content.contains("Getting Started"));
    assert!(content.contains("API Reference"));
    assert!(content.contains("docs/tutorial-getting-started-intro.md"));
}

#[test]
fn test_generate_llms_txt_no_frontmatter() {
    let dir = tempfile::TempDir::new().unwrap();
    let analyses = vec![make_analysis("a.md", "tutorial", "Test")];
    let link_map = make_link_map(&[("a.md", "a.md")]);
    let config = LlmsConfig {
        include_frontmatter: false,
        ..Default::default()
    };

    generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

    let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
    assert!(!content.contains("---"));
    assert!(content.contains("# Documentation"));
}

#[test]
fn test_generate_llms_txt_empty_analyses() {
    let dir = tempfile::TempDir::new().unwrap();
    let analyses: Vec<Analysis> = vec![];
    let link_map: std::collections::HashMap<String, IdMapping> = std::collections::HashMap::new();
    let config = LlmsConfig::default();

    generate_llms_txt(&analyses, &link_map, &config, dir.path()).unwrap();

    let content = std::fs::read_to_string(dir.path().join("llms.txt")).unwrap();
    assert!(content.contains("# Documentation"));
    assert!(content.contains("Total documents: 0"));
}

#[test]
fn test_generate_agents_md_basic() {
    let dir = tempfile::TempDir::new().unwrap();
    let analyses = vec![
        make_analysis("a.md", "tutorial", "Tutorial 1"),
        make_analysis("b.md", "tutorial", "Tutorial 2"),
        make_analysis("c.md", "concept", "Concept 1"),
    ];
    let link_map: std::collections::HashMap<String, IdMapping> = std::collections::HashMap::new();
    let config = LlmsConfig::default();

    generate_agents_md(&analyses, &link_map, &config, dir.path()).unwrap();

    let content = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
    assert!(content.contains("Agent Instructions"));
    assert!(content.contains("3 documents"));
    assert!(content.contains("tutorial"));
    assert!(content.contains("concept"));
}
