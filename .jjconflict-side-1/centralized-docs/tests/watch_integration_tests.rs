#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for the watch/change-tracking module.
//!
//! Tests the full plan/apply workflow including serialization roundtrips,
//! file I/O, directory diffing, and edge cases.

use doc_transformer::scrape::validation::{PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::watch::*;
use proptest::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_page(url: &str, title: &str, content: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: content.to_string(),
        title: title.to_string(),
        links: vec![],
        headers: vec![],
        word_count: content.split_whitespace().count(),
        slug: url.replace('/', "_"),
        filter_status: PageFilterStatus::Unfiltered,
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

fn write_manifest(dir: &Path, result: &ScrapeResult) {
    let file = std::fs::File::create(dir.join("manifest.json")).expect("create manifest");
    serde_json::to_writer_pretty(file, result).expect("write manifest");
}

// ---------------------------------------------------------------------------
// Snapshot persistence roundtrip
// ---------------------------------------------------------------------------

#[test]
fn snapshot_json_roundtrip_preserves_all_hashes() {
    let snapshot = make_snapshot(
        "https://docs.example.com",
        &[
            ("https://docs.example.com/a", "Page A", "hello world"),
            ("https://docs.example.com/b", "Page B", "goodbye world"),
            ("https://docs.example.com/c", "Page C", "new content here"),
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
        assert_eq!(orig.url, restored_page.url);
    }
}

// ---------------------------------------------------------------------------
// Large scale
// ---------------------------------------------------------------------------

#[test]
fn large_scrape_produces_correct_plan() {
    let base = "https://example.com";
    let previous_pages: Vec<(&str, &str, &str)> = (0..100)
        .map(|i| {
            let url = format!("https://example.com/page-{i}");
            let title = format!("Page {i}");
            let content = format!("Content for page {i}");
            // Leak to get 'static references — acceptable in tests
            let url: &'static str = Box::leak(url.into_boxed_str());
            let title: &'static str = Box::leak(title.into_boxed_str());
            let content: &'static str = Box::leak(content.into_boxed_str());
            (url, title, content)
        })
        .collect();

    let prev = make_snapshot(base, &previous_pages);

    // Current: 50 unchanged, 10 modified, 10 removed, 5 added
    let current_pages: Vec<ScrapedPage> = (0..100)
        .filter_map(|i| {
            if i >= 90 {
                // Removed: pages 90-99
                return None;
            }
            if i < 5 {
                // Added: pages 100-104 handled separately
                let url = format!("https://example.com/page-{i}");
                let title = format!("Page {i}");
                let content = format!("Content for page {i}");
                return Some(make_page(&url, &title, &content));
            }
            if i < 10 {
                // Also added (new pages)
                let url = format!("https://example.com/page-{i}");
                let title = format!("Page {i}");
                let content = format!("Content for page {i}");
                return Some(make_page(&url, &title, &content));
            }
            if i < 20 {
                // Modified
                let url = format!("https://example.com/page-{i}");
                let title = format!("Page {i}");
                let content = format!("MODIFIED content for page {i}");
                return Some(make_page(&url, &title, &content));
            }
            // Unchanged
            let url = format!("https://example.com/page-{i}");
            let title = format!("Page {i}");
            let content = format!("Content for page {i}");
            Some(make_page(&url, &title, &content))
        })
        .collect();

    // Add 5 new pages
    let mut all_pages = current_pages;
    for i in 100..105 {
        all_pages.push(make_page(
            &format!("https://example.com/page-{i}"),
            &format!("Page {i}"),
            &format!("NEW content for page {i}"),
        ));
    }

    let current = make_result(base, all_pages);
    let plan = compute_plan(base, &prev, &current);

    assert_eq!(plan.summary.added, 5);
    assert_eq!(plan.summary.removed, 10);
    assert_eq!(plan.summary.modified, 10);
    assert_eq!(plan.summary.total_current, 95);
    assert_eq!(plan.summary.total_previous, 100);

    // Verify conservation
    assert_eq!(
        plan.summary.added + plan.summary.modified + plan.summary.unchanged,
        plan.summary.total_current
    );
}

// ---------------------------------------------------------------------------
// Directory diff
// ---------------------------------------------------------------------------

#[test]
fn diff_directories_compares_manifests() {
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");

    write_manifest(
        dir_a.path(),
        &make_result(
            "https://old.example.com",
            vec![
                make_page("https://old.example.com/a", "Page A", "content a"),
                make_page("https://old.example.com/b", "Page B", "content b"),
            ],
        ),
    );

    write_manifest(
        dir_b.path(),
        &make_result(
            "https://new.example.com",
            vec![
                make_page("https://old.example.com/a", "Page A", "content a modified"),
                make_page("https://old.example.com/c", "Page C", "new page"),
            ],
        ),
    );

    let plan = diff_directories(dir_a.path(), dir_b.path()).expect("diff");

    assert_eq!(plan.summary.added, 1);
    assert_eq!(plan.summary.removed, 1);
    assert_eq!(plan.summary.modified, 1);
    assert_eq!(plan.summary.unchanged, 0);
}

#[test]
fn diff_identical_directories_produces_empty_plan() {
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");

    let result = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "same")],
    );

    write_manifest(dir_a.path(), &result);
    write_manifest(dir_b.path(), &result);

    let plan = diff_directories(dir_a.path(), dir_b.path()).expect("diff");

    assert!(plan.changes.is_empty());
    assert!(plan.summary.is_empty());
}

