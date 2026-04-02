use crate::cache::{composite_hash, CacheType, DocCache};
use crate::config::CategoryConfig;
use crate::discover::DiscoveryFile;
use std::sync::Arc;

use anyhow::Result;
use itertools::Itertools;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use rayon::prelude::*;
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
    pub content: Arc<str>,
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

#[must_use]
pub fn count_categories(analyses: &[Analysis]) -> HashMap<String, usize> {
    analyses
        .iter()
        .map(|a| (a.category.clone(), ()))
        .into_group_map()
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect()
}

pub fn analyze_files(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
) -> Result<AnalyzeResult> {
    let config =
        if let Some(path) = category_config_path {
            use anyhow::Context;
            Some(CategoryConfig::load_from_file(path).with_context(|| {
                format!("Failed to load category config from '{}'", path.display())
            })?)
        } else {
            None
        };

    let input_count = files.len();

    let (analyses, failed_files): (Vec<_>, Vec<_>) = files
        .par_iter()
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

/// Cached version of `analyze_files`.
///
/// For each file, computes `SHA-256(source_path + file_bytes + config_hash)`
/// and checks the cache. Cache hits skip re-parsing entirely.
///
/// Returns `(AnalyzeResult, u64)` where the u64 is the cache hit count.
pub fn analyze_files_cached(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    cache: &DocCache,
) -> Result<(AnalyzeResult, u64)> {
    let config =
        if let Some(path) = category_config_path {
            use anyhow::Context;
            Some(CategoryConfig::load_from_file(path).with_context(|| {
                format!("Failed to load category config from '{}'", path.display())
            })?)
        } else {
            None
        };

    let config_hash = compute_config_hash(category_config_path);
    let input_count = files.len();

    let (analyses, failed_files, hits): (Vec<_>, Vec<_>, u64) = files
        .par_iter()
        .map(|file| {
            let file_path = source_dir.join(&file.source_path);

            // Read file bytes for cache key + cache miss path
            let file_bytes = fs::read(&file_path).map_err(|e| FailedFile {
                source_path: file.source_path.clone(),
                error: e.to_string(),
            })?;

            let cache_key = composite_hash(&[
                file.source_path.as_bytes(),
                &file_bytes,
                config_hash.as_bytes(),
            ]);

            // Check cache first
            if let Some(cached) = cache
                .get::<Analysis>(CacheType::Analysis, cache_key.as_bytes())
                .map_err(|e| FailedFile {
                    source_path: file.source_path.clone(),
                    error: e.to_string(),
                })?
            {
                return Ok((cached, true));
            }

            // Cache miss — run analysis
            let analysis = analyze_single_file(&file.source_path, &file_path, config.as_ref())
                .map_err(|e| FailedFile {
                    source_path: file.source_path.clone(),
                    error: e.to_string(),
                })?;

            // Store in cache (best-effort — don't fail the pipeline on cache write errors)
            let _ = cache.put(CacheType::Analysis, cache_key.as_bytes(), &analysis);

            Ok((analysis, false))
        })
        .fold(
            || (Vec::new(), Vec::new(), 0u64),
            |(mut ok, mut err, mut hits), res: Result<(Analysis, bool), FailedFile>| {
                match res {
                    Ok((analysis, was_cached)) => {
                        if was_cached {
                            hits += 1;
                        }
                        ok.push(analysis);
                    }
                    Err(e) => err.push(e),
                }
                (ok, err, hits)
            },
        )
        .reduce(
            || (Vec::new(), Vec::new(), 0u64),
            |(mut ok1, mut err1, mut hits1), (ok2, err2, hits2)| {
                ok1.extend(ok2);
                err1.extend(err2);
                hits1 += hits2;
                (ok1, err1, hits1)
            },
        );

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

    Ok((
        AnalyzeResult {
            analyses,
            failed_files,
            total_discovered: input_count,
        },
        hits,
    ))
}

/// Compute a deterministic hash of the category config file contents (or empty if none).
fn compute_config_hash(category_config_path: Option<&Path>) -> crate::cache::ContentHash {
    match category_config_path {
        None => crate::cache::content_hash(b""),
        Some(path) => match fs::read(path) {
            Ok(bytes) => crate::cache::content_hash(&bytes),
            Err(_) => crate::cache::content_hash(b""),
        },
    }
}

struct MarkdownMetadata {
    title: Option<String>,
    headings: Vec<Heading>,
    links: Vec<Link>,
    first_paragraph: String,
    has_code: bool,
    has_tables: bool,
}

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
struct MetadataState {
    title: Option<String>,
    headings: Vec<Heading>,
    links: Vec<Link>,
    first_paragraph: String,
    has_code: bool,
    has_tables: bool,
    current_heading: Option<Heading>,
    current_link: Option<Link>,
    in_first_paragraph: bool,
    found_first_paragraph: bool,
}

// AST event accumulation: pulldown-cmark events require sequential stateful
// traversal (heading/link text spans multiple events). Persistent structures
// would add O(log n) overhead per event with no functional benefit.
#[allow(unused_mut)]
#[allow(clippy::too_many_lines)]
fn extract_markdown_metadata(content: &str) -> MarkdownMetadata {
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(
            content
                .match_indices('\n')
                .map(|(i, _)| i.saturating_add(1)),
        )
        .collect();

    let parser = Parser::new(content).into_offset_iter();

    let final_state = parser.fold(MetadataState::default(), |mut state, (event, range)| {
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
                state.current_heading = Some(Heading {
                    level: level_num,
                    text: String::new(),
                    line: line_num.saturating_sub(1),
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut h) = state.current_heading.take() {
                    h.text = h.text.trim().to_string();
                    if h.level == 1 && state.title.is_none() {
                        state.title = Some(h.text.clone());
                    }
                    state.headings.push(h);
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
                state.current_link = Some(Link {
                    text: String::new(),
                    target,
                    kind,
                });
            }
            Event::End(TagEnd::Link) => {
                if let Some(l) = state.current_link.take() {
                    state.links.push(l);
                }
            }
            Event::Start(Tag::Paragraph) => {
                if !state.found_first_paragraph {
                    state.in_first_paragraph = true;
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if state.in_first_paragraph {
                    state.in_first_paragraph = false;
                    state.found_first_paragraph = true;
                }
            }
            Event::Start(Tag::CodeBlock(_)) => state.has_code = true,
            Event::Start(Tag::Table(_)) => state.has_tables = true,
            Event::Text(text) | Event::Code(text) => {
                if let Some(h) = &mut state.current_heading {
                    h.text.push_str(&text);
                }
                if let Some(l) = &mut state.current_link {
                    l.text.push_str(&text);
                }
                if state.in_first_paragraph && state.first_paragraph.len() < 200 {
                    state.first_paragraph.push_str(&text);
                    state.first_paragraph.push(' ');
                }
            }
            _ => {}
        }
        state
    });

    MarkdownMetadata {
        title: final_state.title,
        headings: final_state.headings,
        links: final_state.links,
        first_paragraph: final_state.first_paragraph.trim().to_string(),
        has_code: final_state.has_code,
        has_tables: final_state.has_tables,
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
fn generate_untitled_id(path: &str, content: &str) -> String {
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

fn detect_category(filename: &str, content: &str) -> String {
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frontmatter_tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = "---\ntitle: Test\ncategory: concept\n---\n\n# Body";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.expect("Expected frontmatter");
        assert_eq!(fm.get("title").expect("Expected title"), "Test");
        assert_eq!(fm.get("category").expect("Expected category"), "concept");
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn make_heading(level: u32, text: &str) -> Heading {
        Heading {
            level,
            text: text.to_string(),
            line: 0,
        }
    }

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
    fn test_detect_category_readme() {
        assert_eq!(detect_category("README.md", "some content"), "meta");
    }

    #[test]
    fn test_detect_category_changelog() {
        assert_eq!(detect_category("CHANGELOG.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_contributing() {
        assert_eq!(detect_category("CONTRIBUTING.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_license() {
        assert_eq!(detect_category("LICENSE", "content"), "meta");
    }

    #[test]
    fn test_detect_category_security() {
        assert_eq!(detect_category("SECURITY.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_code_of_conduct() {
        assert_eq!(detect_category("CODE_OF_CONDUCT.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_index_file() {
        assert_eq!(detect_category("INDEX.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_tutorial_content() {
        assert_eq!(
            detect_category("guide.md", "This is a tutorial on testing"),
            "tutorial"
        );
    }

    #[test]
    fn test_detect_category_getting_started() {
        assert_eq!(
            detect_category("start.md", "Getting started with our tool"),
            "tutorial"
        );
    }

    #[test]
    fn test_detect_category_quickstart_content() {
        assert_eq!(
            detect_category("intro.md", "Follow this quickstart guide"),
            "tutorial"
        );
    }

    #[test]
    fn test_detect_category_quickstart_filename() {
        assert_eq!(
            detect_category("quickstart.md", "random content"),
            "tutorial"
        );
    }

    #[test]
    fn test_detect_category_tutorial_filename() {
        assert_eq!(detect_category("tutorial.md", "random content"), "tutorial");
    }

    #[test]
    fn test_detect_category_ref_content() {
        assert_eq!(
            detect_category("docs.md", "The api provides HTTP endpoints"),
            "ref"
        );
    }

    #[test]
    fn test_detect_category_reference_content() {
        assert_eq!(
            detect_category("info.md", "See the reference documentation"),
            "ref"
        );
    }

    #[test]
    fn test_detect_category_function_content() {
        assert_eq!(
            detect_category("lib.md", "The function main() does things"),
            "ref"
        );
    }

    #[test]
    fn test_detect_category_class_content() {
        assert_eq!(
            detect_category("oop.md", "The class Animal has methods"),
            "ref"
        );
    }

    #[test]
    fn test_detect_category_api_filename() {
        assert_eq!(detect_category("api.md", "random content"), "ref");
    }

    #[test]
    fn test_detect_category_reference_filename() {
        assert_eq!(detect_category("reference.md", "random content"), "ref");
    }

    #[test]
    fn test_detect_category_ops_content() {
        assert_eq!(
            detect_category("deploy.md", "This is a how-to guide for deployment"),
            "ops"
        );
    }

    #[test]
    fn test_detect_category_how_to_content() {
        assert_eq!(
            detect_category("steps.md", "how to configure the system"),
            "ops"
        );
    }

    #[test]
    fn test_detect_category_guide_content() {
        assert_eq!(
            detect_category("setup.md", "Follow this guide to install"),
            "ops"
        );
    }

    #[test]
    fn test_detect_category_how_to_filename() {
        assert_eq!(detect_category("how-to-deploy.md", "random"), "ops");
    }

    #[test]
    fn test_detect_category_guide_filename() {
        assert_eq!(detect_category("guide.md", "random"), "ops");
    }

    #[test]
    fn test_detect_category_deployment_filename() {
        assert_eq!(detect_category("deployment.md", "random"), "ops");
    }

    #[test]
    fn test_detect_category_fallback_concept() {
        assert_eq!(
            detect_category("random-file.md", "Just some random content about things."),
            "concept"
        );
    }

    #[test]
    fn test_detect_category_meta_beats_tutorial() {
        assert_eq!(
            detect_category("readme.md", "This is a tutorial getting started guide"),
            "meta"
        );
    }

    #[test]
    fn test_detect_category_tutorial_beats_ref() {
        assert_eq!(
            detect_category("guide.md", "api reference tutorial"),
            "tutorial"
        );
    }

    #[test]
    fn test_extract_frontmatter_with_colon_in_value() {
        let content = "---\ntitle: Hello: World\ndescription: A test: with colons\n---\nBody";
        let (fm_opt, body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.unwrap();
        assert_eq!(fm.get("title").unwrap(), "Hello: World");
        assert_eq!(fm.get("description").unwrap(), "A test: with colons");
        assert_eq!(body.trim(), "Body");
    }

    #[test]
    fn test_extract_frontmatter_no_colon_lines() {
        let content = "---\njust a line without colon\n---\nBody";
        let (fm_opt, _body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.unwrap();
        assert!(fm.is_empty());
    }

    #[test]
    fn test_extract_frontmatter_crlf() {
        let content = "---\r\ntitle: Test\r\n---\r\nBody";
        let (fm_opt, _body) = extract_frontmatter(content);
        assert!(fm_opt.is_some());
        let fm = fm_opt.unwrap();
        assert_eq!(fm.get("title").unwrap(), "Test");
    }

    #[test]
    fn test_generate_untitled_id_deterministic() {
        let id1 = generate_untitled_id("path/to/file.md", "content");
        let id2 = generate_untitled_id("path/to/file.md", "content");
        assert_eq!(id1, id2, "Same input should produce same hash");
    }

    #[test]
    fn test_generate_untitled_id_different_paths() {
        let id1 = generate_untitled_id("path/a.md", "content");
        let id2 = generate_untitled_id("path/b.md", "content");
        assert_ne!(id1, id2, "Different paths should produce different hashes");
    }

    #[test]
    fn test_generate_untitled_id_starts_with_untitled() {
        let id = generate_untitled_id("test.md", "content");
        assert!(id.starts_with("Untitled-"));
    }

    #[test]
    fn test_analyze_single_file_basic() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let file_path = dir.path().join("test.md");
        fs::write(
            &file_path,
            "# Test Document\n\nThis is a paragraph.\n\n## Section\n\nMore text.",
        )?;

        let analysis = analyze_single_file("test.md", &file_path, None)?;

        assert_eq!(analysis.title, "Test Document");
        assert_eq!(analysis.headings.len(), 2);
        assert_eq!(analysis.word_count, 11);
        assert_eq!(analysis.source_path, "test.md");
        assert!(!analysis.first_paragraph.is_empty());
        Ok(())
    }

    #[test]
    fn test_analyze_single_file_with_frontmatter() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let file_path = dir.path().join("test.md");
        fs::write(
            &file_path,
            "---\ntitle: Custom Title\ncategory: tutorial\n---\n\n# Custom Title\n\nContent without heading.",
        )?;

        let analysis = analyze_single_file("test.md", &file_path, None)?;

        assert_eq!(analysis.title, "Custom Title");
        assert!(analysis.frontmatter.is_some());
        let fm = analysis.frontmatter.unwrap();
        assert_eq!(fm.get("title").unwrap(), "Custom Title");
        assert_eq!(fm.get("category").unwrap(), "tutorial");
        Ok(())
    }

    #[test]
    fn test_analyze_single_file_title_from_filename() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let file_path = dir.path().join("my-cool-guide.md");
        fs::write(&file_path, "No heading here, just content.")?;

        let analysis = analyze_single_file("my-cool-guide.md", &file_path, None)?;

        assert_eq!(analysis.title, "My Cool Guide");
        Ok(())
    }

    #[test]
    fn test_analyze_single_file_missing_file() {
        let result = analyze_single_file("missing.md", Path::new("/nonexistent/path.md"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_single_file_code_detection() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let file_path = dir.path().join("test.md");
        fs::write(&file_path, "# Doc\n\n```\ncode block\n```\n\nMore text.")?;

        let analysis = analyze_single_file("test.md", &file_path, None)?;

        assert!(analysis.has_code);
        Ok(())
    }

    #[test]
    fn test_analyze_files_empty() -> Result<()> {
        let result = analyze_files(&[], Path::new("/tmp"), None)?;
        assert!(result.analyses.is_empty());
        assert!(result.failed_files.is_empty());
        assert_eq!(result.total_discovered, 0);
        Ok(())
    }

    #[test]
    fn test_analyze_files_basic() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let file_path = dir.path().join("doc.md");
        fs::write(&file_path, "# My Document\n\nContent paragraph.")?;

        let files = vec![DiscoveryFile {
            source_path: "doc.md".to_string(),
            size_bytes: 10,
        }];

        let result = analyze_files(&files, dir.path(), None)?;

        assert_eq!(result.analyses.len(), 1);
        assert_eq!(result.analyses[0].title, "My Document");
        assert_eq!(result.total_discovered, 1);
        Ok(())
    }

    #[test]
    fn test_analyze_files_with_failed_file() {
        let files = vec![DiscoveryFile {
            source_path: "nonexistent.md".to_string(),
            size_bytes: 10,
        }];

        let result = analyze_files(&files, Path::new("/nonexistent"), None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Failed to analyze any"));
    }

    #[test]
    fn test_analyze_files_mixed_success_failure() -> Result<()> {
        let dir = tempfile::TempDir::new()?;
        let good_file = dir.path().join("good.md");
        fs::write(&good_file, "# Good Doc\n\nContent.")?;

        let files = vec![
            DiscoveryFile {
                source_path: "good.md".to_string(),
                size_bytes: 10,
            },
            DiscoveryFile {
                source_path: "bad.md".to_string(),
                size_bytes: 10,
            },
        ];

        let result = analyze_files(&files, dir.path(), None)?;

        assert_eq!(result.analyses.len(), 1);
        assert_eq!(result.failed_files.len(), 1);
        assert_eq!(result.total_discovered, 2);
        Ok(())
    }

    #[test]
    fn test_analyze_result_len_and_empty() {
        let empty = AnalyzeResult {
            analyses: vec![],
            failed_files: vec![],
            total_discovered: 0,
        };
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let nonempty = AnalyzeResult {
            analyses: vec![Analysis {
                source_path: "a.md".to_string(),
                title: "A".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "c".to_string(),
                content: Arc::from(""),
            }],
            failed_files: vec![],
            total_discovered: 1,
        };
        assert!(!nonempty.is_empty());
        assert_eq!(nonempty.len(), 1);
    }

    #[test]
    fn test_analyze_result_deref() {
        let result = AnalyzeResult {
            analyses: vec![
                Analysis {
                    source_path: "a.md".to_string(),
                    title: "A".to_string(),
                    frontmatter: None,
                    headings: vec![],
                    links: vec![],
                    first_paragraph: String::new(),
                    word_count: 0,
                    has_code: false,
                    has_tables: false,
                    category: "c".to_string(),
                    content: Arc::from(""),
                },
                Analysis {
                    source_path: "b.md".to_string(),
                    title: "B".to_string(),
                    frontmatter: None,
                    headings: vec![],
                    links: vec![],
                    first_paragraph: String::new(),
                    word_count: 0,
                    has_code: false,
                    has_tables: false,
                    category: "c".to_string(),
                    content: Arc::from(""),
                },
            ],
            failed_files: vec![],
            total_discovered: 2,
        };

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "A");
        assert_eq!(result[1].title, "B");
    }

    #[test]
    fn test_count_categories() {
        let analyses = vec![
            Analysis {
                source_path: "a.md".to_string(),
                title: "A".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "tutorial".to_string(),
                content: Arc::from(""),
            },
            Analysis {
                source_path: "b.md".to_string(),
                title: "B".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "tutorial".to_string(),
                content: Arc::from(""),
            },
            Analysis {
                source_path: "c.md".to_string(),
                title: "C".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "ref".to_string(),
                content: Arc::from(""),
            },
        ];

        let counts = count_categories(&analyses);
        assert_eq!(counts.get("tutorial").unwrap(), &2);
        assert_eq!(counts.get("ref").unwrap(), &1);
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn test_count_categories_empty() {
        let counts = count_categories(&[]);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_link_kind_enum() {
        assert_eq!(LinkKind::Internal, LinkKind::Internal);
        assert_eq!(LinkKind::External, LinkKind::External);
        assert_ne!(LinkKind::Internal, LinkKind::External);
    }

    #[test]
    fn test_heading_struct() {
        let h = Heading {
            level: 2,
            text: "Test".to_string(),
            line: 5,
        };
        let cloned = h.clone();
        assert_eq!(h.level, cloned.level);
        assert_eq!(h.text, cloned.text);
    }

    #[test]
    fn test_failed_file_struct() {
        let f = FailedFile {
            source_path: "bad.md".to_string(),
            error: "file not found".to_string(),
        };
        let cloned = f.clone();
        assert_eq!(f.source_path, cloned.source_path);
    }

    #[test]
    fn test_analyze_serialization() {
        let analysis = Analysis {
            source_path: "test.md".to_string(),
            title: "Test".to_string(),
            frontmatter: Some(HashMap::from([("key".to_string(), "val".to_string())])),
            headings: vec![Heading {
                level: 1,
                text: "H1".to_string(),
                line: 0,
            }],
            links: vec![Link {
                text: "link".to_string(),
                target: "url".to_string(),
                kind: LinkKind::External,
            }],
            first_paragraph: "Para".to_string(),
            word_count: 42,
            has_code: true,
            has_tables: false,
            category: "tutorial".to_string(),
            content: Arc::from("content"),
        };

        let json = serde_json::to_string(&analysis).unwrap();
        let deserialized: Analysis = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "Test");
        assert_eq!(deserialized.word_count, 42);
        assert!(deserialized.has_code);
        assert_eq!(deserialized.links[0].kind, LinkKind::External);
    }

    #[test]
    fn test_detect_category_case_insensitive_filename() {
        assert_eq!(detect_category("README.MD", "content"), "meta");
        assert_eq!(detect_category("Readme.md", "content"), "meta");
    }

    #[test]
    fn test_detect_category_with_frontmatter() {
        let content = "---\ntitle: My Tutorial\n---\n\nThis is getting started content.";
        assert_eq!(detect_category("guide.md", content), "tutorial");
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

    #[test]
    fn test_analyze_files_discover_file_struct() {
        let df = DiscoveryFile {
            source_path: "path/to/file.md".to_string(),
            size_bytes: 1024,
        };
        let cloned = df.clone();
        assert_eq!(df.source_path, cloned.source_path);
        assert_eq!(df.size_bytes, cloned.size_bytes);
    }
}
