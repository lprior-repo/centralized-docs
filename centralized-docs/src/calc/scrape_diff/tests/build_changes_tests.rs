use super::*;

// Behavior 12: new and changed pages produce entries
#[test]
fn build_scrape_state_changes_produces_entries_for_new_and_changed_pages() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/new".into()],
        changed: vec!["https://a.com/changed".into()],
        unchanged: vec!["https://a.com/same".into()],
    };
    let pages = vec![
        make_scraped_page("https://a.com/new", "new content"),
        make_scraped_page("https://a.com/changed", "changed content"),
        make_scraped_page("https://a.com/same", "same content"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    assert_eq!(changes.updated_urls.len(), 2);
    let url_keys: Vec<&str> = changes
        .updated_urls
        .iter()
        .map(|(u, _)| u.as_str())
        .collect();
    assert!(url_keys.contains(&"https://a.com/new"));
    assert!(url_keys.contains(&"https://a.com/changed"));
    assert!(!url_keys.contains(&"https://a.com/same"));
    assert_eq!(changes.new_scrapes.len(), 2);
    for (_, state) in &changes.updated_urls {
        assert_eq!(state.last_fetched_secs, 1_700_000_000);
        assert_eq!(state.status_code, 200);
    }
}

// Behavior 14: unchanged pages excluded
#[test]
fn build_scrape_state_changes_excludes_unchanged_pages_from_all_outputs() {
    let diff = ScrapeDiff {
        new: vec![],
        changed: vec![],
        unchanged: vec!["https://a.com/same".into()],
    };
    let changes = build_scrape_state_changes(&diff, &[], 1_700_000_000);
    assert!(changes.updated_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
    assert!(changes.deleted_urls.is_empty());
}

// Behavior 15: correct content_hash per page
#[test]
fn build_scrape_state_changes_sets_content_hash_from_sha256_of_each_pages_markdown() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/p1".into(), "https://a.com/p2".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![
        make_scraped_page("https://a.com/p1", "alpha content here"),
        make_scraped_page("https://a.com/p2", "beta content here"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    assert_eq!(
        changes.updated_urls[0].1.content_hash,
        hash_content(b"alpha content here")
    );
    assert_eq!(
        changes.updated_urls[1].1.content_hash,
        hash_content(b"beta content here")
    );
    assert_ne!(
        changes.updated_urls[0].1.content_hash,
        changes.updated_urls[1].1.content_hash
    );
}

// Behavior 16: unique updated_urls keys
#[test]
fn build_scrape_state_changes_produces_unique_updated_url_keys() {
    let diff = ScrapeDiff {
        new: vec![
            "https://a.com/new1".into(),
            "https://a.com/new2".into(),
            "https://a.com/new3".into(),
        ],
        changed: vec!["https://a.com/ch1".into(), "https://a.com/ch2".into()],
        unchanged: vec![],
    };
    let pages = vec![
        make_scraped_page("https://a.com/new1", "c1"),
        make_scraped_page("https://a.com/new2", "c2"),
        make_scraped_page("https://a.com/new3", "c3"),
        make_scraped_page("https://a.com/ch1", "c4"),
        make_scraped_page("https://a.com/ch2", "c5"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    let url_keys: Vec<&str> = changes
        .updated_urls
        .iter()
        .map(|(u, _)| u.as_str())
        .collect();
    let unique: HashSet<&str> = url_keys.iter().copied().collect();
    assert_eq!(url_keys.len(), unique.len());
}

// Behavior 17: non-zero scrape hash keys
#[test]
fn build_scrape_state_changes_produces_non_zero_scrape_hash_keys() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/new".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/new", "some content")];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    for (hash, _) in &changes.new_scrapes {
        assert_ne!(*hash, [0u8; 32], "new_scrapes key must be non-zero");
    }
}

// Behavior 18: reference integrity
#[test]
fn build_scrape_state_changes_maintains_reference_integrity_for_url_hashes() {
    let diff = ScrapeDiff {
        new: vec![
            "https://a.com/n1".into(),
            "https://a.com/n2".into(),
            "https://a.com/n3".into(),
        ],
        changed: vec!["https://a.com/c1".into(), "https://a.com/c2".into()],
        unchanged: vec![],
    };
    let pages = vec![
        make_scraped_page("https://a.com/n1", "content n1"),
        make_scraped_page("https://a.com/n2", "content n2"),
        make_scraped_page("https://a.com/n3", "content n3"),
        make_scraped_page("https://a.com/c1", "content c1"),
        make_scraped_page("https://a.com/c2", "content c2"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    let scrape_keys: HashSet<[u8; 32]> = changes.new_scrapes.iter().map(|(k, _)| *k).collect();
    for (_, state) in &changes.updated_urls {
        if state.url_hash != [0u8; 32] {
            assert!(
                scrape_keys.contains(&state.url_hash),
                "url_hash {:?} must have matching new_scrapes entry",
                state.url_hash
            );
        }
    }
}

// Behavior 19: last_fetched_secs from timestamp
#[test]
fn build_scrape_state_changes_sets_last_fetched_secs_from_timestamp() {
    let ts = 1_725_432_100_u64;
    let diff = ScrapeDiff {
        new: vec!["https://a.com/p".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/p", "content")];
    let changes = build_scrape_state_changes(&diff, &pages, ts);
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, ts);
}

// Behavior 20: empty ScrapeDiff with only unchanged → empty StateChanges
#[test]
fn build_scrape_state_changes_returns_empty_when_scrape_diff_has_only_unchanged() {
    let diff = ScrapeDiff {
        new: vec![],
        changed: vec![],
        unchanged: vec![
            "https://a.com/p1".into(),
            "https://a.com/p2".into(),
            "https://a.com/p3".into(),
        ],
    };
    let changes = build_scrape_state_changes(&diff, &[], 1_700_000_000);
    assert!(changes.updated_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
}

// Behavior 21: persisted bytes match rkyv
#[test]
fn build_scrape_state_changes_serializes_persisted_scrape_result_for_scrapes() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/new".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/new", "content")];
    let changes = build_scrape_state_changes(&diff, &pages, 1_700_000_000);
    assert_eq!(changes.new_scrapes.len(), 1);
    let (_, bytes) = &changes.new_scrapes[0];
    let archived = rkyv::access::<rkyv::Archived<PersistedScrapeResult>, rkyv::rancor::Error>(
        bytes.as_slice(),
    )
    .expect("valid rkyv");
    assert_eq!(archived.schema_version, 1);
}

// Behavior 22: new pages only
#[test]
fn build_scrape_state_changes_handles_new_pages_only() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/new1".into(), "https://a.com/new2".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![
        make_scraped_page("https://a.com/new1", "content n1"),
        make_scraped_page("https://a.com/new2", "content n2"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_600_000_000);
    assert_eq!(changes.updated_urls.len(), 2);
    assert_eq!(changes.new_scrapes.len(), 2);
}

// Behavior 23: changed pages only
#[test]
fn build_scrape_state_changes_handles_changed_pages_only() {
    let diff = ScrapeDiff {
        new: vec![],
        changed: vec!["https://a.com/ch1".into(), "https://a.com/ch2".into()],
        unchanged: vec![],
    };
    let pages = vec![
        make_scraped_page("https://a.com/ch1", "content ch1"),
        make_scraped_page("https://a.com/ch2", "content ch2"),
    ];
    let changes = build_scrape_state_changes(&diff, &pages, 1_800_000_000);
    assert_eq!(changes.updated_urls.len(), 2);
    assert_eq!(changes.new_scrapes.len(), 2);
}

// Behavior 24/25: timestamp boundary tests
#[test]
fn build_scrape_state_changes_handles_zero_timestamp() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/p".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/p", "content")];
    let changes = build_scrape_state_changes(&diff, &pages, 0);
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, 0);
}

#[test]
fn build_scrape_state_changes_handles_max_timestamp() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/p".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/p", "content")];
    let changes = build_scrape_state_changes(&diff, &pages, u64::MAX);
    assert_eq!(changes.updated_urls[0].1.last_fetched_secs, u64::MAX);
}

// Behavior 26: empty markdown
#[test]
fn build_scrape_state_changes_handles_empty_markdown() {
    let diff = ScrapeDiff {
        new: vec!["https://a.com/empty".into()],
        changed: vec![],
        unchanged: vec![],
    };
    let pages = vec![make_scraped_page("https://a.com/empty", "")];
    let changes = build_scrape_state_changes(&diff, &pages, 1_000_000_000);
    assert_eq!(changes.updated_urls[0].1.content_hash, hash_content(b""));
    let (_, bytes) = &changes.new_scrapes[0];
    let archived = rkyv::access::<rkyv::Archived<PersistedScrapeResult>, rkyv::rancor::Error>(
        bytes.as_slice(),
    )
    .expect("valid rkyv");
    assert_eq!(archived.schema_version, 1);
}
