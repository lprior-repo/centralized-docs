#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

//! HTTP client and website configuration
//!
//! Provides spider-rs website building and HTTP request configuration.

use super::validation::{RobotsPolicy, ScrapeConfig, StealthMode};
use rayon::prelude::*;
use std::time::Duration;
use thiserror::Error;
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
#[derive(Debug, Clone, Default)]
pub struct UrlSet(std::collections::HashSet<String>);

impl UrlSet {
    /// Create a new empty UrlSet.
    pub fn new() -> Self {
        Self(std::collections::HashSet::new())
    }

    /// Insert a URL, returning a new UrlSet (persistent, no mutation).
    #[must_use]
    pub fn insert(&self, url: String) -> Self {
        let mut new_set = self.0.clone();
        new_set.insert(url);
        Self(new_set)
    }

    /// Check if a URL is present in the set.
    #[allow(dead_code)]
    pub fn contains(&self, url: &str) -> bool {
        self.0.contains(url)
    }
}

/// Structured error for scrape failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapeError {
    pub url: String,
    pub message: String,
}

impl ScrapeError {
    pub fn new(url: String, message: String) -> Self {
        Self { url, message }
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
fn apply_timing_and_retry_options(
    website: &mut spider::website::Website,
    config: &ScrapeConfig,
) -> Result<(), HttpError> {
    website.configuration.delay = config.delay_ms;

    website.configuration.concurrency_limit = Some(config.concurrency_limit);

    #[allow(clippy::cast_possible_truncation)]
    let retries = config.max_retries as u8;
    website.configuration.retry = retries;

    website.configuration.request_timeout =
        Some(Box::new(Duration::from_secs(config.request_timeout_secs)));

    Ok(())
}

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
    website.configuration.depth = 10000; // Large depth

    Ok(())
}

/// Apply website options to configuration.
fn apply_website_options(
    website: &mut spider::website::Website,
    config: &ScrapeConfig,
) -> Result<(), HttpError> {
    apply_timing_and_retry_options(website, config)?;
    apply_policy_and_limits(website, config)?;
    Ok(())
}

/// Build a spider `Website` with shared base configuration.
pub fn build_website_base(
    url: ValidatedUrl,
    config: &ScrapeConfig,
) -> Result<spider::website::Website, HttpError> {
    validate_config(config)?;

    let mut website = spider::website::Website::new(url.as_str());
    apply_website_options(&mut website, config)?;

    Ok(website)
}

/// Execute scrape operation with configurable strategy.
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
    if website.get_pages().is_none() {
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
    let merged_seen = b
        .pages
        .iter()
        .fold(a.seen_urls, |acc, p| acc.insert(p.url.clone()));

    ExtractionState {
        pages: a.pages.into_iter().chain(b.pages).collect(),
        errors: a.errors.into_iter().chain(b.errors).collect(),
        seen_urls: merged_seen,
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
        .chain(std::iter::once(ScrapeError::new(url, msg)))
        .collect()
}

/// Check if page exceeds size limit.
fn check_page_size_limit(page: &spider::page::Page, limit: u64) -> Option<String> {
    let html = page.get_html();
    let html_size = html.len() as u64;
    (html_size > limit).then(|| {
        format!(
            "Page exceeds max-page-bytes limit ({} bytes > {} bytes)",
            html_size, limit
        )
    })
}

/// Check if page limit is reached - pure function.
fn is_page_limit_reached(current_count: usize, max_pages: usize) -> bool {
    current_count >= max_pages
}

/// Check if total size limit would be exceeded - pure function.
fn would_exceed_total_size(current_size: u64, page_size: u64, max_total: u64) -> bool {
    if current_size > u64::MAX - page_size {
        return true;
    }
    current_size + page_size > max_total
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
        let error_msg = format!("Reached page limit ({}), stopping scrape", max_pages);
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
        total_content_size: state.total_content_size + page_size,
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
    check_total_size_exceeded(state, page_size, max_total).map(|mut error_state| {
        error_state.errors = append_error(
            error_state.errors,
            url,
            format!(
                "Total content size would exceed limit ({} bytes)",
                max_total
            ),
        );
        error_state
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
    error: anyhow::Error,
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
        Err(error) => handle_transform_error(url, error, state),
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
        .reduce(|| initial_extraction_state(), merge_extraction_states)
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
            .map(|e| (e.url, e.message))
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

        let result = build_website_base(url, &config);
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

        let result = build_website_base(url, &config);
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

        let result = build_website_base(url, &config);
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
        let website = build_website_base(url, &config).unwrap();
        let result = extract_pages_from_website(&website, &config);

        assert_eq!(result.pages.len(), 0);
        assert_eq!(result.total_urls, 0);
    }

    #[test]
    fn test_scrape_error_new() {
        let err = ScrapeError::new("http://example.com".to_string(), "Test error".to_string());
        assert_eq!(err.url, "http://example.com");
        assert_eq!(err.message, "Test error".to_string());
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
        assert_eq!(result[0].url, "url1");

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
}