#[test]
fn diff_missing_manifest_returns_error() {
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");

    // dir_a has manifest, dir_b does not
    write_manifest(dir_a.path(), &make_result("https://example.com", vec![]));

    let result = diff_directories(dir_a.path(), dir_b.path());
    assert!(result.is_err());
}

#[test]
fn diff_invalid_manifest_returns_error() {
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");

    write_manifest(dir_a.path(), &make_result("https://example.com", vec![]));

    // Write invalid JSON
    std::fs::write(dir_b.path().join("manifest.json"), "not json").expect("write invalid");

    let result = diff_directories(dir_a.path(), dir_b.path());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Write plan reports
// ---------------------------------------------------------------------------

#[test]
fn write_plan_reports_creates_both_files() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "old"),
            make_page("https://example.com/b", "Page B", "new"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let dir = tempfile::tempdir().expect("tempdir");

    write_plan_reports(&plan, dir.path()).expect("write");

    let json_path = dir.path().join("change-plan.json");
    let md_path = dir.path().join("change-plan.md");

    assert!(json_path.exists());
    assert!(md_path.exists());

    // Verify JSON is parseable
    let json_content = std::fs::read_to_string(&json_path).expect("read json");
    let parsed: ChangePlan = serde_json::from_str(&json_content).expect("parse json");
    assert_eq!(parsed.summary.added, 1);
    assert_eq!(parsed.target_url, "https://example.com");

    // Verify MD has expected sections
    let md_content = std::fs::read_to_string(&md_path).expect("read md");
    assert!(md_content.contains("# Documentation Change Plan"));
    assert!(md_content.contains("### Added"));
}

#[test]
fn write_plan_reports_creates_output_dir_if_missing() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "old")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let dir = tempfile::tempdir().expect("tempdir");
    let nested = dir.path().join("deeply/nested/output");

    write_plan_reports(&plan, &nested).expect("write to nested");

    assert!(nested.join("change-plan.json").exists());
    assert!(nested.join("change-plan.md").exists());
}

// ---------------------------------------------------------------------------
// Unicode handling
// ---------------------------------------------------------------------------

#[test]
fn handles_unicode_urls_and_titles() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/文档", "文档标题", "中文内容")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/文档",
            "文档标题",
            "修改后的内容",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.modified, 1);
    assert_eq!(plan.changes[0].url, "https://example.com/文档");
    assert_eq!(plan.changes[0].title, "文档标题");
}

#[test]
fn handles_emoji_in_content() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page 🚀", "Hello 🌍")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/a",
            "Page 🚀",
            "Hello 🌍 Updated 🎉",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.modified, 1);
}

// ---------------------------------------------------------------------------
// Idempotency proof
// ---------------------------------------------------------------------------

#[test]
fn same_scrape_twice_produces_identical_plans() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "content a"),
            ("https://example.com/b", "Page B", "content b"),
        ],
    );

    let scrape = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "content a modified"),
            make_page("https://example.com/c", "Page C", "new"),
        ],
    );

    let plan1 = compute_plan("https://example.com", &prev, &scrape);
    let plan2 = compute_plan("https://example.com", &prev, &scrape);

    assert_eq!(plan1.changes.len(), plan2.changes.len());
    assert_eq!(plan1.summary, plan2.summary);

    for (c1, c2) in plan1.changes.iter().zip(plan2.changes.iter()) {
        assert_eq!(c1.url, c2.url);
        assert_eq!(c1.kind, c2.kind);
        assert_eq!(c1.old_hash, c2.old_hash);
        assert_eq!(c1.new_hash, c2.new_hash);
    }
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn empty_previous_all_pages_are_added() {
    let empty = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: BTreeMap::new(),
    };
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "world"),
            make_page("https://example.com/c", "Page C", "foo"),
        ],
    );

    let plan = compute_plan("https://example.com", &empty, &current);

    assert_eq!(plan.summary.added, 3);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.changes.len(), 3);
}

