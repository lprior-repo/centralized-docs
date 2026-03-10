#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
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
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs;
use std::hash::Hasher;
use std::path::Path;

use super::validation::{FilteringMode, PageFilterStatus};

/// Calculate exponential backoff delay with overflow protection
#[must_use]
pub fn calculate_backoff_delay(base_delay_ms: u64, attempt: u32) -> u64 {
    let exponent = attempt.saturating_sub(1).min(62);
    let multiplier = 2_u64.pow(exponent);
    base_delay_ms.saturating_mul(multiplier)
}

/// Convert URL to a filesystem-safe slug
///
/// This function generates a unique slug for each distinct URL, including
/// query parameters and fragments. This prevents slug collisions between
/// URLs like `/docs?page=1` and `/docs?page=2`.
pub fn url_to_slug(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url).context("Failed to parse URL for slug generation")?;

    let path = parsed.path().trim_matches('/');

    let path = path.strip_suffix(".html").map_or(path, |s| s);
    let path = path.strip_suffix(".htm").map_or(path, |s| s);

    let mut raw_slug = path.replace(['/', '.'], "-");

    if let Some(query) = parsed.query() {
        let query_slug = query
            .replace(['=', '&'], "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        if !query_slug.is_empty() {
            raw_slug = format!("{raw_slug}--q-{query_slug}");
        }
    }

    if let Some(fragment) = parsed.fragment() {
        let frag_slug = fragment
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        if !frag_slug.is_empty() {
            raw_slug = format!("{raw_slug}--f-{frag_slug}");
        }
    }

    let slug = raw_slug
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase();

    let slug = slug
        .strip_suffix("-html")
        .map_or(slug.as_str(), |s| s)
        .strip_suffix("-htm")
        .map_or(slug.as_str(), |s| s)
        .to_string();

    // Include query parameters in slug to prevent collisions
    // e.g., /docs?page=1 and /docs?page=2 should have different slugs
    let query = parsed.query();
    let fragment = parsed.fragment();

    let slug = if query.is_some() || fragment.is_some() {
        // Create a short hash of query+fragment to avoid long slugs
        // Use large hash space (10M) to minimize collision probability
        // With 64-bit hash and 10M space, collision probability is ~1 in 10M for random inputs
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        if let Some(q) = query {
            std::hash::Hash::hash(q, &mut hasher);
        }
        if let Some(f) = fragment {
            std::hash::Hash::hash(f, &mut hasher);
        }
        let hash = (hasher.finish() % 10_000_000).to_string();
        format!("{slug}-q{hash}")
    } else {
        slug
    };

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
#[must_use]
pub fn detect_rate_limit_page(html: &str) -> bool {
    let html_lower = html.to_lowercase();
    html_lower.contains("rate limit exceeded")
        || html_lower.contains("429")
        || html_lower.contains("too many requests")
}

/// Statically compiled header regex for extract_headers
static HEADER_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").expect("valid regex"));

/// Extract headers from markdown
#[must_use]
pub fn extract_headers(markdown: &str) -> Vec<super::validation::Header> {
    markdown
        .lines()
        .filter_map(|line| HEADER_REGEX.captures(line.trim()))
        .filter_map(|caps| {
            let level_match = caps.get(1)?;
            let text_match = caps.get(2)?;
            let level = u8::try_from(level_match.as_str().len()).map_or(1, |v| v);
            let text = text_match.as_str().to_string();
            Some(super::validation::Header { level, text })
        })
        .collect::<Vec<_>>()
}

/// Statically compiled link regex for extract_internal_links
static LINK_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("valid regex"));

/// Extract internal links from markdown
#[must_use]
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
#[must_use]
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
        let _ = writeln!(toc, "{}- [{}](#{})", indent, header.text, anchor);
    }
    toc.push_str("\n---\n\n");
    toc
}

/// Find related pages based on shared links
#[must_use]
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

    related.sort_by_key(|b| std::cmp::Reverse(b.0));
    related.into_iter().take(5).map(|(_, page)| page).collect()
}

