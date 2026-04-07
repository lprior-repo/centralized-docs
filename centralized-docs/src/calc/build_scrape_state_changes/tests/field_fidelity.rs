use super::*;

// B06: content_hash fidelity
#[test]
fn scrape_batch_sets_content_hash_from_artifact() {
    let specific_hash: [u8; 32] = [0xAB; 32];
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://a.com".into(),
        make_artifact_with_content_hash(specific_hash, b"payload"),
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls[0].1.content_hash, [0xAB; 32]);
}

// B07: url_hash equals hash_payload of payload_bytes
#[test]
fn scrape_batch_sets_url_hash_to_hash_of_payload_bytes() {
    let payload = b"test_payload";
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://a.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: payload.to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    let expected_hash = hash_payload(payload);
    assert_eq!(changes.updated_urls[0].1.url_hash, expected_hash);
    assert_eq!(changes.new_scrapes[0].0, expected_hash);
}

// B08: timestamp fidelity
#[test]
fn scrape_batch_sets_last_fetched_secs_from_config() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://a.com"]);
    let config = make_config(1_712_345_678);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, 1_712_345_678);
}

// B09: status_code fidelity
#[test]
fn scrape_batch_sets_status_code_from_artifact() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://a.com".into(),
        make_artifact_with_status(301, b"payload"),
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls[0].1.status_code, 301);
}

// B10: reserved field is zeroed
#[test]
fn scrape_batch_zeroes_reserved_field_in_url_state_raw() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://a.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls[0].1.reserved, [0u8; 46]);
}

// B13: Reference integrity
#[test]
fn scrape_batch_url_hash_appears_as_key_in_new_scrapes() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec!["https://b.com".into()],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://a.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: b"payload_a".to_vec(),
        },
    );
    artifacts.insert(
        "https://b.com".into(),
        ScrapeArtifact {
            content_hash: [0x02; 32],
            status_code: 200,
            payload_bytes: b"payload_b".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    let expected_hash_a = hash_payload(b"payload_a");
    let expected_hash_b = hash_payload(b"payload_b");
    let scrape_keys: Vec<&[u8; 32]> = changes.new_scrapes.iter().map(|(k, _)| k).collect();
    assert!(scrape_keys.contains(&&expected_hash_a));
    assert!(scrape_keys.contains(&&expected_hash_b));
    let state_a = changes
        .updated_urls
        .iter()
        .find(|(u, _)| u == "https://a.com")
        .expect("find a.com")
        .1;
    assert_eq!(state_a.url_hash, expected_hash_a);
    let state_b = changes
        .updated_urls
        .iter()
        .find(|(u, _)| u == "https://b.com")
        .expect("find b.com")
        .1;
    assert_eq!(state_b.url_hash, expected_hash_b);
}
