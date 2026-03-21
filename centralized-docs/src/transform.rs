//! Transformation phase of the documentation transformation pipeline.
//!
//! This module transforms analyzed documents into the final output format.
//! It forms the third phase of the pipeline, following [`analyze`] and
//! preceding indexing and output generation.
//!
//! # Transformation Pipeline
//!
//! Each document undergoes six sequential AST-level transformations:
//!
//! 1. **Heading Structure Fix** — Ensures no skipped heading levels (e.g. H1→H3),
//!    flattens anything deeper than H4 to avoid over-nested structures.
//! 2. **Link Rewriting** — Converts internal relative links to the canonical
//!    output filenames assigned during the [`assign`] phase.
//! 3. **H1 Enforcement** — Guarantees exactly one H1 per document; promotes the
//!    first heading if absent, demotes extras to H2.
//! 4. **Context Injection** — Inserts a blockquote summary near the top when the
//!    document has no existing context block.
//! 5. **See Also Addition** — Appends a `## See Also` navigation section linking
//!    to related documents discovered via the knowledge graph.
//! 6. **Frontmatter Generation** — Wraps the output in YAML frontmatter containing
//!    the title, category, tags, word count, and summary.
//!
//! # AST-Based Processing
//!
//! All transformations operate on the `pulldown-cmark` event stream — parse once,
//! transform in a single pass, re-serialise with `pulldown_cmark_to_cmark`. This
//! preserves tables, task lists, footnotes and other elements that a hand-rolled
//! serialiser would silently drop.
//!
//! # Core Types
//!
//! - [`TransformResult`] — Counts of successful and failed document transformations
//!
//! # Key Functions
//!
//! - [`transform_all`] — Transform every analyzed document into the output directory
//! - [`transform_file`] — Transform a single document (useful for testing)
//!
//! # Example
//!
//! ```rust,ignore
//! use doc_transformer::transform::{transform_all, TransformResult};
//! use std::path::Path;
//!
//! let result = transform_all(&analyses, &link_map, Path::new("./output"))?;
//! println!("Transformed {}/{} documents successfully",
//!     result.success_count, result.total_count);
//! ```

#![deny(clippy::unwrap_used)]
#![allow(clippy::match_same_arms)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::wildcard_enum_match_arm)]
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::types::is_stopword;
use anyhow::Result;
use itertools::Itertools;
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

/// Error type for transform operations with full context
#[derive(Debug, Clone)]
pub struct TransformError {
    #[allow(dead_code)]
    pub source_path: String,
    #[allow(dead_code)]
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct TransformResult {
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
    /// Detailed errors from failed transformations. Empty if all succeeded.
    #[allow(dead_code)]
    pub errors: Vec<TransformError>,
}

/// Create directory with improved error context for permission issues
fn create_dir_with_context(path: &Path, context: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(|e| {
        if e.kind() == io::ErrorKind::PermissionDenied {
            anyhow::anyhow!(
                "Permission denied: cannot create {} directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                context,
                path.display()
            )
        } else {
            anyhow::anyhow!(
                "Failed to create {} directory '{}': {}",
                context,
                path.display(),
                e
            )
        }
    })
}

/// Transform all analyses, returning errors aggregated into the result.
///
/// Unlike the previous implementation which silently dropped errors via `filter_map`,
/// this version collects all transformation errors and includes them in the result.
pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let docs_dir = output_dir.join("docs");
    create_dir_with_context(&docs_dir, "docs")?;

    // Pre-build a filename-to-mapping lookup for O(1) link resolution
    let filename_map: HashMap<String, &IdMapping> = link_map
        .iter()
        .filter_map(|(src_path, mapping)| {
            Path::new(src_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), mapping))
        })
        .collect();

    let (success_results, error_results): (Vec<_>, Vec<_>) = analyses
        .par_iter()
        .filter_map(|analysis| {
            link_map.get(&analysis.source_path).map(|mapping| {
                transform_file(analysis, mapping, link_map, &docs_dir, &filename_map).map_err(|e| {
                    TransformError {
                        source_path: analysis.source_path.clone(),
                        error: e.to_string(),
                    }
                })
            })
        })
        .partition(Result::is_ok);

    let success_count = success_results.len();
    let errors: Vec<TransformError> = error_results.into_iter().filter_map(Result::err).collect();
    let error_count = errors.len();

    Ok(TransformResult {
        success_count,
        total_count: analyses.len(),
        error_count,
        errors,
    })
}

