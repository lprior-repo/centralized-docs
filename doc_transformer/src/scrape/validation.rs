#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]

//! Scrape configuration, domain types, and validation utilities.
//!
//! Contains all types shared across the scraping sub-system:
//!
//! - **Behaviour enums** — [`SitemapStrategy`], [`RobotsPolicy`], [`FilteringMode`],
//!   [`RetryStrategy`], [`StealthMode`]: replace `bool` flags with explicit intent.
//! - **[`ScrapeConfig`]** — Complete configuration for a single scrape run.
//! - **[`ScrapedPage`]** — One successfully scraped and converted page.
//! - **[`PageFilterStatus`]** — Whether content-density filtering was applied.
//! - **[`ScrapeResult`]** — Aggregate result with success/error counts.
//!
//! Validation utilities ([`compile_safe_regex`], [`validate_url`],
//! [`check_html_size`], [`validate_scrape_result`]) guard the I/O boundary so that
//! domain functions downstream receive only trusted, well-formed inputs.

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Statically compiled H1 regex for extract_title
static H1_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s+(.+)$").expect("valid regex"));

/// Safely compiles a user-provided regex pattern with ReDoS protection.
///
/// This function validates that the user's regex pattern:
/// - Is not longer than 500 characters
/// - Does not contain known ReDoS patterns that can cause catastrophic backtracking
/// - Can be compiled within memory limits (1MB compiled size, 1MB DFA size)
///
/// # Errors
///
/// Returns an error if:
/// - The pattern exceeds 500 characters
/// - The pattern contains known ReDoS patterns
/// - The regex compilation fails (invalid syntax)
/// - The regex is too complex (exceeds memory limits)
pub(crate) fn compile_safe_regex(pattern: &str) -> Result<Regex> {
    // Use character count, not byte count, for the 500 char limit
    let char_count = pattern.chars().count();
    if char_count > 500 {
        anyhow::bail!(
            "Regex pattern too long (max 500 characters, got {len})",
            len = char_count
        );
    }

    // Detect nested quantifiers: any group followed by a quantifier
    // Catches: (.*)* (.+)+ ([a-z]+)+ (a+)+ (\w+)* (a|a)+ etc.
    // Uses \([^)]+\)[+*] to match any (group)+ or (group)* pattern
    let redos_detector =
        Regex::new(r"\([^)]+\)[+*]").context("failed to compile ReDoS detector regex")?;
    if redos_detector.is_match(pattern) {
        anyhow::bail!(
            "Regex contains potentially slow pattern (ReDoS risk): nested quantifiers detected. \
             This pattern can cause catastrophic backtracking and hang the application.",
        );
    }

    regex::RegexBuilder::new(pattern)
        .size_limit(1024 * 1024)
        .dfa_size_limit(1024 * 1024)
        .build()
        .context("Invalid or too complex regex pattern")
}

/// Whether to discover pages via the site's XML sitemap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SitemapStrategy {
    /// Follow sitemap.xml/sitemap_index.xml entries
    UseSitemap,
    /// Crawl by following HTML links only
    CrawlOnly,
}

/// Whether to honour `robots.txt` exclusion rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RobotsPolicy {
    /// Obey `Disallow` directives in `robots.txt`
    Respect,
    /// Fetch all pages regardless of `robots.txt` (use with permission)
    Ignore,
}

/// Whether to apply content-density filtering to scraped pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilteringMode {
    /// Apply readability / BM25 density filtering
    Enabled,
    /// Store raw extracted markdown unchanged
    Disabled,
}

/// Retry back-off algorithm for transient HTTP failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// Delay doubles on each attempt: 1 s, 2 s, 4 s, …
    ExponentialBackoff,
    /// Fixed inter-request delay regardless of attempt number
    Fixed,
}

/// Whether to present browser-like headers to evade bot-detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StealthMode {
    /// Inject randomised User-Agent and header variations
    Enabled,
    /// Use the configured `user_agent` as-is
    Disabled,
}