/// Transform a spider page into [`ScrapedPage`] format
pub fn transform_page(
    page: &spider::page::Page,
    base_url: &str,
    config: &super::validation::ScrapeConfig,
    filtering_mode: FilteringMode,
) -> Result<super::validation::ScrapedPage> {
    let url = page.get_url().to_string();
    let filter_config = FilterConfig::default();

    let raw_html = page.get_html();

    if detect_rate_limit_page(&raw_html) {
        anyhow::bail!("Rate limit page detected for {url} - skipping");
    }

    super::validation::check_html_size(&raw_html, config.max_page_size_bytes)?;

    let filtering_enabled = filtering_mode == FilteringMode::Enabled;

    let prune_result: FilterResult = if filtering_enabled {
        prune_html(&raw_html, &filter_config)
    } else {
        FilterResult {
            html: raw_html,
            removed_count: 0,
            density_score: crate::math_types::Score::try_new(1.0)
                .unwrap_or_else(|_| crate::math_types::Score::zero()),
            used_readability: false,
            is_empty: false,
        }
    };

    let transform_config = spider_transformations::transformation::content::TransformConfig {
        return_format: spider_transformations::transformation::content::ReturnFormat::Markdown,
        ..Default::default()
    };

    let selector_config = if filtering_enabled {
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

    // Validate that content extraction succeeded - must not be empty or whitespace-only
    if markdown.trim().is_empty() {
        anyhow::bail!(
            "transform_content returned empty result for {url} - content extraction failed"
        );
    }

    let (markdown, filter_status) = if filtering_enabled {
        (
            filter_markdown(&markdown, &filter_config),
            PageFilterStatus::Filtered,
        )
    } else {
        (markdown, PageFilterStatus::Unfiltered)
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
        filter_status,
        elements_removed: prune_result.removed_count,
        density_score: prune_result.density_score.value(),
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
    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    let filenames: Vec<String> = all_pages
        .iter()
        .map(|page| {
            let current_count = slug_counts
                .entry(page.slug.clone())
                .and_modify(|count| *count = count.saturating_add(1))
                .or_insert(1);

            if *current_count == 1 {
                format!("{}.md", page.slug)
            } else {
                format!("{}-{}.md", page.slug, *current_count)
            }
        })
        .collect();

    let url_to_filename: HashMap<String, String> = all_pages
        .iter()
        .zip(filenames.iter())
        .map(|(page, filename)| (page.url.clone(), filename.clone()))
        .collect();

    let collision_count = slug_counts.values().filter(|count| **count > 1).count();
    if collision_count > 0 {
        eprintln!(
            "[WARN] Detected {collision_count} slug collision groups; applied numeric filename suffixes"
        );
    }

    for (page, filename) in all_pages.iter().zip(filenames.iter()) {
        let filepath = scrape_dir.join(filename);

        let toc = generate_toc(&page.headers);

        let related = find_related_pages(page, all_pages);

        let related_section = if related.is_empty() {
            String::new()
        } else {
            let mut section = String::from("\n## Related Pages\n\n");
            for related_page in related {
                use std::fmt::Write;
                let related_link = url_to_filename
                    .get(&related_page.url)
                    .map_or_else(|| format!("{}.md", related_page.slug), Clone::clone);
                let _ = writeln!(section, "- [{}]({related_link})", related_page.title);
            }
            section
        };

        let filter_status_str = match page.filter_status {
            PageFilterStatus::Filtered => "true",
            PageFilterStatus::Unfiltered => "false",
        };
        let content = format!(
            "---\nurl: {}\ntitle: {}\nword_count: {}\nfiltered: {}\nelements_removed: {}\ndensity_score: {:.2}\n---\n\n{}{}{}",
            page.url, page.title, page.word_count, filter_status_str, page.elements_removed, page.density_score,
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
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::scrape::validation;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        std::env::temp_dir().join(format!("{prefix}-{nanos}"))
    }

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
    fn test_url_to_slug_with_query_params() {
        // Query parameters should produce unique slugs
        let slug1 = url_to_slug("https://example.com/docs?page=1").unwrap();
        let slug2 = url_to_slug("https://example.com/docs?page=2").unwrap();

        // Slugs should be different
        assert_ne!(slug1, slug2);
        // Both should contain query indicator
        assert!(slug1.contains("-q"), "slug1: {slug1}");
        assert!(slug2.contains("-q"), "slug2: {slug2}");
    }

    #[test]
    fn test_url_to_slug_with_fragment() {
        // Fragments should produce unique slugs
        let slug1 = url_to_slug("https://example.com/docs#section1").unwrap();
        let slug2 = url_to_slug("https://example.com/docs#section2").unwrap();

        // Slugs should be different
        assert_ne!(slug1, slug2);
        // Both should contain the base path with query hash suffix (-q{n})
        assert!(
            slug1.contains("-q")
                && slug1
                    .chars()
                    .skip_while(|c| *c != 'q')
                    .nth(1)
                    .is_some_and(|c| c.is_ascii_digit())
        );
        assert!(
            slug2.contains("-q")
                && slug2
                    .chars()
                    .skip_while(|c| *c != 'q')
                    .nth(1)
                    .is_some_and(|c| c.is_ascii_digit())
        );
    }

    #[test]
    fn test_url_to_slug_no_query_no_suffix() {
        // URLs without query/fragment should NOT have -q suffix
        let slug = url_to_slug("https://example.com/docs").unwrap();
        assert_eq!(slug, "docs");
        assert!(!slug.contains("-q"));
    }

    #[test]
    fn test_url_to_slug_different_paths_different_slugs() {
        // Different paths should still produce different slugs
        let slug1 = url_to_slug("https://example.com/docs?page=1").unwrap();
        let slug2 = url_to_slug("https://example.com/api?page=1").unwrap();

        assert_ne!(slug1, slug2);
    }

    #[test]
    fn test_detect_rate_limit_page() {
        assert!(detect_rate_limit_page("Rate limit exceeded"));
        assert!(detect_rate_limit_page("Too many requests"));
        assert!(detect_rate_limit_page("Error 429"));
        assert!(!detect_rate_limit_page("Normal content"));
    }

    #[test]
    fn test_write_scraped_pages_handles_slug_collisions() {
        let output_dir = unique_temp_dir("doc-transformer-slug-collision");

        let page_a = validation::ScrapedPage {
            url: "https://example.com/a.html".to_string(),
            markdown: "# A".to_string(),
            title: "A".to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count: 1,
            slug: "a".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let page_b = validation::ScrapedPage {
            url: "https://example.com/a.htm".to_string(),
            markdown: "# B".to_string(),
            title: "B".to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count: 1,
            slug: "a".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let scrape_result = validation::ScrapeResult {
            pages: vec![page_a, page_b],
            total_urls: 2,
            success_count: 2,
            error_count: 0,
            errors: Vec::new(),
            base_url: "https://example.com".to_string(),
        };

        let write_result = write_scraped_pages(&scrape_result, &output_dir);
        assert!(write_result.is_ok());

        let scrape_dir = output_dir.join(".scrape");
        assert!(scrape_dir.join("a.md").exists());
        assert!(scrape_dir.join("a-2.md").exists());

        let _ = fs::remove_dir_all(output_dir);
    }

    #[test]
    fn test_write_scraped_pages_related_links_use_disambiguated_filenames() {
        let output_dir = unique_temp_dir("doc-transformer-related-collision");

        let page_a = validation::ScrapedPage {
            url: "https://example.com/a.html".to_string(),
            markdown: "# A\n\ncontent".to_string(),
            title: "A".to_string(),
            links: vec!["https://example.com/shared".to_string()],
            headers: Vec::new(),
            word_count: 2,
            slug: "a".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let page_b = validation::ScrapedPage {
            url: "https://example.com/a.htm".to_string(),
            markdown: "# B\n\ncontent".to_string(),
            title: "B".to_string(),
            links: vec!["https://example.com/shared".to_string()],
            headers: Vec::new(),
            word_count: 2,
            slug: "a".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let scrape_result = validation::ScrapeResult {
            pages: vec![page_a, page_b],
            total_urls: 2,
            success_count: 2,
            error_count: 0,
            errors: Vec::new(),
            base_url: "https://example.com".to_string(),
        };

        let write_result = write_scraped_pages(&scrape_result, &output_dir);
        assert!(write_result.is_ok());

        let scrape_dir = output_dir.join(".scrape");
        let content_a = fs::read_to_string(scrape_dir.join("a.md"));
        let content_b = fs::read_to_string(scrape_dir.join("a-2.md"));

        assert!(content_a.is_ok());
        assert!(content_b.is_ok());

        if let Ok(text) = content_a {
            assert!(text.contains("[B](a-2.md)"));
        }

        if let Ok(text) = content_b {
            assert!(text.contains("[A](a.md)"));
        }

        let _ = fs::remove_dir_all(output_dir);
    }

    /// Test that demonstrates slug collision bug: multiple pages with same slug
    /// should NOT overwrite each other - all pages must be preserved.
    ///
    /// This test covers:
    /// - 3+ pages producing identical slugs
    /// - Query parameters that create slug collisions
    /// - Verifies ALL pages are preserved (not overwritten)
    #[test]
    fn test_slug_collision_prevents_data_loss() {
        let output_dir = unique_temp_dir("doc-transformer-collision-test");

        // Create 3 pages that will all produce the same slug "page"
        // Different URLs with same path produce same slug
        let page1 = validation::ScrapedPage {
            url: "https://example.com/docs/page".to_string(),
            markdown: "# Page 1\n\nFirst page content".to_string(),
            title: "Page 1".to_string(),
            links: Vec::new(),
            headers: vec![validation::Header {
                level: 1,
                text: "Page 1".to_string(),
            }],
            word_count: 4,
            slug: "docs-page".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let page2 = validation::ScrapedPage {
            url: "https://example.com/docs/page.html".to_string(),
            markdown: "# Page 2\n\nSecond page content".to_string(),
            title: "Page 2".to_string(),
            links: Vec::new(),
            headers: vec![validation::Header {
                level: 1,
                text: "Page 2".to_string(),
            }],
            word_count: 4,
            slug: "docs-page".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let page3 = validation::ScrapedPage {
            url: "https://example.com/docs/page.htm".to_string(),
            markdown: "# Page 3\n\nThird page content".to_string(),
            title: "Page 3".to_string(),
            links: Vec::new(),
            headers: vec![validation::Header {
                level: 1,
                text: "Page 3".to_string(),
            }],
            word_count: 4,
            slug: "docs-page".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let scrape_result = validation::ScrapeResult {
            pages: vec![page1, page2, page3],
            total_urls: 3,
            success_count: 3,
            error_count: 0,
            errors: Vec::new(),
            base_url: "https://example.com".to_string(),
        };

        // The bug: without proper collision handling, later pages overwrite earlier ones
        // Expected behavior: all 3 pages should be preserved with unique filenames
        let write_result = write_scraped_pages(&scrape_result, &output_dir);

        // Should succeed - no error should be raised for collisions
        assert!(
            write_result.is_ok(),
            "Should handle slug collisions without error"
        );

        let scrape_dir = output_dir.join(".scrape");

        // Critical assertion: ALL pages must be preserved, not overwritten
        // If the bug exists, only one file would exist (last one wins)
        assert!(
            scrape_dir.join("docs-page.md").exists(),
            "First page should be preserved as docs-page.md"
        );
        assert!(
            scrape_dir.join("docs-page-2.md").exists(),
            "Second page should be preserved as docs-page-2.md"
        );
        assert!(
            scrape_dir.join("docs-page-3.md").exists(),
            "Third page should be preserved as docs-page-3.md"
        );

        // Verify content is NOT overwritten - each file should have unique content
        let content1 = fs::read_to_string(scrape_dir.join("docs-page.md"));
        let content2 = fs::read_to_string(scrape_dir.join("docs-page-2.md"));
        let content3 = fs::read_to_string(scrape_dir.join("docs-page-3.md"));

        assert!(content1.is_ok());
        assert!(content2.is_ok());
        assert!(content3.is_ok());

        // Each file must have its original content, not overwritten by later pages
        assert!(
            content1.as_ref().is_ok_and(|c| c.contains("Page 1")),
            "First page content must be preserved"
        );
        assert!(
            content2.as_ref().is_ok_and(|c| c.contains("Page 2")),
            "Second page content must be preserved"
        );
        assert!(
            content3.as_ref().is_ok_and(|c| c.contains("Page 3")),
            "Third page content must be preserved"
        );

        let _ = fs::remove_dir_all(output_dir);
    }

    /// Test that URLs with query parameters produce collisions and are handled
    #[test]
    fn test_query_param_collision_handling() {
        let output_dir = unique_temp_dir("doc-transformer-query-collision");

        // These URLs all have the same path but different query parameters
        // They should all produce the same slug and NOT overwrite each other
        let page1 = validation::ScrapedPage {
            url: "https://example.com/api/users?id=1".to_string(),
            markdown: "# User 1\n\nFirst user".to_string(),
            title: "User 1".to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count: 3,
            slug: "api-users".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let page2 = validation::ScrapedPage {
            url: "https://example.com/api/users?id=2".to_string(),
            markdown: "# User 2\n\nSecond user".to_string(),
            title: "User 2".to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count: 3,
            slug: "api-users".to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        };

        let scrape_result = validation::ScrapeResult {
            pages: vec![page1, page2],
            total_urls: 2,
            success_count: 2,
            error_count: 0,
            errors: Vec::new(),
            base_url: "https://example.com".to_string(),
        };

        let write_result = write_scraped_pages(&scrape_result, &output_dir);
        assert!(write_result.is_ok());

        let scrape_dir = output_dir.join(".scrape");

        // Both pages should be preserved
        assert!(
            scrape_dir.join("api-users.md").exists(),
            "First query-param page should exist"
        );
        assert!(
            scrape_dir.join("api-users-2.md").exists(),
            "Second query-param page should exist"
        );

        let _ = fs::remove_dir_all(output_dir);
    }
}
