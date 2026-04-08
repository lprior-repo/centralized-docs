#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Tests for TCP Connect Timeout Fix (cdocs-roi)
//!
//! These tests verify:
//! 1. `connect_timeout_secs` is validated as 1-60 range (not 1-600)
//! 2. Blackhole IP (192.0.2.1) fails fast with ConnectTimeout error
//! 3. ScrapeResult content validation
//! 4. Timeout independence
//!
//! RED PHASE: These tests define the expected contract. Some tests MUST fail
//! until the implementation is corrected to enforce 1-60 range for connect_timeout.

use std::time::{Duration, Instant};
use tempfile::TempDir;

// Import from the library
use doc_transformer::scrape::http::ValidatedUrl;
use doc_transformer::scrape::{
    build_website_base, validate_url, FilteringMode, PageFilterStatus, RetryStrategy, RobotsPolicy,
    ScrapeConfig, ScrapeResult, ScrapedPage, SitemapStrategy, StealthMode,
};
use spider::configuration::RedirectPolicy;

// =============================================================================
// TEST FIXTURES
// =============================================================================

/// Get the path to the compiled binary
fn binary_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

/// Run the CLI with given arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    let binary = binary_path();
    std::process::Command::new(&binary)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute binary: {}", binary.display()))
}

// =============================================================================
// CLI VALIDATION TESTS - connect_timeout_secs must be 1-60
// =============================================================================

/// Verify that scrape command accepts connect_timeout_secs=1 (minimum valid)
#[test]
fn scrape_accepts_connect_timeout_secs_1() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // This should NOT fail due to validation - connect_timeout_secs=1 is valid
    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "1",
    ]);

    // We expect failure due to the URL being inaccessible in test environment,
    // but NOT due to validation error for connect_timeout_secs
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("must be between 1 and 60"),
        "connect_timeout_secs=1 should be valid, but got error: {stderr}"
    );
}

/// Verify that scrape command accepts connect_timeout_secs=60 (maximum valid)
#[test]
fn scrape_accepts_connect_timeout_secs_60() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "60",
    ]);

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("must be between 1 and 60"),
        "connect_timeout_secs=60 should be valid, but got error: {stderr}"
    );
}

/// Verify that scrape command accepts connect_timeout_secs=30 (middle value)
#[test]
fn scrape_accepts_connect_timeout_secs_30() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "30",
    ]);

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("must be between 1 and 60"),
        "connect_timeout_secs=30 should be valid, but got error: {stderr}"
    );
}

/// Verify that scrape command rejects connect_timeout_secs=0 (below minimum)
#[test]
fn scrape_rejects_connect_timeout_secs_0() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "0",
    ]);

    assert!(
        !result.status.success(),
        "connect_timeout_secs=0 should be rejected"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("must be between 1 and 60") || stderr.contains("at least 1"),
        "Error should mention valid range 1-60. Got: {stderr}"
    );
}

/// Verify that scrape command rejects connect_timeout_secs=61 (above maximum)
/// CRITICAL: This test FAILS with current implementation because validate_timeout_secs
/// accepts 1-600, so 61 is currently accepted when it should be rejected.
#[test]
fn scrape_rejects_connect_timeout_secs_61() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "61",
    ]);

    // This assertion will FAIL until the implementation is fixed
    // Current implementation uses validate_timeout_secs which accepts 1-600
    // But connect_timeout_secs should only accept 1-60
    assert!(
        !result.status.success(),
        "connect_timeout_secs=61 should be rejected (max is 60)"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("must be between 1 and 60") || stderr.contains("at most 60"),
        "Error should mention max 60. Got: {stderr}"
    );
}

/// Verify that ingest command rejects connect_timeout_secs=61
#[test]
fn ingest_rejects_connect_timeout_secs_61() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "61",
    ]);

    assert!(
        !result.status.success(),
        "connect_timeout_secs=61 should be rejected on ingest command"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("must be between 1 and 60") || stderr.contains("at most 60"),
        "Error should mention max 60. Got: {stderr}"
    );
}

/// Verify that scrape command rejects connect_timeout_secs=u64::MAX
#[test]
fn scrape_rejects_connect_timeout_secs_u64_max() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "18446744073709551615",
    ]);

    assert!(
        !result.status.success(),
        "connect_timeout_secs=u64::MAX should be rejected"
    );
}

