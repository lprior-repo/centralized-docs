#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![cfg_attr(
    test,
    allow(
        clippy::unreadable_literal,
        clippy::field_reassign_with_default,
        clippy::float_cmp
    )
)]

//! HTTP client and website configuration
//!
//! Provides spider-rs website building and HTTP request configuration.

use super::validation::{RobotsPolicy, ScrapeConfig, StealthMode};
use rayon::prelude::*;
use std::time::Duration;
use thiserror::Error;
use tracing::instrument;
use url::Url;

/// Domain errors for the HTTP scrape module.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HttpError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Configuration overflow: {0}")]
    ConfigOverflow(&'static str),
    #[error("Execution failed: {0}")]
    #[allow(dead_code)]
    ExecutionFailed(String),
    #[error("Scrape failed: {0}")]
    ScrapeFailed(String),
    #[error("Connect timeout after {timeout_secs}s to host '{host}'")]
    ConnectTimeout { host: String, timeout_secs: u64 },
}

/// A validated URL ensuring semantic correctness before passing to the scraper.
#[derive(Debug, Clone)]
pub struct ValidatedUrl(Url);

impl ValidatedUrl {
    /// Attempt to parse and validate a URL string.
    pub fn try_new(url_str: &str) -> Result<Self, HttpError> {
        Url::parse(url_str)
            .map(Self)
            .map_err(|_| HttpError::InvalidUrl(url_str.to_string()))
    }

    /// Return the string representation of the URL.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Return the parsed URL for direct access (avoids re-parsing).
    pub(crate) fn inner(&self) -> &Url {
        &self.0
    }
}

/// A limit guaranteed to be representable precisely as an f64.
#[derive(Debug, Clone, Copy)]
pub struct SafeByteLimit(f64);

impl SafeByteLimit {
    /// Creates a new `SafeByteLimit`, returning an error if the value exceeds 2^53 - 1.
    pub fn try_new(limit: u64) -> Result<Self, HttpError> {
        // max exact representable integer in f64 is 2^53 - 1
        if limit <= 9_007_199_254_740_991 {
            #[allow(clippy::cast_precision_loss)]
            Ok(Self(limit as f64))
        } else {
            Err(HttpError::ConfigOverflow(
                "spider_max_page_bytes exceeds f64 precise range",
            ))
        }
    }

    /// Return the underlying f64 value.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

/// Scrape strategy enumeration for type-safe documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrapeStrategy {
    /// Standard page scraping
    Standard,
    /// Sitemap-based scraping
    Sitemap,
}

/// Halt reason for extraction state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HaltReason {
    PageLimitReached,
    TotalSizeExceeded,
    #[allow(dead_code)]
    IntegerOverflow,
}

/// Extraction status for state machine.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtractionStatus {
    #[default]
    Active,
    Halted(HaltReason),
}

/// A newtype wrapper for URL sets providing domain semantics.
///
/// Uses `im::HashSet` (structural sharing, Arc-based) so `insert` is O(log n)
/// instead of O(n) clone — critical in rayon fold→reduce where UrlSet is
/// cloned per-page.
#[derive(Debug, Clone, Default)]
pub struct UrlSet(im::HashSet<String>);

impl UrlSet {
    /// Create a new empty UrlSet.
    pub fn new() -> Self {
        Self(im::HashSet::new())
    }

    /// Insert a URL, returning a new UrlSet (persistent, no mutation).
    ///
    /// O(log n) via structural sharing — no full clone of the backing HAMT.
    #[must_use]
    pub fn insert(&self, url: String) -> Self {
        Self(self.0.update(url))
    }

    /// Check if a URL is present in the set.
    #[allow(dead_code)]
    pub fn contains(&self, url: &str) -> bool {
        self.0.contains(url)
    }
}

/// Domain errors for scrape failures.
///
/// Variants capture the specific failure mode to enable precise error handling
/// and informative user messages. The `ConnectionSilentlyDropped` variant is
/// critical for detecting TCP blackholes where spider-rs cannot distinguish
/// between a slow connection and a silently dropping connection.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScrapeError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Request timeout for {url} after {timeout_secs}s")]
    RequestTimeout { url: String, timeout_secs: u64 },

    #[error("DNS error for {url}: {message}")]
    DnsError { url: String, message: String },

    #[error("Connection refused for {url}")]
    ConnectionRefused { url: String },

    #[error("Connection silently dropped for {url} — peer unreachable")]
    ConnectionSilentlyDropped { url: String },

    #[error("Too many redirects for {url}")]
    TooManyRedirects { url: String },

    #[error("HTTP {status} error for {url}")]
    HttpError { url: String, status: u16 },

    #[error("SSL error for {url}: {message}")]
    SslError { url: String, message: String },

    #[error("I/O error: {0}")]
    IoError(String),

    /// Generic application-level error with URL and message.
    /// Used for errors that don't fit network-specific categories (e.g., empty page, size limits).
    #[error("{message}")]
    Generic { url: String, message: String },
}