#[test]
fn empty_current_all_pages_are_removed() {
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
fn both_empty_produces_empty_plan() {
    let empty = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: BTreeMap::new(),
    };
    let current = make_result("https://example.com", vec![]);

    let plan = compute_plan("https://example.com", &empty, &current);

    assert!(plan.changes.is_empty());
    assert!(plan.summary.is_empty());
}

#[test]
fn page_url_changes_are_added_plus_removed() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/old-path", "Page", "same content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/new-path",
            "Page",
            "same content",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 1);
    assert_eq!(plan.summary.removed, 1);
    assert_eq!(plan.summary.modified, 0);
}

#[test]
fn title_change_only_not_detected_as_modified() {
    // Title change with same content hash = unchanged
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Old Title", "same content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/a",
            "New Title",
            "same content",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
    assert!(plan.changes.is_empty());
}

#[test]
fn pages_with_empty_content_are_handled() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Empty", "")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/a",
            "Empty",
            "now has content",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.modified, 1);
}

#[test]
fn page_with_only_whitespace_is_hashable() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Whitespace", "   ")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Whitespace", "   ")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(plan.changes.is_empty());
}

// ---------------------------------------------------------------------------
// Markdown report formatting
// ---------------------------------------------------------------------------

#[test]
fn markdown_report_empty_plan_says_up_to_date() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "same")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "same")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let md = format_plan_markdown(&plan);

    assert!(md.contains("No changes detected"));
    assert!(md.contains("up to date"));
}

#[test]
fn markdown_report_shows_all_change_kinds() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/modified", "Mod Page", "old"),
            ("https://example.com/removed", "Rem Page", "gone"),
        ],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/modified", "Mod Page", "new"),
            make_page("https://example.com/added", "Add Page", "here"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let md = format_plan_markdown(&plan);

    assert!(md.contains("### Added"));
    assert!(md.contains("### Removed"));
    assert!(md.contains("### Modified"));
    assert!(md.contains("+ `https://example.com/added`"));
    assert!(md.contains("- `https://example.com/removed`"));
    assert!(md.contains("~ `https://example.com/modified`"));
}

// ---------------------------------------------------------------------------
// JSON report
// ---------------------------------------------------------------------------

#[test]
fn json_report_roundtrips() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "new"),
            make_page("https://example.com/b", "Page B", "added"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let json = format_plan_json(&plan).expect("serialize json");
    let parsed: ChangePlan = serde_json::from_str(&json).expect("parse json");

    assert_eq!(parsed.target_url, plan.target_url);
    assert_eq!(parsed.changes.len(), plan.changes.len());
    assert_eq!(parsed.summary, plan.summary);
}

