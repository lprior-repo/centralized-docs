//! Change tracking and plan/apply workflow for documentation monitoring.
//!
//! Provides a Terraform-style plan/apply model: scrape a site, compare against
//! the previous snapshot, produce a change plan, then optionally apply it.
//!
//! # Workflow
//!
//! ```text
//! ctd watch <URL>       → scrape + diff against stored snapshot → change-plan.json
//! ctd apply <URL>       → commit the new snapshot (idempotent)
//! ctd diff <DIR_A> <DIR_B> → compare two .scrape directories
//! ```
//!
//! # Idempotency
//!
//! Running `watch` on unchanged content produces an empty plan.
//! Running `apply` on an empty plan is a no-op.
//! Running `apply` twice with the same content is a no-op.

use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use crate::scrape::validation::ScrapeResult;

/// A content hash for a single page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageHash {
    /// The canonical URL of the page.
    pub url: String,
    /// `xxh3_128` hash of the page's markdown content.
    pub content_hash: u128,
    /// The page title (for display in diffs).
    pub title: String,
}

/// A point-in-time snapshot of all scraped pages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// The target URL that was scraped.
    pub target_url: String,
    /// When this snapshot was taken.
    pub timestamp: DateTime<Utc>,
    /// Map of `URL` → `PageHash`, sorted for deterministic output.
    pub pages: BTreeMap<String, PageHash>,
}

/// The kind of change detected for a page.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, derive_more::Display,
)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    /// Page appeared that wasn't in the previous snapshot.
    #[display("added")]
    Added,
    /// Page content hash changed.
    #[display("modified")]
    Modified,
    /// Page disappeared from the current scrape.
    #[display("removed")]
    Removed,
}

/// A single page-level change between two snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageChange {
    /// The URL that changed.
    pub url: String,
    /// What kind of change.
    pub kind: ChangeKind,
    /// Previous content hash (None for Added).
    pub old_hash: Option<u128>,
    /// New content hash (None for Removed).
    pub new_hash: Option<u128>,
    /// Page title for display.
    pub title: String,
}

/// A change plan produced by `ctd watch`.
///
/// This is the "terraform plan" equivalent — it shows what *would* change
/// if you ran `ctd apply`. It does NOT modify the stored snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePlan {
    /// The target URL being watched.
    pub target_url: String,
    /// When this plan was generated.
    pub timestamp: DateTime<Utc>,
    /// All detected changes.
    pub changes: Vec<PageChange>,
    /// Summary counts.
    pub summary: ChangeSummary,
    /// The new snapshot (ready to be applied).
    pub pending_snapshot: Snapshot,
}

/// Summary statistics for a change plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub unchanged: usize,
    pub total_current: usize,
    pub total_previous: usize,
}

impl ChangeSummary {
    /// True if there are no changes at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.added == 0 && self.removed == 0 && self.modified == 0
    }
}