/// Verify that ingest command accepts connect_timeout_secs=30
#[test]
fn ingest_accepts_connect_timeout_secs_30() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "30",
    ]);

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("must be between 1 and 60"),
        "connect_timeout_secs=30 should be valid for ingest, but got error: {stderr}"
    );
}

// =============================================================================
// WATCH COMMAND - now has --connect-timeout-secs flag
// =============================================================================

/// Verify that watch command has --connect-timeout-secs flag
#[test]
fn watch_command_has_connect_timeout_flag() {
    let result = run_cli(&["watch", "--help"]);

    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);

    assert!(
        output.contains("--connect-timeout-secs"),
        "watch command should have --connect-timeout-secs flag. Output: {output}"
    );
}

/// Verify that watch command accepts --connect-timeout-secs flag
#[test]
fn watch_command_accepts_connect_timeout_flag() {
    let result = run_cli(&["watch", "--help"]);

    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);

    assert!(
        output.contains("--connect-timeout-secs"),
        "watch command should accept --connect-timeout-secs. Output: {output}"
    );
}

// =============================================================================
// SCRAPECONFIG DEFAULTS TESTS
// =============================================================================

/// Verify that ScrapeConfig defaults connect_timeout_secs to 10
#[test]
fn scrape_config_defaults_connect_timeout_to_10() {
    let config = ScrapeConfig::default();
    assert_eq!(
        config.connect_timeout_secs, 10,
        "ScrapeConfig.connect_timeout_secs should default to 10"
    );
}

/// Verify that ScrapeConfig receives connect_timeout_secs correctly
#[test]
fn scrape_config_receives_connect_timeout_correctly() {
    let config = ScrapeConfig {
        connect_timeout_secs: 25,
        request_timeout_secs: 30,
        ..Default::default()
    };

    assert_eq!(
        config.connect_timeout_secs, 25,
        "ScrapeConfig.connect_timeout_secs should be 25"
    );
}

// =============================================================================
// WEBSITE CONFIGURATION TESTS - connect_timeout is applied correctly
// =============================================================================

/// Verify that connect_timeout is set on website configuration
#[test]
fn connect_timeout_is_applied_to_website_configuration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        connect_timeout_secs: 15,
        request_timeout_secs: 30,
        ..Default::default()
    };

    let url = ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    assert!(
        website.configuration.default_http_connect_timeout.is_some(),
        "default_http_connect_timeout should be set"
    );

    let timeout = website.configuration.default_http_connect_timeout.unwrap();
    assert_eq!(
        timeout,
        Duration::from_secs(15),
        "default_http_connect_timeout should be 15 seconds"
    );
}

/// Verify that connect_timeout=1 produces Duration::from_secs(1)
#[test]
fn connect_timeout_1_sets_1_second_duration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        connect_timeout_secs: 1,
        ..Default::default()
    };

    let url = ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    let timeout = website.configuration.default_http_connect_timeout.unwrap();
    assert_eq!(timeout, Duration::from_secs(1));
}

/// Verify that connect_timeout=60 produces Duration::from_secs(60)
#[test]
fn connect_timeout_60_sets_60_second_duration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        connect_timeout_secs: 60,
        ..Default::default()
    };

    let url = ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    let timeout = website.configuration.default_http_connect_timeout.unwrap();
    assert_eq!(timeout, Duration::from_secs(60));
}

/// Verify that request_timeout is also set (both timeouts work)
#[test]
fn both_timeouts_are_applied_to_website_configuration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        connect_timeout_secs: 5,
        request_timeout_secs: 30,
        ..Default::default()
    };

    let url = ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    // Both timeouts should be set
    assert!(
        website.configuration.default_http_connect_timeout.is_some(),
        "default_http_connect_timeout should be set"
    );
    assert!(
        website.configuration.request_timeout.is_some(),
        "request_timeout should be set"
    );

    let connect_timeout = website.configuration.default_http_connect_timeout.unwrap();
    let request_timeout = website.configuration.request_timeout.unwrap();

    assert_eq!(connect_timeout, Duration::from_secs(5));
    assert_eq!(request_timeout.as_ref(), &Duration::from_secs(30));
}

// =============================================================================
// BLACKHOLE IP E2E TESTS - The critical acceptance tests
// =============================================================================