impl ScrapeError {
    /// Create a connection silently dropped error — the critical variant for TCP blackholes.
    #[must_use]
    pub fn connection_silently_dropped(url: String) -> Self {
        Self::ConnectionSilentlyDropped { url }
    }

    /// Create an I/O error.
    #[must_use]
    pub fn io_error(msg: impl Into<String>) -> Self {
        Self::IoError(msg.into())
    }

    /// Create a DNS error.
    #[must_use]
    pub fn dns_error(url: String, message: String) -> Self {
        Self::DnsError { url, message }
    }

    /// Create a connection refused error.
    #[must_use]
    pub fn connection_refused(url: String) -> Self {
        Self::ConnectionRefused { url }
    }

    /// Create a request timeout error.
    #[must_use]
    pub fn request_timeout(url: String, timeout_secs: u64) -> Self {
        Self::RequestTimeout { url, timeout_secs }
    }

    /// Create an HTTP error with status code.
    #[must_use]
    pub fn http_error(url: String, status: u16) -> Self {
        Self::HttpError { url, status }
    }

    /// Create an SSL error.
    #[must_use]
    pub fn ssl_error(url: String, message: String) -> Self {
        Self::SslError { url, message }
    }

    /// Create a too many redirects error.
    #[must_use]
    pub fn too_many_redirects(url: String) -> Self {
        Self::TooManyRedirects { url }
    }

    /// Create a generic scrape error with URL and message (for application-level errors).
    #[must_use]
    pub fn generic(url: String, message: String) -> Self {
        Self::Generic { url, message }
    }

    /// Extract the URL from this error, if present.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::InvalidUrl(url)
            | Self::RequestTimeout { url, .. }
            | Self::DnsError { url, .. }
            | Self::ConnectionRefused { url }
            | Self::ConnectionSilentlyDropped { url }
            | Self::TooManyRedirects { url }
            | Self::HttpError { url, .. }
            | Self::SslError { url, .. }
            | Self::Generic { url, .. } => Some(url),
            Self::IoError(_) => None,
        }
    }
}

/// Validate configuration and return errors if invalid.
fn validate_config(config: &ScrapeConfig) -> Result<(), HttpError> {
    if config.max_retries > 255 {
        return Err(HttpError::ConfigOverflow("max_retries exceeds u8 limit"));
    }
    if config.concurrency_limit == 0 {
        return Err(HttpError::ConfigOverflow("concurrency_limit cannot be 0"));
    }
    if config.concurrency_limit > u32::MAX as usize {
        return Err(HttpError::ConfigOverflow(
            "concurrency_limit exceeds u32 limit",
        ));
    }
    Ok(())
}

/// Apply timing and retry configuration options.
fn apply_timing_and_retry_options(website: &mut spider::website::Website, config: &ScrapeConfig) {
    website.configuration.delay = config.delay_ms;

    website.configuration.concurrency_limit = Some(config.concurrency_limit);

    #[allow(clippy::cast_possible_truncation)]
    let retries = config.max_retries as u8;
    website.configuration.retry = retries;

    website.configuration.request_timeout =
        Some(Box::new(Duration::from_secs(config.request_timeout_secs)));

    website.configuration.default_http_connect_timeout =
        Some(Duration::from_secs(config.connect_timeout_secs));
}