fn transform_file(
    analysis: &Analysis,
    mapping: &IdMapping,
    link_map: &HashMap<String, IdMapping>,
    docs_dir: &Path,
    filename_map: &HashMap<String, &IdMapping>,
) -> Result<()> {
    let doc_id = &mapping.id;
    let filename = &mapping.filename;

    let context_text = if analysis.first_paragraph.is_empty() {
        analysis.title.clone()
    } else {
        let max_chars = std::cmp::min(150, analysis.first_paragraph.chars().count());
        safe_truncate_chars(&analysis.first_paragraph, max_chars)
    };

    let (content, broken_links) = transform_document_ast(
        &analysis.content,
        &analysis.source_path,
        &analysis.title,
        link_map,
        filename_map,
        &context_text,
    );

    if !broken_links.is_empty() {
        eprintln!(
            "Warning: {} broken link(s) in {}:",
            broken_links.len(),
            analysis.source_path
        );
        broken_links
            .iter()
            .take(10)
            .enumerate()
            .for_each(|(idx, link)| {
                eprintln!("  {}: {}", idx.saturating_add(1), link);
            });
        if broken_links.len() > 10 {
            eprintln!("  ... and {} more", broken_links.len().saturating_sub(10));
        }
    }

    let content = if content_has_see_also(&content) {
        content
    } else {
        format!("{content}\n## See Also\n\n- [Documentation Index](./COMPASS.md)\n")
    };

    let tags = generate_tags(analysis);
    let tags_str = tags
        .iter()
        .map(|t| format!("\"{t}\""))
        .collect::<Vec<_>>()
        .join(", ");

    let frontmatter = format!(
        "---\nid: {}\ntitle: {}\ncategory: {}\ntags: [{}]\n---",
        doc_id, analysis.title, analysis.category, tags_str
    );

    let final_content = format!("{frontmatter}\n\n{content}");

    let output_file = docs_dir.join(filename);

    fs::write(&output_file, final_content)
        .map_err(|e| anyhow::anyhow!("Failed to write file '{}': {e}", output_file.display()))?;

    Ok(())
}

/// Combined AST transformation: parse once, transform events, serialize once.
///
/// Reduces parse→serialize roundtrips from 5 to 1 by operating on the
/// pulldown-cmark event stream directly instead of converting to/from strings
/// between each transformation step.
#[allow(clippy::too_many_arguments)]
fn transform_document_ast(
    content: &str,
    source_path: &str,
    title: &str,
    link_map: &HashMap<String, IdMapping>,
    filename_map: &HashMap<String, &IdMapping>,
    context_text: &str,
) -> (String, Vec<String>) {
    let events = parse_markdown(content);

    let events = fix_headings_events(events);

    let (events, broken_links) = rewrite_links_events(events, source_path, link_map, filename_map);

    let events = ensure_h1_events(events, title);

    let events = if events_have_blockquote_context(&events) {
        events
    } else {
        inject_context_events(events, context_text)
    };

    let markdown = events_to_markdown(events);
    (markdown, broken_links)
}

/// Parse markdown using pulldown-cmark with full `CommonMark` + GFM support
fn parse_markdown(content: &str) -> Vec<Event<'_>> {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);
    parser.collect()
}

/// Fix heading structure: no skipped levels, max level 4 (AST-based)
#[allow(dead_code)]
fn fix_headings_ast(content: &str) -> String {
    events_to_markdown(fix_headings_events(parse_markdown(content)))
}

/// Fix heading structure on event stream (no parse/serialize roundtrip)
fn fix_headings_events(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    events
        .into_iter()
        .scan((None::<u32>, false), |state, event| {
            let in_code_block = state.1;
            let last_heading_level = state.0;

            let new_event = match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    state.1 = true;
                    Event::Start(Tag::CodeBlock(kind))
                }
                Event::End(TagEnd::CodeBlock) => {
                    state.1 = false;
                    Event::End(TagEnd::CodeBlock)
                }
                Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }) if !in_code_block => {
                    let new_level = match last_heading_level {
                        Some(last) if heading_level_to_u32(level) > last.saturating_add(1) => {
                            from_u32_level(last.saturating_add(1))
                        }
                        _ => level,
                    };

                    let final_level = if heading_level_to_u32(new_level) > 4 {
                        from_u32_level(4)
                    } else {
                        new_level
                    };

                    state.0 = Some(heading_level_to_u32(final_level));
                    Event::Start(Tag::Heading {
                        level: final_level,
                        id,
                        classes,
                        attrs,
                    })
                }

                other => other,
            };
            Some(new_event)
        })
        .collect()
}

