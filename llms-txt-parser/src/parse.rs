//! Parsing logic for llms.txt content.

use crate::types::{Frontmatter, Link, LlmsTxt, Section};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Parse llms.txt from a file.
///
/// # Errors
/// Returns an error if the file cannot be read or parsed.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<LlmsTxt> {
    let content = fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read {}", path.as_ref().display()))?;
    parse_content(&content)
}

/// Parse llms.txt from string content.
///
/// # Errors
/// Returns an error if the content is not a valid llms.txt.
pub fn parse_content(content: &str) -> Result<LlmsTxt> {
    let (frontmatter, body) = extract_frontmatter(content)?;
    let lines: Vec<&str> = body.lines().collect();

    let (project_name, description, section_start) = parse_header(&lines);
    let sections = parse_sections(&lines[section_start..]);

    Ok(LlmsTxt {
        frontmatter,
        project_name,
        description,
        sections,
    })
}

/// Extract H1 project name and blockquote description from header lines.
/// Returns `(project_name, description, index_of_first_section_line)`.
fn parse_header(lines: &[&str]) -> (String, Option<String>, usize) {
    let header_end = lines
        .iter()
        .position(|line| line.trim().starts_with("## "))
        .map_or(lines.len(), |i| i);

    let project_name = lines[..header_end]
        .iter()
        .find_map(|line| line.trim().strip_prefix("# "))
        .map_or_else(String::new, |s| s.trim().to_string());

    let description = lines[..header_end]
        .iter()
        .find_map(|line| line.trim().strip_prefix("> "))
        .map(|s| s.trim().to_string());

    (project_name, description, header_end)
}

/// Parse all `## ` sections from lines.
fn parse_sections(lines: &[&str]) -> Vec<Section> {
    let boundaries: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| line.trim().starts_with("## ").then_some(i))
        .collect();

    boundaries
        .iter()
        .enumerate()
        .map(|(idx, &start)| {
            let title = lines[start]
                .trim()
                .strip_prefix("## ")
                .map(str::trim)
                .map_or("", |s| s);
            let end = boundaries.get(idx + 1).copied().map_or(lines.len(), |i| i);
            build_section(title, &lines[start + 1..end])
        })
        .collect()
}

/// Build a [`Section`] from its title and body lines.
fn build_section(title: &str, lines: &[&str]) -> Section {
    let trimmed: Vec<&str> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let links: Vec<Link> = trimmed
        .iter()
        .filter_map(|line| line.strip_prefix("- ").and_then(parse_link))
        .collect();

    let content: String = trimmed
        .iter()
        .filter(|line| line.strip_prefix("- ").and_then(parse_link).is_none())
        .map(|line| format!("{line}\n"))
        .collect();

    Section {
        title: title.to_string(),
        content,
        links,
    }
}

/// Extract YAML frontmatter if present.
fn extract_frontmatter(content: &str) -> Result<(Option<Frontmatter>, String)> {
    let lines: Vec<&str> = content.lines().collect();

    if lines.first().is_none_or(|first| first.trim() != "---") {
        return Ok((None, content.to_string()));
    }

    let end_idx = lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim() == "---")
        .map(|(i, _)| i);

    let Some(end) = end_idx else {
        return Ok((None, content.to_string()));
    };

    let fm_content = lines[1..end].join("\n");
    let frontmatter: Frontmatter =
        serde_yaml::from_str(&fm_content).with_context(|| "Failed to parse YAML frontmatter")?;

    let body = lines[end + 1..].join("\n");

    Ok((Some(frontmatter), body))
}

/// Parse a markdown link: `[text](url)` or `[text](url): description`.
///
/// Strips inline markdown formatting from the description text.
pub(crate) fn parse_link(text: &str) -> Option<Link> {
    let text = text.trim();

    let bracket_start = text.find('[')?;
    let bracket_end = text[bracket_start..].find("](")?;
    let link_text = text[bracket_start + 1..bracket_start + bracket_end].to_string();

    let paren_start = bracket_start + bracket_end + 2;
    let paren_end = text[paren_start..].find(')')?;
    let url = text[paren_start..paren_start + paren_end].to_string();

    let raw_description = text
        .get(paren_start + paren_end + 1..)
        .map_or("", str::trim);
    let description = strip_markdown(
        raw_description
            .strip_prefix(':')
            .map_or(raw_description, str::trim),
    );

    let description = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    Some(Link {
        text: link_text,
        url,
        description,
    })
}

/// Strip markdown formatting markers: bold, italic, code, strikethrough, and inline links.
fn strip_markdown(text: &str) -> String {
    strip_inline_links(text)
        .replace("**", "")
        .replace(['*', '`'], "")
        .replace("~~", "")
}

/// Replace `[text](url)` patterns with just `text`.
fn strip_inline_links(text: &str) -> String {
    text.split('[')
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                return part.to_string();
            }
            match part.split_once("](") {
                Some((link_text, rest)) => match rest.find(')') {
                    Some(pos) => format!("{link_text}{}", &rest[pos + 1..]),
                    None => format!("[{part}"),
                },
                None => format!("[{part}"),
            }
        })
        .collect()
}
