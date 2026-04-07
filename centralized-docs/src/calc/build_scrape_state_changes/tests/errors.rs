use super::*;

// B15: EmptyDiff error
#[test]
fn scrape_batch_returns_empty_diff_error_when_all_categories_empty() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(0);
    match build_scrape_state_changes(&diff, &outputs, &config) {
        Err(ScrapeBatchBuildError::EmptyDiff) => {}
        other => panic!("expected EmptyDiff, got: {other:?}"),
    }
}

// B15b: Only unchanged is not empty diff
#[test]
fn scrape_batch_returns_empty_ok_when_only_unchanged_urls_present() {
    let diff = ScrapeDiff {
        unchanged: vec!["https://u1.com".into()],
        changed: vec![],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(1_700_000_000);
    let changes = build_scrape_state_changes(&diff, &outputs, &config)
        .expect("unchanged-only should return Ok");
    assert!(changes.updated_urls.is_empty());
    assert!(changes.deleted_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
    assert!(changes.updated_files.is_empty());
}

// B16a-f: DuplicateUrl in all pair combinations
macro_rules! test_duplicate {
    ($name:ident, $unchanged:expr, $changed:expr, $new:expr, $deleted:expr) => {
        #[test]
        fn $name() {
            let diff = ScrapeDiff {
                unchanged: $unchanged,
                changed: $changed,
                new_urls: $new,
                deleted: $deleted,
            };
            let outputs = make_scrape_outputs(&["https://dup.com"]);
            let config = make_config(1_700_000_000);
            match build_scrape_state_changes(&diff, &outputs, &config) {
                Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                    assert_eq!(url, "https://dup.com");
                }
                other => panic!("expected DuplicateUrl, got: {other:?}"),
            }
        }
    };
}

test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_changed_and_new,
    vec![],
    vec!["https://dup.com".into()],
    vec!["https://dup.com".into()],
    vec![]
);
test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_unchanged_and_changed,
    vec!["https://dup.com".into()],
    vec!["https://dup.com".into()],
    vec![],
    vec![]
);
test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_unchanged_and_new,
    vec!["https://dup.com".into()],
    vec![],
    vec!["https://dup.com".into()],
    vec![]
);
test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_unchanged_and_deleted,
    vec!["https://dup.com".into()],
    vec![],
    vec![],
    vec!["https://dup.com".into()]
);
test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_changed_and_deleted,
    vec![],
    vec!["https://dup.com".into()],
    vec![],
    vec!["https://dup.com".into()]
);
test_duplicate!(
    scrape_batch_returns_duplicate_url_when_in_new_and_deleted,
    vec![],
    vec![],
    vec!["https://dup.com".into()],
    vec!["https://dup.com".into()]
);

// B17: MissingScrapeArtifact for changed URL
#[test]
fn scrape_batch_returns_missing_artifact_when_changed_url_has_no_artifact() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://missing.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(1_700_000_000);
    match build_scrape_state_changes(&diff, &outputs, &config) {
        Err(ScrapeBatchBuildError::MissingScrapeArtifact { ref url }) => {
            assert_eq!(url, "https://missing.com");
        }
        other => panic!("expected MissingScrapeArtifact, got: {other:?}"),
    }
}

// B18: MissingScrapeArtifact for new URL
#[test]
fn scrape_batch_returns_missing_artifact_when_new_url_has_no_artifact() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec!["https://missing.com".into()],
        deleted: vec![],
    };
    let outputs = ScrapeOutputs {
        artifacts: HashMap::new(),
    };
    let config = make_config(1_700_000_000);
    match build_scrape_state_changes(&diff, &outputs, &config) {
        Err(ScrapeBatchBuildError::MissingScrapeArtifact { ref url }) => {
            assert_eq!(url, "https://missing.com");
        }
        other => panic!("expected MissingScrapeArtifact, got: {other:?}"),
    }
}

// B19: EmptyScrapePayload for changed URL
#[test]
fn scrape_batch_returns_empty_payload_when_changed_url_artifact_has_zero_bytes() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec!["https://empty.com".into()],
        new_urls: vec![],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://empty.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: vec![],
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    match build_scrape_state_changes(&diff, &outputs, &config) {
        Err(ScrapeBatchBuildError::EmptyScrapePayload { ref url }) => {
            assert_eq!(url, "https://empty.com");
        }
        other => panic!("expected EmptyScrapePayload, got: {other:?}"),
    }
}

// B20: EmptyScrapePayload for new URL
#[test]
fn scrape_batch_returns_empty_payload_when_new_url_artifact_has_zero_bytes() {
    let diff = ScrapeDiff {
        unchanged: vec![],
        changed: vec![],
        new_urls: vec!["https://empty.com".into()],
        deleted: vec![],
    };
    let mut artifacts = HashMap::new();
    artifacts.insert(
        "https://empty.com".into(),
        ScrapeArtifact {
            content_hash: [0x01; 32],
            status_code: 200,
            payload_bytes: vec![],
        },
    );
    let outputs = ScrapeOutputs { artifacts };
    let config = make_config(1_700_000_000);
    match build_scrape_state_changes(&diff, &outputs, &config) {
        Err(ScrapeBatchBuildError::EmptyScrapePayload { ref url }) => {
            assert_eq!(url, "https://empty.com");
        }
        other => panic!("expected EmptyScrapePayload, got: {other:?}"),
    }
}

// Error Display tests
#[test]
fn missing_scrape_artifact_error_displays_url() {
    let error = ScrapeBatchBuildError::MissingScrapeArtifact {
        url: "https://missing.com".into(),
    };
    assert!(format!("{error}").contains("https://missing.com"));
}
#[test]
fn empty_scrape_payload_error_displays_url() {
    let error = ScrapeBatchBuildError::EmptyScrapePayload {
        url: "https://empty.com".into(),
    };
    assert!(format!("{error}").contains("https://empty.com"));
}
#[test]
fn duplicate_url_error_displays_url() {
    let error = ScrapeBatchBuildError::DuplicateUrl {
        url: "https://dup.com".into(),
    };
    assert!(format!("{error}").contains("https://dup.com"));
}
#[test]
fn empty_diff_error_displays_message() {
    let error = ScrapeBatchBuildError::EmptyDiff;
    assert!(format!("{error}").contains("empty"));
}
#[test]
fn payload_processing_failed_displays_url_and_reason() {
    let error = ScrapeBatchBuildError::PayloadProcessingFailed {
        url: "https://fail.com".into(),
        reason: "hash function returned error".into(),
    };
    let display = format!("{error}");
    assert!(display.contains("https://fail.com"));
    assert!(display.contains("hash function returned error"));
}