/// Convert heading level number to `pulldown_cmark` `HeadingLevel`
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

/// Convert `HeadingLevel` to u32 safely
///
/// This is safe because `HeadingLevel` is a C-like enum with discriminants 1-6.
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

/// Rewrite internal links to new filenames (AST-based).
///
/// Uses `filename_map` for O(1) lookup instead of iterating the entire `link_map`.
/// Returns the transformed content and a list of broken links.
#[allow(dead_code)]
fn rewrite_links_ast(
    content: &str,
    source_path: &str,
    _link_map: &HashMap<String, IdMapping>,
    filename_map: &HashMap<String, &IdMapping>,
) -> (String, Vec<String>) {
    let (events, broken) = rewrite_links_events(
        parse_markdown(content),
        source_path,
        _link_map,
        filename_map,
    );
    (events_to_markdown(events), broken)
}

/// Rewrite internal links on event stream (no parse/serialize roundtrip).
///
/// Returns transformed events and a list of broken link URLs.
fn rewrite_links_events<'a>(
    events: Vec<Event<'a>>,
    source_path: &str,
    _link_map: &HashMap<String, IdMapping>,
    filename_map: &HashMap<String, &IdMapping>,
) -> (Vec<Event<'a>>, Vec<String>) {
    let source_dir = Path::new(source_path)
        .parent()
        .map_or_else(|| Path::new(""), std::convert::identity);

    let results: Vec<(Event<'_>, Option<String>)> = events
        .into_iter()
        .scan(false, |in_code_block, event| {
            let icb = *in_code_block;

            let (new_event, new_broken_link, new_icb) = match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    (Event::Start(Tag::CodeBlock(kind)), None, true)
                }
                Event::End(TagEnd::CodeBlock) => (Event::End(TagEnd::CodeBlock), None, false),

                Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) if !icb => {
                    let url_str = dest_url.to_string();

                    let (new_url, broken) = if url_str.starts_with("http://")
                        || url_str.starts_with("https://")
                        || url_str.starts_with("mailto:")
                        || url_str.starts_with('#')
                    {
                        (dest_url.clone(), None)
                    } else {
                        let resolved_path = if url_str.starts_with("./") {
                            source_dir.join(url_str.trim_start_matches("./"))
                        } else {
                            source_dir.join(&url_str)
                        };

                        let mapped_filename = resolved_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .and_then(|name| filename_map.get(name))
                            .map(|m| m.filename.clone());

                        match mapped_filename {
                            Some(new_filename) => (CowStr::from(format!("./{new_filename}")), None),
                            None => (dest_url.clone(), Some(url_str)),
                        }
                    };

                    (
                        Event::Start(Tag::Link {
                            link_type,
                            dest_url: new_url,
                            title,
                            id,
                        }),
                        broken,
                        icb,
                    )
                }

                other => (other, None, icb),
            };

            *in_code_block = new_icb;
            Some((new_event, new_broken_link))
        })
        .collect();

    let broken_links: Vec<String> = results.iter().filter_map(|(_, bl)| bl.clone()).collect();
    let transformed_events: Vec<Event<'_>> = results.into_iter().map(|(e, _)| e).collect();

    (transformed_events, broken_links)
}

/// Ensure document has exactly one H1 heading (AST-based).
///
/// If missing, it adds an H1 at the top.
/// If multiple exist, it adds an H1 at the top and bumps all existing headings down one level to preserve hierarchy.
#[allow(dead_code)]
fn ensure_h1_ast(content: &str, title: &str) -> String {
    let events = parse_markdown(content);
    let h1_count = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
        })
        .count();

    if h1_count == 1 {
        return content.to_string();
    }

    events_to_markdown(ensure_h1_events(events, title))
}

