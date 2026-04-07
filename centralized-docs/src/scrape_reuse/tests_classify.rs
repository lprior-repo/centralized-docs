//! Tests for classification logic.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::HashMap;

use super::*;
use crate::scrape::validation::{Header, PageFilterStatus};
use crate::state::UrlStateRaw;

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

fn make_url_state(content_hash: [u8; 32], url_hash: [u8; 32]) -> UrlStateRaw {
    UrlStateRaw {
        content_hash,
        url_hash,
        last_fetched_secs: 1_700_000_000,
        status_code: 200,
        reserved: [0u8; 46],
    }
}

#[test]
fn classify_scraped_pages_returns_empty_partitions_when_no_pages_provided() {
    let pages: Vec<crate::scrape::validation::ScrapedPage> = Vec::new();
    let hashes: Vec<[u8; 32]> = Vec::new();
    let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

    let diff = classify_scraped_pages(&pages, &hashes, &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, Vec::<usize>::new());
}

#[test]
fn classify_scraped_pages_classifies_unchanged_when_hash_matches_stored() {
    let page = make_page("https://a.com", "hello");
    let content_hash = compute_page_content_hash("hello");
    let url_state = make_url_state(content_hash, [1u8; 32]);

    let mut url_states = HashMap::new();
    url_states.insert("https://a.com".to_string(), url_state);

    let diff = classify_scraped_pages(&[page], &[content_hash], &url_states);

    assert_eq!(diff.unchanged, vec![0]);
    assert_eq!(diff.changed_or_new, Vec::<usize>::new());
}

#[test]
fn classify_scraped_pages_classifies_changed_when_hash_mismatches_stored() {
    let page = make_page("https://a.com", "new content");
    let fresh_hash = compute_page_content_hash("new content");
    let stored_hash = compute_page_content_hash("old content");
    let url_state = make_url_state(stored_hash, [1u8; 32]);

    let mut url_states = HashMap::new();
    url_states.insert("https://a.com".to_string(), url_state);

    let diff = classify_scraped_pages(&[page], &[fresh_hash], &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, vec![0]);
}

#[test]
fn classify_scraped_pages_classifies_changed_when_stored_hash_is_lexicographically_greater() {
    let page = make_page("https://trap.com", "aaa");
    let fresh_hash = compute_page_content_hash("aaa");
    let url_state = make_url_state([0xFF; 32], [1u8; 32]);

    let mut url_states = HashMap::new();
    url_states.insert("https://trap.com".to_string(), url_state);

    let diff = classify_scraped_pages(&[page], &[fresh_hash], &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, vec![0]);
}

#[test]
fn classify_scraped_pages_classifies_changed_or_new_when_url_state_missing() {
    let page = make_page("https://new.com", "content");
    let hash = compute_page_content_hash("content");
    let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

    let diff = classify_scraped_pages(&[page], &[hash], &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, vec![0]);
}

#[test]
fn classify_scraped_pages_classifies_changed_or_new_when_url_hash_is_zero() {
    let page = make_page("https://a.com", "hello");
    let content_hash = compute_page_content_hash("hello");
    let url_state = make_url_state(content_hash, [0u8; 32]);

    let mut url_states = HashMap::new();
    url_states.insert("https://a.com".to_string(), url_state);

    let diff = classify_scraped_pages(&[page], &[content_hash], &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, vec![0]);
}

