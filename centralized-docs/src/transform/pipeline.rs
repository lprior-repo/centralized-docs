//! Transform pipeline entry points: transform_all, transform_file, transform_to_content.

use super::ast_context::{
    content_has_see_also, ensure_h1_events, events_have_blockquote_context, generate_tags,
    inject_context_events, safe_truncate_chars,
};
use super::ast_transforms::{
    events_to_markdown, fix_headings_events, parse_markdown, rewrite_links_events,
};
use super::types::{create_dir_with_context, TransformError, TransformResult};
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Transform all analyses, returning errors aggregated into the result.
pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let docs_dir = output_dir.join("docs");
    create_dir_with_context(&docs_dir, "docs")?;

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

/// Compute the fully-transformed markdown content for a single analysis.
pub(crate) fn transform_to_content(
    analysis: &Analysis,
    mapping: &IdMapping,
    link_map: &HashMap<String, IdMapping>,
    filename_map: &HashMap<String, &IdMapping>,
) -> String {
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
        format!("{content}\n## See Also\n\n- [Documentation Index](./NAVIGATION.md)\n")
    };

    let tags = generate_tags(analysis);
    let tags_str = tags
        .iter()
        .map(|t| format!("\"{t}\""))
        .collect::<Vec<_>>()
        .join(", ");

    let frontmatter = format!(
        "---\nid: {}\ntitle: {}\ncategory: {}\ntags: [{}]\n---",
        &mapping.id, analysis.title, analysis.category, tags_str
    );
    format!("{frontmatter}\n\n{content}")
}

/// Transform a single document to disk.
pub fn transform_file(
    analysis: &Analysis,
    mapping: &IdMapping,
    link_map: &HashMap<String, IdMapping>,
    docs_dir: &Path,
    filename_map: &HashMap<String, &IdMapping>,
) -> Result<()> {
    let final_content = transform_to_content(analysis, mapping, link_map, filename_map);
    let output_file = docs_dir.join(&mapping.filename);
    fs::write(&output_file, final_content)
        .map_err(|e| anyhow::anyhow!("Failed to write file '{}': {e}", output_file.display()))?;
    Ok(())
}

/// Combined AST transformation: parse once, transform events, serialize once.
#[allow(clippy::too_many_arguments)]
pub(crate) fn transform_document_ast(
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
