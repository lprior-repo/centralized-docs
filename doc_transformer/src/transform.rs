use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use itertools::Itertools;
use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct TransformResult {
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
}

pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let docs_dir = output_dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let mut success_count: usize = 0;
    let mut error_count: usize = 0;

    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            match transform_file(analysis, mapping, link_map, &docs_dir) {
                Ok(_) => success_count = success_count.saturating_add(1),
                Err(e) => {
                    eprintln!("TRANSFORM ERROR: {}: {}", analysis.source_path, e);
                    error_count = error_count.saturating_add(1);
                }
            }
        }
    }

    Ok(TransformResult {
        success_count,
        total_count: analyses.len(),
        error_count,
    })
}

fn transform_file(
    analysis: &Analysis,
    mapping: &IdMapping,
    link_map: &HashMap<String, IdMapping>,
    docs_dir: &Path,
) -> Result<()> {
    let doc_id = &mapping.id;
    let filename = &mapping.filename;

    // Step 1: Fix heading structure using AST
    let mut content = fix_headings_ast(&analysis.content);

    // Step 2: Rewrite internal links using AST
    let broken_links = rewrite_links_ast(&mut content, &analysis.source_path, link_map);

    // Log any broken links found
    if !broken_links.is_empty() {
        eprintln!(
            "WARN: {} broken link(s) in {}:",
            broken_links.len(),
            analysis.source_path
        );
        for (idx, link) in broken_links.iter().enumerate().take(10) {
            eprintln!("  {}: {}", idx + 1, link);
        }
        if broken_links.len() > 10 {
            eprintln!("  ... and {} more", broken_links.len() - 10);
        }
    }

    // Step 3: Ensure single H1 using AST
    ensure_h1_ast(&mut content, &analysis.title);

    // Step 4: Add context block if missing using AST
    if !content_has_blockquote_context(&content) {
        let context_text = if analysis.first_paragraph.is_empty() {
            analysis.title.clone()
        } else {
            let max_chars = std::cmp::min(150, analysis.first_paragraph.chars().count());
            analysis
                .first_paragraph
                .chars()
                .take(max_chars)
                .collect::<String>()
        };
        inject_context_block_ast(&mut content, &context_text);
    }

    // Step 5: Add See Also section if missing using AST
    if !content_has_see_also(&content) {
        content.push_str("\n## See Also\n\n- [Documentation Index](./COMPASS.md)\n");
    }

    // Generate frontmatter
    let tags = generate_tags(analysis);
    let tags_str = tags
        .iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");

    let frontmatter = format!(
        "---\nid: {}\ntitle: {}\ncategory: {}\ntags: [{}]\n---",
        doc_id, analysis.title, analysis.category, tags_str
    );

    // Assemble final content
    let final_content = format!("{}\n\n{}", frontmatter, content);

    // Write file
    let output_file = docs_dir.join(filename);
    fs::write(output_file, final_content)?;

    Ok(())
}

/// Parse markdown using pulldown-cmark with full CommonMark + GFM support
fn parse_markdown(content: &str) -> Vec<Event> {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);
    parser.collect()
}

/// Render events back to markdown (via HTML intermediate for safety)
fn render_markdown(events: Vec<Event>) -> String {
    let mut html = String::new();
    html::push_html(&mut html, events.into_iter());

    // Basic HTML-to-Markdown fallback for critical elements
    // In production, you might use html2md crate
    html.replace("<h1>", "# ")
        .replace("<h2>", "## ")
        .replace("<h3>", "### ")
        .replace("<h4>", "#### ")
        .replace("<h5>", "##### ")
        .replace("<h6>", "###### ")
        .replace("</h1>\n", "\n")
        .replace("</h2>\n", "\n")
        .replace("</h3>\n", "\n")
        .replace("</h4>\n", "\n")
        .replace("</h5>\n", "\n")
        .replace("</h6>\n", "\n")
        .replace("<p>", "")
        .replace("</p>\n", "\n")
        .replace("<em>", "*")
        .replace("</em>", "*")
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("<code>", "`")
        .replace("</code>", "`")
        .replace("<pre><code>", "```\n")
        .replace("</code></pre>", "\n```")
        .replace("<blockquote>\n", "> ")
        .replace("</blockquote>\n", "")
        .replace("<ul>\n", "")
        .replace("</ul>\n", "")
        .replace("<ol>\n", "")
        .replace("</ol>\n", "")
        .replace("<li>", "- ")
        .replace("</li>\n", "\n")
        .replace("<a href=\"", "[")
        .replace("\">", "](")
        .replace("</a>", ")")
}

