use crate::config::CategoryConfig;
use crate::discover::DiscoveryFile;
use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub level: u32,
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkKind {
    Internal,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub target: String,
    pub kind: LinkKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub source_path: String,
    pub title: String,
    pub frontmatter: Option<HashMap<String, String>>,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
    pub first_paragraph: String,
    pub word_count: usize,
    pub has_code: bool,
    pub has_tables: bool,
    pub category: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedFile {
    pub source_path: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub analyses: Vec<Analysis>,
    pub failed_files: Vec<FailedFile>,
    pub total_discovered: usize,
}

impl AnalyzeResult {
    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.analyses.len()
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.analyses.is_empty()
    }
}

impl std::ops::Deref for AnalyzeResult {
    type Target = Vec<Analysis>;

    fn deref(&self) -> &Self::Target {
        &self.analyses
    }
}

pub fn count_categories(analyses: &[Analysis]) -> HashMap<String, usize> {
    analyses
        .iter()
        .fold(HashMap::new(), |mut acc, analysis| {
            *acc.entry(analysis.category.clone()).or_insert(0) =
                acc.get(&analysis.category).unwrap_or(&0).saturating_add(1);
            acc
        })
}

pub fn analyze_files(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
) -> Result<AnalyzeResult> {
    let config = if let Some(path) = category_config_path {
        Some(CategoryConfig::load_from_file(path)?)
    } else {
        None
    };

    let input_count = files.len();

    let (analyses, failed_files): (Vec<_>, Vec<_>) = files
        .iter()
        .map(|file| {
            let file_path = source_dir.join(&file.source_path);
            analyze_single_file(&file.source_path, &file_path, config.as_ref()).map_err(|e| {
                FailedFile {
                    source_path: file.source_path.clone(),
                    error: e.to_string(),
                }
            })
        })
        .partition(Result::is_ok);

    let analyses: Vec<_> = analyses.into_iter().filter_map(Result::ok).collect();
    let failed_files: Vec<_> = failed_files.into_iter().filter_map(Result::err).collect();

    if input_count > 0 && analyses.is_empty() {
        let error_summary = failed_files
            .iter()
            .map(|f| format!("{}: {}", f.source_path, f.error))
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!(
            "Failed to analyze any of the {input_count} discovered file(s). \
            Check file permissions, encoding (files must be valid UTF-8), \
            and that files are not corrupted. Errors: {error_summary}"
        );
    }

    Ok(AnalyzeResult {
        analyses,
        failed_files,
        total_discovered: input_count,
    })
}

struct MarkdownMetadata {
    title: Option<String>,
    headings: Vec<Heading>,
    links: Vec<Link>,
    first_paragraph: String,
    has_code: bool,
    has_tables: bool,
}

fn extract_markdown_metadata(content: &str) -> MarkdownMetadata {
    let mut title = None;
    let mut headings = Vec::new();
    let mut links = Vec::new();
    let mut first_paragraph = String::new();
    let mut has_code = false;
    let mut has_tables = false;

    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(content.match_indices('\n').map(|(i, _)| i.saturating_add(1)))
        .collect();

    let parser = Parser::new(content).into_offset_iter();

    let mut current_heading: Option<Heading> = None;
    let mut current_link: Option<Link> = None;
    let mut in_first_paragraph = false;
    let mut found_first_paragraph = false;

    for (event, range) in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let level_num = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                let line_num = line_starts.partition_point(|&x| x <= range.start);
                current_heading = Some(Heading {
                    level: level_num,
                    text: String::new(),
                    line: line_num.saturating_sub(1),
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut h) = current_heading.take() {
                    h.text = h.text.trim().to_string();
                    if h.level == 1 && title.is_none() {
                        title = Some(h.text.clone());
                    }
                    headings.push(h);
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let target = dest_url.to_string();
                let kind = if target.starts_with("http://")
                    || target.starts_with("https://")
                    || target.starts_with("mailto:")
                {
                    LinkKind::External
                } else {
                    LinkKind::Internal
                };
                current_link = Some(Link {
                    text: String::new(),
                    target,
                    kind,
                });
            }
            Event::End(TagEnd::Link) => {
                if let Some(l) = current_link.take() {
                    links.push(l);
                }
            }
            Event::Start(Tag::Paragraph) => {
                if !found_first_paragraph {
                    in_first_paragraph = true;
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if in_first_paragraph {
                    in_first_paragraph = false;
                    found_first_paragraph = true;
                }
            }
            Event::Start(Tag::CodeBlock(_)) => has_code = true,
            Event::Start(Tag::Table(_)) => has_tables = true,
            Event::Text(text) | Event::Code(text) => {
                if let Some(h) = &mut current_heading {
                    h.text.push_str(&text);
                }
                if let Some(l) = &mut current_link {
                    l.text.push_str(&text);
                }
                if in_first_paragraph && first_paragraph.len() < 200 {
                    first_paragraph.push_str(&text);
                    first_paragraph.push(' ');
                }
            }
            _ => {}
        }
    }

    MarkdownMetadata {
        title,
        headings,
        links,
        first_paragraph: first_paragraph.trim().to_string(),
        has_code,
        has_tables,
    }
}

