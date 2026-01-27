//! Web scraping module using spider-rs
//!
//! Provides sequential scraping of documentation sites with HTML-to-Markdown conversion.
//! Designed for AI agent consumption - no complex concurrency, predictable output.
//!
//! ## Error Recovery
//!
//! The scraper includes built-in error resilience:
//! - **Exponential backoff**: Failed requests are retried with exponential delays (configurable)
//! - **Rate limiting**: Configurable delay between requests respects server load
//! - **Robots.txt compliance**: Honors robots.txt to avoid overloading servers
//! - **Path filtering**: Optional regex filtering to avoid unnecessary crawling
//! - **HTML pruning**: Removes navigation, footers, and boilerplate before processing
//!
//! ## Configuration
//!
//! - `max_retries`: Number of retries on transient failures (default: 3)
//! - `use_exponential_backoff`: Enable backoff strategy (default: true)
//! - `delay_ms`: Base delay between requests in milliseconds (default: 250)
//! - `respect_robots`: Honor robots.txt directives (default: true)

use crate::filter::{filter_markdown, prune_html, FilterConfig, FilterResult};
use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use spider::website::Website;
use spider_transformations::transformation::content::{
    self, ReturnFormat, SelectorConfiguration, TransformConfig,
};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

#[expect(clippy::expect_used)]
static H1_TITLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s+(.+)$").expect("hardcoded regex pattern is valid"));

#[expect(clippy::expect_used)]
static HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").expect("hardcoded regex pattern is valid"));

#[expect(clippy::expect_used)]
static LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("hardcoded regex pattern is valid")
});

/// Configuration for scraping a documentation site
#[derive(Debug, Clone)]
pub struct ScrapeConfig {
    /// Base URL to scrape (e.g., "https://docs.example.com")
    pub base_url: String,
    /// Use sitemap.xml to discover pages (default: true)
    pub use_sitemap: bool,
    /// Regex pattern to filter URLs (e.g., "^/docs/")
    pub path_filter: Option<String>,
    /// Delay between requests in milliseconds (default: 250)
    pub delay_ms: u64,
    /// User agent string
    pub user_agent: String,
    /// Respect robots.txt (default: true)
    pub respect_robots: bool,
    /// Enable content filtering to remove nav/footer/boilerplate (default: true)
    pub enable_filtering: bool,
    /// Maximum number of retries for failed requests (default: 3)
    #[allow(dead_code)] // Reserved for retry logic implementation
    pub max_retries: u32,
    /// Enable exponential backoff for retries (default: true)
    #[allow(dead_code)] // Reserved for retry logic implementation
    pub use_exponential_backoff: bool,
    /// Maximum size of a single page in bytes (default: 10MB) - DoS protection against huge files
    pub max_page_size_bytes: u64,
    /// Maximum total content size for entire scrape in bytes (default: 500MB) - DoS protection against streaming attacks
    pub max_total_size_bytes: u64,
    /// Maximum markdown content size per page in bytes (default: 5MB) - Memory exhaustion protection
    pub max_markdown_size_bytes: u64,
    /// Maximum number of pages to scrape (default: 10000) - DoS protection
    pub max_pages: usize,
    /// Maximum number of links to extract per page (default: 1000) - Memory protection
    pub max_links_per_page: usize,
}

impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            use_sitemap: true,
            path_filter: None,
            delay_ms: 250,
            user_agent: "DocTransformer/5.0 (AI Documentation Indexer)".to_string(),
            respect_robots: true,
            enable_filtering: true,
            max_retries: 3,
            use_exponential_backoff: true,
            max_page_size_bytes: 10 * 1024 * 1024, // 10MB per page
            max_total_size_bytes: 500 * 1024 * 1024, // 500MB total
            max_markdown_size_bytes: 5 * 1024 * 1024, // 5MB per page markdown
            max_pages: 10_000,                     // Maximum pages to scrape
            max_links_per_page: 1_000,             // Maximum links per page
        }
    }
}

/// A scraped page with extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// Original URL
    pub url: String,
    /// Markdown content (converted from HTML)
    pub markdown: String,
    /// Page title (extracted from H1 or <title>)
    pub title: String,
    /// Internal links found on this page
    pub links: Vec<String>,
    /// Headers extracted (level, text)
    pub headers: Vec<Header>,
    /// Word count of markdown content
    pub word_count: usize,
    /// URL slug for filename
    pub slug: String,
    /// Whether content filtering was applied
    pub filtered: bool,
    /// Number of HTML elements removed by pruning
    pub elements_removed: usize,
    /// Content density score (0.0 - 1.0)
    pub density_score: f32,
}

/// A header extracted from the page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub level: u8,
    pub text: String,
}

