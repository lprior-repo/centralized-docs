//! HTTP client building and connectivity checks.

use super::types::{HttpError, SafeByteLimit, ScrapeStrategy, ValidatedUrl};
use crate::scrape::validation::{RobotsPolicy, ScrapeConfig, StealthMode};
use std::time::Duration;
use tracing::instrument;

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
