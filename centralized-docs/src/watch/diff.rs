//! Diff computation between snapshots and directories.

use chrono::Utc;
use itertools::Itertools;
use std::path::{Path, PathBuf};

use super::{ChangeKind, ChangePlan, ChangeSummary, PageChange, Snapshot};
use crate::scrape::validation::ScrapeResult;

/// Errors from resolving a manifest directory from a user-supplied path.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ManifestResolveError {
    /// Neither `path/manifest.json` nor `path/.scrape/manifest.json` exists.
    #[error(
        "No manifest.json found in '{path}' or '{scrape_subdir}'. \
         Searched:\n  - {direct}\n  - {nested}\n\
         Tip: Run 'ctd scrape --output <DIR>' first, then pass '<DIR>' to this command."
    )]
    NotFound {
        /// The user-supplied path.
        path: PathBuf,
        /// `path/.scrape`
        scrape_subdir: PathBuf,
        /// `path/manifest.json`
        direct: PathBuf,
        /// `path/.scrape/manifest.json`
        nested: PathBuf,
    },
}

/// Resolve a user-supplied path to the directory containing `manifest.json`.
///
/// Checks two candidate locations in order:
/// 1. `path/manifest.json` — direct match (takes precedence)
/// 2. `path/.scrape/manifest.json` — nested match (what `ctd scrape` produces)
///
/// # Errors
///
/// Returns `ManifestResolveError::NotFound` if neither candidate exists.
pub fn resolve_manifest_dir(path: &Path) -> Result<PathBuf, ManifestResolveError> {
    let direct = path.join("manifest.json");
    let scrape_subdir = path.join(".scrape");
    let nested = scrape_subdir.join("manifest.json");

    if direct.is_file() {
        Ok(path.to_path_buf())
    } else if nested.is_file() {
        Ok(scrape_subdir)
    } else {
        Err(ManifestResolveError::NotFound {
            path: path.to_path_buf(),
            scrape_subdir,
            direct,
            nested,
        })
    }
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
            let hash = crate::cache::content_hash(page.markdown.as_bytes());
            let entry = super::PageHash {
                url: page.url.clone(),
                content_hash: hash.into(),
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
    changes
        .iter()
        .fold((0usize, 0usize, 0usize), |(a, r, m), c| match c.kind {
            ChangeKind::Added => (a.saturating_add(1), r, m),
            ChangeKind::Removed => (a, r.saturating_add(1), m),
            ChangeKind::Modified => (a, r, m.saturating_add(1)),
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
    let unchanged = current_snapshot
        .pages
        .len()
        .saturating_sub(added)
        .saturating_sub(modified);

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
    let resolved_a = resolve_manifest_dir(dir_a).map_err(|e| anyhow::anyhow!("{e}"))?;
    let resolved_b = resolve_manifest_dir(dir_b).map_err(|e| anyhow::anyhow!("{e}"))?;
    let manifest_a = resolved_a.join("manifest.json");
    let manifest_b = resolved_b.join("manifest.json");

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
    let unchanged = snapshot_b
        .pages
        .len()
        .saturating_sub(added)
        .saturating_sub(modified);

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