/// Configuration for scraping a documentation site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeConfig {
    pub base_url: String,
    pub sitemap_strategy: SitemapStrategy,
    pub path_filter: Option<String>,
    pub delay_ms: u64,
    pub user_agent: String,
    pub robots_policy: RobotsPolicy,
    pub filtering_mode: FilteringMode,
    pub retry_strategy: RetryStrategy,
    pub max_page_size_bytes: u64,
    pub max_total_size_bytes: u64,
    pub max_markdown_size_bytes: u64,
    pub max_pages: usize,
    pub max_links_per_page: usize,
    pub stealth_mode: StealthMode,
    pub concurrency_limit: usize,
    pub request_timeout_secs: u64,
    pub max_retries: u32,
    pub redirect_policy: spider::configuration::RedirectPolicy,
    pub spider_max_page_bytes: Option<u64>,
    pub spider_max_total_bytes: Option<u64>,
}

impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            sitemap_strategy: SitemapStrategy::UseSitemap,
            path_filter: None,
            delay_ms: 1000,
            user_agent: "DocTransformer/5.0 (AI Documentation Indexer)".to_string(),
            robots_policy: RobotsPolicy::Respect,
            filtering_mode: FilteringMode::Enabled,
            retry_strategy: RetryStrategy::ExponentialBackoff,
            max_page_size_bytes: 10 * 1024 * 1024,
            max_total_size_bytes: 500 * 1024 * 1024,
            max_markdown_size_bytes: 5 * 1024 * 1024,
            max_pages: 10_000,
            max_links_per_page: 1_000,
            stealth_mode: StealthMode::Enabled,
            concurrency_limit: 1,
            request_timeout_secs: 30,
            max_retries: 3,
            redirect_policy: spider::configuration::RedirectPolicy::Loose,
            spider_max_page_bytes: None,
            spider_max_total_bytes: None,
        }
    }
}

/// Whether content-density filtering was applied to a scraped page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageFilterStatus {
    /// Filtering ran and may have removed low-density elements
    Filtered,
    /// Raw markdown stored, no filtering applied
    Unfiltered,
}

/// A scraped page with extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    pub url: String,
    pub markdown: String,
    pub title: String,
    pub links: Vec<String>,
    pub headers: Vec<Header>,
    pub word_count: usize,
    pub slug: String,
    pub filter_status: PageFilterStatus,
    pub elements_removed: usize,
    pub density_score: f32,
}

/// A header extracted from page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub level: u8,
    pub text: String,
}

/// Result of scraping a site
#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapeResult {
    pub pages: Vec<ScrapedPage>,
    pub total_urls: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<(String, String)>,
    pub base_url: String,
}

/// Validate URL format before passing to spider
pub fn validate_url(url: &str) -> Result<url::Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    // Check the original input string for invalid characters BEFORE parsing
    // The URL parser auto-encodes some things, so we need to catch issues in the input
    if trimmed.contains(' ') {
        anyhow::bail!(
            "URL contains spaces: '{trimmed}'. Use '%20' instead of spaces or remove them."
        );
    }

    if trimmed.chars().any(char::is_control) {
        anyhow::bail!(
            "URL contains control characters (for example tab/newline), which are not allowed"
        );
    }

    // Check for common unencoded special characters in the original input.
    // '[' and ']' are allowed only inside authority for IPv6 host literals.
    if let Some(found) = find_unencoded_special_char(trimmed) {
        anyhow::bail!(
            "URL contains unencoded special character '{found}'. \
            Characters like [ ] {{ }} | \\ ^ ` < > must be percent-encoded.",
        );
    }

    let parsed = url::Url::parse(trimmed).context("Invalid URL format")?;

    match parsed.scheme() {
        "http" | "https" => {}
        scheme => anyhow::bail!("Invalid URL scheme '{scheme}': only http and https are supported"),
    }

    match parsed.host_str() {
        Some(host) if !host.is_empty() => {}
        Some(_) => anyhow::bail!("URL host cannot be empty"),
        None => anyhow::bail!("URL must have a valid host"),
    }

    // Validate URL can be serialized without information loss
    let serialized = parsed.to_string();
    let reparsed = url::Url::parse(&serialized);
    if reparsed.is_err() {
        anyhow::bail!(
            "URL contains invalid encoding. Please ensure special characters are percent-encoded."
        );
    }

    Ok(parsed)
}