// ---------------------------------------------------------------------------
// Proptest: hash stability
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn same_content_always_produces_same_hash(content in "[a-zA-Z0-9 ]{0,1000}") {
        let result = make_result(
            "https://example.com",
            vec![make_page("https://example.com/a", "Page", &content)],
        );
        let snap1 = snapshot_from_scrape("https://example.com", &result);
        let snap2 = snapshot_from_scrape("https://example.com", &result);

        let hash1 = snap1.pages.get("https://example.com/a").unwrap().content_hash;
        let hash2 = snap2.pages.get("https://example.com/a").unwrap().content_hash;
        prop_assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_content_produces_different_hash(
        a in "[a-zA-Z]{1,100}",
        b in "[a-zA-Z]{1,100}"
    ) {
        prop_assume!(a != b);
        let result_a = make_result("https://example.com", vec![make_page("https://example.com/x", "X", &a)]);
        let result_b = make_result("https://example.com", vec![make_page("https://example.com/x", "X", &b)]);

        let snap_a = snapshot_from_scrape("https://example.com", &result_a);
        let snap_b = snapshot_from_scrape("https://example.com", &result_b);

        let hash_a = snap_a.pages.get("https://example.com/x").unwrap().content_hash;
        let hash_b = snap_b.pages.get("https://example.com/x").unwrap().content_hash;
        prop_assert_ne!(hash_a, hash_b);
    }

    #[test]
    fn snapshot_from_same_scrape_is_deterministic(
        urls in prop::collection::vec("[a-z]{1,20}", 1..50),
    ) {
        let pages: Vec<ScrapedPage> = urls.iter().map(|u| {
            make_page(
                &format!("https://example.com/{u}"),
                &format!("Title {u}"),
                &format!("Content {u}"),
            )
        }).collect();

        let result = make_result("https://example.com", pages);
        let snap1 = snapshot_from_scrape("https://example.com", &result);
        let snap2 = snapshot_from_scrape("https://example.com", &result);

        prop_assert_eq!(snap1.pages.len(), snap2.pages.len());
        for (url, p1) in &snap1.pages {
            let p2 = snap2.pages.get(url).unwrap();
            prop_assert_eq!(p1.content_hash, p2.content_hash);
        }
    }

    #[test]
    fn summary_conservation_always_holds(
        prev_urls in prop::collection::vec("[a-z]{1,10}", 0..20),
        curr_urls in prop::collection::vec("[a-z]{1,10}", 0..20),
    ) {
        // Deduplicate — BTreeMap deduplicates by URL, which is correct behavior
        let prev_unique: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            prev_urls.into_iter().filter(|u| seen.insert(u.clone())).collect()
        };
        let curr_unique: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            curr_urls.into_iter().filter(|u| seen.insert(u.clone())).collect()
        };

        let prev_pages: Vec<(&str, &str, &str)> = prev_unique.iter().map(|u| {
            let url = format!("https://example.com/{u}");
            let title = format!("Title {u}");
            let content = format!("Content {u}");
            let url: &'static str = Box::leak(url.into_boxed_str());
            let title: &'static str = Box::leak(title.into_boxed_str());
            let content: &'static str = Box::leak(content.into_boxed_str());
            (url, title, content)
        }).collect();

        let prev = make_snapshot("https://example.com", &prev_pages);

        let curr_pages: Vec<ScrapedPage> = curr_unique.iter().map(|u| {
            make_page(
                &format!("https://example.com/{u}"),
                &format!("Title {u}"),
                &format!("Content {u}"),
            )
        }).collect();

        let current = make_result("https://example.com", curr_pages);
        let plan = compute_plan("https://example.com", &prev, &current);

        // Conservation invariant: added + modified + unchanged == total_current
        prop_assert_eq!(
            plan.summary.added + plan.summary.modified + plan.summary.unchanged,
            plan.summary.total_current,
            "Summary conservation violated: {} + {} + {} != {}",
            plan.summary.added, plan.summary.modified, plan.summary.unchanged, plan.summary.total_current
        );

        // Removed count matches pages in prev but not in current
        prop_assert_eq!(plan.summary.total_previous, prev.pages.len());
        prop_assert_eq!(plan.summary.total_current, current.pages.len());
    }

    #[test]
    fn compute_plan_is_idempotent(
        prev_count in 0usize..15,
        curr_count in 0usize..15,
        seed in 0u64..1000,
    ) {
        let prev: Vec<(&str, &str, &str)> = (0..prev_count).map(|i| {
            let url = format!("https://example.com/p{seed}_{i}");
            let title = format!("Page {i}");
            let content = format!("content {seed} {i}");
            let url: &'static str = Box::leak(url.into_boxed_str());
            let title: &'static str = Box::leak(title.into_boxed_str());
            let content: &'static str = Box::leak(content.into_boxed_str());
            (url, title, content)
        }).collect();

        let snapshot = make_snapshot("https://example.com", &prev);

        let current: Vec<ScrapedPage> = (0..curr_count).map(|i| {
            make_page(
                &format!("https://example.com/c{seed}_{i}"),
                &format!("Current {i}"),
                &format!("current content {seed} {i}"),
            )
        }).collect();

        let scrape = make_result("https://example.com", current);

        let plan1 = compute_plan("https://example.com", &snapshot, &scrape);
        let plan2 = compute_plan("https://example.com", &snapshot, &scrape);

        prop_assert_eq!(plan1.summary, plan2.summary);
        prop_assert_eq!(plan1.changes.len(), plan2.changes.len());
    }
}

// ---------------------------------------------------------------------------
// Sort order verification
// ---------------------------------------------------------------------------

#[test]
fn changes_are_sorted_by_kind_then_url() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/z-modified", "Z", "old"),
            ("https://example.com/a-removed", "A", "gone"),
        ],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/z-modified", "Z", "new"),
            make_page("https://example.com/b-added", "B", "here"),
            make_page("https://example.com/m-added", "M", "also"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    // Changes should be sorted: added first, then modified, then removed
    let kinds: Vec<_> = plan.changes.iter().map(|c| format!("{}", c.kind)).collect();
    let mut sorted_kinds = kinds.clone();
    sorted_kinds.sort();
    assert_eq!(kinds, sorted_kinds, "Changes are not sorted by kind");

    // Within each kind, URLs should be sorted
    let added_urls: Vec<_> = plan
        .changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Added)
        .map(|c| c.url.clone())
        .collect();
    let mut sorted_added = added_urls.clone();
    sorted_added.sort();
    assert_eq!(added_urls, sorted_added, "Added changes not sorted by URL");
}

// ---------------------------------------------------------------------------
// ChangeSummary::is_empty dedicated test
// ---------------------------------------------------------------------------

#[test]
fn change_summary_is_empty_when_all_zero() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "same")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "same")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(plan.summary.is_empty());
    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
}