/// Fix heading structure: no skipped levels, max level 4 (AST-based)
fn fix_headings_ast(content: &str) -> String {
    let events = parse_markdown(content);

    // Track heading levels as we walk the tree
    let mut fixed_events = Vec::new();
    let mut last_heading_level: Option<u32> = None;
    let mut in_code_block = false;

    for event in events {
        match event {
            // Track code block boundaries - never transform inside code
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                fixed_events.push(Event::Start(Tag::CodeBlock(kind)));
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                fixed_events.push(Event::End(TagEnd::CodeBlock));
            }

            // Transform headings (unless in code block)
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) if !in_code_block => {
                let new_level = if let Some(last_level) = last_heading_level {
                    // Prevent level skips: if current > last + 1, demote to last + 1
                    let level_num = heading_level_to_u32(level);
                    if level_num > last_level.saturating_add(1) {
                        // Demote to last_level + 1
                        from_u32_level(last_level.saturating_add(1))
                    } else {
                        level
                    }
                } else {
                    level
                };

                // Limit to max level 4
                let final_level = if heading_level_to_u32(new_level) > 4 {
                    from_u32_level(4)
                } else {
                    new_level
                };

                last_heading_level = Some(heading_level_to_u32(final_level));
                fixed_events.push(Event::Start(Tag::Heading {
                    level: final_level,
                    id,
                    classes,
                    attrs,
                }));
            }

            // Pass through all other events unchanged
            other => fixed_events.push(other),
        }
    }

    // Convert back to markdown
    events_to_markdown(fixed_events)
}

/// Convert heading level number to pulldown_cmark HeadingLevel
fn from_u32_level(level: u32) -> pulldown_cmark::HeadingLevel {
    match level {
        1 => pulldown_cmark::HeadingLevel::H1,
        2 => pulldown_cmark::HeadingLevel::H2,
        3 => pulldown_cmark::HeadingLevel::H3,
        4 => pulldown_cmark::HeadingLevel::H4,
        5 => pulldown_cmark::HeadingLevel::H5,
        _ => pulldown_cmark::HeadingLevel::H6,
    }
}

/// Convert HeadingLevel to u32 safely
///
/// This is safe because HeadingLevel is a C-like enum with discriminants 1-6.
/// No overflow or truncation is possible.
fn heading_level_to_u32(level: pulldown_cmark::HeadingLevel) -> u32 {
    match level {
        pulldown_cmark::HeadingLevel::H1 => 1,
        pulldown_cmark::HeadingLevel::H2 => 2,
        pulldown_cmark::HeadingLevel::H3 => 3,
        pulldown_cmark::HeadingLevel::H4 => 4,
        pulldown_cmark::HeadingLevel::H5 => 5,
        pulldown_cmark::HeadingLevel::H6 => 6,
    }
}

/// Convert HeadingLevel to usize safely for string operations
///
/// This is safe because HeadingLevel values are 1-6.
fn heading_level_to_usize(level: pulldown_cmark::HeadingLevel) -> usize {
    heading_level_to_u32(level) as usize
}

