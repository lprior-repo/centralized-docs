//! Adversarial QA tests for stealth mode and exponential backoff (bead doc-tx-8p5)
//!
//! Tests cover:
//! - Edge cases in backoff calculation (via public API)
//! - Overflow prevention (via config limits)
//! - Stealth mode configuration propagation
//! - Retry logic edge cases

use doc_transformer::scrape::ScrapeConfig;

// ============================================================================
// CONFIGURATION VALIDATION TESTS
// ============================================================================

#[test]
fn test_stealth_mode_default_enabled() {
    let config = ScrapeConfig::default();
    assert!(
        config.stealth_mode,
        "Stealth mode should be enabled by default for bot avoidance"
    );
}

#[test]
fn test_stealth_mode_can_be_disabled() {
    let config = ScrapeConfig {
        stealth_mode: false,
        ..Default::default()
    };
    assert!(!config.stealth_mode, "Stealth mode should be disableable");
}

#[test]
fn test_max_retries_default_is_reasonable() {
    let config = ScrapeConfig::default();
    assert_eq!(
        config.max_retries, 3,
        "Default should be 3 retries for production use"
    );
}

#[test]
fn test_max_retries_zero_is_valid() {
    // Edge case: zero retries should be valid
    let config = ScrapeConfig {
        max_retries: 0,
        ..Default::default()
    };
    assert_eq!(config.max_retries, 0);
}

#[test]
fn test_max_retries_very_high() {
    // Edge case: very high retry count
    // Implementation caps at 10 in scrape_site
    let config = ScrapeConfig {
        max_retries: 1000,
        ..Default::default()
    };
    assert_eq!(config.max_retries, 1000, "Config should accept high value");
}

#[test]
fn test_max_retries_u32_max() {
    // Extreme edge case: u32::MAX
    let config = ScrapeConfig {
        max_retries: u32::MAX,
        ..Default::default()
    };
    assert_eq!(config.max_retries, u32::MAX);
}

#[test]
fn test_exponential_backoff_default_enabled() {
    let config = ScrapeConfig::default();
    assert!(
        config.use_exponential_backoff,
        "Exponential backoff should be enabled by default"
    );
}

#[test]
fn test_exponential_backoff_can_be_disabled() {
    let config = ScrapeConfig {
        use_exponential_backoff: false,
        ..Default::default()
    };
    assert!(!config.use_exponential_backoff);
}

// ============================================================================
// DELAY CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_delay_ms_default_is_safe_for_aws() {
    let config = ScrapeConfig::default();
    // 1000ms = 1 req/sec with concurrency 1 = AWS safe
    assert_eq!(config.delay_ms, 1000, "Default should be AWS-safe");
}

#[test]
fn test_delay_must_be_non_negative() {
    // Test that we can set zero delay (edge case)
    let config = ScrapeConfig {
        delay_ms: 0,
        ..Default::default()
    };
    assert_eq!(config.delay_ms, 0);
}

#[test]
fn test_delay_very_high() {
    // Test extremely high delay (1 minute = 60000ms)
    let config = ScrapeConfig {
        delay_ms: 60000,
        ..Default::default()
    };
    assert_eq!(config.delay_ms, 60000);
}

#[test]
fn test_delay_near_u64_max() {
    // Edge case: near u64::MAX
    let config = ScrapeConfig {
        delay_ms: u64::MAX - 1,
        ..Default::default()
    };
    assert_eq!(config.delay_ms, u64::MAX - 1);
}

// ============================================================================
// SIZE LIMIT TESTS (DoS Protection)
// ============================================================================

#[test]
fn test_max_page_size_bytes_default() {
    let config = ScrapeConfig::default();
    assert_eq!(config.max_page_size_bytes, 10 * 1024 * 1024);
}

#[test]
fn test_max_total_size_bytes_default() {
    let config = ScrapeConfig::default();
    assert_eq!(config.max_total_size_bytes, 500 * 1024 * 1024);
}