#[test]
fn change_summary_is_not_empty_when_any_nonzero() {
    let prev = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: BTreeMap::new(),
    };
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "new")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(!plan.summary.is_empty());
}

// ---------------------------------------------------------------------------
// PageChange field consistency (Added → old_hash None, Removed → new_hash None)
// ---------------------------------------------------------------------------

#[test]
fn added_changes_have_no_old_hash() {
    let prev = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: BTreeMap::new(),
    };
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "new")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    let added: Vec<_> = plan
        .changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Added)
        .collect();
    for change in added {
        assert!(change.old_hash.is_none(), "Added change has old_hash");
        assert!(change.new_hash.is_some(), "Added change missing new_hash");
    }
}

#[test]
fn removed_changes_have_no_new_hash() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "gone")],
    );
    let current = make_result("https://example.com", vec![]);

    let plan = compute_plan("https://example.com", &prev, &current);

    let removed: Vec<_> = plan
        .changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Removed)
        .collect();
    for change in removed {
        assert!(change.old_hash.is_some(), "Removed change missing old_hash");
        assert!(change.new_hash.is_none(), "Removed change has new_hash");
    }
}

#[test]
fn modified_changes_have_both_hashes() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "new")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    let modified: Vec<_> = plan
        .changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Modified)
        .collect();
    for change in modified {
        assert!(
            change.old_hash.is_some(),
            "Modified change missing old_hash"
        );
        assert!(
            change.new_hash.is_some(),
            "Modified change missing new_hash"
        );
        assert_ne!(
            change.old_hash, change.new_hash,
            "Modified change has same old and new hash"
        );
    }
}

// ---------------------------------------------------------------------------
// ChangeKind Display
// ---------------------------------------------------------------------------

#[test]
fn change_kind_display_formatting() {
    assert_eq!(format!("{}", ChangeKind::Added), "added");
    assert_eq!(format!("{}", ChangeKind::Removed), "removed");
    assert_eq!(format!("{}", ChangeKind::Modified), "modified");
}

// ---------------------------------------------------------------------------
// Diff error paths: dir_a missing
// ---------------------------------------------------------------------------

#[test]
fn diff_returns_error_when_dir_a_missing() {
    let dir_b = tempfile::tempdir().expect("tempdir");
    write_manifest(dir_b.path(), &make_result("https://example.com", vec![]));

    let result = diff_directories(Path::new("/nonexistent/path/a"), dir_b.path());
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Fuzz-equivalent: deserialization must never panic on arbitrary bytes
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn manifest_deserialization_never_panics_on_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        // Must not panic — any input is valid to attempt deserialization
        let _ = serde_json::from_slice::<ScrapeResult>(&bytes);
    }

    #[test]
    fn snapshot_deserialization_never_panics_on_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = serde_json::from_slice::<Snapshot>(&bytes);
    }

    #[test]
    fn change_plan_deserialization_never_panics_on_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = serde_json::from_slice::<ChangePlan>(&bytes);
    }
}

// ===========================================================================
// RED QUEEN ADVERSARIAL TEST SUITE — 15 attacks
// ===========================================================================

// ---------------------------------------------------------------------------
// Dimension: boundary-attacks
// ---------------------------------------------------------------------------

#[test]
fn plan_handles_url_with_query_params_and_fragment() {
    let tricky_url = "https://example.com/page?foo=bar&baz=qux#section-1";
    let prev = make_snapshot(
        "https://example.com",
        &[(tricky_url, "Page", "original content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(tricky_url, "Page", "modified content")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(
        plan.summary.modified, 1,
        "Should detect modification on URL with query+fragment"
    );
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.changes.len(), 1);
    assert_eq!(
        plan.changes[0].url, tricky_url,
        "URL should be preserved verbatim"
    );
}

#[test]
fn plan_handles_very_long_url_2000_chars() {
    let path_segment: String = "a".repeat(2000);
    let long_url = format!("https://example.com/{path_segment}");
    assert!(long_url.len() > 2000, "URL must exceed 2000 chars");

    let prev = make_snapshot(
        "https://example.com",
        &[(&long_url, "Long URL Page", "before")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(&long_url, "Long URL Page", "after")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(
        plan.summary.modified, 1,
        "Must detect change even with 2000+ char URL"
    );
    assert_eq!(
        plan.changes[0].url, long_url,
        "Long URL must be preserved in change record"
    );
}

#[test]
fn plan_handles_page_with_only_newlines() {
    let content = "\n\n\n\n";
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Newline Page", content)],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Newline Page", content)],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(
        plan.changes.is_empty(),
        "Identical newline-only content should produce no changes"
    );
    assert_eq!(plan.summary.unchanged, 1);

    // Now change the newline count
    let current2 = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/a",
            "Newline Page",
            "\n\n\n\n\n",
        )],
    );
    let plan2 = compute_plan("https://example.com", &prev, &current2);

    assert_eq!(
        plan2.summary.modified, 1,
        "Adding one newline must be detected as modified"
    );
}

