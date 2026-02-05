#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::format_push_string)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

//! Data transformation utilities
//!
//! Provides URL-to-slug conversion, exponential backoff calculation,
//! table of contents generation, and related page finding.

use crate::filter::filter_markdown;
use crate::filter::{prune_html, FilterConfig, FilterResult};
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

#[expect(clippy::expect_used)]
static HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").expect("hardcoded regex pattern is valid"));

#[expect(clippy::expect_used)]
static LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("hardcoded regex pattern is valid")
});

/// Calculate exponential backoff delay with overflow protection
pub fn calculate_backoff_delay(base_delay_ms: u64, attempt: u32) -> u64 {
    let exponent = attempt.saturating_sub(1).min(62);
    let multiplier = 2_u64.pow(exponent);
    base_delay_ms.saturating_mul(multiplier)
}

/// Convert URL to a filesystem-safe slug
pub fn url_to_slug(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url).context("Failed to parse URL for slug generation")?;

    let path = parsed.path().trim_matches('/');

    let path = path.strip_suffix(".html").unwrap_or(path);
    let path = path.strip_suffix(".htm").unwrap_or(path);

    let raw_slug = path.replace(['/', '.'], "-");

    let slug = raw_slug
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase();

    let slug = slug
        .strip_suffix("-html")
        .unwrap_or(&slug)
        .strip_suffix("-htm")
        .unwrap_or(&slug)
        .to_string();

    let slug = if slug.len() > 200 {
        // Safe truncation at character boundary (BEAD-001 fix)
        let boundary = slug
            .char_indices()
            .take(200)
            .last()
            .map_or(slug.len(), |(i, c)| i.saturating_add(c.len_utf8()));
        slug[..boundary].to_string()
    } else {
        slug
    };

    let slug = if slug.trim().is_empty() {
        "index".to_string()
    } else {
        slug
    };

    super::validation::validate_slug(&slug)?;

    Ok(slug)
}

/// Detect if a page is a rate limit response
pub fn detect_rate_limit_page(html: &str) -> bool {
    let html_lower = html.to_lowercase();
    html_lower.contains("rate limit exceeded")
        || html_lower.contains("429")
        || html_lower.contains("too many requests")
}

/// Extract headers from markdown
pub fn extract_headers(markdown: &str) -> Vec<super::validation::Header> {
    let mut headers = Vec::new();

    markdown
        .lines()
        .filter_map(|line| HEADER_REGEX.captures(line.trim()))
        .filter_map(|caps| {
            let level_match = caps.get(1)?;
            let text_match = caps.get(2)?;
            let level = u8::try_from(level_match.as_str().len()).unwrap_or(1);
            let text = text_match.as_str().to_string();
            Some(super::validation::Header { level, text })
        })
        .for_each(|header| headers.push(header));

    headers
}

/// Extract internal links from markdown
pub fn extract_internal_links(markdown: &str, base_url: &str) -> Vec<String> {
    let base = url::Url::parse(base_url).ok();
    let mut links = Vec::new();

    for caps in LINK_REGEX.captures_iter(markdown) {
        if let Some(href_match) = caps.get(2) {
            let href = href_match.as_str();

            if let Some(ref base) = base {
                if let Ok(resolved) = base.join(href) {
                    if resolved.host() == base.host() {
                        links.push(resolved.to_string());
                    }
                }
            }

            if href.starts_with('/') || href.starts_with("./") {
                links.push(href.to_string());
            }
        }
    }

    links.sort();
    links.dedup();
    links
}

/// Generate table of contents from headers
pub fn generate_toc(headers: &[super::validation::Header]) -> String {
    if headers.is_empty() {
        return String::new();
    }

    let mut toc = String::from("## Table of Contents\n\n");
    for header in headers {
        let indent = "  ".repeat(header.level.saturating_sub(1) as usize);
        let anchor = header
            .text
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        toc.push_str(&format!("{}- [{}](#{})\n", indent, header.text, anchor));
    }
    toc.push_str("\n---\n\n");
    toc
}

/// Find related pages based on shared links
pub fn find_related_pages<'a>(
    current_page: &super::validation::ScrapedPage,
    all_pages: &'a [super::validation::ScrapedPage],
) -> Vec<&'a super::validation::ScrapedPage> {
    let current_links: HashSet<_> = current_page.links.iter().collect();

    let mut related: Vec<_> = all_pages
        .iter()
        .filter(|p| p.url != current_page.url)
        .map(|p| {
            let page_links: HashSet<_> = p.links.iter().collect();
            let shared = current_links.intersection(&page_links).count();
            (shared, p)
        })
        .filter(|(shared, _)| *shared > 0)
        .collect();

    related.sort_by(|a, b| b.0.cmp(&a.0));
    related.into_iter().take(5).map(|(_, page)| page).collect()
}

