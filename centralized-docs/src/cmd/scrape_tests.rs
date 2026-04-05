// Tests for cmd::scrape — extracted from scrape.rs for file-length compliance (<300 lines).

use crate::cmd::scrape::{apply_query_filter, validate_query_length};
use crate::scrape;

fn make_scraped_page(url: &str, markdown: &str, word_count: usize) -> scrape::ScrapedPage {
    scrape::ScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: "Title".to_string(),
        links: Vec::new(),
        headers: Vec::new(),
        word_count,
        slug: "slug".to_string(),
        filter_status: scrape::PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

#[test]
fn test_apply_query_filter_no_query_keeps_all_pages() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "alpha beta", 2),
        make_scraped_page("https://example.com/b", "gamma delta", 2),
    ];

    let result = apply_query_filter(pages.clone(), None, 0.1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap_or_default().len(), pages.len());
}

#[test]
fn test_apply_query_filter_filters_non_matching_pages() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "rust async runtime", 3),
        make_scraped_page("https://example.com/b", "python data science", 3),
    ];

    let result = apply_query_filter(pages, Some("rust"), 0.1);
    assert!(result.is_ok());

    let kept = result.unwrap_or_default();
    assert_eq!(kept.len(), 1);
    assert!(kept[0].markdown.contains("rust"));
}

#[test]
fn test_apply_query_filter_errors_when_all_filtered() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "alpha beta", 2),
        make_scraped_page("https://example.com/b", "gamma delta", 2),
    ];

    let result = apply_query_filter(pages, Some("zzzzzz_no_match"), 0.1);
    assert!(result.is_err());
}

#[test]
fn test_apply_query_filter_threshold_zero_keeps_all() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "rust async", 2),
        make_scraped_page("https://example.com/b", "python data", 2),
    ];

    let result = apply_query_filter(pages.clone(), Some("rust"), 0.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap_or_default().len(), pages.len());
}

#[test]
fn test_apply_query_filter_empty_query_keeps_all() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "rust async", 2),
        make_scraped_page("https://example.com/b", "python data", 2),
    ];

    let result = apply_query_filter(pages.clone(), Some("   "), 0.1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap_or_default().len(), pages.len());
}

#[test]
fn test_apply_query_filter_with_different_thresholds() {
    let pages = vec![
        make_scraped_page("https://example.com/a", "rust rust rust rust", 4),
        make_scraped_page("https://example.com/b", "rust", 1),
        make_scraped_page("https://example.com/c", "python", 1),
    ];

    let result0 = apply_query_filter(pages.clone(), Some("rust"), 0.0);
    assert!(result0.is_ok());
    assert_eq!(result0.unwrap_or_default().len(), 3);

    let result_high = apply_query_filter(pages, Some("rust"), 100.0);
    assert!(result_high.is_err()); // All filtered out due to high threshold
}

#[test]
fn test_validate_query_none() {
    let query: Option<&str> = None;
    assert!(validate_query_length(&query).is_ok());
}

#[test]
fn test_validate_query_empty_string() {
    let query: Option<&str> = Some("");
    assert!(validate_query_length(&query).is_ok());
}

#[test]
fn test_validate_query_exceeds_limit() {
    let too_long_query = "a".repeat(1001);
    let query: Option<&str> = Some(&too_long_query);
    let result = validate_query_length(&query);

    assert!(result.is_err());
}