/// Check if the host is reachable within the connect timeout.
///
/// This performs a pre-check TCP connection attempt to enforce application-level
/// connect timeout, since spider's internal HTTP client doesn't properly apply
/// `default_http_connect_timeout` to TCP connect operations.
pub async fn check_connectivity_with_timeout(
    url: &ValidatedUrl,
    connect_timeout: Duration,
) -> Result<(), HttpError> {
    let parsed = url.inner();

    let host = parsed
        .host_str()
        .ok_or_else(|| HttpError::InvalidUrl("No host in URL".to_string()))?;

    let port = parsed
        .port()
        .unwrap_or_else(|| if parsed.scheme() == "https" { 443 } else { 80 });

    let addr = format!("{host}:{port}");

    let connect_result =
        tokio::time::timeout(connect_timeout, tokio::net::TcpStream::connect(&addr))
            .await
            .map_err(|_| HttpError::ConnectTimeout {
                host: host.to_string(),
                timeout_secs: connect_timeout.as_secs() as u64,
            })?;

    // Flatten: connect_result is Result<T, io::Error>, we just care if it succeeded
    connect_result.map_err(|e| HttpError::ExecutionFailed(format!("TCP connect failed: {e}")))?;

    Ok(())
}

/// Maximum pages to crawl per site. 10,000 balances completeness vs. resource usage.
const DEFAULT_CRAWL_DEPTH_LIMIT: usize = 10_000;

/// Apply policy and limits configuration options.
fn apply_policy_and_limits(
    website: &mut spider::website::Website,
    config: &ScrapeConfig,
) -> Result<(), HttpError> {
    website.configuration.respect_robots_txt = config.robots_policy == RobotsPolicy::Respect;
    website.configuration.user_agent = Some(Box::new(config.user_agent.clone().into()));
    website.configuration.modify_headers = config.stealth_mode == StealthMode::Enabled;
    website.configuration.redirect_policy = config.redirect_policy.clone();

    if let Some(limit) = config.spider_max_page_bytes {
        let safe_limit = SafeByteLimit::try_new(limit)?;
        website.configuration.max_page_bytes = Some(safe_limit.as_f64());
    }

    website.configuration.max_bytes_allowed = config.spider_max_total_bytes;

    let page_limit = u32::try_from(config.max_pages)
        .map_err(|_| HttpError::ConfigOverflow("max_pages exceeds u32 limit"))?;
    website.configuration.with_limit(page_limit);

    website.configuration.normalize = true;
    website.configuration.depth = DEFAULT_CRAWL_DEPTH_LIMIT;

    Ok(())
}

/// Apply website options to configuration.
fn apply_website_options(
    website: &mut spider::website::Website,
    config: &ScrapeConfig,
) -> Result<(), HttpError> {
    apply_timing_and_retry_options(website, config);
    apply_policy_and_limits(website, config)?;
    Ok(())
}

/// Build a spider `Website` with shared base configuration.
pub fn build_website_base(
    url: &ValidatedUrl,
    config: &ScrapeConfig,
) -> Result<spider::website::Website, HttpError> {
    validate_config(config)?;

    #[allow(unused_mut)] // spider::Website API requires &mut self for configuration
    let mut website = spider::website::Website::new(url.as_str());
    apply_website_options(&mut website, config)?;

    Ok(website)
}

/// Execute scrape operation with configurable strategy.
#[instrument(skip_all, fields(strategy = ?strategy))]
pub async fn execute_scrape_with_website(
    website: &mut spider::website::Website,
    strategy: ScrapeStrategy,
) -> Result<(), HttpError> {
    match strategy {
        ScrapeStrategy::Sitemap => {
            website.scrape_sitemap().await;
        }
        ScrapeStrategy::Standard => {
            website.scrape().await;
        }
    }

    // Verify pages were actually scraped
    // Note: spider API returns () from scrape(), errors are internal
    // If no pages extracted, something went wrong during scraping
    if website.get_pages().is_none() {
        tracing::warn!(
            strategy = ?strategy,
            "Scraping completed but no pages were extracted"
        );
        return Err(HttpError::ScrapeFailed(
            "Scraping completed but no pages were extracted".to_string(),
        ));
    }

    Ok(())
}

/// State used during the functional fold of scraped pages.
#[derive(Debug, Clone, Default)]
struct ExtractionState {
    pages: Vec<super::validation::ScrapedPage>,
    errors: Vec<ScrapeError>,
    seen_urls: UrlSet,
    total_content_size: u64,
    status: ExtractionStatus,
}

/// Merge two extraction states into a new state without mutation.
fn merge_extraction_states(a: ExtractionState, b: ExtractionState) -> ExtractionState {
    let merged_seen = a.seen_urls.0.union(b.seen_urls.0);

    ExtractionState {
        pages: a.pages.into_iter().chain(b.pages).collect(),
        errors: a.errors.into_iter().chain(b.errors).collect(),
        seen_urls: UrlSet(merged_seen),
        total_content_size: a.total_content_size.saturating_add(b.total_content_size),
        status: match (a.status, b.status) {
            (ExtractionStatus::Halted(r), _) | (_, ExtractionStatus::Halted(r)) => {
                ExtractionStatus::Halted(r)
            }
            _ => ExtractionStatus::Active,
        },
    }
}