fn find_unencoded_special_char(url: &str) -> Option<char> {
    let authority_bounds = parse_authority_bounds(url);

    url.char_indices().find_map(|(index, ch)| {
        let is_unencoded_special = matches!(
            ch,
            '[' | ']' | '{' | '}' | '|' | '\\' | '^' | '`' | '<' | '>'
        );

        if is_unencoded_special && !is_ipv6_host_bracket(index, ch, authority_bounds) {
            Some(ch)
        } else {
            None
        }
    })
}

fn parse_authority_bounds(url: &str) -> Option<(usize, usize)> {
    url.find("://").and_then(|scheme_separator_index| {
        let authority_start = scheme_separator_index.checked_add(3)?;
        let after_scheme = url.get(authority_start..)?;
        let authority_end = after_scheme
            .find(|ch| ['/', '?', '#'].contains(&ch))
            .map_or(url.len(), |offset| authority_start.saturating_add(offset));
        Some((authority_start, authority_end))
    })
}

fn is_ipv6_host_bracket(index: usize, ch: char, authority_bounds: Option<(usize, usize)>) -> bool {
    matches!(ch, '[' | ']')
        && authority_bounds.is_some_and(|(authority_start, authority_end)| {
            index >= authority_start && index < authority_end
        })
}

/// Check if HTML content exceeds size limit
pub fn check_html_size(html: &str, max_size: u64) -> Result<()> {
    let size_bytes = html.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page HTML too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Check if markdown content exceeds size limit
pub fn check_markdown_size(markdown: &str, max_size: u64) -> Result<()> {
    let size_bytes = markdown.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page markdown too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Enforce maximum links per page limit
#[must_use]
pub fn limit_links_per_page(links: Vec<String>, max_links: usize) -> (Vec<String>, bool) {
    if links.len() <= max_links {
        (links, false)
    } else {
        (links.into_iter().take(max_links).collect(), true)
    }
}

/// Validate that a slug is non-empty and filesystem-safe
pub fn validate_slug(slug: &str) -> Result<()> {
    if slug.trim().is_empty() {
        anyhow::bail!("URL slug cannot be empty: all URLs must produce non-empty identifiers");
    }
    Ok(())
}

/// Validate that a scrape result contains at least one page
pub fn validate_scrape_result(result: &ScrapeResult) -> Result<()> {
    if result.success_count == 0 {
        if result.total_urls == 0 {
            anyhow::bail!(
                "Failed to reach '{}'. The domain may not exist or DNS resolution failed. \
                Please verify the URL is correct and accessible in a browser.",
                result.base_url
            );
        }
        anyhow::bail!(
            "Failed to scrape any pages from '{}'. \
            Please verify:\n  \
            - The URL is accessible in a browser\n  \
            - The site has HTML content (not just API endpoints)\n  \
            - The site allows scraping (check robots.txt)",
            result.base_url
        );
    }
    Ok(())
}

/// Minimum page count threshold for SPA detection
const SPA_DETECTION_PAGE_THRESHOLD: usize = 5;

/// Potential SPA detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaDetectionResult {
    pub is_potential_spa: bool,
    pub pages_scraped: usize,
    pub total_urls_discovered: usize,
    pub warning_message: Option<String>,
}

/// Detect if a site may be a Single-Page Application (SPA) requiring JavaScript rendering.
///
/// Analyzes the scrape result to identify patterns suggesting the site uses client-side
/// rendering that static HTML scraping cannot fully capture.
///
/// Returns a warning if:
/// - Fewer than 5 pages were scraped AND
/// - More than 5 URLs were discovered (indicating the site has more content)
/// - The site likely requires JavaScript to render content
#[allow(clippy::cast_precision_loss)]
pub fn detect_potential_spa(result: &ScrapeResult) -> SpaDetectionResult {
    let pages_scraped = result.success_count;
    let total_urls = result.total_urls;

    // Calculate ratio of scraped to discovered URLs
    let scrape_ratio = if total_urls > 0 {
        pages_scraped as f64 / total_urls as f64
    } else {
        0.0
    };

    // Detect SPA pattern: few pages scraped but many URLs discovered
    let is_potential_spa = pages_scraped < SPA_DETECTION_PAGE_THRESHOLD
        && total_urls > SPA_DETECTION_PAGE_THRESHOLD
        && scrape_ratio < 0.5;

    let warning_message = if is_potential_spa {
        Some(format!(
            "⚠️  POTENTIAL SPA DETECTED\n\
            Only {pages_scraped} pages scraped from {total_urls} discovered URLs.\n\
            This site may require JavaScript rendering to capture all content.\n\
            \n\
            Suggestions:\n\
            - Use a headless browser (Playwright, Puppeteer) for JavaScript rendering\n\
            - Check if the site provides a sitemap.xml with all content URLs\n\
            - Verify the site doesn't use client-side routing (React, Vue, Angular)\n\
            - Consider using --no-sitemap flag if sitemap URLs are empty"
        ))
    } else {
        None
    };

    SpaDetectionResult {
        is_potential_spa,
        pages_scraped,
        total_urls_discovered: total_urls,
        warning_message,
    }
}

#[cfg(test)]
mod spa_detection_tests {
    use super::*;

    #[test]
    fn test_detect_potential_spa_low_pages_high_urls() {
        let result = ScrapeResult {
            pages: vec![],
            total_urls: 100,
            success_count: 3,
            error_count: 0,
            errors: vec![],
            base_url: "https://example.com".to_string(),
        };

        let spa_result = detect_potential_spa(&result);
        assert!(spa_result.is_potential_spa);
        assert!(spa_result.warning_message.is_some());
    }

    #[test]
    fn test_detect_potential_spa_not_enough_pages() {
        let result = ScrapeResult {
            pages: vec![],
            total_urls: 3,
            success_count: 2,
            error_count: 0,
            errors: vec![],
            base_url: "https://example.com".to_string(),
        };

        let spa_result = detect_potential_spa(&result);
        assert!(!spa_result.is_potential_spa);
        assert!(spa_result.warning_message.is_none());
    }

    #[test]
    fn test_detect_potential_spa_healthy_scrape() {
        let result = ScrapeResult {
            pages: vec![],
            total_urls: 50,
            success_count: 45,
            error_count: 5,
            errors: vec![],
            base_url: "https://example.com".to_string(),
        };

        let spa_result = detect_potential_spa(&result);
        assert!(!spa_result.is_potential_spa);
        assert!(spa_result.warning_message.is_none());
    }

    #[test]
    fn test_detect_potential_spa_zero_total_urls() {
        let result = ScrapeResult {
            pages: vec![],
            total_urls: 0,
            success_count: 0,
            error_count: 0,
            errors: vec![],
            base_url: "https://example.com".to_string(),
        };

        let spa_result = detect_potential_spa(&result);
        assert!(!spa_result.is_potential_spa);
    }
}

/// Extract title from markdown content
/// Uses statically compiled H1 regex for performance
pub fn extract_title(markdown: &str, url: &str) -> String {
    for line in markdown.lines() {
        if let Some(caps) = H1_REGEX.captures(line.trim()) {
            if let Some(title_match) = caps.get(1) {
                return title_match.as_str().to_string();
            }
        }
    }

    url::Url::parse(url).map_or_else(
        |_| "Untitled".to_string(),
        |u| {
            u.path()
                .trim_matches('/')
                .split('/')
                .next_back()
                .map_or_else(
                    || "Untitled".to_string(),
                    |s| {
                        // Decode percent-encoded characters (e.g., %20 -> space, %3A -> :)
                        // Using form_urlencoded which handles common URL encoding
                        let decoded: String = url::form_urlencoded::parse(s.as_bytes())
                            .map(|(key, _)| key.into_owned())
                            .collect();
                        decoded.replace(['-', '_'], " ")
                    },
                )
        },
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

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
        assert!(validate_url("example.com").is_err());
    }

    #[test]
    fn test_extract_title() {
        let md = "# Getting Started\n\nThis is content.";
        assert_eq!(extract_title(md, "https://example.com"), "Getting Started");

        let md_no_h1 = "Some content without header";
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/getting-started"),
            "getting started"
        );
    }

    #[test]
    fn test_extract_title_encoding_fallback_unicode() {
        // Test UTF-8 encoded characters in H1 title
        let md = "# Getting Started © 2024\n\nContent here.";
        assert_eq!(
            extract_title(md, "https://example.com"),
            "Getting Started © 2024"
        );

        // Test with accented characters
        let md_accent = "# Résumé\n\nContent here.";
        assert_eq!(extract_title(md_accent, "https://example.com"), "Résumé");

        // Test with emoji in title
        let md_emoji = "# Hello World 🌍\n\nContent here.";
        assert_eq!(
            extract_title(md_emoji, "https://example.com"),
            "Hello World 🌍"
        );
    }

    #[test]
    fn test_extract_title_url_encoding_fallback() {
        // Test URL-encoded characters in fallback URL path
        let md_no_h1 = "Some content without header";

        // URL with encoded space (%20)
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/hello%20world"),
            "hello world"
        );

        // URL with encoded underscore (%5F)
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/hello_world"),
            "hello world"
        );

        // URL with encoded hyphen (%2D)
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/hello-world"),
            "hello world"
        );

        // URL with mixed encoded characters
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/my%20test_page"),
            "my test page"
        );

        // URL with multiple path segments - should use last segment
        assert_eq!(
            extract_title(md_no_h1, "https://example.com/docs/api/v2"),
            "v2"
        );
    }

    #[test]
    fn test_extract_title_edge_cases() {
        // Empty markdown with valid URL
        assert_eq!(
            extract_title("", "https://example.com/my-document"),
            "my document"
        );

        // Whitespace-only markdown
        assert_eq!(
            extract_title("   \n\n   ", "https://example.com/doc"),
            "doc"
        );

        // H1 with only whitespace after #
        let md_whitespace = "#   \n\nContent";
        assert_eq!(
            extract_title(md_whitespace, "https://example.com/fallback"),
            "fallback"
        );

        // Multiple H1s - should use first
        let md_multi = "# First Title\n\n# Second Title";
        assert_eq!(
            extract_title(md_multi, "https://example.com"),
            "First Title"
        );

        // H1 with leading/trailing whitespace
        let md_trim = "#   Trimmed Title   \n\nContent";
        assert_eq!(
            extract_title(md_trim, "https://example.com"),
            "Trimmed Title"
        );
    }

    #[test]
    fn test_extract_title_invalid_url_fallback() {
        // Invalid URL should fall back to "Untitled"
        let md_no_h1 = "Some content";
        assert_eq!(extract_title(md_no_h1, "not-a-valid-url"), "Untitled");
        assert_eq!(extract_title(md_no_h1, ""), "Untitled");
    }

    #[test]
    fn test_compile_safe_regex_rejects_redos_pattern() {
        let redos_pattern = "([a-z]+)+$";
        let start = std::time::Instant::now();
        let result = compile_safe_regex(redos_pattern);

        assert!(
            result.is_err(),
            "ReDoS pattern should be rejected, got: {result:?}"
        );
        assert!(
            start.elapsed() < std::time::Duration::from_secs(1),
            "ReDoS rejection should be fast, took: {elapsed:?}",
            elapsed = start.elapsed()
        );

        let error_msg = match &result {
            Err(e) => e.to_string(),
            Ok(_) => unreachable!(),
        };
        assert!(
            error_msg.contains("ReDoS"),
            "Error message should mention ReDoS: {error_msg}"
        );
    }

    #[test]
    fn test_compile_safe_regex_rejects_nested_star_pattern() {
        let nested_star = "(.*)*";
        let result = compile_safe_regex(nested_star);

        assert!(result.is_err(), "Nested star pattern should be rejected");

        let error_msg = match &result {
            Err(e) => e.to_string(),
            Ok(_) => unreachable!(),
        };
        assert!(
            error_msg.contains("ReDoS"),
            "Error message should mention ReDoS: {error_msg}"
        );
    }

    #[test]
    fn test_compile_safe_regex_rejects_long_pattern() {
        let long_pattern = "a".repeat(1000);
        let result = compile_safe_regex(&long_pattern);

        assert!(result.is_err(), "Pattern > 500 chars should be rejected");

        let error_msg = match &result {
            Err(e) => e.to_string(),
            Ok(_) => unreachable!(),
        };
        assert!(
            error_msg.contains("too long"),
            "Error message should mention length limit: {error_msg}"
        );
    }

    #[test]
    fn test_compile_safe_regex_accepts_valid_pattern() {
        let valid_pattern = r"^/docs/.*\.md$";
        let result = compile_safe_regex(valid_pattern);

        assert!(
            result.is_ok(),
            "Valid pattern should be accepted, got: {result:?}"
        );
    }

    #[test]
    fn test_compile_safe_regex_rejects_invalid_syntax() {
        let invalid_syntax = "(?P<invalid";
        let result = compile_safe_regex(invalid_syntax);

        assert!(result.is_err(), "Invalid syntax should be rejected");

        let error_msg = match &result {
            Err(e) => e.to_string(),
            Ok(_) => unreachable!(),
        };
        assert!(
            error_msg.contains("Invalid") || error_msg.contains("regex"),
            "Error message should describe the error: {error_msg}"
        );
    }

    #[test]
    fn test_compile_safe_regex_accepts_empty_string() {
        let empty_pattern = "";
        let result = compile_safe_regex(empty_pattern);

        assert!(result.is_ok(), "Empty pattern should be valid");
    }

    #[test]
    fn test_compile_safe_regex_rejects_single_char_nested_quantifier() {
        // (a+)+ is the canonical ReDoS pattern — must be caught
        let result = compile_safe_regex("(a+)+$");
        assert!(result.is_err(), "(a+)+$ must be rejected as ReDoS");
    }

    #[test]
    fn test_compile_safe_regex_rejects_alternation_nested_quantifier() {
        // (a|a)+ was previously not detected - this was the bug
        let result = compile_safe_regex("(a|a)+");
        assert!(result.is_err(), "(a|a)+ must be rejected as ReDoS");
    }

    #[test]
    fn test_compile_safe_regex_rejects_word_char_nested_quantifier() {
        // (\w)+ was also previously not detected
        let result = compile_safe_regex(r"(\w)+");
        assert!(result.is_err(), "(\\w)+ must be rejected as ReDoS");
    }

    #[test]
    fn test_validate_url_missing_host() {
        assert!(validate_url("https://").is_err());
        assert!(validate_url("https://?foo=bar").is_err());
    }

    #[test]
    fn test_validate_url_valid_hosts() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("https://localhost:3000").is_ok());
        assert!(validate_url("https://[::1]:3000/docs").is_ok());
    }

    #[test]
    fn test_validate_url_rejects_spaces() {
        let result = validate_url("https://example.com/foo bar");
        assert!(result.is_err(), "URL with spaces should be rejected");

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("space"),
            "Error should mention spaces: {error_msg}"
        );
    }

    #[test]
    fn test_validate_url_rejects_control_characters() {
        let tab = validate_url("https://example.com/foo\tbar");
        assert!(tab.is_err(), "URL with tab should be rejected");

        let newline = validate_url("https://example.com/foo\nbar");
        assert!(newline.is_err(), "URL with newline should be rejected");
    }

    #[test]
    fn test_validate_url_rejects_unencoded_special_chars() {
        // Test various special characters that should be percent-encoded
        let special_urls = [
            "https://example.com/foo[bar]",
            "https://example.com/foo{bar}",
            "https://example.com/foo|bar",
            "https://example.com/foo^bar",
            "https://example.com/foo`bar",
            "https://example.com/foo<bar>",
        ];

        for url in special_urls {
            let result = validate_url(url);
            assert!(
                result.is_err(),
                "URL with special chars should be rejected: {url}"
            );

            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains("unencoded") || error_msg.contains("percent-encoded"),
                "Error should mention encoding: {error_msg}"
            );
        }
    }

    #[test]
    fn test_validate_url_accepts_percent_encoded() {
        // Percent-encoded URLs should be accepted
        assert!(validate_url("https://example.com/foo%20bar").is_ok());
        assert!(validate_url("https://example.com/foo%5Bbar%5D").is_ok());
    }
}