/// Format a change plan as a human-readable report.
#[must_use]
pub fn format_plan_markdown(plan: &ChangePlan) -> String {
    fn changes_body(changes: &[PageChange]) -> String {
        // Single-pass fold into 3 buffers - no mut in hot path, functional style
        let (added, removed, modified) = changes.iter().fold(
            (String::new(), String::new(), String::new()),
            |(mut a, mut r, mut m), c| {
                let buf = match c.kind {
                    ChangeKind::Added => &mut a,
                    ChangeKind::Removed => &mut r,
                    ChangeKind::Modified => &mut m,
                };
                let prefix = match c.kind {
                    ChangeKind::Added => "+ ",
                    ChangeKind::Removed => "- ",
                    ChangeKind::Modified => "~ ",
                };
                let _ = writeln!(buf, "{prefix}`{}` — {}", c.url, c.title);
                (a, r, m)
            },
        );

        fn section(header: &str, body: &str) -> Option<String> {
            (!body.is_empty()).then(|| format!("### {header}\n\n{body}\n"))
        }

        [
            section("Added", &added),
            section("Removed", &removed),
            section("Modified", &modified),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn summary_lines(s: &ChangeSummary) -> String {
        [
            format!("- **Added:** {} pages", s.added),
            format!("- **Removed:** {} pages", s.removed),
            format!("- **Modified:** {} pages", s.modified),
            format!("- **Unchanged:** {} pages", s.unchanged),
            format!(
                "- **Total:** {} (was {})",
                s.total_current, s.total_previous
            ),
        ]
        .into_iter()
        .join("\n")
    }

    // Pre-allocate estimated capacity: ~80 chars per change line + overhead
    let estimated = 200 + plan.changes.len() * 80;
    let mut out = String::with_capacity(estimated);

    let timestamp_str = plan.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let _ = writeln!(out, "# Documentation Change Plan");
    let _ = writeln!(out);
    let _ = writeln!(out, "**Target:** {}", plan.target_url);
    let _ = writeln!(out, "**Generated:** {timestamp_str}");
    let _ = writeln!(out);

    let _ = writeln!(out, "## Summary");
    let _ = writeln!(out);
    let summary = summary_lines(&plan.summary);
    let _ = writeln!(out, "{summary}");

    let _ = writeln!(out);
    let body = if plan.changes.is_empty() {
        "No changes detected. The documentation is up to date.".to_string()
    } else {
        format!(
            "## Changes\n\n{body}---\n\nRun `ctd apply` to commit these changes.",
            body = changes_body(&plan.changes)
        )
    };

    let _ = writeln!(out, "{body}");
    out
}

/// Format a change plan as JSON.
///
/// # Errors
///
/// Returns an error if serialization fails (should not happen with valid plan).
pub fn format_plan_json(plan: &ChangePlan) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(plan)
}

/// Build a `Snapshot` from a `ScrapeResult`.
///
/// Pure calculation — hashes each page's markdown content.
#[must_use]
pub fn snapshot_from_scrape(target_url: &str, result: &ScrapeResult) -> Snapshot {
    let pages = result
        .pages
        .iter()
        .map(|page| {
            let hash = crate::cache::hash::content_hash(page.markdown.as_bytes());
            let entry = PageHash {
                url: page.url.clone(),
                content_hash: hash,
                title: page.title.clone(),
            };
            (page.url.clone(), entry)
        })
        .collect();

    Snapshot {
        target_url: target_url.to_string(),
        timestamp: Utc::now(),
        pages,
    }
}

/// Count changes by kind in a single pass.
fn count_by_kind(changes: &[PageChange]) -> (usize, usize, usize) {
    changes.iter().fold((0, 0, 0), |(a, r, m), c| match c.kind {
        ChangeKind::Added => (a + 1, r, m),
        ChangeKind::Removed => (a, r + 1, m),
        ChangeKind::Modified => (a, r, m + 1),
    })
}

/// Compute the diff between a previous snapshot and a new scrape.
///
/// Pure calculation — no side effects.
#[must_use]
pub fn compute_plan(
    target_url: &str,
    previous: &Snapshot,
    current_scrape: &ScrapeResult,
) -> ChangePlan {
    let current_snapshot = snapshot_from_scrape(target_url, current_scrape);
    let changes = diff_snapshots(previous, &current_snapshot);
    let (added, removed, modified) = count_by_kind(&changes);
    let unchanged = current_snapshot.pages.len() - added - modified;

    ChangePlan {
        target_url: target_url.to_string(),
        timestamp: Utc::now(),
        changes,
        summary: ChangeSummary {
            added,
            removed,
            modified,
            unchanged,
            total_current: current_snapshot.pages.len(),
            total_previous: previous.pages.len(),
        },
        pending_snapshot: current_snapshot,
    }
}

/// Diff two snapshots to produce a list of changes.
///
/// Pure calculation — iterates both maps and classifies each URL.
fn diff_snapshots(previous: &Snapshot, current: &Snapshot) -> Vec<PageChange> {
    let added_modified = current.pages.iter().filter_map(|(url, curr)| {
        previous.pages.get(url).map_or_else(
            || {
                Some(PageChange {
                    url: url.clone(),
                    kind: ChangeKind::Added,
                    old_hash: None,
                    new_hash: Some(curr.content_hash),
                    title: curr.title.clone(),
                })
            },
            |prev| {
                (prev.content_hash != curr.content_hash).then_some(PageChange {
                    url: url.clone(),
                    kind: ChangeKind::Modified,
                    old_hash: Some(prev.content_hash),
                    new_hash: Some(curr.content_hash),
                    title: curr.title.clone(),
                })
            },
        )
    });

    let removed = previous
        .pages
        .iter()
        .filter(|(url, _)| !current.pages.contains_key(url.as_str()))
        .map(|(url, prev)| PageChange {
            url: url.clone(),
            kind: ChangeKind::Removed,
            old_hash: Some(prev.content_hash),
            new_hash: None,
            title: prev.title.clone(),
        });

    added_modified
        .chain(removed)
        .sorted_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.url.cmp(&b.url)))
        .collect()
}