/// Initial extraction state factory.
fn initial_extraction_state() -> ExtractionState {
    ExtractionState {
        pages: Vec::new(),
        errors: Vec::new(),
        seen_urls: UrlSet::new(),
        total_content_size: 0,
        status: ExtractionStatus::Active,
    }
}

/// Append error to error vector - pure function.
fn append_error(errors: Vec<ScrapeError>, url: String, msg: String) -> Vec<ScrapeError> {
    errors
        .into_iter()
        .chain(std::iter::once(ScrapeError::generic(url, msg)))
        .collect()
}

/// Check if page exceeds size limit.
fn check_page_size_limit(page: &spider::page::Page, limit: u64) -> Option<String> {
    let html = page.get_html();
    let html_size = html.len() as u64;
    (html_size > limit)
        .then(|| format!("Page exceeds max-page-bytes limit ({html_size} bytes > {limit} bytes)"))
}

/// Check if page limit is reached - pure function.
fn is_page_limit_reached(current_count: usize, max_pages: usize) -> bool {
    current_count >= max_pages
}

/// Check if total size limit would be exceeded - pure function.
fn would_exceed_total_size(current_size: u64, page_size: u64, max_total: u64) -> bool {
    if page_size > u64::MAX.saturating_sub(current_size) {
        return true;
    }
    current_size.saturating_add(page_size) > max_total
}

/// Check if page is eligible for processing - returns error and halt reason if not.
fn check_page_eligibility(
    page: &spider::page::Page,
    config: &ScrapeConfig,
    current_count: usize,
    max_pages: usize,
) -> Option<(String, String, Option<HaltReason>)> {
    // Check page limit
    if is_page_limit_reached(current_count, max_pages) {
        let url = page.get_url().to_string();
        let error_msg = format!("Reached page limit ({max_pages}), stopping scrape");
        return Some((url, error_msg, Some(HaltReason::PageLimitReached)));
    }

    // Check page size
    if let Some(limit) = config.spider_max_page_bytes {
        if let Some(error_msg) = check_page_size_limit(page, limit) {
            let url = page.get_url().to_string();
            return Some((url, error_msg, None));
        }
    }

    None
}

/// Handle transformed page content - check empty and size limits.
fn handle_transformed_content(
    url: String,
    scraped: super::validation::ScrapedPage,
    state: ExtractionState,
    config: &ScrapeConfig,
) -> ExtractionState {
    // Check empty content first
    if scraped.markdown.trim().is_empty() {
        return ExtractionState {
            pages: state.pages,
            errors: append_error(
                state.errors,
                url,
                "Skipped page with empty markdown content".to_string(),
            ),
            seen_urls: state.seen_urls,
            total_content_size: state.total_content_size,
            status: state.status,
        };
    }

    // Check size limit and accumulate
    check_size_and_accumulate(url, scraped, state, config)
}

/// Check if total size limit would be exceeded.
fn check_total_size_exceeded(
    state: ExtractionState,
    page_size: u64,
    max_total: u64,
) -> Option<ExtractionState> {
    if would_exceed_total_size(state.total_content_size, page_size, max_total) {
        Some(ExtractionState {
            pages: state.pages,
            errors: state.errors,
            seen_urls: state.seen_urls,
            total_content_size: state.total_content_size,
            status: ExtractionStatus::Halted(HaltReason::TotalSizeExceeded),
        })
    } else {
        None
    }
}

/// Accumulate a valid page into state.
fn accumulate_page(
    url: String,
    scraped: super::validation::ScrapedPage,
    state: ExtractionState,
    page_size: u64,
) -> ExtractionState {
    let new_seen = state.seen_urls.insert(url);
    let new_pages = state
        .pages
        .into_iter()
        .chain(std::iter::once(scraped))
        .collect();

    ExtractionState {
        pages: new_pages,
        errors: state.errors,
        seen_urls: new_seen,
        total_content_size: state.total_content_size.saturating_add(page_size),
        status: state.status,
    }
}

