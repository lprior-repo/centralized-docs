//! Tests for snapshot construction and change-plan computation.

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
fn snapshot_from_scrape_produces_identical_hashes_for_same_input() {
    let result = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "content a"),
            make_page("https://example.com/b", "Page B", "content b"),
        ],
    );

    let snap1 = snapshot_from_scrape("https://example.com", &result);
    let snap2 = snapshot_from_scrape("https://example.com", &result);

    assert_eq!(
        snap1.pages, snap2.pages,
        "same scrape must produce identical page hashes"
    );
}

#[test]
fn compute_plan_returns_empty_changes_when_content_is_identical() {
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
fn compute_plan_detects_added_page_when_new_url_appears_in_current() {
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
fn compute_plan_detects_removed_page_when_url_disappears_from_current() {
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
fn compute_plan_detects_modified_page_when_content_hash_differs() {
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
fn compute_plan_produces_identical_empty_plans_for_same_inputs() {
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
fn compute_plan_marks_all_pages_as_added_when_previous_is_empty() {
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
fn compute_plan_marks_all_pages_as_removed_when_current_is_empty() {
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
fn snapshot_serialization_roundtrip_preserves_all_page_data() {
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
    assert_eq!(
        snapshot.pages, restored.pages,
        "all page hashes must survive JSON roundtrip"
    );
}
