use super::*;

// Behavior 1: mixed classification
#[test]
fn classify_scrape_diff_returns_correct_partitions_for_mixed_pages() {
    let hash_a = hash_content(b"old content");
    let stored: HashMap<String, UrlStateRaw> = [
        make_stored("https://a.com/page1", hash_a),
        make_stored("https://a.com/page2", [0x22u8; 32]),
    ]
    .into_iter()
    .collect();
    let pages = vec![
        make_scraped_page("https://a.com/page1", "old content"),
        make_scraped_page("https://a.com/page2", "new content"),
        make_scraped_page("https://a.com/page3", "brand new"),
    ];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.unchanged, vec!["https://a.com/page1".to_string()]);
    assert_eq!(result.changed, vec!["https://a.com/page2".to_string()]);
    assert_eq!(result.new, vec!["https://a.com/page3".to_string()]);
}

// Behavior 2: all pages New on first run
#[test]
fn classify_scrape_diff_classifies_all_as_new_when_stored_states_empty() {
    let stored: HashMap<String, UrlStateRaw> = HashMap::new();
    let pages = vec![
        make_scraped_page("https://a.com/page1", "content 1"),
        make_scraped_page("https://b.com/page2", "content 2"),
        make_scraped_page("https://c.com/page3", "content 3"),
    ];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.new.len(), 3);
    assert!(result.changed.is_empty());
    assert!(result.unchanged.is_empty());
}

// Behavior 3: all Unchanged
#[test]
fn classify_scrape_diff_classifies_all_as_unchanged_when_hashes_match() {
    let hash_p = hash_content(b"matching content");
    let stored: HashMap<String, UrlStateRaw> = [make_stored("https://a.com/p", hash_p)]
        .into_iter()
        .collect();
    let pages = vec![make_scraped_page("https://a.com/p", "matching content")];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.unchanged.len(), 1);
    assert!(result.new.is_empty());
    assert!(result.changed.is_empty());
}

// Behavior 4: empty scraped pages
#[test]
fn classify_scrape_diff_returns_empty_when_no_pages_scraped() {
    let stored: HashMap<String, UrlStateRaw> = [
        make_stored("https://a.com/p1", [0xAA; 32]),
        make_stored("https://a.com/p2", [0xBB; 32]),
    ]
    .into_iter()
    .collect();
    let pages: Vec<ScrapedPage> = vec![];
    let result = classify_scrape_diff(&stored, &pages);
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
}

// Behavior 5: both inputs empty
#[test]
fn classify_scrape_diff_returns_empty_when_both_inputs_empty() {
    let stored: HashMap<String, UrlStateRaw> = HashMap::new();
    let pages: Vec<ScrapedPage> = vec![];
    let result = classify_scrape_diff(&stored, &pages);
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
}

// Behavior 8: all Changed
#[test]
fn classify_scrape_diff_classifies_all_as_changed_when_all_hashes_differ() {
    let stored: HashMap<String, UrlStateRaw> = [
        make_stored("https://a.com/p1", [0x01; 32]),
        make_stored("https://a.com/p2", [0x02; 32]),
        make_stored("https://a.com/p3", [0x03; 32]),
    ]
    .into_iter()
    .collect();
    let pages = vec![
        make_scraped_page("https://a.com/p1", "completely different 1"),
        make_scraped_page("https://a.com/p2", "completely different 2"),
        make_scraped_page("https://a.com/p3", "completely different 3"),
    ];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.changed.len(), 3);
    assert!(result.new.is_empty());
    assert!(result.unchanged.is_empty());
}

// Behavior 9: zero content_hash boundary
#[test]
fn classify_scrape_diff_handles_zero_content_hash_boundary() {
    let stored: HashMap<String, UrlStateRaw> = [make_stored("https://a.com/zero", [0u8; 32])]
        .into_iter()
        .collect();
    let pages = vec![make_scraped_page("https://a.com/zero", "")];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.changed, vec!["https://a.com/zero".to_string()]);
}

// Behavior 10: partial URL overlap
#[test]
fn classify_scrape_diff_handles_partial_url_overlap() {
    let hash_p1 = hash_content(b"content for p1");
    let hash_p2 = hash_content(b"content for p2");
    let stored: HashMap<String, UrlStateRaw> = [
        make_stored("https://a.com/p1", hash_p1),
        make_stored("https://a.com/p2", hash_p2),
        make_stored("https://a.com/p3", [0x03; 32]),
        make_stored("https://a.com/p4", [0x04; 32]),
        make_stored("https://a.com/p5", [0x05; 32]),
    ]
    .into_iter()
    .collect();
    let pages = vec![
        make_scraped_page("https://a.com/p1", "content for p1"),
        make_scraped_page("https://a.com/p2", "different content"),
        make_scraped_page("https://a.com/p6", "brand new page"),
    ];
    let result = classify_scrape_diff(&stored, &pages);
    assert!(result.unchanged.contains(&"https://a.com/p1".to_string()));
    assert!(result.changed.contains(&"https://a.com/p2".to_string()));
    assert!(result.new.contains(&"https://a.com/p6".to_string()));
    assert_eq!(result.total_len(), 3);
}

// Behavior 11: non-zero content_hash boundary
#[test]
fn classify_scrape_diff_handles_non_zero_content_hash_boundary() {
    let content = "some specific content here";
    let actual_hash = hash_content(content.as_bytes());
    let stored: HashMap<String, UrlStateRaw> = [make_stored("https://a.com/p", actual_hash)]
        .into_iter()
        .collect();
    let pages = vec![make_scraped_page("https://a.com/p", content)];
    let result = classify_scrape_diff(&stored, &pages);
    assert_eq!(result.unchanged, vec!["https://a.com/p".to_string()]);
}
