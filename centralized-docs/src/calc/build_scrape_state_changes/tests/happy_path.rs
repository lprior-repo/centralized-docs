use super::*;

// B01: Changed URLs produce updated rows and payload blobs
#[test]
fn scrape_batch_produces_updated_rows_for_changed_urls() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".to_string(), "https://b.com".to_string()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://a.com", "https://b.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 2);
    assert_eq!(changes.updated_urls[0].0, "https://a.com");
    assert_eq!(changes.updated_urls[1].0, "https://b.com");
    assert_eq!(changes.new_scrapes.len(), 2);
    assert!(changes.deleted_urls.is_empty());
}

// B02: New URLs produce updated rows and payload blobs
#[test]
fn scrape_batch_produces_updated_rows_for_new_urls() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec!["https://new.com".to_string()],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://new.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 1);
    assert_eq!(changes.updated_urls[0].0, "https://new.com");
    assert_eq!(changes.new_scrapes.len(), 1);
    assert!(changes.deleted_urls.is_empty());
}

// B03: Payload blobs are produced in new_scrapes
#[test]
fn scrape_batch_produces_payload_blobs_for_changed_and_new_urls() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".to_string()],
        new_urls: vec!["https://b.com".to_string()],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://a.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: b"serialized_page_1".to_vec(),
        },
    );
    artifacts.insert(
        "https://b.com".into(),
        ScrapeArtifact {
            content_hash: [0x02; 32],
            status_code: 200,
            payload_bytes: b"serialized_page_2".to_vec(),
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.new_scrapes.len(), 2);
    assert_eq!(changes.new_scrapes[0].1, b"serialized_page_1");
    assert_eq!(changes.new_scrapes[1].1, b"serialized_page_2");
}

// B04: Deleted URLs produce only delete entries
#[test]
fn scrape_batch_produces_only_delete_entries_for_deleted_urls() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec![],
        deleted: vec![
            "https://old1.com".into(),
            "https://old2.com".into(),
            "https://old3.com".into(),
        ],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.deleted_urls.len(), 3);
    assert!(changes.updated_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
}

// B05: Unchanged URLs produce no output
#[test]
fn scrape_batch_excludes_unchanged_urls_from_all_outputs() {
    let diff = ScrapeDiff {
        unchanged: vec![
            "https://u1.com".into(),
            "https://u2.com".into(),
            "https://u3.com".into(),
        ],
        changed: vec!["https://c.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = make_scrape_outputs(&["https://c.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 1);
    assert_eq!(changes.updated_urls[0].0, "https://c.com");
    for (url, _) in &changes.updated_urls {
        assert_ne!(url, "https://u1.com");
        assert_ne!(url, "https://u2.com");
        assert_ne!(url, "https://u3.com");
    }
    assert_eq!(changes.new_scrapes.len(), 1);
}

// B11: Non-URL fields are empty
#[test]
fn scrape_batch_leaves_file_state_fields_empty() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into()],
        new_urls: vec![],
        deleted: vec!["https://b.com".into()],
    };
    let outputs = make_scrape_outputs(&["https://a.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert!(changes.updated_files.is_empty());
    assert!(changes.deleted_files.is_empty());
    assert!(changes.new_analyses.is_empty());
    assert!(changes.new_transforms.is_empty());
    assert!(changes.new_chunks.is_empty());
    assert!(changes.new_snapshots.is_empty());
    assert!(changes.deleted_snapshots.is_empty());
}

// B12: Determinism
#[test]
fn scrape_batch_produces_identical_output_for_identical_inputs() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://a.com".into(), "https://b.com".into()],
        new_urls: vec!["https://c.com".into()],
        deleted: vec!["https://d.com".into()],
    };
    let outputs = make_scrape_outputs(&["https://a.com", "https://b.com", "https://c.com"]);
    let config = make_config(1_700_000_000);
    let c1 = build_scrape_state_changes(&diff, &outputs, &config).expect("ok");
    let c2 = build_scrape_state_changes(&diff, &outputs, &config).expect("ok");
    assert_eq!(c1.updated_urls, c2.updated_urls);
    assert_eq!(c1.deleted_urls, c2.deleted_urls);
    assert_eq!(c1.new_scrapes, c2.new_scrapes);
}

// B14: Output ordering (changed then new_urls then deleted)
#[test]
fn scrape_batch_maintains_changed_then_new_then_deleted_ordering() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://c1.com".into()],
        new_urls: vec!["https://n1.com".into()],
        deleted: vec!["https://d1.com".into()],
    };
    let outputs = make_scrape_outputs(&["https://c1.com", "https://n1.com"]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls[0].0, "https://c1.com");
    assert_eq!(changes.updated_urls[1].0, "https://n1.com");
    assert_eq!(changes.deleted_urls[0], "https://d1.com");
}

// MIX: Mixed diff categories
#[test]
fn scrape_batch_handles_mixed_diff_categories_correctly() {
    let diff = ScrapeDiff {
        unchanged: vec!["https://u1.com".into(), "https://u2.com".into()],
        changed: vec![
            "https://c1.com".into(),
            "https://c2.com".into(),
            "https://c3.com".into(),
        ],
        new_urls: vec!["https://n1.com".into()],
        deleted: vec!["https://d1.com".into(), "https://d2.com".into()],
    };
    let outputs = make_scrape_outputs(&[
        "https://c1.com",
        "https://c2.com",
        "https://c3.com",
        "https://n1.com",
    ]);
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config).expect("should succeed");
    assert_eq!(changes.updated_urls.len(), 4); // 3 changed + 1 new
    assert_eq!(changes.deleted_urls.len(), 2);
    assert_eq!(changes.new_scrapes.len(), 4);
    for (url, _) in &changes.updated_urls {
        assert_ne!(url, "https://u1.com");
        assert_ne!(url, "https://u2.com");
    }
}
