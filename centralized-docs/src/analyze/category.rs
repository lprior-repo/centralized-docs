use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::config::CategoryConfig;

use super::extract::extract_markdown_metadata;
use super::types::Analysis;

pub fn analyze_single_file(
    source_path: &str,
    file_path: &Path,
    category_config: Option<&CategoryConfig>,
) -> Result<Analysis> {
    let content = fs::read_to_string(file_path)?;

    let (frontmatter, clean_content) = extract_frontmatter(&content);
    let metadata = extract_markdown_metadata(&clean_content);

    let title = match metadata.title {
        Some(t) => t,
        None => Path::new(source_path)
            .file_stem()
            .filter(|s| !s.is_empty())
            .map_or_else(
                || generate_untitled_id(source_path, &content),
                |s| {
                    let s = s.to_string_lossy().replace(['-', '_'], " ");
                    s.split_whitespace()
                        .map(|w| {
                            let first = w.chars().next();
                            match first {
                                None => String::new(),
                                Some(f) => {
                                    f.to_uppercase().collect::<String>() + &w[f.len_utf8()..]
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                },
            ),
    };

    let word_count = clean_content.split_whitespace().count();

    let category = if let Some(config) = category_config {
        let filename = Path::new(source_path)
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid path: no filename in {source_path}"))?
            .to_string_lossy();
        config.detect_category(&filename, &clean_content, source_path)
    } else {
        detect_category(source_path, &clean_content)
    };

    Ok(Analysis {
        source_path: source_path.to_string(),
        title,
        frontmatter,
        headings: metadata.headings,
        links: metadata.links,
        first_paragraph: metadata.first_paragraph,
        word_count,
        has_code: metadata.has_code,
        has_tables: metadata.has_tables,
        category,
        content: clean_content.into(),
    })
}

// I/O boundary: std::hash::Hash requires &mut Hasher — no functional alternative exists.
#[allow(unused_mut)]
pub(super) fn generate_untitled_id(path: &str, content: &str) -> String {
    use std::hash::{Hash, Hasher};
    let hash_val = [path, content]
        .iter()
        .fold(
            std::collections::hash_map::DefaultHasher::new(),
            |mut h, &s| {
                s.hash(&mut h);
                h
            },
        )
        .finish();
    format!("Untitled-{hash_val:x}")
}

pub(super) fn extract_frontmatter(content: &str) -> (Option<HashMap<String, String>>, String) {
    if !content.starts_with("---") {
        return (None, content.to_string());
    }

    let lines: Vec<&str> = content.lines().collect();

    let end_idx = lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.starts_with("---"))
        .map(|(i, _)| i);

    let Some(end_idx) = end_idx else {
        return (None, content.to_string());
    };

    let fm: HashMap<String, String> = if lines.len() >= 2 && end_idx > 1 {
        lines[1..end_idx]
            .iter()
            .filter_map(|line| {
                let pos = line.find(':')?;
                let key = line[..pos].trim().to_string();
                let val = line
                    .get(pos.saturating_add(1)..)
                    .map_or("", |s| s)
                    .trim()
                    .to_string();
                Some((key, val))
            })
            .collect()
    } else {
        HashMap::new()
    };

    let remaining = lines
        .get(end_idx.saturating_add(1)..)
        .map_or_else(String::new, |slice| slice.join("\n"));
    (Some(fm), remaining)
}

pub(super) fn detect_category(filename: &str, content: &str) -> String {
    let fname_lower = Path::new(filename)
        .file_stem()
        .filter(|s| !s.is_empty())
        .map_or_else(
            || generate_untitled_id(filename, content),
            |s| s.to_string_lossy().into_owned(),
        )
        .to_lowercase();

    let (_, clean_content) = extract_frontmatter(content);

    // Prevent massive memory allocation on large files by only checking the first ~5000 chars
    let content_lower: String = clean_content
        .chars()
        .take(5000)
        .flat_map(char::to_lowercase)
        .collect();

    if matches!(
        fname_lower.as_str(),
        "readme"
            | "changelog"
            | "contributing"
            | "license"
            | "security"
            | "code_of_conduct"
            | "index"
    ) {
        return "meta".to_string();
    }

    if content_lower.contains("tutorial")
        || content_lower.contains("getting started")
        || content_lower.contains("quickstart")
        || fname_lower.contains("tutorial")
        || fname_lower.contains("quickstart")
    {
        return "tutorial".to_string();
    }

    if content_lower.contains("api")
        || content_lower.contains("reference")
        || content_lower.contains("function ")
        || content_lower.contains("class ")
        || fname_lower.contains("api")
        || fname_lower.contains("reference")
    {
        return "ref".to_string();
    }

    if content_lower.contains("how-to")
        || content_lower.contains("how to")
        || content_lower.contains("guide")
        || content_lower.contains("deployment")
        || fname_lower.contains("how-to")
        || fname_lower.contains("guide")
        || fname_lower.contains("deployment")
    {
        return "ops".to_string();
    }

    "concept".to_string()
}
