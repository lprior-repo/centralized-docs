#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! Adversarial QA tests for stealth mode and exponential backoff
//!
//! Tests ScrapeConfig construction and field correctness using the current API.

#![cfg(test)]

use doc_transformer::scrape::{
    FilteringMode, RetryStrategy, RobotsPolicy, ScrapeConfig, SitemapStrategy, StealthMode,
};

// ============================================================================
// BASIC CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_scrape_config_can_be_constructed() {
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        sitemap_strategy: SitemapStrategy::UseSitemap,
        path_filter: Some("^/docs/".to_string()),
        delay_ms: 1000,
        request_timeout_secs: 30,
        max_retries: 3,
        concurrency_limit: 4,
        ..Default::default()
    };

    assert_eq!(config.base_url, "https://example.com");
    assert_eq!(config.sitemap_strategy, SitemapStrategy::UseSitemap);
    assert_eq!(config.path_filter, Some("^/docs/".to_string()));
    assert_eq!(config.delay_ms, 1000);
    assert_eq!(config.max_retries, 3);
}

#[test]
fn test_scrape_config_with_none_values() {
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        path_filter: None,
        spider_max_page_bytes: None,
        spider_max_total_bytes: None,
        concurrency_limit: 1,
        ..Default::default()
    };

    assert!(config.path_filter.is_none());
    assert!(config.spider_max_page_bytes.is_none());
    assert!(config.spider_max_total_bytes.is_none());
}

#[test]
fn test_scrape_config_uses_sitemap() {
    let config = ScrapeConfig {
        sitemap_strategy: SitemapStrategy::UseSitemap,
        ..Default::default()
    };
    assert_eq!(config.sitemap_strategy, SitemapStrategy::UseSitemap);
}

#[test]
fn test_scrape_config_crawl_only() {
    let config = ScrapeConfig {
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        ..Default::default()
    };
    assert_eq!(config.sitemap_strategy, SitemapStrategy::CrawlOnly);
}

#[test]
fn test_scrape_config_stealth_mode_variants() {
    let stealth = ScrapeConfig {
        stealth_mode: StealthMode::Enabled,
        ..Default::default()
    };
    assert_eq!(stealth.stealth_mode, StealthMode::Enabled);

    let plain = ScrapeConfig {
        stealth_mode: StealthMode::Disabled,
        ..Default::default()
    };
    assert_eq!(plain.stealth_mode, StealthMode::Disabled);
}

#[test]
fn test_scrape_config_retry_strategy_variants() {
    let exponential = ScrapeConfig {
        retry_strategy: RetryStrategy::ExponentialBackoff,
        ..Default::default()
    };
    assert_eq!(
        exponential.retry_strategy,
        RetryStrategy::ExponentialBackoff
    );

    let fixed = ScrapeConfig {
        retry_strategy: RetryStrategy::Fixed,
        ..Default::default()
    };
    assert_eq!(fixed.retry_strategy, RetryStrategy::Fixed);
}

#[test]
fn test_scrape_config_robots_policy_variants() {
    let respectful = ScrapeConfig {
        robots_policy: RobotsPolicy::Respect,
        ..Default::default()
    };
    assert_eq!(respectful.robots_policy, RobotsPolicy::Respect);

    let aggressive = ScrapeConfig {
        robots_policy: RobotsPolicy::Ignore,
        ..Default::default()
    };
    assert_eq!(aggressive.robots_policy, RobotsPolicy::Ignore);
}

#[test]
fn test_scrape_config_filtering_mode_variants() {
    let filtered = ScrapeConfig {
        filtering_mode: FilteringMode::Enabled,
        ..Default::default()
    };
    assert_eq!(filtered.filtering_mode, FilteringMode::Enabled);

    let raw = ScrapeConfig {
        filtering_mode: FilteringMode::Disabled,
        ..Default::default()
    };
    assert_eq!(raw.filtering_mode, FilteringMode::Disabled);
}
