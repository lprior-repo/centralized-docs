use super::*;

// Single changed URL with one-byte payload
#[test]
fn scrape_batch_handles_single_changed_url_with_one_byte_payload() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://single.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://single.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: vec![0x42],
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 1);
    assert_eq!(changes.new_scrapes.len(), 1);
    assert_eq!(changes.new_scrapes[0].1, vec![0x42]);
}

// Deleted-only diff
#[test]
fn scrape_batch_deleted_only_produces_correct_state_changes() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec![],
        deleted: vec!["https://gone.com".into()],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 0);
    assert_eq!(changes.deleted_urls.len(), 1);
    assert_eq!(changes.new_scrapes.len(), 0);
}

// Timestamp zero is valid
#[test]
fn scrape_batch_accepts_zero_timestamp_in_config() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://a.com"]);
    let config = make_config(0);
    let changes =
        build_scrape_state_changes(&diff, &outputs, &config).expect("zero timestamp valid");
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, 0);
}

// Max u64 timestamp
#[test]
fn scrape_batch_accepts_max_u64_timestamp_in_config() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://a.com"]);
    let config = make_config(u64::MAX);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("max u64 valid");
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, u64::MAX);
}

// Status code 0 is valid
#[test]
fn scrape_batch_accepts_status_code_zero() {
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
            status_code: 0,
            payload_bytes: b"payload".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes =
        build_scrape_state_changes(&diff, &outputs, &config).expect("status code 0 valid");
    assert_eq!(changes.updated_urls[0].1.status_code, 0);
}

// Status code 599 is valid
#[test]
fn scrape_batch_accepts_status_code_599() {
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
            status_code: 599,
            payload_bytes: b"payload".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes =
        build_scrape_state_changes(&diff, &outputs, &config).expect("status code 599 valid");
    assert_eq!(changes.updated_urls[0].1.status_code, 599);
}

// All-zero content hash is valid
#[test]
fn scrape_batch_accepts_all_zero_content_hash() {
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
            content_hash: [0u8; 32],
            status_code: 200,
            payload_bytes: b"payload".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes =
        build_scrape_state_changes(&diff, &outputs, &config).expect("zero content hash valid");
    assert_eq!(changes.updated_urls[0].1.content_hash, [0u8; 32]);
}

// Multiple changed URLs produce correct new_scrapes entries
#[test]
fn scrape_batch_multiple_changed_urls_produce_correct_new_scrapes() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![
            "https://x.com".into(),
            "https://y.com".into(),
            "https://z.com".into(),
        ],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://x.com", "https://y.com", "https://z.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.new_scrapes.len(), 3);
    for (hash, _payload) in &changes.new_scrapes {
        assert_ne!(*hash, [0u8; 32], "scrape payload hash must be non-zero");
    }
}

// Only new_urls with no changed produces correct output
#[test]
fn scrape_batch_new_urls_only_produces_correct_output() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec!["https://new1.com".into(), "https://new2.com".into()],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://new1.com", "https://new2.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 2);
    assert_eq!(changes.new_scrapes.len(), 2);
    assert!(changes.deleted_urls.is_empty());
}