#[test]
fn plan_handles_very_long_title_10k_chars() {
    let long_title: String = "T".repeat(10_000);
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", &long_title, "content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page(
            "https://example.com/a",
            &long_title,
            "changed content",
        )],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(
        plan.summary.modified, 1,
        "Must detect content change despite 10k-char title"
    );
    assert_eq!(
        plan.changes[0].title.len(),
        10_000,
        "Title must be preserved at full length"
    );
    assert_eq!(plan.changes[0].title, long_title);
}

// ---------------------------------------------------------------------------
// Dimension: hash-collision-resistance
// ---------------------------------------------------------------------------

#[test]
fn different_single_char_content_produces_different_hashes() {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'z', 'A', 'Z', '0', '9', ' ', '\n', '\t',
    ];
    let mut hashes = Vec::new();

    for &ch in &chars {
        let content = ch.to_string();
        let result = make_result(
            "https://example.com",
            vec![make_page("https://example.com/x", "X", &content)],
        );
        let snap = snapshot_from_scrape("https://example.com", &result);
        let hash = snap
            .pages
            .get("https://example.com/x")
            .expect("page must exist")
            .content_hash;
        hashes.push((ch, hash));
    }

    // Every single-char content must produce a unique hash
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i].1, hashes[j].1,
                "Hash collision: char {:?} and char {:?} produced same hash",
                hashes[i].0, hashes[j].0
            );
        }
    }
}

#[test]
fn whitespace_variations_detected_as_modified() {
    let variants = ["hello", "hello ", " hello", "\thello", "hello\t", "hello\n"];
    let base_url = "https://example.com";
    let url_key = "https://example.com/a";

    // Build hashes for each whitespace variant
    let mut hashes = Vec::new();
    for content in &variants {
        let result = make_result(base_url, vec![make_page(url_key, "Page", content)]);
        let snap = snapshot_from_scrape(base_url, &result);
        let hash = snap
            .pages
            .get(url_key)
            .expect("page must exist")
            .content_hash;
        hashes.push(hash);
    }

    // All variants must differ from each other
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Whitespace variation '{}' vs '{}' produced same hash — content not distinguished",
                variants[i], variants[j]
            );
        }
    }

    // Also verify compute_plan detects the difference
    let prev = make_snapshot(base_url, &[(url_key, "Page", "hello")]);
    let current = make_result(base_url, vec![make_page(url_key, "Page", "hello ")]);
    let plan = compute_plan(base_url, &prev, &current);
    assert_eq!(
        plan.summary.modified, 1,
        "Trailing space must trigger modification"
    );
}

#[test]
fn case_sensitive_hashing() {
    let variants = ["Hello", "hello", "HELLO", "hELLo"];
    let base_url = "https://example.com";
    let url_key = "https://example.com/a";

    let mut hashes = Vec::new();
    for content in &variants {
        let result = make_result(base_url, vec![make_page(url_key, "Page", content)]);
        let snap = snapshot_from_scrape(base_url, &result);
        let hash = snap
            .pages
            .get(url_key)
            .expect("page must exist")
            .content_hash;
        hashes.push(hash);
    }

    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Case variation '{}' vs '{}' produced same hash — case-insensitive hashing detected",
                variants[i], variants[j]
            );
        }
    }

    // Verify plan detects case change
    let prev = make_snapshot(base_url, &[(url_key, "Page", "Hello")]);
    let current = make_result(base_url, vec![make_page(url_key, "Page", "hello")]);
    let plan = compute_plan(base_url, &prev, &current);
    assert_eq!(
        plan.summary.modified, 1,
        "Case change 'Hello'→'hello' must be detected"
    );
}

// ---------------------------------------------------------------------------
// Dimension: scale-stress
// ---------------------------------------------------------------------------

#[test]
fn plan_with_1000_pages_all_added() {
    let empty = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: BTreeMap::new(),
    };

    let pages: Vec<ScrapedPage> = (0..1000)
        .map(|i| {
            make_page(
                &format!("https://example.com/page-{i}"),
                &format!("Page {i}"),
                &format!("Content {i}"),
            )
        })
        .collect();

    let current = make_result("https://example.com", pages);
    let plan = compute_plan("https://example.com", &empty, &current);

    assert_eq!(plan.summary.added, 1000, "All 1000 pages should be added");
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.summary.total_current, 1000);
    assert_eq!(plan.summary.total_previous, 0);
    assert_eq!(plan.changes.len(), 1000);
}

