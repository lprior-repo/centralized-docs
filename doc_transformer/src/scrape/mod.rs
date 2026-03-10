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

/// Scrape a documentation site with exponential backoff retry on rate limits
///
/// # Errors
/// Returns an error if the scrape operation fails after all retries are exhausted.
pub async fn scrape_site(config: &ScrapeConfig) -> Result<ScrapeResult> {
    const BASE_DELAY_MS: u64 = 2000;

    let max_retries = config.max_retries.min(10);
    let mut attempt: u32 = 0;

    // Quick sitemap check - try sitemap first with a small page limit (5 pages max)
    // This quickly determines if sitemap exists before doing a full crawl
    let (strategy, effective_max_pages) = if config.sitemap_strategy == SitemapStrategy::UseSitemap
    {
        let quick_config = validation::ScrapeConfig {
            max_pages: 5,
            ..config.clone()
        };

        let test_result = scrape_single_attempt(&quick_config, http::ScrapeStrategy::Sitemap).await;
        let pages_found = test_result.as_ref().is_ok_and(|r| r.success_count > 0);

        if pages_found {
            println!("[SCRAPE] Sitemap found, using sitemap strategy...");
            (http::ScrapeStrategy::Sitemap, config.max_pages)
        } else {
            println!("[SCRAPE] No URLs found in sitemap, falling back to crawling...");
            // Cap at 100 pages for fallback crawl to avoid infinite crawls on SPA sites
            let capped = config.max_pages.min(100);
            if config.max_pages > 100 {
                println!("[SCRAPE] Limiting to {capped} pages for sitemap fallback crawl");
            }
            (http::ScrapeStrategy::Standard, capped)
        }
    } else {
        (http::ScrapeStrategy::Standard, config.max_pages)
    };

    // Create config with effective page limit for the actual scrape
    let effective_config = validation::ScrapeConfig {
        max_pages: effective_max_pages,
        ..config.clone()
    };

    loop {
        attempt = attempt.saturating_add(1);

        match scrape_single_attempt(&effective_config, strategy).await {
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
                    println!(
                        "[RATE LIMIT] High error rate detected ({} errors / {} total)",
                        result.error_count, total_requests
                    );
                    println!("[RETRY] Waiting {delay_ms}ms before retry {attempt}...");

                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    continue;
                }

                if attempt > 1 {
                    println!("[SUCCESS] Scrape completed after {attempt} attempts");
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
                    println!("[RETRY] Transient error: {error_msg}. Retrying in {delay_ms}ms (attempt {attempt}/{max_retries})");
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
        .map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?;
    let mut website = build_website_base(valid_url_wrapper, config)
        .map_err(|e| anyhow::anyhow!("Config error: {}", e))?;

    if let Some(ref pattern) = config.path_filter {
        validation::compile_safe_regex(pattern).context("Path filter regex validation failed")?;

        let base_domain = url::Url::parse(&config.base_url)
            .map(|u| u.host_str().unwrap_or("").to_string())
            .unwrap_or_default();
        let scheme = url::Url::parse(&config.base_url)
            .map_or_else(|_| "https".to_string(), |u| u.scheme().to_string());

        let domain_escaped = regex::escape(&base_domain);
        let pattern_stripped = pattern.strip_prefix('^').unwrap_or(pattern);
        let full_url_pattern = format!("^{scheme}://{domain_escaped}{pattern_stripped}");
        let _ = website
            .configuration
            .with_whitelist_url(Some(vec![full_url_pattern.as_str().into()]));
        website.configuration.configure_allowlist();
    }

    execute_scrape_with_website(&mut website, strategy)
        .await
        .map_err(|e| anyhow::anyhow!("Execution failed: {}", e))?;

    Ok(extract_pages_from_website(&website, config))
}