/// Check size limit - returns Some with error state if limit exceeded, None otherwise.
fn check_size_limit(
    state: ExtractionState,
    url: String,
    page_size: u64,
    max_total: u64,
) -> Option<ExtractionState> {
    check_total_size_exceeded(state, page_size, max_total).map(|error_state| ExtractionState {
        pages: error_state.pages,
        errors: append_error(
            error_state.errors,
            url,
            format!("Total content size would exceed limit ({max_total} bytes)"),
        ),
        seen_urls: error_state.seen_urls,
        total_content_size: error_state.total_content_size,
        status: error_state.status,
    })
}

/// Check size limit and accumulate valid page.
fn check_size_and_accumulate(
    url: String,
    scraped: super::validation::ScrapedPage,
    state: ExtractionState,
    config: &ScrapeConfig,
) -> ExtractionState {
    let page_size = scraped.markdown.len() as u64;

    // Check total size limit
    if let Some(error_state) = check_size_limit(
        state.clone(),
        url.clone(),
        page_size,
        config.max_total_size_bytes,
    ) {
        return error_state;
    }

    // Accumulate page - simple, readable code over clever chains
    accumulate_page(url, scraped, state, page_size)
}

/// Handle transform error - return state with error appended.
fn handle_transform_error(
    url: String,
    error: &anyhow::Error,
    state: ExtractionState,
) -> ExtractionState {
    ExtractionState {
        pages: state.pages,
        errors: append_error(state.errors, url, error.to_string()),
        seen_urls: state.seen_urls,
        total_content_size: state.total_content_size,
        status: state.status,
    }
}

/// Build error state from eligibility check failure.
fn build_eligibility_error_state(
    url: String,
    error_msg: String,
    halt_reason: Option<HaltReason>,
    state: ExtractionState,
) -> ExtractionState {
    ExtractionState {
        pages: state.pages,
        errors: append_error(state.errors, url, error_msg),
        seen_urls: state.seen_urls,
        total_content_size: state.total_content_size,
        status: halt_reason.map_or(state.status, ExtractionStatus::Halted),
    }
}

/// Transform and accumulate a single page - returns new state.
fn transform_and_accumulate_page(
    state: ExtractionState,
    page: &spider::page::Page,
    config: &ScrapeConfig,
    max_pages: usize,
) -> ExtractionState {
    // Check page eligibility
    if let Some((url, error_msg, halt_reason)) =
        check_page_eligibility(page, config, state.pages.len(), max_pages)
    {
        return build_eligibility_error_state(url, error_msg, halt_reason, state);
    }

    // Transform page
    let url = page.get_url().to_string();
    let transformed =
        super::transformers::transform_page(page, &config.base_url, config, config.filtering_mode);

    match transformed {
        Ok(scraped) => handle_transformed_content(url, scraped, state, config),
        Err(error) => handle_transform_error(url, &error, state),
    }
}

/// Process all spider pages using functional fold - pure function.
fn process_pages_with_fold(
    spider_pages: &[spider::page::Page],
    config: &ScrapeConfig,
) -> ExtractionState {
    spider_pages
        .into_par_iter()
        .fold(initial_extraction_state, |state, page| {
            if matches!(state.status, ExtractionStatus::Halted(_)) {
                return state;
            }
            transform_and_accumulate_page(state, page, config, config.max_pages)
        })
        .reduce(initial_extraction_state, merge_extraction_states)
}

/// Extract the total URL count from website pages.
fn get_total_url_count(website: &spider::website::Website) -> usize {
    website.get_pages().as_ref().map_or(0, |p| p.len())
}

/// Build ScrapeResult from extraction state.
fn build_scrape_result(
    final_state: ExtractionState,
    total_urls: usize,
    base_url: String,
) -> super::validation::ScrapeResult {
    let success_count = final_state.pages.len();
    super::validation::ScrapeResult {
        pages: final_state.pages,
        total_urls,
        success_count,
        error_count: final_state.errors.len(),
        errors: final_state
            .errors
            .into_iter()
            .map(|e| {
                let url = e.url().unwrap_or("unknown").to_string();
                let message = e.to_string();
                (url, message)
            })
            .collect(),
        base_url,
    }
}