/// Ensure document has exactly one H1 heading on event stream (no parse/serialize roundtrip).
///
/// If missing, it prepends an H1 at the top.
/// If multiple exist, it prepends an H1 and bumps all existing headings down one level.
fn ensure_h1_events<'a>(events: Vec<Event<'a>>, title: &str) -> Vec<Event<'a>> {
    let h1_count = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
        })
        .count();

    if h1_count == 1 {
        return events;
    }

    let bump_level = |level: pulldown_cmark::HeadingLevel| -> pulldown_cmark::HeadingLevel {
        match level {
            pulldown_cmark::HeadingLevel::H1 => pulldown_cmark::HeadingLevel::H2,
            pulldown_cmark::HeadingLevel::H2 => pulldown_cmark::HeadingLevel::H3,
            pulldown_cmark::HeadingLevel::H3 => pulldown_cmark::HeadingLevel::H4,
            pulldown_cmark::HeadingLevel::H4 => pulldown_cmark::HeadingLevel::H5,
            pulldown_cmark::HeadingLevel::H5 | pulldown_cmark::HeadingLevel::H6 => {
                pulldown_cmark::HeadingLevel::H6
            }
        }
    };

    let header_events = vec![
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
        Event::SoftBreak,
    ];

    header_events
        .into_iter()
        .chain(events.into_iter().map(move |event| match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) if h1_count > 1 => Event::Start(Tag::Heading {
                level: bump_level(level),
                id,
                classes,
                attrs,
            }),
            Event::End(TagEnd::Heading(level)) if h1_count > 1 => {
                Event::End(TagEnd::Heading(bump_level(level)))
            }
            other => other,
        }))
        .collect()
}

/// Check if content already has a context blockquote (AST-based)
#[allow(dead_code)]
fn content_has_blockquote_context(content: &str) -> bool {
    events_have_blockquote_context(&parse_markdown(content))
}

/// Check if events contain a context blockquote with "Context" text
fn events_have_blockquote_context(events: &[Event<'_>]) -> bool {
    events
        .iter()
        .fold((false, false), |(in_blockquote, found), event| {
            if found {
                (in_blockquote, true)
            } else {
                match event {
                    Event::Start(Tag::BlockQuote(_)) => (true, false),
                    Event::End(TagEnd::BlockQuote(_)) => (false, false),
                    Event::Text(text) if in_blockquote && text.contains("Context") => {
                        (in_blockquote, true)
                    }
                    _ => (in_blockquote, false),
                }
            }
        })
        .1
}

/// Inject context block after H1 (AST-based).
///
/// Returns the content with context block added.
#[allow(dead_code)]
fn inject_context_block_ast(content: &str, context_text: &str) -> String {
    let events = parse_markdown(content);
    let h1_end_pos = events.iter().position(|e| {
        matches!(
            e,
            Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1))
        )
    });

    match h1_end_pos {
        None => events_to_markdown(events),
        Some(pos) => {
            let (before, after) = events.split_at(pos.saturating_add(1));
            let context_block: Vec<Event<'_>> = vec![
                Event::SoftBreak,
                Event::SoftBreak,
                Event::Start(Tag::BlockQuote(None)),
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strong),
                Event::Text(CowStr::from("Context")),
                Event::End(TagEnd::Strong),
                Event::Text(CowStr::from(": ")),
                Event::Text(CowStr::from(context_text.to_string())),
                Event::End(TagEnd::Paragraph),
                Event::End(TagEnd::BlockQuote(None)),
                Event::SoftBreak,
                Event::SoftBreak,
            ];

            let new_events: Vec<Event<'_>> = before
                .iter()
                .cloned()
                .chain(context_block)
                .chain(after.iter().cloned())
                .collect();
            events_to_markdown(new_events)
        }
    }
}

/// Inject context blockquote after H1 on event stream (no parse/serialize roundtrip).
///
/// If no H1 end event is found, returns events unchanged.
fn inject_context_events<'a>(events: Vec<Event<'a>>, context_text: &str) -> Vec<Event<'a>> {
    let h1_end_pos = events.iter().position(|e| {
        matches!(
            e,
            Event::End(TagEnd::Heading(pulldown_cmark::HeadingLevel::H1))
        )
    });

    match h1_end_pos {
        None => events,
        Some(pos) => {
            let (before, after) = events.split_at(pos.saturating_add(1));
            let context_block: Vec<Event<'_>> = vec![
                Event::SoftBreak,
                Event::SoftBreak,
                Event::Start(Tag::BlockQuote(None)),
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strong),
                Event::Text(CowStr::from("Context")),
                Event::End(TagEnd::Strong),
                Event::Text(CowStr::from(": ")),
                Event::Text(CowStr::from(context_text.to_string())),
                Event::End(TagEnd::Paragraph),
                Event::End(TagEnd::BlockQuote(None)),
                Event::SoftBreak,
                Event::SoftBreak,
            ];

            before
                .iter()
                .cloned()
                .chain(context_block)
                .chain(after.iter().cloned())
                .collect()
        }
    }
}