#[test]
fn classify_scraped_pages_produces_mutually_exclusive_collectively_exhaustive_partition() {
    let pages = vec![
        make_page("https://a.com", "content_a"),
        make_page("https://b.com", "content_b"),
        make_page("https://c.com", "content_c"),
    ];
    let hashes: Vec<[u8; 32]> = pages
        .iter()
        .map(|p| compute_page_content_hash(&p.markdown))
        .collect();

    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(hashes[0], [1u8; 32]),
    );
    url_states.insert(
        "https://b.com".to_string(),
        make_url_state(compute_page_content_hash("different"), [1u8; 32]),
    );

    let diff = classify_scraped_pages(&pages, &hashes, &url_states);

    assert_eq!(
        diff.unchanged.len() + diff.changed_or_new.len(),
        3,
        "partition must cover all 3 pages"
    );

    let unchanged_set: std::collections::HashSet<usize> = diff.unchanged.iter().copied().collect();
    let changed_set: std::collections::HashSet<usize> =
        diff.changed_or_new.iter().copied().collect();
    assert!(
        unchanged_set.is_disjoint(&changed_set),
        "partitions must be disjoint"
    );

    for i in 0..3 {
        assert!(
            unchanged_set.contains(&i) || changed_set.contains(&i),
            "index {i} must be in exactly one partition"
        );
    }
}

#[test]
#[should_panic(expected = "length")]
fn classify_scraped_pages_panics_or_errors_when_input_lengths_mismatch() {
    let pages = vec![
        make_page("https://a.com", "a"),
        make_page("https://b.com", "b"),
    ];
    let hashes = vec![[0u8; 32]];
    let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

    let _ = classify_scraped_pages(&pages, &hashes, &url_states);
}

#[test]
fn classify_scraped_pages_all_unchanged_when_all_match() {
    let pages = vec![
        make_page("https://a.com", "a"),
        make_page("https://b.com", "b"),
    ];
    let hashes: Vec<[u8; 32]> = pages
        .iter()
        .map(|p| compute_page_content_hash(&p.markdown))
        .collect();

    let mut url_states = HashMap::new();
    for (i, page) in pages.iter().enumerate() {
        url_states.insert(
            page.url.clone(),
            make_url_state(hashes[i], [i as u8 + 1; 32]),
        );
    }

    let diff = classify_scraped_pages(&pages, &hashes, &url_states);

    assert_eq!(diff.unchanged, vec![0, 1]);
    assert_eq!(diff.changed_or_new, Vec::<usize>::new());
}

#[test]
fn classify_scraped_pages_all_changed_when_all_mismatch() {
    let pages = vec![
        make_page("https://a.com", "new_a"),
        make_page("https://b.com", "new_b"),
    ];
    let hashes: Vec<[u8; 32]> = pages
        .iter()
        .map(|p| compute_page_content_hash(&p.markdown))
        .collect();

    let mut url_states = HashMap::new();
    for page in &pages {
        url_states.insert(
            page.url.clone(),
            make_url_state(compute_page_content_hash("old"), [1u8; 32]),
        );
    }

    let diff = classify_scraped_pages(&pages, &hashes, &url_states);

    assert_eq!(diff.unchanged, Vec::<usize>::new());
    assert_eq!(diff.changed_or_new, vec![0, 1]);
}

#[test]
fn classify_scraped_pages_preserves_indices_in_changed_or_new() {
    let pages = vec![
        make_page("https://a.com", "same"),
        make_page("https://b.com", "changed"),
        make_page("https://c.com", "same"),
        make_page("https://d.com", "changed"),
    ];
    let hashes: Vec<[u8; 32]> = pages
        .iter()
        .map(|p| compute_page_content_hash(&p.markdown))
        .collect();

    let mut url_states = HashMap::new();
    url_states.insert(
        "https://a.com".to_string(),
        make_url_state(hashes[0], [1u8; 32]),
    );
    url_states.insert(
        "https://b.com".to_string(),
        make_url_state(compute_page_content_hash("different"), [1u8; 32]),
    );
    url_states.insert(
        "https://c.com".to_string(),
        make_url_state(hashes[2], [1u8; 32]),
    );
    url_states.insert(
        "https://d.com".to_string(),
        make_url_state(hashes[3], [0u8; 32]),
    );

    let diff = classify_scraped_pages(&pages, &hashes, &url_states);

    assert_eq!(diff.unchanged, vec![0, 2]);
    assert_eq!(diff.changed_or_new, vec![1, 3]);
}
