#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

//! Scraping module orchestration
//!
//! This module coordinates scraping operations across submodules:
//! - HTTP client (via `http`)
//! - HTML parsing (via `html_parser`)
//! - Markdown conversion (via `markdown`)
//! - Content filtering (via `filtering`)
//! - URL validation (via `validation`)
//! - Data transformations (via `transformers`)

use anyhow::{Context, Result};
use tracing::instrument;

pub mod filtering;
pub mod html_parser;
pub mod http;
pub mod markdown;
pub mod transformers;
pub mod validation;

// Re-export public API
pub use http::{build_website_base, execute_scrape_with_website, extract_pages_from_website};
pub use transformers::{calculate_backoff_delay, write_scraped_pages};
pub use validation::{detect_potential_spa, validate_scrape_result, validate_url};

// Re-export types — public API for library consumers; binary may not use all variants
#[allow(unused_imports)]
pub use validation::{
    FilteringMode, PageFilterStatus, RetryStrategy, RobotsPolicy, ScrapeConfig, ScrapeResult,
    ScrapedPage, SitemapStrategy, StealthMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScrapePlan {
    strategy: http::ScrapeStrategy,
    max_pages: usize,
}

fn derive_scrape_plan(config: &ScrapeConfig, sitemap_found: bool) -> ScrapePlan {
    match (config.sitemap_strategy, sitemap_found) {
        (SitemapStrategy::UseSitemap, true) => ScrapePlan {
            strategy: http::ScrapeStrategy::Sitemap,
            max_pages: config.max_pages,
        },
        (SitemapStrategy::UseSitemap, false) | (SitemapStrategy::CrawlOnly, _) => ScrapePlan {
            strategy: http::ScrapeStrategy::Standard,
            max_pages: config.max_pages,
        },
    }
}

/// Scrape a documentation site with exponential backoff retry on rate limits
///
/// # Errors
/// Returns an error if the scrape operation fails after all retries are exhausted.
#[instrument(skip_all, fields(base_url = %config.base_url, max_pages = config.max_pages))]
pub async fn scrape_site(config: &ScrapeConfig) -> Result<ScrapeResult> {
    const BASE_DELAY_MS: u64 = 2000;

    let max_retries = config.max_retries.min(10);
    #[allow(unused_mut)] // I/O boundary: async retry loop requires mutable attempt counter
    let mut attempt: u32 = 0;

    // Quick sitemap check - try sitemap first with a small page limit (5 pages max)
    // This quickly determines if sitemap exists before doing a full crawl
    let plan = if config.sitemap_strategy == SitemapStrategy::UseSitemap {
        let quick_config = validation::ScrapeConfig {
            max_pages: 5,
            ..config.clone()
        };

        let test_result = scrape_single_attempt(&quick_config, http::ScrapeStrategy::Sitemap).await;
        let pages_found = test_result.as_ref().is_ok_and(|r| r.success_count > 0);

        if pages_found {
            tracing::info!("Sitemap found, using sitemap strategy");
        } else {
            tracing::info!("No URLs found in sitemap, falling back to crawling");
        }
        derive_scrape_plan(config, pages_found)
    } else {
        derive_scrape_plan(config, false)
    };

    // Create config with effective page limit for the actual scrape
    let effective_config = validation::ScrapeConfig {
        max_pages: plan.max_pages,
        ..config.clone()
    };

    loop {
        attempt = attempt.saturating_add(1);

        match scrape_single_attempt(&effective_config, plan.strategy).await {
            Ok(result) => {
                if config.retry_strategy == RetryStrategy::Fixed {
                    return Ok(result);
                }

                let total_requests = result.success_count.saturating_add(result.error_count);

                if total_requests > 10
                    && result.error_count > result.success_count
                    && attempt <= max_retries
                {
                    let delay_ms = calculate_backoff_delay(BASE_DELAY_MS, attempt);
                    tracing::warn!(
                        error_count = result.error_count,
                        total_requests = total_requests,
                        "High error rate detected"
                    );
                    tracing::warn!(
                        delay_ms = delay_ms,
                        attempt = attempt,
                        "Waiting before retry"
                    );

                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    continue;
                }

                if attempt > 1 {
                    tracing::info!(attempt = attempt, "Scrape completed after retry attempts");
                }

                return Ok(result);
            }
            Err(e) => {
                let error_msg = e.to_string();

                let is_transient = error_msg.contains("timeout")
                    || error_msg.contains("connection")
                    || error_msg.contains("dns")
                    || error_msg.contains("rate");

                if config.retry_strategy == RetryStrategy::ExponentialBackoff
                    && is_transient
                    && attempt <= max_retries
                {
                    let delay_ms = calculate_backoff_delay(BASE_DELAY_MS, attempt);
                    tracing::warn!(
                        error_msg = %error_msg,
                        delay_ms = delay_ms,
                        attempt = attempt,
                        max_retries = max_retries,
                        "Transient error, retrying"
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    continue;
                }

                return Err(e);
            }
        }
    }
}

/// Execute a single scrape attempt with explicit sitemap strategy
async fn scrape_single_attempt(
    config: &validation::ScrapeConfig,
    strategy: http::ScrapeStrategy,
) -> Result<validation::ScrapeResult> {
    let validated_url = validate_url(&config.base_url)?;

    let valid_url_wrapper = http::ValidatedUrl::try_new(validated_url.as_str())
        .map_err(|e| anyhow::anyhow!("Invalid URL: {e}"))?;
    #[allow(unused_mut)] // I/O boundary: spider::Website requires &mut for scrape operations
    let mut website = build_website_base(&valid_url_wrapper, config)
        .map_err(|e| anyhow::anyhow!("Config error: {e}"))?;

    if let Some(ref pattern) = config.path_filter {
        validation::compile_safe_regex(pattern).context("Path filter regex validation failed")?;

        let base_domain = url::Url::parse(&config.base_url)
            .map(|u| u.host_str().map_or("", |h| h).to_string())
            .unwrap_or_default();
        let scheme = url::Url::parse(&config.base_url)
            .map_or_else(|_| "https".to_string(), |u| u.scheme().to_string());

        let domain_escaped = regex::escape(&base_domain);
        let pattern_stripped = pattern.strip_prefix('^').map_or(pattern.as_str(), |s| s);
        let full_url_pattern = format!("^{scheme}://{domain_escaped}{pattern_stripped}");
        let _ = website
            .configuration
            .with_whitelist_url(Some(vec![full_url_pattern.as_str().into()]));
        website.configuration.configure_allowlist();
    }

    execute_scrape_with_website(&mut website, strategy)
        .await
        .map_err(|e| anyhow::anyhow!("Execution failed: {e}"))?;

    Ok(extract_pages_from_website(&website, config))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{derive_scrape_plan, http, ScrapeConfig, SitemapStrategy};

    #[test]
    fn sitemap_success_uses_sitemap_strategy_with_requested_budget() {
        let config = ScrapeConfig {
            sitemap_strategy: SitemapStrategy::UseSitemap,
            max_pages: 500,
            ..Default::default()
        };

        let plan = derive_scrape_plan(&config, true);

        assert_eq!(plan.strategy, http::ScrapeStrategy::Sitemap);
        assert_eq!(plan.max_pages, 500);
    }

    #[test]
    fn sitemap_fallback_preserves_requested_budget() {
        let config = ScrapeConfig {
            sitemap_strategy: SitemapStrategy::UseSitemap,
            max_pages: 500,
            ..Default::default()
        };

        let plan = derive_scrape_plan(&config, false);

        assert_eq!(plan.strategy, http::ScrapeStrategy::Standard);
        assert_eq!(plan.max_pages, 500);
    }

    #[test]
    fn crawl_only_preserves_requested_budget() {
        let config = ScrapeConfig {
            sitemap_strategy: SitemapStrategy::CrawlOnly,
            max_pages: 37,
            ..Default::default()
        };

        let plan = derive_scrape_plan(&config, false);

        assert_eq!(plan.strategy, http::ScrapeStrategy::Standard);
        assert_eq!(plan.max_pages, 37);
    }

    #[test]
    fn crawl_only_always_standard_even_with_sitemap() {
        let config = ScrapeConfig {
            sitemap_strategy: SitemapStrategy::CrawlOnly,
            max_pages: 100,
            ..Default::default()
        };

        let plan = derive_scrape_plan(&config, true);

        assert_eq!(plan.strategy, http::ScrapeStrategy::Standard);
    }

    #[test]
    fn scrape_plan_debug_clone() {
        let plan = derive_scrape_plan(
            &ScrapeConfig {
                sitemap_strategy: SitemapStrategy::UseSitemap,
                max_pages: 10,
                ..Default::default()
            },
            true,
        );
        let plan2 = plan.clone();
        assert_eq!(plan, plan2);
        let dbg = format!("{plan:?}");
        assert!(dbg.contains("Sitemap"));
    }

    #[test]
    fn derive_scrape_plan_default_config() {
        let config = ScrapeConfig::default();
        let plan = derive_scrape_plan(&config, false);
        assert_eq!(plan.strategy, http::ScrapeStrategy::Standard);
    }
}
