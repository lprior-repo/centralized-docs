use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct TransformResult {
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
    pub skipped_count: usize,
}

pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let docs_dir = output_dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let mut success_count = 0;
    let mut error_count = 0;
    let mut skipped_count = 0;
    let mut skipped_paths: Vec<String> = Vec::new();

    for analysis in analyses {
        match link_map.get(&analysis.source_path) {
            Some(mapping) => {
                match transform_file(analysis, mapping, link_map, &docs_dir) {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        eprintln!("TRANSFORM ERROR: {}: {}", analysis.source_path, e);
                        error_count += 1;
                    }
                }
            }
            None => {
                skipped_count += 1;
                skipped_paths.push(analysis.source_path.clone());
                eprintln!("WARNING: No ID mapping for {}", analysis.source_path);
            }
        }
    }

    if !skipped_paths.is_empty() {
        eprintln!("WARNING: {} documents skipped (no ID mapping)", skipped_count);
    }

    Ok(TransformResult {
        success_count,
        total_count: analyses.len(),
        error_count,
        skipped_count,
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
    if !Regex::new(r"^# [^#]")?.is_match(&content) {
        content = format!("# {}\n\n{}", analysis.title, content);
    }

    // Step 4: Add context block if missing
    if !content.contains("> **Context**:") {
        let context_text = if analysis.first_paragraph.is_empty() {
            analysis.title.clone()
        } else {
            let max_len = std::cmp::min(150, analysis.first_paragraph.len());
            safe_truncate(&analysis.first_paragraph, max_len).to_string()
        };
        let context_block = format!("> **Context**: {}\n", context_text);

        // Insert after H1
        let h1_pattern = Regex::new(r"^(# .+\n)")?;
        content = h1_pattern
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
    let heading_pattern = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();

    // Find all headings and their levels
    let mut heading_lines: Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = heading_pattern.captures(line) {
            if let Some(level_match) = caps.get(1) {
                let level = level_match.as_str().len();
                heading_lines.push((i, level));
            }
        }
    }

    // Fix skipped levels
    for j in 1..heading_lines.len() {
        let prev_level = heading_lines[j - 1].1;
        let curr_level = heading_lines[j].1;
        let line_idx = heading_lines[j].0;

        if curr_level > prev_level + 1 {
            // Demote to prev_level + 1
            let new_level = prev_level + 1;
            let new_hashes = "#".repeat(new_level);
            let text = lines[line_idx]
                .trim_start_matches('#')
                .trim_start();
            lines[line_idx] = format!("{} {}", new_hashes, text);
        }
    }

    // Limit heading level to 4
    for line in &mut lines {
        if let Some(caps) = heading_pattern.captures(line) {
            if let (Some(hashes_match), Some(text_match)) = (caps.get(1), caps.get(2)) {
                let hashes = hashes_match.as_str();
                if hashes.len() > 4 {
                    let text = text_match.as_str();
                    *line = format!("#### {}", text);
                }
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
    let link_pattern = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let source_dir = Path::new(source_path).parent().unwrap_or_else(|| Path::new(""));

    let result = link_pattern
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
                    return format!("[{}](./{})", text, mapping.filename);
                }
            }

            // Not found - track as broken
            broken_links.push(target.to_string());
            caps[0].to_string()
        })
        .to_string();

    (result, broken_links)
}

fn generate_tags(analysis: &Analysis) -> Vec<String> {
    let mut tags = vec![analysis.category.clone()];

    // Add heading nouns
    for heading in analysis.headings.iter().take(3) {
        for word in heading.text.split_whitespace() {
            if word.len() > 4 && !is_stopword(word) {
                tags.push(word.to_lowercase());
            }
        }
    }

    // Unique and limit to 5
    tags.sort();
    tags.dedup();
    tags.truncate(5);
    tags
}

fn is_stopword(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "this" | "that" | "these" | "those" | "about" | "guide"
    )
}

/// Safely truncate a string to max_bytes, ensuring we don't split UTF-8 characters
fn safe_truncate(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }

    // Find the largest valid UTF-8 boundary <= max_bytes
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