/// Transform a spider page into [`ScrapedPage`] format
pub fn transform_page(
    page: &spider::page::Page,
    base_url: &str,
    enable_filtering: bool,
) -> Result<super::validation::ScrapedPage> {
    let url = page.get_url().to_string();
    let filter_config = FilterConfig::default();

    let raw_html = page.get_html();

    if detect_rate_limit_page(&raw_html) {
        anyhow::bail!("Rate limit page detected for {url} - skipping");
    }

    let config = super::validation::ScrapeConfig::default();
    super::validation::check_html_size(&raw_html, config.max_page_size_bytes)?;

    let prune_result: FilterResult = if enable_filtering {
        prune_html(&raw_html, &filter_config)
    } else {
        FilterResult {
            html: raw_html,
            removed_count: 0,
            density_score: 1.0,
            used_readability: false,
        }
    };

    let transform_config = spider_transformations::transformation::content::TransformConfig {
        return_format: spider_transformations::transformation::content::ReturnFormat::Markdown,
        ..Default::default()
    };

    let selector_config = if enable_filtering {
        let mut exclude_tags: Vec<String> = filter_config.remove_tags.clone();
        for pattern in &filter_config.nav_patterns {
            exclude_tags.push(format!(".{pattern}"));
            exclude_tags.push(format!("#{pattern}"));
        }
        Some(
            spider_transformations::transformation::content::SelectorConfiguration {
                root_selector: None,
                exclude_selector: Some(exclude_tags.join(", ")),
            },
        )
    } else {
        None
    };

    let markdown = spider_transformations::transformation::content::transform_content(
        page,
        &transform_config,
        &None,
        &selector_config,
        &None,
    );

    let (markdown, filtered) = if enable_filtering {
        (filter_markdown(&markdown, &filter_config), true)
    } else {
        (markdown, false)
    };

    super::validation::check_markdown_size(&markdown, config.max_markdown_size_bytes)?;

    let title = super::validation::extract_title(&markdown, &url);

    let headers = extract_headers(&markdown);

    let links = extract_internal_links(&markdown, base_url);
    let (links, was_truncated) =
        super::validation::limit_links_per_page(links, config.max_links_per_page);
    if was_truncated {
        eprintln!(
            "[WARN] Page {} had too many links, truncated to {}",
            url, config.max_links_per_page
        );
    }

    let word_count = markdown.split_whitespace().count();

    let slug = url_to_slug(&url).context(format!(
        "Failed to generate slug for URL {url}: ensure URL has a valid path or hostname"
    ))?;

    Ok(super::validation::ScrapedPage {
        url,
        markdown,
        title,
        links,
        headers,
        word_count,
        slug,
        filtered,
        elements_removed: prune_result.removed_count,
        density_score: prune_result.density_score,
    })
}

/// Write scraped pages to output directory with TOC and related pages
pub fn write_scraped_pages(
    result: &super::validation::ScrapeResult,
    output_dir: &Path,
) -> Result<()> {
    let scrape_dir = output_dir.join(".scrape");
    fs::create_dir_all(&scrape_dir)?;

    let all_pages = &result.pages;

    for page in all_pages {
        let filename = format!("{}.md", page.slug);
        let filepath = scrape_dir.join(&filename);

        let toc = generate_toc(&page.headers);

        let related = find_related_pages(page, all_pages);

        let related_section = if related.is_empty() {
            String::new()
        } else {
            let mut section = String::from("\n## Related Pages\n\n");
            for related_page in related {
                section.push_str(&format!(
                    "- [{}]({})\n",
                    related_page.title, related_page.slug
                ));
            }
            section
        };

        let content = format!(
            "---\nurl: {}\ntitle: {}\nword_count: {}\nfiltered: {}\nelements_removed: {}\ndensity_score: {:.2}\n---\n\n{}{}{}",
            page.url, page.title, page.word_count, page.filtered, page.elements_removed, page.density_score,
            toc, page.markdown, related_section
        );

        fs::write(&filepath, content)?;
    }

    let manifest = serde_json::to_string_pretty(result)?;
    fs::write(scrape_dir.join("manifest.json"), manifest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_backoff_delay() {
        assert_eq!(calculate_backoff_delay(2000, 1), 2000);
        assert_eq!(calculate_backoff_delay(2000, 2), 4000);
        assert_eq!(calculate_backoff_delay(2000, 3), 8000);
    }

    #[test]
    fn test_extract_headers() {
        let md = "# H1\n\n## H2\n\n### H3";
        let headers = extract_headers(md);
        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0].level, 1);
        assert_eq!(headers[0].text, "H1");
    }

    #[test]
    fn test_url_to_slug_basic() {
        let result = url_to_slug("https://example.com/docs/getting-started");
        assert!(result.is_ok());
        if let Ok(slug) = result {
            assert_eq!(slug, "docs-getting-started");
        }
    }

    #[test]
    fn test_detect_rate_limit_page() {
        assert!(detect_rate_limit_page("Rate limit exceeded"));
        assert!(detect_rate_limit_page("Too many requests"));
        assert!(detect_rate_limit_page("Error 429"));
        assert!(!detect_rate_limit_page("Normal content"));
    }
}
