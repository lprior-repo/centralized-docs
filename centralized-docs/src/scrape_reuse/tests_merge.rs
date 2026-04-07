//! Tests for merge logic and domain type defaults.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::HashMap;

use super::*;
use crate::scrape::validation::{Header, PageFilterStatus};

fn make_page(url: &str, markdown: &str) -> crate::scrape::validation::ScrapedPage {
    crate::scrape::validation::ScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: url.to_string(),
        links: Vec::new(),
        headers: vec![Header {
            level: 1,
            text: url.to_string(),
        }],
        word_count: markdown.split_whitespace().count(),
        slug: url.to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

#[test]
fn merge_scrape_pages_in_order_returns_fresh_pages_when_archived_is_empty() {
    let page_a = make_page("https://a.com", "a");
    let page_b = make_page("https://b.com", "b");
    let page_c = make_page("https://c.com", "c");

    let result = merge_scrape_pages_in_order(
        vec![page_a.clone(), page_b.clone(), page_c.clone()],
        HashMap::new(),
    );

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].url, page_a.url);
    assert_eq!(result[1].url, page_b.url);
    assert_eq!(result[2].url, page_c.url);
}

#[test]
fn merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order() {
    let page_a = make_page("https://a.com", "fresh_a");
    let page_b = make_page("https://b.com", "fresh_b");
    let page_c = make_page("https://c.com", "fresh_c");

    let archived_a = make_page("https://a.com", "archived_a");
    let archived_c = make_page("https://c.com", "archived_c");

    let mut archived_pages = HashMap::new();
    archived_pages.insert(0, archived_a.clone());
    archived_pages.insert(2, archived_c.clone());

    let result = merge_scrape_pages_in_order(vec![page_a, page_b.clone(), page_c], archived_pages);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].url, archived_a.url);
    assert_eq!(result[0].markdown, "archived_a");
    assert_eq!(result[1].url, page_b.url);
    assert_eq!(result[1].markdown, "fresh_b");
    assert_eq!(result[2].url, archived_c.url);
    assert_eq!(result[2].markdown, "archived_c");
}

#[test]
fn scrape_page_diff_default_returns_empty_partitions() {
    let diff = ScrapePageDiff::default();
    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, Vec::<usize>::new());
}

#[test]
fn scrape_reuse_stats_default_returns_zero_counts() {
    let stats = ScrapeReuseStats::default();
    assert_eq!(stats.reused, 0);
    assert_eq!(stats.scraped, 0);
}

#[test]
fn merge_scrape_pages_in_order_single_page_no_archive() {
    let page = make_page("https://a.com", "a");
    let result = merge_scrape_pages_in_order(vec![page.clone()], HashMap::new());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].url, page.url);
}

#[test]
fn merge_scrape_pages_in_order_all_archived() {
    let page_a = make_page("https://a.com", "fresh");
    let page_b = make_page("https://b.com", "fresh");
    let archived_a = make_page("https://a.com", "archived_a");
    let archived_b = make_page("https://b.com", "archived_b");

    let mut archived = HashMap::new();
    archived.insert(0, archived_a);
    archived.insert(1, archived_b);

    let result = merge_scrape_pages_in_order(vec![page_a, page_b], archived);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].markdown, "archived_a");
    assert_eq!(result[1].markdown, "archived_b");
}