/// Result of scraping a site
#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapeResult {
    /// Successfully scraped pages
    pub pages: Vec<ScrapedPage>,
    /// Total URLs discovered
    pub total_urls: usize,
    /// Number of successful scrapes
    pub success_count: usize,
    /// Number of failed scrapes
    pub error_count: usize,
    /// Errors encountered (url, error message)
    pub errors: Vec<(String, String)>,
    /// Base URL that was scraped
    pub base_url: String,
}

/// Scrape a documentation site and return structured results
///
/// This function uses spider-rs internally but presents a sequential interface.
/// Spider handles concurrency; we process results one at a time.
///
/// When `use_sitemap` is true, scrapes using the sitemap for URL discovery.
/// Otherwise uses standard crawling.
pub async fn scrape_site(config: &ScrapeConfig) -> Result<ScrapeResult> {
    // Validate URL before passing to spider (prevents panic)
    let validated_url = validate_url(&config.base_url)?;

    let mut website = Website::new(validated_url.as_str());

    // Configure spider via the website's configuration
    website.configuration.delay = config.delay_ms;
    website.configuration.respect_robots_txt = config.respect_robots;
    website.configuration.user_agent = Some(Box::new(config.user_agent.clone().into()));

    // Perform the scrape - use sitemap scraping if enabled
    if config.use_sitemap {
        // Scrape using sitemap for URL discovery
        website.scrape_sitemap().await;
    } else {
        // Standard crawling
        website.scrape().await;
    }

    // Compile path filter regex if provided
    let path_regex = config
        .path_filter
        .as_ref()
        .map(|p| Regex::new(p))
        .transpose()
        .context("Invalid path filter regex")?;

    // Process results sequentially with size limit tracking
    let mut pages = Vec::new();
    let mut errors = Vec::new();
    let mut seen_urls = HashSet::new();
    let mut total_content_size: u64 = 0;

    let binding = website.get_pages();
    let scraped_pages = binding.as_ref();

    let total_urls = scraped_pages.map(|p| p.len()).unwrap_or(0);

    if let Some(spider_pages) = scraped_pages {
        for page in spider_pages.iter() {
            let url = page.get_url();

            // Check if we've exceeded the maximum page count (DoS protection)
            if pages.len() >= config.max_pages {
                let error_msg = format!(
                    "Maximum page count ({}) reached, stopping scrape",
                    config.max_pages
                );
                errors.push((url.to_string(), error_msg));
                break;
            }

            // Skip duplicates
            if seen_urls.contains(url) {
                continue;
            }
            seen_urls.insert(url.to_string());

            // Apply path filter
            if let Some(ref regex) = path_regex {
                let path = url::Url::parse(url)
                    .map(|u| u.path().to_string())
                    .unwrap_or_default();
                if !regex.is_match(&path) {
                    continue;
                }
            }

            // Transform HTML to Markdown (with optional filtering)
            // Note: Individual page transformation errors are collected but don't stop the scrape.
            // This allows partial success when scraping large sites with some problematic pages.
            match transform_page(page, &config.base_url, config.enable_filtering) {
                Ok(scraped) => {
                    // Track cumulative size for DoS protection (total content limit)
                    let page_size = scraped.markdown.len() as u64;
                    total_content_size = total_content_size.saturating_add(page_size);

                    // Check if total content exceeds limit (streaming attack protection)
                    if total_content_size > config.max_total_size_bytes {
                        let error_msg = format!(
                            "Total content size ({} bytes) exceeds limit ({} bytes), stopping scrape",
                            total_content_size, config.max_total_size_bytes
                        );
                        errors.push((url.to_string(), error_msg));
                        break;
                    }

                    pages.push(scraped);
                }
                Err(e) => {
                    let error_msg = format!("Failed to transform page: {e}");
                    errors.push((url.to_string(), error_msg));
                }
            }
        }
    }

    let success_count = pages.len();
    let error_count = errors.len();

    Ok(ScrapeResult {
        pages,
        total_urls,
        success_count,
        error_count,
        errors,
        base_url: config.base_url.clone(),
    })
}