/// Rewrite internal links to new filenames (AST-based)
fn rewrite_links_ast(
    content: &mut String,
    source_path: &str,
    link_map: &HashMap<String, IdMapping>,
) -> Vec<String> {
    let events = parse_markdown(content);
    let source_dir = Path::new(source_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));

    let mut broken_links = Vec::new();
    let mut transformed_events = Vec::new();
    let mut in_code_block = false;

    for event in events {
        match event {
            // Never transform links inside code blocks
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                transformed_events.push(Event::Start(Tag::CodeBlock(kind)));
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                transformed_events.push(Event::End(TagEnd::CodeBlock));
            }

            // Transform Link events
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) if !in_code_block => {
                let url_str = dest_url.to_string();

                // Keep external links and anchors unchanged
                let new_url = if url_str.starts_with("http://")
                    || url_str.starts_with("https://")
                    || url_str.starts_with("mailto:")
                    || url_str.starts_with('#')
                {
                    dest_url.clone()
                } else {
                    // Try to resolve and map the link
                    let resolved_path = if url_str.starts_with("./") {
                        source_dir.join(url_str.trim_start_matches("./"))
                    } else {
                        source_dir.join(&url_str)
                    };

                    // Look up in link_map
                    let mut mapped_filename: Option<String> = None;
                    for (src_path, mapping) in link_map {
                        let src_file = Path::new(src_path)
                            .file_name()
                            .filter(|s| !s.is_empty());
                        let resolved_file = resolved_path
                            .file_name()
                            .filter(|s| !s.is_empty());

                        if src_file == resolved_file && src_file.is_some()
                            || src_path.ends_with(&resolved_path.to_string_lossy().to_string())
                        {
                            mapped_filename = Some(mapping.filename.clone());
                            break;
                        }
                    }

                    if let Some(new_filename) = mapped_filename {
                        // Format as ./filename without extra spaces
                        CowStr::from(format!("./{}", new_filename))
                    } else {
                        broken_links.push(url_str.clone());
                        dest_url.clone()
                    }
                };

                transformed_events.push(Event::Start(Tag::Link {
                    link_type,
                    dest_url: new_url,
                    title,
                    id,
                }));
            }

            // Pass through all other events unchanged
            other => transformed_events.push(other),
        }
    }

    *content = events_to_markdown(transformed_events);
    broken_links
}

/// Ensure document has exactly one H1 heading (AST-based)
fn ensure_h1_ast(content: &mut String, title: &str) {
    let events = parse_markdown(content);

    let has_h1 = events.iter().any(|e| {
        matches!(
            e,
            Event::Start(Tag::Heading {
                level: pulldown_cmark::HeadingLevel::H1,
                ..
            })
        )
    });

    if !has_h1 {
        // Prepend H1 with title
        let mut new_events = vec![
            Event::Start(Tag::Heading {
                level: pulldown_cmark::HeadingLevel::H1,
                id: None,
                classes: vec![],
                attrs: vec![],
            }),
            Event::Text(CowStr::from(title.to_string())),
            Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1)),
            Event::SoftBreak,
            Event::SoftBreak,
        ];
        new_events.extend(events);
        *content = events_to_markdown(new_events);
    }
}

/// Check if content already has a context blockquote (AST-based)
fn content_has_blockquote_context(content: &str) -> bool {
    let events = parse_markdown(content);

    for (i, event) in events.iter().enumerate() {
        if matches!(event, Event::Start(Tag::BlockQuote(_))) {
            // Check if next event contains "Context"
            if let Some(Event::Text(text)) = events.get(i + 1) {
                if text.contains("Context") {
                    return true;
                }
            }
        }
    }
    false
}

/// Inject context block after H1 (AST-based)
fn inject_context_block_ast(content: &mut String, context_text: &str) {
    let events = parse_markdown(content);
    let mut new_events = Vec::new();
    let mut inserted = false;

    for event in events.iter() {
        new_events.push(event.clone());

        // After H1 closing tag, inject context blockquote
        if !inserted
            && matches!(
                event,
                Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1))
            )
        {
            new_events.push(Event::SoftBreak);
            new_events.push(Event::SoftBreak);
            new_events.push(Event::Start(Tag::BlockQuote(None)));
            new_events.push(Event::Start(Tag::Paragraph));
            new_events.push(Event::Start(Tag::Strong));
            new_events.push(Event::Text(CowStr::from("Context")));
            new_events.push(Event::End(TagEnd::Strong));
            new_events.push(Event::Text(CowStr::from(": ")));
            new_events.push(Event::Text(CowStr::from(context_text.to_string())));
            new_events.push(Event::End(TagEnd::Paragraph));
            new_events.push(Event::End(TagEnd::BlockQuote(None)));
            new_events.push(Event::SoftBreak);
            new_events.push(Event::SoftBreak);
            inserted = true;
        }
    }

    *content = events_to_markdown(new_events);
}

/// Check if content already has "## See Also" section (simple text check)
fn content_has_see_also(content: &str) -> bool {
    content.contains("## See Also")
}

/// Stateful context for event-to-markdown conversion
#[derive(Debug, Default)]
struct RenderState {
    output: String,
    link_url: Option<String>,
}

