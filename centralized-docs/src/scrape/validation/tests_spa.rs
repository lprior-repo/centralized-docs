//! Tests for SPA detection.

use super::*;
use crate::scrape::validation::ScrapeResult;

#[test]
fn test_detect_potential_spa_low_pages_high_urls() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 100,
        success_count: 3,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    let spa_result = detect_potential_spa(&result);
    assert!(spa_result.is_potential_spa);
    assert!(spa_result.warning_message.is_some());
}

#[test]
fn test_detect_potential_spa_not_enough_pages() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 3,
        success_count: 2,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    let spa_result = detect_potential_spa(&result);
    assert!(!spa_result.is_potential_spa);
    assert!(spa_result.warning_message.is_none());
}

#[test]
fn test_detect_potential_spa_healthy_scrape() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 50,
        success_count: 45,
        error_count: 5,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    let spa_result = detect_potential_spa(&result);
    assert!(!spa_result.is_potential_spa);
    assert!(spa_result.warning_message.is_none());
}

#[test]
fn test_detect_potential_spa_zero_total_urls() {
    let result = ScrapeResult {
        pages: vec![],
        total_urls: 0,
        success_count: 0,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };

    let spa_result = detect_potential_spa(&result);
    assert!(!spa_result.is_potential_spa);
}