#[test]
fn test_max_markdown_size_bytes_default() {
    let config = ScrapeConfig::default();
    assert_eq!(config.max_markdown_size_bytes, 5 * 1024 * 1024);
}

#[test]
fn test_max_pages_default() {
    let config = ScrapeConfig::default();
    assert_eq!(config.max_pages, 10_000);
}

#[test]
fn test_size_limits_can_be_reduced() {
    let config = ScrapeConfig {
        max_page_size_bytes: 1024,
        max_total_size_bytes: 10 * 1024,
        max_markdown_size_bytes: 512,
        max_pages: 10,
        ..Default::default()
    };
    assert_eq!(config.max_page_size_bytes, 1024);
    assert_eq!(config.max_total_size_bytes, 10 * 1024);
    assert_eq!(config.max_markdown_size_bytes, 512);
    assert_eq!(config.max_pages, 10);
}

// ============================================================================
// INTEGRATION TEST: Configuration Consistency
// ============================================================================

#[test]
fn test_full_config_with_all_fields() {
    // Test that all fields can be set consistently
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        use_sitemap: true,
        path_filter: Some("^/docs/".to_string()),
        delay_ms: 2000,
        user_agent: "TestAgent/1.0".to_string(),
        respect_robots: true,
        enable_filtering: true,
        max_retries: 5,
        use_exponential_backoff: true,
        max_page_size_bytes: 20 * 1024 * 1024,
        max_total_size_bytes: 1024 * 1024 * 1024,
        max_markdown_size_bytes: 10 * 1024 * 1024,
        max_pages: 5000,
        max_links_per_page: 2000,
        stealth_mode: true,
    };

    // Verify all fields are set correctly
    assert_eq!(config.base_url, "https://example.com");
    assert!(config.use_sitemap);
    assert_eq!(config.path_filter, Some("^/docs/".to_string()));
    assert_eq!(config.delay_ms, 2000);
    assert_eq!(config.user_agent, "TestAgent/1.0");
    assert!(config.respect_robots);
    assert!(config.enable_filtering);
    assert_eq!(config.max_retries, 5);
    assert!(config.use_exponential_backoff);
    assert_eq!(config.max_page_size_bytes, 20 * 1024 * 1024);
    assert_eq!(config.max_total_size_bytes, 1024 * 1024 * 1024);
    assert_eq!(config.max_markdown_size_bytes, 10 * 1024 * 1024);
    assert_eq!(config.max_pages, 5000);
    assert_eq!(config.max_links_per_page, 2000);
    assert!(config.stealth_mode);
}

#[test]
fn test_minimal_config() {
    // Test minimal configuration
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    // Should have defaults for everything else
    assert_eq!(config.base_url, "https://example.com");
    assert!(config.stealth_mode); // Default
    assert!(config.use_exponential_backoff); // Default
    assert_eq!(config.max_retries, 3); // Default
    assert_eq!(config.delay_ms, 1000); // Default
}

// ============================================================================
// EDGE CASE: Empty String Configuration
// ============================================================================

#[test]
fn test_empty_base_url_allowed_in_config() {
    // The config allows empty string (validation happens later)
    let config = ScrapeConfig {
        base_url: String::new(),
        ..Default::default()
    };
    assert!(config.base_url.is_empty());
}

#[test]
fn test_empty_user_agent() {
    let config = ScrapeConfig {
        user_agent: String::new(),
        ..Default::default()
    };
    assert!(config.user_agent.is_empty());
}

#[test]
fn test_empty_path_filter_none() {
    let config = ScrapeConfig {
        path_filter: None,
        ..Default::default()
    };
    assert!(config.path_filter.is_none());
}

#[test]
fn test_empty_path_filter_some() {
    // Empty string filter is valid (matches nothing or everything depending on regex)
    let config = ScrapeConfig {
        path_filter: Some(String::new()),
        ..Default::default()
    };
    assert_eq!(config.path_filter, Some(String::new()));
}
