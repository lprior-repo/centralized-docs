use super::*;
use proptest::prelude::*;

proptest! {
    /// Proptest 1: partitions are mutually exclusive and collectively exhaustive
    #[test]
    fn classify_scrape_diff_partitions_are_exhaustive(
        stored_count in 0usize..20, scraped_count in 0usize..20, seed in 0u8..255,
    ) {
        let mut stored: HashMap<String, UrlStateRaw> = HashMap::new();
        for i in 0..stored_count {
            stored.insert(format!("https://example.com/page-{i}"), UrlStateRaw {
                content_hash: [i as u8; 32], url_hash: [0u8; 32], last_fetched_secs: 0, status_code: 200, reserved: [0u8; 46],
            });
        }
        let mut pages: Vec<ScrapedPage> = Vec::new();
        for i in 0..scraped_count {
            let url_idx = i % (stored_count.max(1) + 1);
            pages.push(make_scraped_page(&format!("https://example.com/page-{url_idx}"), &format!("content-{seed}-{i}")));
        }
        let result = classify_scrape_diff(&stored, &pages);
        let mut all_urls: HashSet<String> = HashSet::new();
        for url in &result.new { prop_assert!(!all_urls.contains(url)); all_urls.insert(url.clone()); }
        for url in &result.changed { prop_assert!(!all_urls.contains(url)); all_urls.insert(url.clone()); }
        for url in &result.unchanged { prop_assert!(!all_urls.contains(url)); all_urls.insert(url.clone()); }
        for page in &pages { prop_assert!(all_urls.contains(&page.url)); }
    }

    /// Proptest 2: unchanged iff content_hash matches
    #[test]
    fn classify_scrape_diff_unchanged_iff_hash_matches(markdown_seed in 0u8..255) {
        let markdown = format!("content-{markdown_seed}");
        let hash = hash_content(markdown.as_bytes());
        let stored: HashMap<String, UrlStateRaw> = [make_stored("https://match.com/p", hash), make_stored("https://nomatch.com/p", [0xFF; 32])].into_iter().collect();
        let result_match = classify_scrape_diff(&stored, &[make_scraped_page("https://match.com/p", &markdown)]);
        prop_assert!(result_match.unchanged.contains(&"https://match.com/p".to_string()));
        let result_nomatch = classify_scrape_diff(&stored, &[make_scraped_page("https://nomatch.com/p", &markdown)]);
        prop_assert!(result_nomatch.changed.contains(&"https://nomatch.com/p".to_string()));
    }

    /// Proptest 3: build_scrape_state_changes is deterministic
    #[test]
    fn build_scrape_state_changes_is_deterministic(page_count in 0usize..10, seed in 0u64..1_000_000, timestamp in 0u64..u64::MAX) {
        let mut diff = ScrapeDiff { new: vec![], changed: vec![], unchanged: vec![] };
        let mut pages: Vec<ScrapedPage> = Vec::new();
        for i in 0..page_count {
            let url = format!("https://example.com/page-{i}");
            diff.new.push(url.clone());
            pages.push(make_scraped_page(&url, &format!("content-{seed}-{i}")));
        }
        let c1 = build_scrape_state_changes(&diff, &pages, timestamp);
        let c2 = build_scrape_state_changes(&diff.clone(), &pages.clone(), timestamp);
        assert_eq!(c1.updated_urls.len(), c2.updated_urls.len());
        assert_eq!(c1.new_scrapes.len(), c2.new_scrapes.len());
    }

    /// Proptest 4: every new_scrapes key == SHA-256 of its value bytes
    #[test]
    fn build_scrape_state_changes_keys_are_sha256_of_values(page_count in 1usize..10, seed in 0u64..1_000_000) {
        let mut diff = ScrapeDiff { new: vec![], changed: vec![], unchanged: vec![] };
        let mut pages: Vec<ScrapedPage> = Vec::new();
        for i in 0..page_count {
            let url = format!("https://example.com/page-{i}");
            diff.new.push(url.clone());
            pages.push(make_scraped_page(&url, &format!("content-{seed}-{i}")));
        }
        let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
        for (hash, bytes) in &changes.new_scrapes {
            let expected = hash_content(bytes);
            prop_assert_eq!(*hash, expected);
        }
    }

    /// Proptest 5: output field counts match input
    #[test]
    fn build_scrape_state_changes_output_counts_match_input(
        new_count in 0usize..10, changed_count in 0usize..10, unchanged_count in 0usize..10, seed in 0u64..1_000_000,
    ) {
        let mut diff = ScrapeDiff { new: vec![], changed: vec![], unchanged: vec![] };
        let mut pages: Vec<ScrapedPage> = Vec::new();
        let mut idx = 0usize;
        for i in 0..new_count { let url = format!("https://example.com/new-{i}"); diff.new.push(url.clone()); pages.push(make_scraped_page(&url, &format!("content-{seed}-{idx}"))); idx += 1; }
        for i in 0..changed_count { let url = format!("https://example.com/ch-{i}"); diff.changed.push(url.clone()); pages.push(make_scraped_page(&url, &format!("content-{seed}-{idx}"))); idx += 1; }
        for i in 0..unchanged_count { let url = format!("https://example.com/unch-{i}"); diff.unchanged.push(url.clone()); pages.push(make_scraped_page(&url, &format!("content-{seed}-{idx}"))); idx += 1; }
        let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
        prop_assert_eq!(changes.updated_urls.len(), new_count + changed_count);
        prop_assert_eq!(changes.new_scrapes.len(), new_count + changed_count);
        prop_assert_eq!(changes.deleted_urls.len(), 0);
    }

    /// Proptest 6: hash_payload determinism
    #[test]
    fn hash_payload_is_deterministic(bytes in proptest::collection::vec(any::<u8>(), 0..256)) {
        let h1 = hash_content(&bytes);
        let h2 = hash_content(&bytes);
        prop_assert_eq!(h1, h2);
    }
}

// Kani Harnesses
#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn classify_scrape_diff_preserves_all_scraped_urls() {
        let scraped_count: usize = kani::any();
        kani::assume(scraped_count <= 3);
        let stored: HashMap<String, UrlStateRaw> = HashMap::new();
        let mut pages: Vec<ScrapedPage> = Vec::new();
        for i in 0..scraped_count {
            pages.push(make_scraped_page(
                &format!("https://example.com/page-{i}"),
                &format!("content-{i}"),
            ));
        }
        let result = classify_scrape_diff(&stored, &pages);
        assert!(result.new.len() + result.changed.len() + result.unchanged.len() == scraped_count);
    }

    #[kani::proof]
    fn build_scrape_state_changes_output_count_matches_input() {
        let new_count: usize = kani::any();
        kani::assume(new_count <= 3);
        let changed_count: usize = kani::any();
        kani::assume(changed_count <= 3);
        let mut diff = ScrapeDiff {
            new: vec![],
            changed: vec![],
            unchanged: vec![],
        };
        let mut pages: Vec<ScrapedPage> = Vec::new();
        for i in 0..new_count {
            let url = format!("https://example.com/new-{i}");
            diff.new.push(url.clone());
            pages.push(make_scraped_page(&url, &format!("content-{i}")));
        }
        for i in 0..changed_count {
            let url = format!("https://example.com/ch-{i}");
            diff.changed.push(url.clone());
            pages.push(make_scraped_page(&url, &format!("content-ch-{i}")));
        }
        let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
        assert!(changes.updated_urls.len() == new_count + changed_count);
    }
}
