//! Basic transform tests: headings, links, context, unicode.

use std::collections::HashMap;

use super::*;
use crate::assign::IdMapping;

#[test]
fn test_heading_level_conversion() {
    assert_eq!(
        ast_transforms::heading_level_to_u32(ast_transforms::from_u32_level(1)),
        1
    );
    assert_eq!(
        ast_transforms::heading_level_to_u32(ast_transforms::from_u32_level(4)),
        4
    );
    assert_eq!(
        ast_transforms::heading_level_to_u32(ast_transforms::from_u32_level(6)),
        6
    );
    assert_eq!(
        ast_transforms::heading_level_to_u32(ast_transforms::from_u32_level(10)),
        6
    );
}

#[test]
fn test_fix_headings_simple() {
    let content = "## First\n### Second";
    let result = ast_transforms::fix_headings_ast(content);
    assert!(result.contains("##"));
    assert!(result.contains("###"));
}

#[test]
fn test_fix_headings_skipped_levels() {
    let content = "## First\n#### Skipped";
    let result = ast_transforms::fix_headings_ast(content);
    assert!(result.contains("###"));
}

#[test]
fn test_code_block_preservation() {
    let content = "```\n## Not a heading\n[Not a link](fake.md)\n```";
    let result = ast_transforms::fix_headings_ast(content);
    assert!(result.contains("## Not a heading"));
}

#[test]
fn test_ensure_h1() {
    let content = "No heading here";
    let content = ast_context::ensure_h1_ast(content, "Test Title");
    assert!(content.contains("# Test Title"));
}

#[test]
fn test_h1_already_exists() {
    let content = "# Already H1\n\nContent";
    let content = ast_context::ensure_h1_ast(content, "New Title");
    let h1_count = content.matches("# ").count();
    assert_eq!(h1_count, 1);
}

#[test]
fn test_context_blockquote_detection() {
    let content = "> **Context**: Some text";
    assert!(ast_context::content_has_blockquote_context(content));
}

#[test]
fn test_context_blockquote_missing() {
    let content = "No context here";
    assert!(!ast_context::content_has_blockquote_context(content));
}

#[test]
fn test_see_also_detection() {
    let content = "## See Also\n- Link";
    assert!(ast_context::content_has_see_also(content));
}

#[test]
fn test_parse_markdown_simple() {
    let content = "# Heading\n\nParagraph";
    let events = ast_transforms::parse_markdown(content);
    assert!(!events.is_empty());
}

#[test]
fn test_unicode_preservation() {
    let content = "## \u{417}\u{430}\u{433}\u{43e}\u{43b}\u{43e}\u{432}\u{43e}\u{43a} (Cyrillic)";
    let result = ast_transforms::fix_headings_ast(content);
    assert!(result.contains("\u{417}\u{430}\u{433}\u{43e}\u{43b}\u{43e}\u{432}\u{43e}\u{43a}"));
}

#[test]
fn test_nested_blockquote_heading() {
    let content = "> ## Quote heading";
    let result = ast_transforms::fix_headings_ast(content);
    assert!(result.contains('>'));
    assert!(result.contains("##"));
}

#[test]
fn test_link_rewrite_with_mapping() {
    let mut link_map = HashMap::new();
    link_map.insert(
        "/docs/target.md".to_string(),
        IdMapping {
            id: "target-123".to_string(),
            filename: "target-123.md".to_string(),
            subcategory: "docs".to_string(),
            slug: "target".to_string(),
        },
    );

    let content = "[Click here](target.md)";
    let filename_map: HashMap<String, &IdMapping> = link_map
        .iter()
        .filter_map(|(src_path, mapping)| {
            std::path::Path::new(src_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), mapping))
        })
        .collect();
    let (content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 0);
    assert!(content.contains("](./target-123.md)"));
    assert!(!content.contains("](./ target-123.md)"));
}

#[test]
fn test_broken_links_collected() {
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let filename_map: HashMap<String, &IdMapping> = HashMap::new();
    let content = "[link1](missing1.md) [link2](missing2.md)";
    let (_content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 2);
    assert!(broken.contains(&"missing1.md".to_string()));
    assert!(broken.contains(&"missing2.md".to_string()));
}

#[test]
fn test_external_links_unchanged() {
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let filename_map: HashMap<String, &IdMapping> = HashMap::new();
    let content = "[External](https://example.com) [Mailto](mailto:test@example.com)";
    let (content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 0);
    assert!(content.contains("https://example.com"));
    assert!(content.contains("mailto:test@example.com"));
}

#[test]
fn test_anchor_links_unchanged() {
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let filename_map: HashMap<String, &IdMapping> = HashMap::new();
    let content = "[Section](#some-section)";
    let (content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 0);
    assert!(content.contains("#some-section"));
}

#[test]
fn test_relative_links_with_dot_slash() {
    let mut link_map = HashMap::new();
    link_map.insert(
        "/docs/target.md".to_string(),
        IdMapping {
            id: "target-456".to_string(),
            filename: "target-456.md".to_string(),
            subcategory: "docs".to_string(),
            slug: "target".to_string(),
        },
    );
    let filename_map: HashMap<String, &IdMapping> = link_map
        .iter()
        .filter_map(|(src_path, mapping)| {
            std::path::Path::new(src_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), mapping))
        })
        .collect();
    let content = "[Link](./target.md)";
    let (content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 0);
    assert!(content.contains("](./target-456.md)"));
}

#[test]
fn test_no_false_positives_in_code_blocks() {
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let filename_map: HashMap<String, &IdMapping> = HashMap::new();
    let content = "```\n[fake](nonexistent.md)\n```";
    let (_content, broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert_eq!(broken.len(), 0);
}

#[test]
fn test_link_format_no_spaces() {
    let mut link_map = HashMap::new();
    link_map.insert(
        "/docs/example.md".to_string(),
        IdMapping {
            id: "example-789".to_string(),
            filename: "example-789.md".to_string(),
            subcategory: "docs".to_string(),
            slug: "example".to_string(),
        },
    );
    let filename_map: HashMap<String, &IdMapping> = link_map
        .iter()
        .filter_map(|(src_path, mapping)| {
            std::path::Path::new(src_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), mapping))
        })
        .collect();
    let content = "[Example Doc](example.md)";
    let (content, _broken) =
        ast_transforms::rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);
    assert!(content.contains("](./example-789.md)"));
    assert!(!content.contains("](./ "));
}
