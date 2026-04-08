//! Tests for formatting and report generation.

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
fn test_markdown_report_format() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "hello")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "updated"),
            make_page("https://example.com/b", "Page B", "new"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let md = format_plan_markdown(&plan);

    assert!(md.contains("# Documentation Change Plan"));
    assert!(md.contains("**Added:** 1"));
    assert!(md.contains("**Modified:** 1"));
    assert!(md.contains("### Added"));
    assert!(md.contains("### Modified"));
    assert!(md.contains("Run `ctd apply`"));
}

#[test]
fn test_json_report_format() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "hello")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "hello")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let json = format_plan_json(&plan).expect("json serialize");

    let parsed: ChangePlan = serde_json::from_str(&json).expect("json parse");
    assert!(parsed.changes.is_empty());
    assert_eq!(parsed.target_url, "https://example.com");
}

#[test]
fn test_write_plan_reports_creates_files() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "hello")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "new"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);
    let dir = tempfile::tempdir().expect("tempdir");

    write_plan_reports(&plan, dir.path()).expect("write reports");

    assert!(dir.path().join("change-plan.json").exists());
    assert!(dir.path().join("change-plan.md").exists());

    let json_content =
        std::fs::read_to_string(dir.path().join("change-plan.json")).expect("read json");
    let parsed: ChangePlan = serde_json::from_str(&json_content).expect("parse json");
    assert_eq!(parsed.summary.added, 1);
}