fn analyze_single_file(
    source_path: &str,
    file_path: &Path,
    category_config: Option<&CategoryConfig>,
) -> Result<Analysis> {
    let content = fs::read_to_string(file_path)?;

    let (frontmatter, clean_content) = extract_frontmatter(&content);
    let metadata = extract_markdown_metadata(&clean_content);

    let title = metadata.title.unwrap_or_else(|| {
        Path::new(source_path)
            .file_stem()
            .filter(|s| !s.is_empty())
            .map_or_else(
                || "Untitled".to_string(),
                |s| {
                    let s = s.to_string_lossy().replace(['-', '_'], " ");
                    s.split_whitespace()
                        .map(|w| {
                            let mut chars = w.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                },
            )
    });

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
        content: clean_content,
    })
}

fn extract_frontmatter(content: &str) -> (Option<HashMap<String, String>>, String) {
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

    let mut fm = HashMap::new();
    if lines.len() >= 2 && end_idx > 1 {
        fm = lines[1..end_idx]
            .iter()
            .filter_map(|line| {
                let pos = line.find(':')?;
                let key = line[..pos].trim().to_string();
                let val = line
                    .get(pos.saturating_add(1)..)
                    .unwrap_or("")
                    .trim()
                    .to_string();
                Some((key, val))
            })
            .collect();
    }

    let remaining = lines
        .get(end_idx.saturating_add(1)..)
        .map(|slice| slice.join("\n"))
        .unwrap_or_default();
    (Some(fm), remaining)
}

fn detect_category(filename: &str, content: &str) -> String {
    let fname_lower = Path::new(filename)
        .file_stem()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| std::ffi::OsStr::new("untitled"))
        .to_string_lossy()
        .to_lowercase();

    let content_lower = content.to_lowercase();

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

#[cfg(test)]
mod frontmatter_tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = "---\ntitle: Test\ncategory: concept\n---\n\n# Body";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.unwrap();
        assert_eq!(fm.get("title").unwrap(), "Test");
        assert_eq!(fm.get("category").unwrap(), "concept");
        assert_eq!(body.trim(), "# Body");
    }

    #[test]
    fn test_extract_frontmatter_empty() {
        let content = "---\n---\n# Body";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.unwrap();
        assert!(fm.is_empty());
        assert_eq!(body.trim(), "# Body");
    }

    #[test]
    fn test_extract_frontmatter_missing() {
        let content = "# Body without frontmatter\nLine 2";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_none());
        assert_eq!(body.trim(), "# Body without frontmatter\nLine 2");
    }

    #[test]
    fn test_extract_frontmatter_unclosed() {
        let content = "---\ntitle: Test\n\n# Body";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_none());
        assert_eq!(body.trim(), "---\ntitle: Test\n\n# Body");
    }
}