/// CRITICAL TEST: Blackhole IP (192.0.2.1) should fail within connect_timeout,
/// NOT the OS TCP timeout (~30-40s).
///
/// This test uses 192.0.2.1 (RFC 5737 TEST-NET-1) which is a blackhole IP
/// that never responds to TCP connections.
///
/// The test verifies that the scrape fails fast with the configured timeout.
///
/// NOTE: This test is IGNORED because spider v2.39's `default_http_connect_timeout`
/// is not properly applied to TCP connect operations. The OS TCP timeout (~15s)
/// is used instead of the application-specified connect_timeout.
/// See: https://github.com/spider-rs/spider/issues/[relevant-issue]
#[tokio::test]
async fn blackhole_ip_fails_within_connect_timeout_not_os_timeout() {
    let config = ScrapeConfig {
        base_url: "http://192.0.2.1".to_string(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        connect_timeout_secs: 3, // 3 second connect timeout
        request_timeout_secs: 30,
        stealth_mode: StealthMode::Disabled,
        robots_policy: RobotsPolicy::Ignore,
        retry_strategy: RetryStrategy::Fixed,
        filtering_mode: FilteringMode::Disabled,
        redirect_policy: RedirectPolicy::Loose,
        max_pages: 10,
        max_retries: 0,
        ..Default::default()
    };

    let start = Instant::now();

    // This should fail fast with ConnectTimeout, not hang for 30-40s
    let result = doc_transformer::scrape::scrape_site(&config).await;

    let elapsed = start.elapsed();

    // The scrape MUST fail (blackhole IP never responds)
    assert!(
        result.is_err(),
        "Scrape to blackhole IP should fail, got: {:?}",
        result
    );

    // CRITICAL: The error should occur in ~3 seconds, NOT 30-40 seconds
    // We allow 8 seconds total (3s timeout + 5s tolerance) to account for
    // OS-level TCP retransmission timing variations
    assert!(
        elapsed < Duration::from_secs(8),
        "FAIL: Scrape took {}s — OS TCP timeout (~30-40s) was used instead of connect_timeout (3s). \
         This indicates the connect_timeout_secs setting is not being applied correctly.",
        elapsed.as_secs()
    );

    // Verify it's a timeout-related error
    let error_msg = result.unwrap_err().to_string().to_lowercase();
    assert!(
        error_msg.contains("timeout") || error_msg.contains("connect"),
        "Error should mention timeout or connect, got: {}",
        error_msg
    );
}

/// CRITICAL TEST: Verify 5s connect timeout fails within ~5s, not ~40s
///
/// NOTE: This test is IGNORED because spider v2.39's `default_http_connect_timeout`
/// is not properly applied to TCP connect operations. The OS TCP timeout (~15s)
/// is used instead of the application-specified connect_timeout.
#[tokio::test]
async fn blackhole_ip_5s_timeout_fails_within_5_seconds() {
    let config = ScrapeConfig {
        base_url: "http://192.0.2.1".to_string(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        connect_timeout_secs: 5, // 5 second connect timeout
        request_timeout_secs: 60,
        stealth_mode: StealthMode::Disabled,
        robots_policy: RobotsPolicy::Ignore,
        retry_strategy: RetryStrategy::Fixed,
        filtering_mode: FilteringMode::Disabled,
        redirect_policy: RedirectPolicy::Loose,
        max_pages: 10,
        max_retries: 0,
        ..Default::default()
    };

    let start = Instant::now();
    let _result = doc_transformer::scrape::scrape_site(&config).await;
    let elapsed = start.elapsed();

    // Should fail in ~5s, not ~40s
    assert!(
        elapsed < Duration::from_secs(10),
        "FAIL: Scrape took {}s — OS TCP timeout was used instead of 5s connect_timeout",
        elapsed.as_secs()
    );
}

// =============================================================================
// SCRAPE ERROR TAXONOMY TESTS
// =============================================================================

/// Verify that validate_url returns error for malformed URLs
#[test]
fn validate_url_returns_error_for_malformed_input() {
    let result = validate_url("not-a-url");
    assert!(
        result.is_err(),
        "validate_url should return error for malformed URL"
    );
}

/// Verify that validate_url returns Ok for well-formed URLs
#[test]
fn validate_url_returns_ok_for_well_formed_url() {
    let result = validate_url("https://example.com");
    assert!(
        result.is_ok(),
        "validate_url should return Ok for well-formed URL, got: {:?}",
        result
    );
}

// =============================================================================
// SCRAPE RESULT CONTENT VALIDATION TESTS
// =============================================================================

/// Verify ScrapeResult has expected fields
#[test]
fn scrape_result_has_expected_fields() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 0,
        success_count: 0,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    assert_eq!(result.base_url, "https://example.com");
    assert_eq!(result.success_count, 0);
    assert_eq!(result.error_count, 0);
    assert!(result.pages.is_empty());
    assert!(result.errors.is_empty());
}

/// Verify ScrapeResult can hold multiple pages
#[test]
fn scrape_result_can_hold_multiple_pages() {
    let page1 = ScrapedPage {
        url: "https://example.com/page1".to_string(),
        markdown: "# Page 1".to_string(),
        title: "Page 1".to_string(),
        links: vec![],
        headers: vec![],
        word_count: 10,
        slug: "page1".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page2 = ScrapedPage {
        url: "https://example.com/page2".to_string(),
        markdown: "# Page 2".to_string(),
        title: "Page 2".to_string(),
        links: vec![],
        headers: vec![],
        word_count: 20,
        slug: "page2".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let result = ScrapeResult {
        pages: vec![page1, page2],
        total_urls: 2,
        success_count: 2,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    assert_eq!(result.pages.len(), 2);
    assert_eq!(result.success_count, 2);
}

/// Verify ScrapeResult can track errors
#[test]
fn scrape_result_can_track_errors() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 5,
        success_count: 3,
        error_count: 2,
        errors: vec![
            (
                "https://example.com/fail1".to_string(),
                "Connection refused".to_string(),
            ),
            (
                "https://example.com/fail2".to_string(),
                "Timeout".to_string(),
            ),
        ],
        base_url: "https://example.com".to_string(),
    };

    assert_eq!(result.error_count, 2);
    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0].0, "https://example.com/fail1");
}

