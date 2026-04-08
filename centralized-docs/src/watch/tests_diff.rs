//! Tests for diff computation and snapshot handling.

use super::*;
use crate::scrape::validation::{ScrapeResult, ScrapedPage};

fn make_page(url: &str, title: &str, content: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: content.to_string(),
        title: title.to_string(),
        links: vec![],
        headers: vec![],
        word_count: content.split_whitespace().count(),
        slug: url.replace('/', "_"),
        filter_status: crate::scrape::validation::PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

fn make_result(base: &str, pages: Vec<ScrapedPage>) -> ScrapeResult {
    ScrapeResult {
        total_urls: pages.len(),
        success_count: pages.len(),
        error_count: 0,
        errors: vec![],
        base_url: base.to_string(),
        pages,
    }
}

fn make_snapshot(target: &str, pages: &[(&str, &str, &str)]) -> Snapshot {
    let result = make_result(
        target,
        pages
            .iter()
            .map(|(url, title, content)| make_page(url, title, content))
            .collect(),
    );
    snapshot_from_scrape(target, &result)
}

#[test]
fn test_snapshot_from_scrape_deterministic() {
    let result = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "content a"),
            make_page("https://example.com/b", "Page B", "content b"),
        ],
    );

    let snap1 = snapshot_from_scrape("https://example.com", &result);
    let snap2 = snapshot_from_scrape("https://example.com", &result);

    for (url, hash1) in &snap1.pages {
        let hash2 = snap2.pages.get(url).expect("url missing");
        assert_eq!(hash1.content_hash, hash2.content_hash);
    }
}

#[test]
fn test_empty_plan_on_identical_content() {
    let pages = vec![
        make_page("https://example.com/a", "Page A", "hello"),
        make_page("https://example.com/b", "Page B", "world"),
    ];
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result("https://example.com", pages);

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(plan.changes.is_empty());
    assert!(plan.summary.is_empty());
    assert_eq!(plan.summary.unchanged, 2);
}

#[test]
fn test_detects_added_page() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "hello")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "new page"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 1);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
    assert_eq!(plan.changes[0].url, "https://example.com/b");
    assert_eq!(plan.changes[0].kind, ChangeKind::Added);
}

#[test]
fn test_detects_removed_page() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "hello")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.removed, 1);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
    assert_eq!(plan.changes[0].url, "https://example.com/b");
    assert_eq!(plan.changes[0].kind, ChangeKind::Removed);
}

#[test]
fn test_detects_modified_page() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "new content")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 1);
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.changes[0].kind, ChangeKind::Modified);
    assert!(plan.changes[0].old_hash.is_some());
    assert!(plan.changes[0].new_hash.is_some());
    assert_ne!(plan.changes[0].old_hash, plan.changes[0].new_hash);
}

#[test]
fn test_apply_is_idempotent() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "content")],
    );

    let scrape1 = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "content")],
    );
    let scrape2 = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "content")],
    );

    let plan1 = compute_plan("https://example.com", &prev, &scrape1);
    let plan2 = compute_plan("https://example.com", &prev, &scrape2);

    assert!(plan1.changes.is_empty());
    assert!(plan2.changes.is_empty());
}

#[test]
fn test_first_scrape_all_added() {
    let empty = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: std::collections::BTreeMap::new(),
    };
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "world"),
        ],
    );

    let plan = compute_plan("https://example.com", &empty, &current);

    assert_eq!(plan.summary.added, 2);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.unchanged, 0);
}

#[test]
fn test_complete_removal() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result("https://example.com", vec![]);

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.removed, 2);
    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.total_current, 0);
    assert_eq!(plan.summary.total_previous, 2);
}

#[test]
fn test_snapshot_serialization_roundtrip() {
    let snapshot = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );

    let json = serde_json::to_string(&snapshot).expect("serialize");
    let restored: Snapshot = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(snapshot.target_url, restored.target_url);
    assert_eq!(snapshot.pages.len(), restored.pages.len());
    for (url, orig) in &snapshot.pages {
        let restored_page = restored.pages.get(url).expect("url missing");
        assert_eq!(orig.content_hash, restored_page.content_hash);
        assert_eq!(orig.title, restored_page.title);
    }
}