#[test]
fn plan_with_1000_pages_all_removed() {
    let prev_pages: Vec<(&str, &str, &str)> = (0..1000)
        .map(|i| {
            let url = format!("https://example.com/page-{i}");
            let title = format!("Page {i}");
            let content = format!("Content {i}");
            let url: &'static str = Box::leak(url.into_boxed_str());
            let title: &'static str = Box::leak(title.into_boxed_str());
            let content: &'static str = Box::leak(content.into_boxed_str());
            (url, title, content)
        })
        .collect();

    let prev = make_snapshot("https://example.com", &prev_pages);
    let current = make_result("https://example.com", vec![]);

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(
        plan.summary.removed, 1000,
        "All 1000 pages should be removed"
    );
    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.total_current, 0);
    assert_eq!(plan.summary.total_previous, 1000);
    assert_eq!(plan.changes.len(), 1000);
}

#[test]
fn plan_with_500_added_500_removed_simultaneously() {
    // Previous: pages 0..500
    let prev_pages: Vec<(&str, &str, &str)> = (0..500)
        .map(|i| {
            let url = format!("https://example.com/old-{i}");
            let title = format!("Old {i}");
            let content = format!("Old content {i}");
            let url: &'static str = Box::leak(url.into_boxed_str());
            let title: &'static str = Box::leak(title.into_boxed_str());
            let content: &'static str = Box::leak(content.into_boxed_str());
            (url, title, content)
        })
        .collect();

    let prev = make_snapshot("https://example.com", &prev_pages);

    // Current: pages 500..1000 (completely different URLs)
    let current_pages: Vec<ScrapedPage> = (500..1000)
        .map(|i| {
            make_page(
                &format!("https://example.com/new-{i}"),
                &format!("New {i}"),
                &format!("New content {i}"),
            )
        })
        .collect();

    let current = make_result("https://example.com", current_pages);
    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 500, "500 new pages should be added");
    assert_eq!(plan.summary.removed, 500, "500 old pages should be removed");
    assert_eq!(
        plan.summary.modified, 0,
        "No pages share URLs so none are modified"
    );
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.summary.total_current, 500);
    assert_eq!(plan.summary.total_previous, 500);

    // Conservation check
    assert_eq!(
        plan.summary.added + plan.summary.modified + plan.summary.unchanged,
        plan.summary.total_current,
        "Summary conservation violated"
    );
}

// ---------------------------------------------------------------------------
// Dimension: serde-boundary
// ---------------------------------------------------------------------------

#[test]
fn snapshot_roundtrip_with_extreme_hash_values() {
    let base_url = "https://example.com";

    // Manually construct PageHash with extreme [u8; 32] values
    let all_max = [u8::MAX; 32];
    let all_zero = [0u8; 32];
    let mixed = {
        let mut arr = [0u8; 32];
        arr[0] = u8::MAX;
        arr[16] = u8::MAX;
        arr[31] = u8::MAX;
        arr
    };

    let mut pages = BTreeMap::new();
    pages.insert(
        format!("{base_url}/max"),
        PageHash {
            url: format!("{base_url}/max"),
            content_hash: all_max,
            title: "Max Hash".to_string(),
        },
    );
    pages.insert(
        format!("{base_url}/zero"),
        PageHash {
            url: format!("{base_url}/zero"),
            content_hash: all_zero,
            title: "Zero Hash".to_string(),
        },
    );
    pages.insert(
        format!("{base_url}/mid"),
        PageHash {
            url: format!("{base_url}/mid"),
            content_hash: mixed,
            title: "Mid Hash".to_string(),
        },
    );

    let snapshot = Snapshot {
        target_url: base_url.to_string(),
        timestamp: chrono::Utc::now(),
        pages,
    };

    let json = serde_json::to_string(&snapshot).expect("serialize extreme hashes");
    let restored: Snapshot = serde_json::from_str(&json).expect("deserialize extreme hashes");

    assert_eq!(
        restored
            .pages
            .get(&format!("{base_url}/max"))
            .expect("max page")
            .content_hash,
        all_max,
        "all-max hash must survive roundtrip"
    );
    assert_eq!(
        restored
            .pages
            .get(&format!("{base_url}/zero"))
            .expect("zero page")
            .content_hash,
        all_zero,
        "all-zero hash must survive roundtrip"
    );
    assert_eq!(
        restored
            .pages
            .get(&format!("{base_url}/mid"))
            .expect("mid page")
            .content_hash,
        mixed,
        "mixed hash must survive roundtrip"
    );
}

