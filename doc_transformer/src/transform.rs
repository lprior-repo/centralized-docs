use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

static HEADING_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").expect("valid heading regex"));

static LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("valid link regex"));

static H1_START_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^# [^#]").expect("valid H1 start regex"));

static H1_LINE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(# .+\n)").expect("valid H1 line regex"));

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

    // Step 1: Fix heading structure
    let mut content = fix_headings(&analysis.content);

    // Step 2: Rewrite internal links
    let (_content, _broken) = rewrite_links(&content, &analysis.source_path, link_map);
    content = _content;

    // Step 3: Ensure single H1
    if !H1_START_REGEX.is_match(&content) {
        content = format!("# {}\n\n{}", analysis.title, content);
    }

    // Step 4: Add context block if missing
    if !content.contains("> **Context**:") {
        let context_text = if analysis.first_paragraph.is_empty() {
            analysis.title.clone()
        } else {
            let max_chars = std::cmp::min(150, analysis.first_paragraph.chars().count());
            analysis.first_paragraph
                .chars()
                .take(max_chars)
                .collect::<String>()
        };
        let context_block = format!("> **Context**: {}\n", context_text);

        // Insert after H1
        content = H1_LINE_REGEX
            .replace(&content, format!("$1\n{}\n", context_block))
            .to_string();
    }

    // Step 5: Add See Also section if missing
    if !content.contains("## See Also") {
        let see_also = "\n## See Also\n\n- [Documentation Index](./COMPASS.md)\n";
        content.push_str(see_also);
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

/// Fix heading structure: no skipped levels, max level 4
fn fix_headings(content: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Find all headings and their levels
    let mut heading_lines: Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = HEADING_REGEX.captures(line) {
            let level = caps.get(1).expect("capture group 1 exists").as_str().len();
            heading_lines.push((i, level));
        }
    }

    // Fix skipped levels
    for j in 1..heading_lines.len() {
        let prev_level = heading_lines[j.saturating_sub(1)].1;
        let curr_level = heading_lines[j].1;
        let line_idx = heading_lines[j].0;

        if curr_level > prev_level.saturating_add(1) {
            // Demote to prev_level + 1
            let new_level = prev_level.saturating_add(1);
            let new_hashes = "#".repeat(new_level);
            let text = lines[line_idx].trim_start_matches('#').trim_start();
            lines[line_idx] = format!("{} {}", new_hashes, text);
        }
    }

    // Limit heading level to 4
    for line in &mut lines {
        if let Some(caps) = HEADING_REGEX.captures(line) {
            let hashes = caps.get(1).expect("capture group 1 exists").as_str();
            if hashes.len() > 4 {
                let text = caps.get(2).expect("capture group 2 exists").as_str();
                *line = format!("#### {}", text);
            }
        }
    }

    lines.join("\n")
}

/// Rewrite internal links to new filenames
fn rewrite_links(
    content: &str,
    source_path: &str,
    link_map: &HashMap<String, IdMapping>,
) -> (String, Vec<String>) {
    let mut broken_links = Vec::new();
    let source_dir = Path::new(source_path).parent().unwrap_or_else(|| Path::new(""));

    let result = LINK_REGEX
        .replace_all(content, |caps: &regex::Captures| {
            let text = &caps[1];
            let target = &caps[2];

            // Keep external links and anchors
            if target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
                || target.starts_with('#')
            {
                return caps[0].to_string();
            }

            // Try to resolve relative path
            let resolved_path = if target.starts_with("./") {
                source_dir.join(target.trim_start_matches("./"))
            } else {
                source_dir.join(target)
            };

            // Look up in link_map
            for (src_path, mapping) in link_map {
                let src_file = Path::new(src_path).file_name().unwrap_or_default();
                let resolved_file = resolved_path.file_name().unwrap_or_default();

                if src_file == resolved_file || src_path.ends_with(&resolved_path.to_string_lossy().to_string()) {
                    return format!("[{}](./ {})", text, mapping.filename);
                }
            }

            // Not found - track as broken
            broken_links.push(target.to_string());
            caps[0].to_string()
        })
        .to_string();

    (result, broken_links)
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
