use super::*;

// P1: Reference integrity for arbitrary valid inputs
proptest! {
    #![proptest_config(ProptestConfig { cases: 1000, ..ProptestConfig::default() })]

    #[test]
    fn proptest_reference_integrity_for_valid_inputs(
        changed_urls in proptest::collection::vec(
            proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(), 0..5
        ),
        new_urls in proptest::collection::vec(
            proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(), 0..5
        ),
    ) {
        prop_assume!(!changed_urls.is_empty() || !new_urls.is_empty());
        let diff = ScrapeDiff { unchanged: vec![], changed: changed_urls.clone(), new_urls: new_urls.clone(), deleted: vec![] };
        let mut artifacts = HashMap::new();
        for url in changed_urls.iter().chain(new_urls.iter()) {
            artifacts.insert(url.clone(), ScrapeArtifact { content_hash: [0x42; 32], status_code: 200, payload_bytes: url.as_bytes().to_vec() });
        }
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);
        let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
        let scrape_keys: std::collections::HashSet<[u8; 32]> = changes.new_scrapes.iter().map(|(k, _)| *k).collect();
        for (_, state) in &changes.updated_urls {
            prop_assert!(scrape_keys.contains(&state.url_hash), "url_hash {:?} not in new_scrapes", state.url_hash);
        }
    }
}

// P2: One-to-one URL-to-row mapping
proptest! {
    #![proptest_config(ProptestConfig { cases: 1000, ..ProptestConfig::default() })]

    #[test]
    fn proptest_one_to_one_url_mapping(changed_count in 0usize..10, new_count in 0usize..10) {
        prop_assume!(changed_count > 0 || new_count > 0);
        let changed_urls: Vec<String> = (0..changed_count).map(|i| format!("https://changed{i}.com/page")).collect();
        let new_urls: Vec<String> = (0..new_count).map(|i| format!("https://new{i}.com/page")).collect();
        let diff = ScrapeDiff { unchanged: vec![], changed: changed_urls.clone(), new_urls: new_urls.clone(), deleted: vec![] };
        let mut artifacts = HashMap::new();
        for url in changed_urls.iter().chain(new_urls.iter()) {
            artifacts.insert(url.clone(), ScrapeArtifact { content_hash: [0x01; 32], status_code: 200, payload_bytes: url.as_bytes().to_vec() });
        }
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);
        let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
        prop_assert_eq!(changes.updated_urls.len(), changed_count + new_count);
    }
}

// P3: Determinism
proptest! {
    #![proptest_config(ProptestConfig { cases: 1000, ..ProptestConfig::default() })]

    #[test]
    fn proptest_determinism(
        changed_urls in proptest::collection::vec(
            proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(), 1..5
        ),
    ) {
        let diff = ScrapeDiff { unchanged: vec![], changed: changed_urls.clone(), new_urls: vec![], deleted: vec![] };
        let mut artifacts = HashMap::new();
        for url in &changed_urls {
            artifacts.insert(url.clone(), ScrapeArtifact { content_hash: [0x01; 32], status_code: 200, payload_bytes: url.as_bytes().to_vec() });
        }
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);
        let c1 = build_scrape_state_changes(&diff, &outputs, &config).expect("ok");
        let c2 = build_scrape_state_changes(&diff, &outputs, &config).expect("ok");
        prop_assert_eq!(c1.updated_urls, c2.updated_urls);
        prop_assert_eq!(c1.new_scrapes, c2.new_scrapes);
    }
}

// P4: build_url_state_raw round-trip through bytes
proptest! {
    #![proptest_config(ProptestConfig { cases: 1000, ..ProptestConfig::default() })]

    #[test]
    fn proptest_url_state_raw_roundtrip(
        content_hash in proptest::array::uniform32(proptest::num::u8::ANY),
        url_hash in proptest::array::uniform32(proptest::num::u8::ANY),
        last_fetched_secs in proptest::num::u64::ANY,
        status_code in proptest::num::u16::ANY,
    ) {
        let original = build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code);
        prop_assert_eq!(original.content_hash, content_hash);
        prop_assert_eq!(original.url_hash, url_hash);
        prop_assert_eq!(original.last_fetched_secs, last_fetched_secs);
        prop_assert_eq!(original.status_code, status_code);
        prop_assert_eq!(original.reserved, [0u8; 46]);
        let bytes = original.to_bytes();
        prop_assert_eq!(bytes.len(), 120);
        let restored = crate::state::UrlStateRaw::from_bytes(&bytes).expect("from_bytes ok");
        prop_assert_eq!(restored.content_hash, content_hash);
    }
}