/// Extract pages from `website`.
pub fn extract_pages_from_website(
    website: &spider::website::Website,
    config: &ScrapeConfig,
) -> super::validation::ScrapeResult {
    let total_urls = get_total_url_count(website);

    let final_state = website
        .get_pages()
        .as_ref()
        .map_or_else(initial_extraction_state, |spider_pages| {
            process_pages_with_fold(spider_pages.as_slice(), config)
        });

    build_scrape_result(final_state, total_urls, config.base_url.clone())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_violation_returns_invalid_url() {
        let invalid = "htp://in^valid";
        let result = ValidatedUrl::try_new(invalid);
        assert!(matches!(result, Err(HttpError::InvalidUrl(_))));
    }

    #[test]
    fn test_p2_violation_max_retries_returns_config_overflow() {
        let mut config = ScrapeConfig::default();
        config.max_retries = 256;
        let url = ValidatedUrl::try_new("https://example.com").unwrap();

        let result = build_website_base(&url, &config);
        assert_eq!(
            result.unwrap_err(),
            HttpError::ConfigOverflow("max_retries exceeds u8 limit")
        );
    }

    #[test]
    fn test_p2_violation_concurrency_limit_zero_returns_config_overflow() {
        let mut config = ScrapeConfig::default();
        config.concurrency_limit = 0;
        let url = ValidatedUrl::try_new("https://example.com").unwrap();

        let result = build_website_base(&url, &config);
        assert_eq!(
            result.unwrap_err(),
            HttpError::ConfigOverflow("concurrency_limit cannot be 0")
        );
    }

    #[test]
    fn test_p2_violation_concurrency_limit_overflow_returns_config_overflow() {
        let mut config = ScrapeConfig::default();
        config.concurrency_limit = 4294967296; // 2^32
        let url = ValidatedUrl::try_new("https://example.com").unwrap();

        let result = build_website_base(&url, &config);
        assert_eq!(
            result.unwrap_err(),
            HttpError::ConfigOverflow("concurrency_limit exceeds u32 limit")
        );
    }

    #[test]
    fn test_p3_violation_returns_config_overflow() {
        // 9007199254740992 is 2^53
        let result = SafeByteLimit::try_new(9007199254740992);
        assert_eq!(
            result.unwrap_err(),
            HttpError::ConfigOverflow("spider_max_page_bytes exceeds f64 precise range")
        );
    }

    #[test]
    fn test_extract_pages_from_website_empty() {
        let config = ScrapeConfig::default();
        let url = ValidatedUrl::try_new("https://example.com").unwrap();
        let website = build_website_base(&url, &config).unwrap();
        let result = extract_pages_from_website(&website, &config);

        assert_eq!(result.pages.len(), 0);
        assert_eq!(result.total_urls, 0);
    }

    #[test]
    fn test_scrape_error_new() {
        let err = ScrapeError::generic("http://example.com".to_string(), "Test error".to_string());
        assert_eq!(err.url(), Some("http://example.com"));
        assert!(err.to_string().contains("Test error"));
    }

    #[test]
    fn test_scrape_strategy_variants() {
        let _ = ScrapeStrategy::Standard;
        let _ = ScrapeStrategy::Sitemap;
    }

    #[test]
    fn test_extraction_status_variants() {
        let _ = ExtractionStatus::Active;
        let _ = ExtractionStatus::Halted(HaltReason::PageLimitReached);
        let _ = ExtractionStatus::Halted(HaltReason::TotalSizeExceeded);
        let _ = ExtractionStatus::Halted(HaltReason::IntegerOverflow);
    }

    #[test]
    fn test_append_error_dry_helper() {
        let errors = vec![];
        let result = append_error(errors, "url1".to_string(), "msg1".to_string());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].url(), Some("url1"));

        let result = append_error(result, "url2".to_string(), "msg2".to_string());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_urlset_new() {
        let urls = UrlSet::new();
        assert!(!urls.contains("http://example.com"));
    }

    #[test]
    fn test_urlset_insert() {
        let urls = UrlSet::new();
        let urls2 = urls.insert("http://example.com".to_string());
        assert!(!urls.contains("http://example.com"));
        assert!(urls2.contains("http://example.com"));
    }

    #[test]
    fn test_urlset_insert_persistent() {
        let urls = UrlSet::new()
            .insert("http://a.com".to_string())
            .insert("http://b.com".to_string())
            .insert("http://a.com".to_string());
        assert!(urls.contains("http://a.com"));
        assert!(urls.contains("http://b.com"));
    }

    #[test]
    fn test_urlset_default() {
        let urls = UrlSet::default();
        assert!(!urls.contains("anything"));
    }

    #[test]
    fn test_validated_url_valid() {
        let url = ValidatedUrl::try_new("https://example.com/path?query=1").unwrap();
        assert_eq!(url.as_str(), "https://example.com/path?query=1");
    }

    #[test]
    fn test_validated_url_various_schemes() {
        assert!(ValidatedUrl::try_new("https://example.com").is_ok());
        assert!(ValidatedUrl::try_new("http://example.com").is_ok());
        assert!(ValidatedUrl::try_new("ftp://example.com").is_ok());
        assert!(ValidatedUrl::try_new("not-a-url").is_err());
        assert!(ValidatedUrl::try_new("").is_err());
    }

    #[test]
    fn test_safe_byte_limit_valid() {
        let limit = SafeByteLimit::try_new(1024).unwrap();
        assert!((limit.as_f64() - 1024.0).abs() < 0.001);
    }

    #[test]
    fn test_safe_byte_limit_max_exact() {
        let limit = SafeByteLimit::try_new(9_007_199_254_740_991).unwrap();
        assert!((limit.as_f64() - 9_007_199_254_740_991.0).abs() < 1.0);
    }

    #[test]
    fn test_http_error_variants() {
        let err1 = HttpError::InvalidUrl("bad".to_string());
        assert_eq!(err1, HttpError::InvalidUrl("bad".to_string()));

        let err2 = HttpError::ConfigOverflow("overflow");
        assert_eq!(err2, HttpError::ConfigOverflow("overflow"));

        let err3 = HttpError::ExecutionFailed("exec".to_string());
        assert_eq!(err3, HttpError::ExecutionFailed("exec".to_string()));

        let err4 = HttpError::ScrapeFailed("scrape".to_string());
        assert_eq!(err4, HttpError::ScrapeFailed("scrape".to_string()));

        assert_ne!(err1, err2);
    }

    #[test]
    fn test_http_error_display() {
        let err = HttpError::InvalidUrl("http://bad".to_string());
        let msg = format!("{err}");
        assert!(msg.contains("Invalid URL"));
        assert!(msg.contains("http://bad"));

        let err2 = HttpError::ScrapeFailed("timeout".to_string());
        let msg2 = format!("{err2}");
        assert!(msg2.contains("Scrape failed"));
    }

    #[test]
    fn test_extraction_status_default() {
        assert_eq!(ExtractionStatus::default(), ExtractionStatus::Active);
    }

    #[test]
    fn test_extraction_status_equality() {
        let a = ExtractionStatus::Halted(HaltReason::PageLimitReached);
        let b = ExtractionStatus::Halted(HaltReason::PageLimitReached);
        let c = ExtractionStatus::Halted(HaltReason::TotalSizeExceeded);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_scrape_strategy_equality() {
        assert_eq!(ScrapeStrategy::Standard, ScrapeStrategy::Standard);
        assert_ne!(ScrapeStrategy::Standard, ScrapeStrategy::Sitemap);
    }

    #[test]
    fn test_is_page_limit_reached() {
        assert!(is_page_limit_reached(10, 10));
        assert!(is_page_limit_reached(11, 10));
        assert!(!is_page_limit_reached(9, 10));
    }

    #[test]
    fn test_would_exceed_total_size() {
        assert!(would_exceed_total_size(100, 1, 100));
        assert!(would_exceed_total_size(50, 60, 100));
        assert!(!would_exceed_total_size(50, 49, 100));
        assert!(would_exceed_total_size(u64::MAX, 1, u64::MAX));
    }

    #[test]
    fn test_scrape_error_equality() {
        let a = ScrapeError::generic("url".to_string(), "msg".to_string());
        let b = ScrapeError::generic("url".to_string(), "msg".to_string());
        let c = ScrapeError::generic("url2".to_string(), "msg2".to_string());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_validated_url_clone() {
        let url = ValidatedUrl::try_new("https://example.com").unwrap();
        let cloned = url.clone();
        assert_eq!(url.as_str(), cloned.as_str());
    }

    #[test]
    fn test_safe_byte_limit_copy() {
        let limit = SafeByteLimit::try_new(500).unwrap();
        let copied = limit;
        assert_eq!(limit.as_f64(), copied.as_f64());
    }

    #[test]
    fn test_halt_reason_clone() {
        let r1 = HaltReason::PageLimitReached.clone();
        assert_eq!(r1, HaltReason::PageLimitReached);
        let r2 = HaltReason::IntegerOverflow.clone();
        assert_eq!(r2, HaltReason::IntegerOverflow);
    }
}
