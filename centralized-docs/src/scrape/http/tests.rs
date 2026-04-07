//! Tests for the HTTP scrape module.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use crate::scrape::validation::ScrapeConfig;

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

// Import private helpers for testing
use checks::{append_error, is_page_limit_reached, would_exceed_total_size};
