use super::extract::extract_markdown_metadata;
use crate::analyze::LinkKind;

#[test]
fn test_extract_markdown_metadata_title_from_h1() {
    let md = "# My Title\n\nSome content here.";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.title, Some("My Title".to_string()));
    assert_eq!(meta.headings.len(), 1);
    assert_eq!(meta.headings[0].text, "My Title");
    assert_eq!(meta.headings[0].level, 1);
}

#[test]
fn test_extract_markdown_metadata_multiple_headings() {
    let md = "# Top\n\n## Section A\n\n### Subsection\n\n## Section B";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.headings.len(), 4);
    assert_eq!(meta.headings[0].level, 1);
    assert_eq!(meta.headings[0].text, "Top");
    assert_eq!(meta.headings[1].level, 2);
    assert_eq!(meta.headings[1].text, "Section A");
    assert_eq!(meta.headings[2].level, 3);
    assert_eq!(meta.headings[2].text, "Subsection");
    assert_eq!(meta.headings[3].level, 2);
    assert_eq!(meta.headings[3].text, "Section B");
}

#[test]
fn test_extract_markdown_metadata_heading_levels_h4_h5_h6() {
    let md = "#### H4\n\n##### H5\n\n###### H6";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.headings.len(), 3);
    assert_eq!(meta.headings[0].level, 4);
    assert_eq!(meta.headings[1].level, 5);
    assert_eq!(meta.headings[2].level, 6);
}

#[test]
fn test_extract_markdown_metadata_no_headings() {
    let md = "Just some plain text content.\n\nMultiple paragraphs.";
    let meta = extract_markdown_metadata(md);

    assert!(meta.title.is_none());
    assert!(meta.headings.is_empty());
}

#[test]
fn test_extract_markdown_metadata_title_from_second_h1() {
    let md = "# First\n\nContent\n\n# Second";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.title, Some("First".to_string()));
}

#[test]
fn test_extract_markdown_metadata_links_internal() {
    let md = "Check out [our guide](./guide.md) for more info.";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.links.len(), 1);
    assert_eq!(meta.links[0].text, "our guide");
    assert_eq!(meta.links[0].target, "./guide.md");
    assert_eq!(meta.links[0].kind, LinkKind::Internal);
}

#[test]
fn test_extract_markdown_metadata_links_external() {
    let md = "Visit [Google](https://google.com) or [us](http://example.com).";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.links.len(), 2);
    assert_eq!(meta.links[0].kind, LinkKind::External);
    assert_eq!(meta.links[1].kind, LinkKind::External);
}

#[test]
fn test_extract_markdown_metadata_links_mailto() {
    let md = "Email [support](mailto:help@example.com).";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.links.len(), 1);
    assert_eq!(meta.links[0].kind, LinkKind::External);
}

#[test]
fn test_extract_markdown_metadata_first_paragraph() {
    let md = "First paragraph with some text.\n\nSecond paragraph.";
    let meta = extract_markdown_metadata(md);

    assert!(meta.first_paragraph.contains("First paragraph"));
    assert!(!meta.first_paragraph.contains("Second paragraph"));
}

#[test]
fn test_extract_markdown_metadata_first_paragraph_before_heading() {
    let md = "Intro text before heading.\n\n# Title\n\nMore content.";
    let meta = extract_markdown_metadata(md);

    assert!(meta.first_paragraph.contains("Intro text before heading"));
    assert!(meta.title.is_some());
}

#[test]
fn test_extract_markdown_metadata_code_block_detection() {
    let md = "Some text.\n\n```rust\nfn main() {}\n```\n\nMore text.";
    let meta = extract_markdown_metadata(md);

    assert!(meta.has_code);
    assert!(!meta.has_tables);
}

#[test]
fn test_extract_markdown_metadata_no_code_no_tables() {
    let md = "Just plain text.\n\nWith paragraphs.";
    let meta = extract_markdown_metadata(md);

    assert!(!meta.has_code);
    assert!(!meta.has_tables);
}

#[test]
fn test_extract_markdown_metadata_inline_code() {
    let md = "Use `println!` for debugging.";
    let meta = extract_markdown_metadata(md);

    assert!(!meta.has_code, "Inline code should not set has_code");
}

#[test]
fn test_extract_markdown_metadata_heading_with_inline_formatting() {
    let md = "# **Bold** and *italic* heading";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.title, Some("Bold and italic heading".to_string()));
}

#[test]
fn test_extract_markdown_metadata_heading_trimmed() {
    let md = "#   Spaced Title   ";
    let meta = extract_markdown_metadata(md);

    assert_eq!(meta.title, Some("Spaced Title".to_string()));
}

#[test]
fn test_extract_markdown_metadata_empty_content() {
    let meta = extract_markdown_metadata("");
    assert!(meta.title.is_none());
    assert!(meta.headings.is_empty());
    assert!(meta.links.is_empty());
    assert!(meta.first_paragraph.is_empty());
    assert!(!meta.has_code);
    assert!(!meta.has_tables);
}