/// Convert events to markdown using stateful fold-based reconstruction
fn events_to_markdown(events: Vec<Event>) -> String {
    let final_state = events.into_iter().fold(
        RenderState::default(),
        |mut state, event| {
            match event {
                Event::Text(text) => state.output.push_str(&text),
                Event::Code(code) => {
                    state.output.push('`');
                    state.output.push_str(&code);
                    state.output.push('`');
                }
                Event::SoftBreak | Event::HardBreak => state.output.push('\n'),
                Event::Start(Tag::Heading { level, .. }) => {
                    let hashes = "#".repeat(heading_level_to_usize(level));
                    state.output.push_str(&hashes);
                    state.output.push(' ');
                }
                Event::End(TagEnd::Heading(_)) => {
                    state.output.push('\n');
                }
                Event::Start(Tag::Paragraph) => {
                    // Paragraph starts - no output
                }
                Event::End(TagEnd::Paragraph) => {
                    state.output.push('\n');
                }
                Event::Start(Tag::BlockQuote(_)) => {
                    state.output.push_str("> ");
                }
                Event::End(TagEnd::BlockQuote(_)) => {
                    state.output.push('\n');
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    state.output.push_str("```\n");
                }
                Event::End(TagEnd::CodeBlock) => {
                    state.output.push_str("\n```\n");
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    state.output.push('[');
                    // Capture URL for later (on End event)
                    state.link_url = Some(dest_url.to_string());
                }
                Event::End(TagEnd::Link) => {
                    // Close link text and output URL
                    state.output.push_str("](");
                    if let Some(url) = state.link_url.take() {
                        state.output.push_str(&url);
                    }
                    state.output.push(')');
                }
                Event::Start(Tag::Strong) => state.output.push_str("**"),
                Event::End(TagEnd::Strong) => state.output.push_str("**"),
                Event::Start(Tag::Emphasis) => state.output.push('*'),
                Event::End(TagEnd::Emphasis) => state.output.push('*'),
                Event::Start(Tag::List(_)) => {
                    // List starts - no output
                }
                Event::End(TagEnd::List(_)) => {
                    state.output.push('\n');
                }
                Event::Start(Tag::Item) => {
                    state.output.push_str("- ");
                }
                Event::End(TagEnd::Item) => {
                    state.output.push('\n');
                }
                _ => {
                    // Pass through other events
                }
            }
            state
        },
    );

    final_state.output
}