/// Compute a plan by comparing two `.scrape` directories.
///
/// Reads `manifest.json` from each directory and compares page hashes.
///
/// # Errors
///
/// Returns an error if either manifest.json is missing or invalid.
pub fn diff_directories(dir_a: &Path, dir_b: &Path) -> Result<ChangePlan, anyhow::Error> {
    let manifest_a = dir_a.join("manifest.json");
    let manifest_b = dir_b.join("manifest.json");

    let result_a: ScrapeResult = {
        let file = std::fs::File::open(&manifest_a)
            .map_err(|e| anyhow::anyhow!("Cannot read {}: {e}", manifest_a.display()))?;
        serde_json::from_reader(file)
            .map_err(|e| anyhow::anyhow!("Invalid manifest at {}: {e}", manifest_a.display()))?
    };

    let result_b: ScrapeResult = {
        let file = std::fs::File::open(&manifest_b)
            .map_err(|e| anyhow::anyhow!("Cannot read {}: {e}", manifest_b.display()))?;
        serde_json::from_reader(file)
            .map_err(|e| anyhow::anyhow!("Invalid manifest at {}: {e}", manifest_b.display()))?
    };

    let snapshot_a = snapshot_from_scrape(&result_a.base_url, &result_a);
    let snapshot_b = snapshot_from_scrape(&result_b.base_url, &result_b);

    let changes = diff_snapshots(&snapshot_a, &snapshot_b);
    let (added, removed, modified) = count_by_kind(&changes);
    let unchanged = snapshot_b.pages.len() - added - modified;

    Ok(ChangePlan {
        target_url: format!("{} → {}", result_a.base_url, result_b.base_url),
        timestamp: Utc::now(),
        changes,
        summary: ChangeSummary {
            added,
            removed,
            modified,
            unchanged,
            total_current: snapshot_b.pages.len(),
            total_previous: snapshot_a.pages.len(),
        },
        pending_snapshot: snapshot_b,
    })
}

/// Write a change plan to disk as both JSON and Markdown.
///
/// # Errors
///
/// Returns an error if file writing fails.
pub fn write_plan_reports(plan: &ChangePlan, output_dir: &Path) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(output_dir)?;

    let json_path = output_dir.join("change-plan.json");
    let md_path = output_dir.join("change-plan.md");

    let json_content = format_plan_json(plan)?;
    let md_content = format_plan_markdown(plan);

    std::fs::write(&json_path, json_content)?;
    std::fs::write(&md_path, md_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrape::validation::ScrapedPage;

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

        // Same content → same hashes
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
        // Two identical scrapes produce empty plans against the same snapshot
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
    fn test_first_scrape_all_added() {
        let empty = Snapshot {
            target_url: "https://example.com".to_string(),
            timestamp: Utc::now(),
            pages: BTreeMap::new(),
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
}