// =============================================================================
// TWO TIMEOUT INDEPENDENCE TESTS
// =============================================================================

/// Verify that connect_timeout and request_timeout are independent
#[test]
fn connect_and_request_timeouts_are_independent() {
    let mut config = ScrapeConfig {
        connect_timeout_secs: 15,
        request_timeout_secs: 120,
        ..Default::default()
    };

    assert_eq!(config.connect_timeout_secs, 15);
    assert_eq!(config.request_timeout_secs, 120);

    // Modify one should not affect the other
    config.connect_timeout_secs = 30;
    assert_eq!(config.request_timeout_secs, 120); // Unchanged
    assert_eq!(config.connect_timeout_secs, 30);
}

/// Verify that both timeouts can be set to their minimum values
#[test]
fn both_timeouts_can_be_set_to_minimum() {
    let config = ScrapeConfig {
        connect_timeout_secs: 1, // Minimum for connect
        request_timeout_secs: 1, // Minimum for request
        ..Default::default()
    };

    assert_eq!(config.connect_timeout_secs, 1);
    assert_eq!(config.request_timeout_secs, 1);
}

/// Verify that both timeouts can be set to their maximum values
#[test]
fn both_timeouts_can_be_set_to_maximum() {
    let config = ScrapeConfig {
        connect_timeout_secs: 60,  // Maximum for connect
        request_timeout_secs: 600, // Maximum for request
        ..Default::default()
    };

    assert_eq!(config.connect_timeout_secs, 60);
    assert_eq!(config.request_timeout_secs, 600);
}

// =============================================================================
// INTEGRATION: CLI + SCRAPE SITE WIRING
// =============================================================================

/// Verify that scrape command with valid connect_timeout_secs does not fail on CLI parsing
#[test]
fn scrape_cli_parses_connect_timeout_secs_correctly() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Use a URL that will fail for other reasons but should pass CLI validation
    let result = run_cli(&[
        "scrape",
        "http://invalid.invalid", // Will fail with DNS, but not validation
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "45",
    ]);

    // Should fail with DNS/network error, NOT validation error
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);

    // The important thing is it shouldn't fail with "must be between 1 and 60"
    // It might fail with DNS resolution error or similar, which is expected
    assert!(
        !stderr.contains("must be between 1 and 60")
            && !stdout.contains("must be between 1 and 60"),
        "CLI should accept connect_timeout_secs=45, but got: {stderr}"
    );
}

/// Verify that ingest command with valid connect_timeout_secs does not fail on CLI parsing
#[test]
fn ingest_cli_parses_connect_timeout_secs_correctly() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "http://invalid.invalid",
        "--output",
        output_dir.to_str().unwrap(),
        "--connect-timeout-secs",
        "45",
    ]);

    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);

    assert!(
        !stderr.contains("must be between 1 and 60")
            && !stdout.contains("must be between 1 and 60"),
        "CLI should accept connect_timeout_secs=45, but got: {stderr}"
    );
}
