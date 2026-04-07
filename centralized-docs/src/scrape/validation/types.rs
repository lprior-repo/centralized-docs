//! All domain types for the scrape validation module.

use serde::{Deserialize, Serialize};

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

/// TCP connect timeout in seconds.
///
/// Type-safe wrapper that enforces the 1-60 second range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectTimeoutSecs(u64);

impl ConnectTimeoutSecs {
    /// Create a new ConnectTimeoutSecs, validating the range 1-60.
    ///
    /// # Errors
    /// Returns an error if value is not in range 1-60.
    pub fn new(value: u64) -> Result<Self, String> {
        if value < 1 {
            Err("connect timeout must be at least 1 second".to_string())
        } else if value > 60 {
            Err("connect timeout must be at most 60 seconds".to_string())
        } else {
            Ok(Self(value))
        }
    }

    /// Create a ConnectTimeoutSecs without validation (for trusted sources).
    #[must_use]
    pub const fn new_unchecked(value: u64) -> Self {
        Self(value)
    }

    /// Return the underlying value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl Default for ConnectTimeoutSecs {
    fn default() -> Self {
        Self(10)
    }
}

/// HTTP request timeout in seconds.
///
/// Type-safe wrapper that enforces the 1-600 second range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestTimeoutSecs(u64);

impl RequestTimeoutSecs {
    /// Create a new RequestTimeoutSecs, validating the range 1-600.
    ///
    /// # Errors
    /// Returns an error if value is not in range 1-600.
    pub fn new(value: u64) -> Result<Self, String> {
        if value < 1 {
            Err("request timeout must be at least 1 second".to_string())
        } else if value > 600 {
            Err("request timeout must be at most 600 seconds (10 minutes)".to_string())
        } else {
            Ok(Self(value))
        }
    }

    /// Create a RequestTimeoutSecs without validation (for trusted sources).
    #[must_use]
    pub const fn new_unchecked(value: u64) -> Self {
        Self(value)
    }

    /// Return the underlying value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl Default for RequestTimeoutSecs {
    fn default() -> Self {
        Self(30)
    }
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
    pub connect_timeout_secs: u64,
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
            delay_ms: 0,
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
            concurrency_limit: 4,
            request_timeout_secs: 30,
            connect_timeout_secs: 10,
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