#[test]
fn change_plan_json_survives_pretty_print_roundtrip() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "old content"),
            ("https://example.com/b", "Page B", "to be removed"),
        ],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "new content"),
            make_page("https://example.com/c", "Page C", "added page"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    // Serialize → pretty print → parse → compare
    let json = serde_json::to_string_pretty(&plan).expect("pretty serialize");
    let restored: ChangePlan = serde_json::from_str(&json).expect("parse pretty json");

    assert_eq!(restored.target_url, plan.target_url);
    assert_eq!(restored.changes.len(), plan.changes.len());
    assert_eq!(restored.summary, plan.summary);

    for (orig, rest) in plan.changes.iter().zip(restored.changes.iter()) {
        assert_eq!(orig.url, rest.url, "URL mismatch after roundtrip");
        assert_eq!(orig.kind, rest.kind, "Kind mismatch after roundtrip");
        assert_eq!(
            orig.old_hash, rest.old_hash,
            "old_hash mismatch after roundtrip"
        );
        assert_eq!(
            orig.new_hash, rest.new_hash,
            "new_hash mismatch after roundtrip"
        );
        assert_eq!(orig.title, rest.title, "title mismatch after roundtrip");
    }

    assert_eq!(
        restored.pending_snapshot.pages.len(),
        plan.pending_snapshot.pages.len()
    );
}

// ---------------------------------------------------------------------------
// Dimension: adversarial-inputs
// ---------------------------------------------------------------------------

#[test]
fn plan_with_empty_string_url() {
    // Page with empty URL — does it blow up or degrade gracefully?
    let prev = make_snapshot("https://example.com", &[("", "Empty URL Page", "content")]);
    let current = make_result(
        "https://example.com",
        vec![make_page("", "Empty URL Page", "modified content")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    // The system must not panic. It should either handle it or report it.
    // With empty URL, BTreeMap key is "" — it should still diff correctly.
    assert_eq!(
        plan.summary.modified, 1,
        "Empty URL page should still be tracked for modification"
    );
    // The change record should have the empty URL
    assert_eq!(
        plan.changes[0].url, "",
        "Empty URL should be preserved in change record"
    );
}

#[test]
fn plan_with_special_chars_in_title() {
    let malicious_titles = vec![
        "<script>alert('xss')</script>",
        "'; DROP TABLE users; --",
        "Title with \0 null byte",
        "Title\nwith\nnewlines",
        "Title with ${env:SECRET}",
        "../../../etc/passwd",
        "Title with emoji 🚀🎉 and unicode ñ ü ö",
    ];

    for title in &malicious_titles {
        let prev = make_snapshot(
            "https://example.com",
            &[("https://example.com/a", title, "content")],
        );
        let current = make_result(
            "https://example.com",
            vec![make_page("https://example.com/a", title, "changed")],
        );

        let plan = compute_plan("https://example.com", &prev, &current);

        assert_eq!(
            plan.summary.modified, 1,
            "Must detect modification for malicious title: {title:?}"
        );
        assert_eq!(
            plan.changes[0].title, *title,
            "Title must be preserved verbatim for: {title:?}"
        );

        // Also verify JSON serialization doesn't choke
        let json = format_plan_json(&plan)
            .unwrap_or_else(|_| panic!("JSON serialize must not fail for title: {title:?}"));
        let parsed: ChangePlan = serde_json::from_str(&json)
            .unwrap_or_else(|_| panic!("JSON parse must not fail for title: {title:?}"));
        assert_eq!(parsed.changes[0].title, *title);
    }
}

#[test]
fn diff_with_one_empty_manifest_and_one_large() {
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");

    // dir_a: empty manifest (0 pages)
    write_manifest(dir_a.path(), &make_result("https://example.com", vec![]));

    // dir_b: 500 pages
    let large_pages: Vec<ScrapedPage> = (0..500)
        .map(|i| {
            make_page(
                &format!("https://example.com/page-{i}"),
                &format!("Page {i}"),
                &format!("Content for page {i}"),
            )
        })
        .collect();
    write_manifest(
        dir_b.path(),
        &make_result("https://example.com", large_pages),
    );

    let plan = diff_directories(dir_a.path(), dir_b.path()).expect("diff must succeed");

    assert_eq!(plan.summary.added, 500, "All 500 pages should be added");
    assert_eq!(plan.summary.removed, 0, "Nothing to remove from empty");
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.total_current, 500);
    assert_eq!(plan.summary.total_previous, 0);

    // Reverse: large → empty
    let plan_rev = diff_directories(dir_b.path(), dir_a.path()).expect("reverse diff must succeed");

    assert_eq!(
        plan_rev.summary.removed, 500,
        "All 500 pages should be removed"
    );
    assert_eq!(plan_rev.summary.added, 0);
    assert_eq!(plan_rev.summary.total_current, 0);
    assert_eq!(plan_rev.summary.total_previous, 500);
}