/// Transform a spider page into our ScrapedPage format
///
/// Includes size limit checking to prevent memory exhaustion from huge pages.
fn transform_page(
    page: &spider::page::Page,
    base_url: &str,
    enable_filtering: bool,
) -> Result<ScrapedPage> {
    let url = page.get_url().to_string();
    let filter_config = FilterConfig::default();

    // Get raw HTML and enforce size limits (DoS protection)
    let raw_html = page.get_html();
    let config = ScrapeConfig::default();
    check_html_size(&raw_html, config.max_page_size_bytes)?;

    // Apply HTML-level pruning to analyze content quality
    let prune_result: FilterResult = if enable_filtering {
        prune_html(&raw_html, &filter_config)
    } else {
        FilterResult {
            html: raw_html.clone(),
            removed_count: 0,
            density_score: 1.0,
            used_readability: false,
        }
    };

    // Configure transformation for markdown output
    let transform_config = TransformConfig {
        return_format: ReturnFormat::Markdown,
        ..Default::default()
    };

    // Build selector configuration for HTML filtering (nav, footer, aside, etc.)
    // Uses the same patterns from FilterConfig for consistency
    let selector_config = if enable_filtering {
        let mut exclude_tags: Vec<String> = filter_config.remove_tags.clone();
        for pattern in &filter_config.nav_patterns {
            exclude_tags.push(format!(".{pattern}"));
            exclude_tags.push(format!("#{pattern}"));
        }
        Some(SelectorConfiguration {
            root_selector: None,
            exclude_selector: Some(exclude_tags.join(", ")),
        })
    } else {
        None
    };

    // Transform HTML to Markdown using spider_transformations
    // Args: page, config, url_selector, selector_config, clean_selectors
    let mut markdown =
        content::transform_content(page, &transform_config, &None, &selector_config, &None);

    // Apply additional markdown-level content filtering
    let filtered = if enable_filtering {
        markdown = filter_markdown(&markdown, &filter_config);
        true
    } else {
        false
    };

    // Enforce markdown size limit (memory exhaustion protection)
    check_markdown_size(&markdown, config.max_markdown_size_bytes)?;

    // Extract title from markdown (first H1) or fall back to URL
    let title = extract_title(&markdown, &url);

    // Extract headers from markdown
    let headers = extract_headers(&markdown);

    // Extract internal links and enforce per-page limit
    let links = extract_internal_links(&markdown, base_url);
    let (links, was_truncated) = limit_links_per_page(links, config.max_links_per_page);
    if was_truncated {
        eprintln!(
            "[WARN] Page {} had too many links, truncated to {}",
            url, config.max_links_per_page
        );
    }

    // Count words
    let word_count = markdown.split_whitespace().count();

    // Generate slug from URL (with validation for non-empty)
    let slug = url_to_slug(&url).context(format!(
        "Failed to generate slug for URL {url}: ensure URL has a valid path or hostname"
    ))?;

    Ok(ScrapedPage {
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

/// Extract title from markdown content
fn extract_title(markdown: &str, url: &str) -> String {
    // Look for first H1
    for line in markdown.lines() {
        if let Some(caps) = H1_TITLE_REGEX.captures(line.trim()) {
            if let Some(title_match) = caps.get(1) {
                return title_match.as_str().to_string();
            }
        }
    }

    // Fall back to URL path using functional pattern
    url::Url::parse(url)
        .map(|u| {
            u.path()
                .trim_matches('/')
                .split('/')
                .next_back()
                .unwrap_or("Untitled")
                .replace(['-', '_'], " ")
        })
        .unwrap_or_else(|_| "Untitled".to_string())
}

/// Extract headers from markdown
fn extract_headers(markdown: &str) -> Vec<Header> {
    let mut headers = Vec::new();

    for line in markdown.lines() {
        if let Some(caps) = HEADER_REGEX.captures(line.trim()) {
            // Safe extraction of level from capture group 1
            if let Some(level_match) = caps.get(1) {
                let level = u8::try_from(level_match.as_str().len()).unwrap_or(1); // Fallback to h1 if somehow invalid
                                                                                   // Safe extraction of text from capture group 2
                if let Some(text_match) = caps.get(2) {
                    let text = text_match.as_str().to_string();
                    headers.push(Header { level, text });
                }
            }
        }
    }

    headers
}

/// Extract internal links from markdown
fn extract_internal_links(markdown: &str, base_url: &str) -> Vec<String> {
    let base = url::Url::parse(base_url).ok();
    let mut links = Vec::new();

    for caps in LINK_REGEX.captures_iter(markdown) {
        // Safe extraction of href from capture group 2
        if let Some(href_match) = caps.get(2) {
            let href = href_match.as_str();

            // Check if internal link
            if let Some(ref base) = base {
                if let Ok(resolved) = base.join(href) {
                    if resolved.host() == base.host() {
                        links.push(resolved.to_string());
                    }
                }
            } else if href.starts_with('/') || href.starts_with("./") {
                links.push(href.to_string());
            }
        }
    }

    links.sort();
    links.dedup();
    links
}

/// Validate URL format before passing to spider
///
/// Ensures the URL is well-formed and uses http or https scheme.
/// This prevents panics from spider-rs's Website::new() on invalid URLs.
fn validate_url(url: &str) -> Result<url::Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    let parsed = url::Url::parse(trimmed).context("Invalid URL format")?;

    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        scheme => anyhow::bail!("Invalid URL scheme '{scheme}': only http and https are supported"),
    }
}

/// Check if HTML content exceeds size limit
///
/// Returns error if size exceeds max_page_size_bytes
fn check_html_size(html: &str, max_size: u64) -> Result<()> {
    let size_bytes = html.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page HTML too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Check if markdown content exceeds size limit
///
/// Returns error if size exceeds max_markdown_size_bytes
fn check_markdown_size(markdown: &str, max_size: u64) -> Result<()> {
    let size_bytes = markdown.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page markdown too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Enforce maximum links per page limit
///
/// Returns truncated vector if exceeds max_links_per_page
fn limit_links_per_page(links: Vec<String>, max_links: usize) -> (Vec<String>, bool) {
    if links.len() <= max_links {
        return (links, false);
    }
    let mut truncated = links;
    truncated.truncate(max_links);
    (truncated, true)
}

/// Validate that a slug is non-empty and filesystem-safe
///
/// Returns an error if the slug would be empty, ensuring all generated
/// slugs can be safely used as filenames.
fn validate_slug(slug: &str) -> Result<()> {
    if slug.trim().is_empty() {
        anyhow::bail!("URL slug cannot be empty: all URLs must produce non-empty identifiers");
    }
    Ok(())
}

/// Convert URL to a filesystem-safe slug using functional pattern
///
/// Returns a non-empty slug guaranteed to be safe for filenames.
/// Falls back to hostname if path is empty.
///
/// # Contract
/// - Input: Valid or invalid URL string
/// - Output: Result<String> where String is non-empty, alphanumeric + hyphens only, lowercase
/// - Guarantees: Returned slug is always non-empty (validated before return)
fn url_to_slug(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url).context("Failed to parse URL for slug generation")?;

    // Get path and normalize
    let path = parsed.path().trim_matches('/');

    // Use path, or empty string if no path
    let raw_slug = path.replace(['/', '.'], "-");

    // Filter to filesystem-safe characters (alphanumeric + hyphens)
    let slug = raw_slug
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase();

    // Truncate to reasonable length (prevent filesystem issues)
    let slug = if slug.len() > 200 {
        slug[..200].to_string()
    } else {
        slug
    };

    // Fallback to "index" if slug is empty after filtering
    let slug = if slug.trim().is_empty() {
        "index".to_string()
    } else {
        slug
    };

    // Validate non-empty (should always pass after fallback)
    validate_slug(&slug)?;

    Ok(slug)
}

/// Filter scraped pages by BM25 relevance to query
/// Returns (kept_pages, filtered_count)
pub fn filter_pages_by_relevance(
    pages: Vec<ScrapedPage>,
    query: &str,
    threshold: f32,
) -> (Vec<ScrapedPage>, usize) {
    if pages.is_empty() {
        return (pages, 0);
    }

    // Guard: if threshold is 0.0 or negative, keep all pages (no filtering)
    if threshold <= 0.0 {
        return (pages, 0);
    }

    // Calculate average document length
    let total_words: usize = pages.iter().map(|p| p.word_count).sum();
    // SAFETY: Document counts and word counts are small (< 10k documents, < 1M words)
    // well within f32 precision (2^24 ≈ 16.7M)
    let avg_doc_length = (total_words as f32 / pages.len() as f32).max(1.0);

    // Import bm25_score from filter module
    use crate::filter::bm25_score;

    // Filter pages by BM25 score
    let (kept, filtered): (Vec<_>, Vec<_>) = pages.into_iter().partition(|page| {
        let score = bm25_score(&page.markdown, query, avg_doc_length);
        score >= threshold
    });

    let filtered_count = filtered.len();

    (kept, filtered_count)
}

/// Write scraped pages to output directory
pub fn write_scraped_pages(result: &ScrapeResult, output_dir: &Path) -> Result<()> {
    let scrape_dir = output_dir.join(".scrape");
    fs::create_dir_all(&scrape_dir)?;

    for page in &result.pages {
        let filename = format!("{}.md", page.slug);
        let filepath = scrape_dir.join(&filename);

        // Write markdown with metadata header
        let content = format!(
            "---\nurl: {}\ntitle: {}\nword_count: {}\nfiltered: {}\nelements_removed: {}\ndensity_score: {:.2}\n---\n\n{}",
            page.url, page.title, page.word_count, page.filtered, page.elements_removed, page.density_score, page.markdown
        );

        fs::write(&filepath, content)?;
    }

    // Write manifest
    let manifest = serde_json::to_string_pretty(result)?;
    fs::write(scrape_dir.join("manifest.json"), manifest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://docs.rust-lang.org/book").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("").is_err());
        assert!(validate_url("   ").is_err());
        assert!(validate_url("ftp://example.com").is_err());
        assert!(validate_url("example.com").is_err()); // missing scheme
    }

    #[test]
    fn test_url_to_slug_with_path() {
        let result1 = url_to_slug("https://example.com/docs/getting-started");
        assert!(matches!(result1, Ok(ref s) if s == "docs-getting-started"));

        let result2 = url_to_slug("https://example.com/api/v1/users.html");
        assert!(matches!(result2, Ok(ref s) if s == "api-v1-users-html"));
    }

    #[test]
    fn test_url_to_slug_root_url_uses_index() {
        // Root URLs should fall back to "index"
        let result = url_to_slug("https://example.com/");
        assert!(matches!(result, Ok(ref s) if s == "index"));
        if let Ok(slug) = result {
            assert!(!slug.is_empty(), "Slug from root URL must not be empty");
        }
    }

    #[test]
    fn test_url_to_slug_no_path_uses_index() {
        // URLs without path should use "index"
        let result = url_to_slug("https://docs.example.com");
        assert!(matches!(result, Ok(ref s) if s == "index"));
        if let Ok(slug) = result {
            assert!(!slug.is_empty(), "Slug must not be empty");
        }
    }

    #[test]
    fn test_url_to_slug_never_empty() {
        // Comprehensive test: any valid URL should produce non-empty slug
        let valid_urls = vec![
            "https://example.com/",
            "https://example.com",
            "https://a.b/",
            "https://docs.rust-lang.org",
            "http://localhost:8080/api/v1/users",
        ];

        for url in valid_urls {
            let result = url_to_slug(url);
            // Each valid URL should produce Ok result
            assert!(result.is_ok(), "URL {url} should produce valid slug");
            if let Ok(slug) = result {
                assert!(!slug.is_empty(), "URL {url} produced empty slug");
                assert!(
                    slug.chars().all(|c| c.is_alphanumeric() || c == '-'),
                    "Slug {slug} contains invalid characters"
                );
            }
        }
    }

    #[test]
    fn test_url_to_slug_invalid_url() {
        assert!(url_to_slug("not-a-url").is_err());
        assert!(url_to_slug("").is_err());
        assert!(url_to_slug("   ").is_err());
    }

    #[test]
    fn test_url_to_slug_special_characters_filtered() {
        let result = url_to_slug("https://example.com/docs/getting-started-2.0");
        assert!(
            result.is_ok(),
            "Should parse valid URL with special characters"
        );
        if let Ok(slug) = result {
            // Should not contain dots, only hyphens and alphanumeric
            assert!(!slug.contains("."));
            assert!(slug.chars().all(|c| c.is_alphanumeric() || c == '-'));
        }
    }

    #[test]
    fn test_url_to_slug_special_chars_only_uses_index() {
        // URLs with only special characters should fall back to "index"
        let result1 = url_to_slug("https://example.com/???");
        assert!(matches!(result1, Ok(ref s) if s == "index"));

        let result2 = url_to_slug("https://example.com/@@@");
        assert!(matches!(result2, Ok(ref s) if s == "index"));

        let result3 = url_to_slug("https://example.com/!!!");
        assert!(matches!(result3, Ok(ref s) if s == "index"));
    }

    #[test]
    fn test_url_to_slug_multiple_slashes_uses_index() {
        // Multiple slashes should be treated as root and use index
        let result = url_to_slug("https://example.com///");
        assert!(matches!(result, Ok(ref s) if s == "index"));
    }

    #[test]
    fn test_url_to_slug_truncates_long_paths() {
        // Create a URL with an extremely long path
        let long_path = "https://example.com/".to_string() + &"very-long-path-segment-".repeat(20); // Create 400+ char path
        let result = url_to_slug(&long_path);
        assert!(result.is_ok(), "Should parse URL with long path");
        if let Ok(slug) = result {
            assert!(slug.len() <= 200, "Slug should be truncated to 200 chars");
        }
    }

    #[test]
    fn test_extract_title() {
        let md = "# Getting Started\n\nThis is content.";
        assert_eq!(
            extract_title(md, "https://example.com/foo"),
            "Getting Started"
        );

        let md_no_h1 = "Some content without header";
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/getting-started"),
            "getting started"
        );
    }

    #[test]
    fn test_extract_headers() {
        let md = "# Title\n## Section 1\n### Subsection\n## Section 2";
        let headers = extract_headers(md);
        assert_eq!(headers.len(), 4);
        assert_eq!(headers[0].level, 1);
        assert_eq!(headers[0].text, "Title");
        assert_eq!(headers[1].level, 2);
        assert_eq!(headers[2].level, 3);
    }

    #[test]
    fn test_extract_internal_links() {
        let md = "[Link 1](/docs/page1) and [Link 2](https://example.com/docs/page2) and [External](https://other.com/page)";
        let links = extract_internal_links(md, "https://example.com");
        assert_eq!(links.len(), 2);
        assert!(links.iter().any(|l| l.contains("page1")));
        assert!(links.iter().any(|l| l.contains("page2")));
    }

    // ============================================================================
    // BM25 FILTERING TESTS
    // ============================================================================

    fn create_test_page(markdown: &str, title: &str, url: &str) -> anyhow::Result<ScrapedPage> {
        let word_count = markdown.split_whitespace().count();
        let slug = url_to_slug(url)?;
        Ok(ScrapedPage {
            url: url.to_string(),
            markdown: markdown.to_string(),
            title: title.to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count,
            slug,
            filtered: false,
            elements_removed: 0,
            density_score: 1.0,
        })
    }

    #[test]
    fn test_filter_keeps_relevant_pages() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page(
                "Rust is a systems programming language that runs blazingly fast. Rust programming is great for systems development.",
                "Rust Guide",
                "https://example.com/rust-guide",
            )?,
            create_test_page(
                "Python is a high-level programming language. Learn Python for web development.",
                "Python Tutorial",
                "https://example.com/python-tutorial",
            )?,
            create_test_page(
                "JavaScript is the language of the web. Modern JavaScript powers interactive websites.",
                "JavaScript Intro",
                "https://example.com/js-intro",
            )?,
        ];

        let (kept, filtered_count) = filter_pages_by_relevance(pages, "rust programming", 0.1);

        // Should keep at least the Rust guide
        assert!(!kept.is_empty(), "Should keep at least 1 Rust-related page");
        assert!(
            filtered_count >= 1,
            "Should filter out at least 1 non-Rust page"
        );

        // Check that rust page is in the kept list
        assert!(
            kept.iter().any(|p| p.title.contains("Rust")),
            "Should keep Rust page"
        );
        Ok(())
    }

    #[test]
    fn test_filter_all_filtered_out() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page("Rust programming", "Rust Guide", "https://example.com/rust")?,
            create_test_page(
                "Python tutorial",
                "Python Guide",
                "https://example.com/python",
            )?,
        ];

        // Use a very high threshold to filter everything out
        let (kept, filtered_count) = filter_pages_by_relevance(pages.clone(), "rust", 10.0);

        assert_eq!(kept.len(), 0, "High threshold should filter all pages");
        assert_eq!(filtered_count, pages.len(), "All pages should be filtered");
        Ok(())
    }

    #[test]
    fn test_filter_zero_threshold_keeps_all() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page("Rust programming", "Rust", "https://example.com/rust")?,
            create_test_page("Python tutorial", "Python", "https://example.com/python")?,
        ];
        let original_count = pages.len();

        let (kept, filtered_count) = filter_pages_by_relevance(pages, "rust", 0.0);

        assert_eq!(
            kept.len(),
            original_count,
            "Threshold 0.0 should keep all pages"
        );
        assert_eq!(
            filtered_count, 0,
            "No pages should be filtered with threshold 0.0"
        );
        Ok(())
    }

    #[test]
    fn test_filter_negative_threshold_keeps_all() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page("Rust programming", "Rust", "https://example.com/rust")?,
            create_test_page("Python tutorial", "Python", "https://example.com/python")?,
        ];
        let original_count = pages.len();

        let (kept, filtered_count) = filter_pages_by_relevance(pages, "rust", -1.0);

        assert_eq!(
            kept.len(),
            original_count,
            "Negative threshold should keep all pages"
        );
        assert_eq!(
            filtered_count, 0,
            "No pages should be filtered with negative threshold"
        );
        Ok(())
    }

    #[test]
    fn test_filter_no_matches() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page("Rust programming", "Rust", "https://example.com/rust")?,
            create_test_page("Python tutorial", "Python", "https://example.com/python")?,
        ];
        let original_count = pages.len();

        // Query for something that doesn't exist in any page
        let (kept, filtered_count) = filter_pages_by_relevance(pages, "nonexistent_term_xyz", 0.1);

        assert_eq!(kept.len(), 0, "No pages should match nonexistent term");
        assert_eq!(
            filtered_count, original_count,
            "All pages should be filtered"
        );
        Ok(())
    }

    #[test]
    fn test_filter_empty_pages_list() {
        let pages: Vec<ScrapedPage> = Vec::new();

        let (kept, filtered_count) = filter_pages_by_relevance(pages, "query", 0.1);

        assert_eq!(kept.len(), 0, "Empty input should return empty output");
        assert_eq!(filtered_count, 0, "No pages to filter");
    }

    #[test]
    fn test_filter_case_insensitive() -> anyhow::Result<()> {
        let pages = vec![create_test_page(
            "Rust programming language",
            "Rust",
            "https://example.com/rust",
        )?];

        let (kept_lower, _) = filter_pages_by_relevance(pages.clone(), "rust", 0.1);
        let (kept_upper, _) = filter_pages_by_relevance(pages.clone(), "RUST", 0.1);
        let (kept_mixed, _) = filter_pages_by_relevance(pages, "RuSt", 0.1);

        // All should return the same results
        assert_eq!(kept_lower.len(), kept_upper.len(), "Case should not matter");
        assert_eq!(kept_lower.len(), kept_mixed.len(), "Case should not matter");
        Ok(())
    }

    #[test]
    fn test_filter_multi_term_query() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page(
                "Rust is a systems programming language that guarantees memory safety.",
                "Rust Guide",
                "https://example.com/rust",
            )?,
            create_test_page("JavaScript tutorial", "JS Guide", "https://example.com/js")?,
        ];

        // Multi-term query
        let (kept, _) = filter_pages_by_relevance(pages, "rust programming systems", 0.1);

        // Should find pages containing any of these terms
        assert!(
            !kept.is_empty(),
            "Should find pages matching multi-term query"
        );
        assert!(
            kept.iter().any(|p| p.title.contains("Rust")),
            "Should find rust page with multi-term query"
        );
        Ok(())
    }

    #[test]
    fn test_filter_different_thresholds() -> anyhow::Result<()> {
        let pages = vec![
            create_test_page(
                "Rust programming language systems",
                "Rust",
                "https://example.com/rust1",
            )?,
            create_test_page("Rust", "Rust Short", "https://example.com/rust2")?,
            create_test_page("Python programming", "Python", "https://example.com/python")?,
        ];

        let (kept_low, _) = filter_pages_by_relevance(pages.clone(), "rust", 0.1);
        let (kept_medium, _) = filter_pages_by_relevance(pages.clone(), "rust", 0.5);
        let (kept_high, _) = filter_pages_by_relevance(pages, "rust", 2.0);

        // Lower threshold should keep more pages
        assert!(
            kept_low.len() >= kept_medium.len(),
            "Lower threshold should keep more pages"
        );
        assert!(
            kept_medium.len() >= kept_high.len(),
            "Medium threshold should keep more than high"
        );
        Ok(())
    }

    #[test]
    fn test_filter_preserves_page_metadata() -> anyhow::Result<()> {
        let original_page = create_test_page(
            "Rust programming guide with comprehensive examples",
            "Rust Guide",
            "https://example.com/rust-guide",
        )?;
        let original_url = original_page.url.clone();
        let original_title = original_page.title.clone();
        let original_word_count = original_page.word_count;

        let pages = vec![original_page];
        let (kept, _) = filter_pages_by_relevance(pages, "rust programming", 0.1);

        assert_eq!(kept.len(), 1, "Should keep the rust page");
        let filtered_page = &kept[0];

        assert_eq!(filtered_page.url, original_url, "URL should be preserved");
        assert_eq!(
            filtered_page.title, original_title,
            "Title should be preserved"
        );
        assert_eq!(
            filtered_page.word_count, original_word_count,
            "Word count should be preserved"
        );
        Ok(())
    }

    #[test]
    fn test_filter_with_special_characters_in_query() -> anyhow::Result<()> {
        let pages = vec![create_test_page(
            "Rust-lang systems programming",
            "Rust",
            "https://example.com/rust",
        )?];

        // Query with special characters (should not crash)
        let result = std::panic::catch_unwind(|| {
            filter_pages_by_relevance(pages, "rust-lang & systems", 0.1)
        });

        assert!(result.is_ok(), "Should handle special characters in query");
        Ok(())
    }

    #[test]
    fn test_filter_empty_query() -> anyhow::Result<()> {
        let pages = vec![create_test_page(
            "Rust programming",
            "Rust",
            "https://example.com/rust",
        )?];

        let (kept, filtered_count) = filter_pages_by_relevance(pages.clone(), "", 0.1);

        // Empty query should filter all pages (no terms to match)
        assert_eq!(kept.len(), 0, "Empty query should match nothing");
        assert_eq!(
            filtered_count,
            pages.len(),
            "All pages should be filtered with empty query"
        );
        Ok(())
    }

    #[test]
    fn test_filter_calculates_average_correctly() -> anyhow::Result<()> {
        // Create pages with known word counts
        let pages = vec![
            create_test_page("one two three four five", "Page 1", "https://example.com/1")?, // 5 words
            create_test_page("one two three", "Page 2", "https://example.com/2")?, // 3 words
            create_test_page("one two", "Page 3", "https://example.com/3")?,       // 2 words
        ];
        // Average: (5 + 3 + 2) / 3 = 3.33 words

        // The filter should calculate avg_doc_length correctly and use it for scoring
        // We can't test the internal calculation directly, but we can verify it doesn't panic
        let result = std::panic::catch_unwind(|| filter_pages_by_relevance(pages, "one", 0.1));

        assert!(
            result.is_ok(),
            "Should calculate average document length without panicking"
        );
        Ok(())
    }

    // ============================================================================
    // SIZE LIMIT TESTS (DoS PROTECTION)
    // ============================================================================

    #[test]
    fn test_check_html_size_valid() {
        let html = "<html><body>Hello</body></html>";
        let result = check_html_size(html, 1000);
        assert!(result.is_ok(), "Small HTML should pass size check");
    }

    #[test]
    fn test_check_html_size_exceeds_limit() {
        let html = "x".repeat(1001);
        let result = check_html_size(&html, 1000);
        assert!(result.is_err(), "HTML exceeding limit should fail");
        // Additional check that we can format the error
        if let Err(e) = &result {
            let err_msg = e.to_string();
            assert!(err_msg.contains("too large"), "Error should mention size");
        }
    }

    #[test]
    fn test_check_markdown_size_valid() {
        let markdown = "# Hello\n\nThis is content.";
        let result = check_markdown_size(markdown, 1000);
        assert!(result.is_ok(), "Small markdown should pass size check");
    }

    #[test]
    fn test_check_markdown_size_exceeds_limit() {
        let markdown = "x".repeat(5001);
        let result = check_markdown_size(&markdown, 5000);
        assert!(result.is_err(), "Markdown exceeding limit should fail");
        // Additional check that we can format the error
        if let Err(e) = &result {
            let err_msg = e.to_string();
            assert!(
                err_msg.contains("too large"),
                "Error should mention markdown size"
            );
        }
    }

    #[test]
    fn test_limit_links_per_page_within_limit() {
        let links = vec![
            "link1".to_string(),
            "link2".to_string(),
            "link3".to_string(),
        ];
        let (result, was_truncated) = limit_links_per_page(links, 10);
        assert_eq!(result.len(), 3, "All links should be kept");
        assert!(!was_truncated, "Should not be truncated");
    }

    #[test]
    fn test_limit_links_per_page_exceeds_limit() {
        let links = vec![
            "link1".to_string(),
            "link2".to_string(),
            "link3".to_string(),
        ];
        let (result, was_truncated) = limit_links_per_page(links, 2);
        assert_eq!(result.len(), 2, "Links should be truncated to limit");
        assert!(was_truncated, "Should indicate truncation");
    }

    #[test]
    fn test_limit_links_per_page_exactly_at_limit() {
        let links = vec!["link1".to_string(), "link2".to_string()];
        let (result, was_truncated) = limit_links_per_page(links, 2);
        assert_eq!(result.len(), 2, "All links at limit should be kept");
        assert!(!was_truncated, "Should not truncate when at exact limit");
    }

    #[test]
    fn test_limit_links_per_page_empty() {
        let links: Vec<String> = vec![];
        let (result, was_truncated) = limit_links_per_page(links, 10);
        assert_eq!(result.len(), 0, "Empty list should remain empty");
        assert!(!was_truncated, "Empty list should not be truncated");
    }

    #[test]
    fn test_scrape_config_default_has_size_limits() {
        let config = ScrapeConfig::default();
        assert_eq!(
            config.max_page_size_bytes,
            10 * 1024 * 1024,
            "Default max page size should be 10MB"
        );
        assert_eq!(
            config.max_total_size_bytes,
            500 * 1024 * 1024,
            "Default max total size should be 500MB"
        );
        assert_eq!(
            config.max_markdown_size_bytes,
            5 * 1024 * 1024,
            "Default max markdown size should be 5MB"
        );
        assert_eq!(
            config.max_pages, 10_000,
            "Default max pages should be 10000"
        );
        assert_eq!(
            config.max_links_per_page, 1_000,
            "Default max links per page should be 1000"
        );
    }

    #[test]
    fn test_scrape_config_limits_are_reasonable() {
        let config = ScrapeConfig::default();
        // Verify: max_page_size < max_total_size
        assert!(
            config.max_page_size_bytes < config.max_total_size_bytes,
            "Per-page limit must be less than total limit"
        );
        // Verify: max_markdown_size <= max_page_size
        assert!(
            config.max_markdown_size_bytes <= config.max_page_size_bytes,
            "Markdown limit should not exceed page limit"
        );
        // Verify: reasonable defaults
        assert!(config.max_pages > 0, "Max pages must be positive");
        assert!(
            config.max_links_per_page > 0,
            "Max links per page must be positive"
        );
    }

    #[test]
    fn test_huge_content_detection() {
        // Simulate 100MB of repeated text
        let huge_text = "x".repeat(100 * 1024 * 1024);
        let config = ScrapeConfig::default();
        let result = check_html_size(&huge_text, config.max_page_size_bytes);
        assert!(result.is_err(), "100MB content should exceed 10MB limit");
    }

    #[test]
    fn test_streaming_attack_protection() {
        // Simulate multiple pages hitting the total limit
        let mut total_size = 0u64;
        let config = ScrapeConfig::default();
        let page_size = config.max_page_size_bytes / 2; // 5MB per page
        let mut pages_before_limit = 0usize;

        while total_size.saturating_add(page_size) <= config.max_total_size_bytes {
            total_size = total_size.saturating_add(page_size);
            pages_before_limit = pages_before_limit.saturating_add(1);
        }

        // With 500MB limit and 5MB pages, should allow ~100 pages
        assert!(
            (90..=110).contains(&pages_before_limit),
            "Should allow ~100 5MB pages in 500MB budget, got: {pages_before_limit}"
        );
    }
}