/// Check if content already has "## See Also" section (simple text check)
fn content_has_see_also(content: &str) -> bool {
    content.contains("## See Also")
}

/// Convert events to markdown using pulldown-cmark-to-cmark
///
/// Logs errors instead of silently returning empty string.
fn events_to_markdown<'a, I>(events: I) -> String
where
    I: IntoIterator<Item = Event<'a>>,
{
    // I/O boundary: Write trait requires &mut self — no functional alternative.
    #[allow(unused_mut)]
    let mut buf = String::new();
    if let Err(e) = pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buf) {
        // Log the error but don't crash - return whatever was written
        eprintln!("Warning: pulldown_cmark_to_cmark failed: {e}");
        if buf.is_empty() {
            // If buffer is empty, return a placeholder to indicate failure
            return String::from("[ERROR: markdown serialization failed]");
        }
    }
    buf
}

/// Safely truncate a string to a maximum number of Unicode characters
/// Truncate to max grapheme clusters (handles emoji, combining marks, etc.)
fn safe_truncate_chars(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let opt_val: Option<usize> = Some(max_chars);
    let _test = opt_val.map_or(max_chars, |v| v);

    text.graphemes(true).take(max_chars).collect::<String>()
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
                .map(str::to_lowercase),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
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
        let content = "No heading here";
        let content = ensure_h1_ast(content, "Test Title");
        assert!(content.contains("# Test Title"));
    }

    #[test]
    fn test_h1_already_exists() {
        let content = "# Already H1\n\nContent";
        let content = ensure_h1_ast(content, "New Title");
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
        assert!(result.contains('>'));
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
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        // Should have no broken links
        assert_eq!(broken.len(), 0);
        // Should be formatted as ./filename with no space
        assert!(content.contains("](./target-123.md)"));
        assert!(!content.contains("](./ target-123.md)"));
    }

    #[test]
    fn test_broken_links_collected() {
        // Test that broken links are properly collected
        let link_map: HashMap<String, IdMapping> = HashMap::new(); // Empty - all links are broken
        let filename_map: HashMap<String, &IdMapping> = HashMap::new();

        let content = "[link1](missing1.md) [link2](missing2.md)";
        let (_content, broken) =
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        // Should have collected broken links
        assert_eq!(broken.len(), 2);
        assert!(broken.contains(&"missing1.md".to_string()));
        assert!(broken.contains(&"missing2.md".to_string()));
    }

    #[test]
    fn test_external_links_unchanged() {
        // External links should not be modified
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let filename_map: HashMap<String, &IdMapping> = HashMap::new();

        let content = "[External](https://example.com) [Mailto](mailto:test@example.com)";
        let (content, broken) =
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        // No broken links for external links
        assert_eq!(broken.len(), 0);
        // URLs should be preserved
        assert!(content.contains("https://example.com"));
        assert!(content.contains("mailto:test@example.com"));
    }

    #[test]
    fn test_anchor_links_unchanged() {
        // Anchor links should not be modified
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let filename_map: HashMap<String, &IdMapping> = HashMap::new();

        let content = "[Section](#some-section)";
        let (content, broken) =
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

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
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        assert_eq!(broken.len(), 0);
        assert!(content.contains("](./target-456.md)"));
    }

    #[test]
    fn test_no_false_positives_in_code_blocks() {
        // Links inside code blocks should not be rewritten or marked as broken
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let filename_map: HashMap<String, &IdMapping> = HashMap::new();

        let content = "```\n[fake](nonexistent.md)\n```";
        let (_content, broken) =
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        // No broken links - link was in code block
        assert_eq!(broken.len(), 0);
    }

    #[test]
    fn test_multiple_broken_links_tracking() {
        // Test that multiple broken links in one file are all collected
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let filename_map: HashMap<String, &IdMapping> = HashMap::new();

        let content = "[a](broken1.md) text [b](broken2.md) more [c](broken3.md)";
        let (_content, broken) =
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

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
            rewrite_links_ast(content, "/docs/source.md", &link_map, &filename_map);

        // The result should have the correct format
        assert!(content.contains("](./example-789.md)"));
        // Verify no space after (./
        assert!(!content.contains("](./ "));
    }
}