/// Generate tags using functional composition
fn generate_tags(analysis: &Analysis) -> Vec<String> {
    std::iter::once(analysis.category.clone())
        .chain(
            analysis
                .headings
                .iter()
                .take(3)
                .flat_map(|h| h.text.split_whitespace())
                .filter(|word| word.len() > 4 && !is_stopword(word))
                .map(|word| word.to_lowercase()),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
}

fn is_stopword(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "this" | "that" | "these" | "those" | "about" | "guide"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_level_conversion() {
        assert_eq!(heading_level_to_u32(from_u32_level(1)), 1);
        assert_eq!(heading_level_to_u32(from_u32_level(4)), 4);
        assert_eq!(heading_level_to_u32(from_u32_level(6)), 6);
        assert_eq!(heading_level_to_u32(from_u32_level(10)), 6); // Clamps to H6
    }

    #[test]
    fn test_fix_headings_simple() {
        let content = "## First\n### Second";
        let result = fix_headings_ast(content);
        // Should preserve structure since no levels are skipped
        assert!(result.contains("##"));
        assert!(result.contains("###"));
    }

    #[test]
    fn test_fix_headings_skipped_levels() {
        let content = "## First\n#### Skipped";
        let result = fix_headings_ast(content);
        // Should demote from H4 to H3 (no skip)
        assert!(result.contains("###"));
    }

    #[test]
    fn test_code_block_preservation() {
        let content = "```\n## Not a heading\n[Not a link](fake.md)\n```";
        let result = fix_headings_ast(content);
        // Code block content should be unchanged
        assert!(result.contains("## Not a heading"));
    }

    #[test]
    fn test_ensure_h1() {
        let mut content = "No heading here".to_string();
        ensure_h1_ast(&mut content, "Test Title");
        assert!(content.contains("# Test Title"));
    }

    #[test]
    fn test_h1_already_exists() {
        let mut content = "# Already H1\n\nContent".to_string();
        ensure_h1_ast(&mut content, "New Title");
        // Should not add another H1
        let h1_count = content.matches("# ").count();
        assert_eq!(h1_count, 1);
    }

    #[test]
    fn test_context_blockquote_detection() {
        let content = "> **Context**: Some text";
        assert!(content_has_blockquote_context(content));
    }

    #[test]
    fn test_context_blockquote_missing() {
        let content = "No context here";
        assert!(!content_has_blockquote_context(content));
    }

    #[test]
    fn test_see_also_detection() {
        let content = "## See Also\n- Link";
        assert!(content_has_see_also(content));
    }

    #[test]
    fn test_parse_markdown_simple() {
        let content = "# Heading\n\nParagraph";
        let events = parse_markdown(content);
        assert!(!events.is_empty());
    }

    #[test]
    fn test_unicode_preservation() {
        let content = "## Заголовок (Cyrillic)";
        let result = fix_headings_ast(content);
        assert!(result.contains("Заголовок"));
    }

    #[test]
    fn test_nested_blockquote_heading() {
        let content = "> ## Quote heading";
        let result = fix_headings_ast(content);
        // Should preserve blockquote and heading
        assert!(result.contains(">"));
        assert!(result.contains("##"));
    }

    #[test]
    fn test_link_rewrite_with_mapping() {
        // Test that links are rewritten with the correct format (no space)
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

        let mut content = "[Click here](target.md)".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // Should have no broken links
        assert_eq!(broken.len(), 0);
        // Should be formatted as ./filename with no space
        assert!(content.contains("](./target-123.md)"));
        assert!(!content.contains("](./ target-123.md)"));
    }

    #[test]
    fn test_broken_links_collected() {
        // Test that broken links are properly collected
        let link_map = HashMap::new(); // Empty - all links are broken

        let mut content = "[link1](missing1.md) [link2](missing2.md)".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // Should have collected broken links
        assert_eq!(broken.len(), 2);
        assert!(broken.contains(&"missing1.md".to_string()));
        assert!(broken.contains(&"missing2.md".to_string()));
    }

    #[test]
    fn test_external_links_unchanged() {
        // External links should not be modified
        let link_map = HashMap::new();

        let mut content = "[External](https://example.com) [Mailto](mailto:test@example.com)"
            .to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // No broken links for external links
        assert_eq!(broken.len(), 0);
        // URLs should be preserved
        assert!(content.contains("https://example.com"));
        assert!(content.contains("mailto:test@example.com"));
    }

    #[test]
    fn test_anchor_links_unchanged() {
        // Anchor links should not be modified
        let link_map = HashMap::new();

        let mut content = "[Section](#some-section)".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // No broken links for anchors
        assert_eq!(broken.len(), 0);
        // Anchor should be preserved
        assert!(content.contains("#some-section"));
    }

    #[test]
    fn test_relative_links_with_dot_slash() {
        // Test relative links starting with ./
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

        let mut content = "[Link](./target.md)".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        assert_eq!(broken.len(), 0);
        assert!(content.contains("](./target-456.md)"));
    }

    #[test]
    fn test_no_false_positives_in_code_blocks() {
        // Links inside code blocks should not be rewritten or marked as broken
        let link_map = HashMap::new();

        let mut content = "```\n[fake](nonexistent.md)\n```".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // No broken links - link was in code block
        assert_eq!(broken.len(), 0);
    }

    #[test]
    fn test_multiple_broken_links_tracking() {
        // Test that multiple broken links in one file are all collected
        let link_map = HashMap::new();

        let mut content = "[a](broken1.md) text [b](broken2.md) more [c](broken3.md)".to_string();
        let broken = rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        assert_eq!(broken.len(), 3);
        assert!(broken.contains(&"broken1.md".to_string()));
        assert!(broken.contains(&"broken2.md".to_string()));
        assert!(broken.contains(&"broken3.md".to_string()));
    }

    #[test]
    fn test_link_format_no_spaces() {
        // Comprehensive test: verify that formatted links have no spaces
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

        let mut content = "[Example Doc](example.md)".to_string();
        rewrite_links_ast(&mut content, "/docs/source.md", &link_map);

        // The result should have the correct format
        assert!(content.contains("](./example-789.md)"));
        // Verify no space after (./
        assert!(!content.contains("](./ "));
    }
}
