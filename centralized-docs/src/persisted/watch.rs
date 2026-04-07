//! Persisted types and conversions for the Watch/Snapshot pipeline phases.

use super::error::{require_schema_v1, PersistError};
use crate::watch::{ChangeKind, ChangePlan, ChangeSummary, PageChange, PageHash, Snapshot};
use chrono::{TimeZone, Utc};

// ---------------------------------------------------------------------------
// Persisted Record Types — Watch/Snapshot Family
// ---------------------------------------------------------------------------

/// Persisted page hash: URL + SHA-256 content hash + title.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageHash {
    /// Canonical URL of the page.
    pub url: String,
    /// SHA-256 hash of the page's markdown content.
    pub content_hash: [u8; 32],
    /// Page title for display.
    pub title: String,
}

/// Persisted change kind: Added / Modified / Removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChangeKind {
    /// Page appeared that wasn't in the previous snapshot.
    Added,
    /// Page content hash changed.
    Modified,
    /// Page disappeared from the current scrape.
    Removed,
}

/// Persisted page-level change between two snapshots.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageChange {
    /// URL that changed.
    pub url: String,
    /// Kind of change.
    pub kind: PersistedChangeKind,
    /// Previous content hash (None for Added).
    pub old_hash: Option<[u8; 32]>,
    /// New content hash (None for Removed).
    pub new_hash: Option<[u8; 32]>,
    /// Page title for display.
    pub title: String,
}

/// Persisted change summary counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangeSummary {
    /// Pages added.
    pub added: usize,
    /// Pages removed.
    pub removed: usize,
    /// Pages modified.
    pub modified: usize,
    /// Pages unchanged.
    pub unchanged: usize,
    /// Total pages in current snapshot.
    pub total_current: usize,
    /// Total pages in previous snapshot.
    pub total_previous: usize,
}

/// Persisted point-in-time snapshot of all scraped pages.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedSnapshot {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Target URL that was scraped.
    pub target_url: String,
    /// Unix epoch seconds (replaces `DateTime<Utc>`).
    pub timestamp_secs: i64,
    /// Page hashes sorted by URL key for deterministic serialization.
    pub pages: Vec<(String, PersistedPageHash)>,
}

/// Persisted change plan (Terraform-style plan/apply model).
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangePlan {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Target URL being watched.
    pub target_url: String,
    /// Unix epoch seconds (replaces `DateTime<Utc>`).
    pub timestamp_secs: i64,
    /// All detected changes.
    pub changes: Vec<PersistedPageChange>,
    /// Summary counts.
    pub summary: PersistedChangeSummary,
    /// The new snapshot (ready to be applied).
    pub pending_snapshot: PersistedSnapshot,
}

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

/// Convert a runtime [`PageHash`] to its persisted form.
#[must_use]
pub fn page_hash_to_persisted(p: &PageHash) -> PersistedPageHash {
    PersistedPageHash {
        url: p.url.clone(),
        content_hash: p.content_hash,
        title: p.title.clone(),
    }
}

/// Convert a runtime [`ChangeKind`] to its persisted form.
#[must_use]
pub fn change_kind_to_persisted(k: &ChangeKind) -> PersistedChangeKind {
    match k {
        ChangeKind::Added => PersistedChangeKind::Added,
        ChangeKind::Modified => PersistedChangeKind::Modified,
        ChangeKind::Removed => PersistedChangeKind::Removed,
    }
}

/// Convert a runtime [`PageChange`] to its persisted form.
#[must_use]
pub fn page_change_to_persisted(p: &PageChange) -> PersistedPageChange {
    PersistedPageChange {
        url: p.url.clone(),
        kind: change_kind_to_persisted(&p.kind),
        old_hash: p.old_hash,
        new_hash: p.new_hash,
        title: p.title.clone(),
    }
}

/// Convert a runtime [`ChangeSummary`] to its persisted form.
#[must_use]
pub fn change_summary_to_persisted(s: &ChangeSummary) -> PersistedChangeSummary {
    PersistedChangeSummary {
        added: s.added,
        removed: s.removed,
        modified: s.modified,
        unchanged: s.unchanged,
        total_current: s.total_current,
        total_previous: s.total_previous,
    }
}

/// Convert a runtime [`Snapshot`] to its persisted form.
///
/// `DateTime<Utc>` is converted to unix epoch seconds (i64).
/// Pages are sorted by URL key for deterministic serialization.
#[must_use]
pub fn snapshot_to_persisted(s: &Snapshot) -> PersistedSnapshot {
    PersistedSnapshot {
        schema_version: 1,
        target_url: s.target_url.clone(),
        timestamp_secs: s.timestamp.timestamp(),
        pages: s
            .pages
            .iter()
            .map(|(url, ph)| (url.clone(), page_hash_to_persisted(ph)))
            .collect(),
    }
}

/// Convert a runtime [`ChangePlan`] to its persisted form.
#[must_use]
pub fn change_plan_to_persisted(p: &ChangePlan) -> PersistedChangePlan {
    PersistedChangePlan {
        schema_version: 1,
        target_url: p.target_url.clone(),
        timestamp_secs: p.timestamp.timestamp(),
        changes: p.changes.iter().map(page_change_to_persisted).collect(),
        summary: change_summary_to_persisted(&p.summary),
        pending_snapshot: snapshot_to_persisted(&p.pending_snapshot),
    }
}

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

/// Convert a persisted page hash back to runtime form.
pub fn persisted_page_hash_to_runtime(p: &PersistedPageHash) -> Result<PageHash, PersistError> {
    Ok(PageHash {
        url: p.url.clone(),
        content_hash: p.content_hash,
        title: p.title.clone(),
    })
}

/// Convert a persisted change kind back to runtime form (1:1 mapping).
pub fn persisted_change_kind_to_runtime(
    p: PersistedChangeKind,
) -> Result<ChangeKind, PersistError> {
    match p {
        PersistedChangeKind::Added => Ok(ChangeKind::Added),
        PersistedChangeKind::Modified => Ok(ChangeKind::Modified),
        PersistedChangeKind::Removed => Ok(ChangeKind::Removed),
    }
}

/// Convert a persisted page change back to runtime form.
pub fn persisted_page_change_to_runtime(
    p: &PersistedPageChange,
) -> Result<PageChange, PersistError> {
    Ok(PageChange {
        url: p.url.clone(),
        kind: persisted_change_kind_to_runtime(p.kind)?,
        old_hash: p.old_hash,
        new_hash: p.new_hash,
        title: p.title.clone(),
    })
}

/// Convert a persisted change summary back to runtime form (direct field copy).
pub fn persisted_change_summary_to_runtime(
    p: &PersistedChangeSummary,
) -> Result<ChangeSummary, PersistError> {
    Ok(ChangeSummary {
        added: p.added,
        removed: p.removed,
        modified: p.modified,
        unchanged: p.unchanged,
        total_current: p.total_current,
        total_previous: p.total_previous,
    })
}

/// Convert a persisted snapshot back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Returns [`PersistError::DeserializationFailed`] if `timestamp_secs` is invalid.
/// Propagates any errors from nested page hash conversions.
pub fn persisted_snapshot_to_runtime(p: &PersistedSnapshot) -> Result<Snapshot, PersistError> {
    require_schema_v1(p.schema_version)?;
    let timestamp = Utc
        .timestamp_opt(p.timestamp_secs, 0)
        .single()
        .ok_or_else(|| PersistError::DeserializationFailed {
            reason: format!("invalid unix timestamp: {}", p.timestamp_secs),
        })?;
    let pages = p
        .pages
        .iter()
        .map(|(url, ph)| persisted_page_hash_to_runtime(ph).map(|h| (url.clone(), h)))
        .collect::<Result<std::collections::BTreeMap<String, PageHash>, _>>()?;
    Ok(Snapshot {
        target_url: p.target_url.clone(),
        timestamp,
        pages,
    })
}

/// Convert a persisted change plan back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested change/snapshot conversions.
pub fn persisted_change_plan_to_runtime(
    p: &PersistedChangePlan,
) -> Result<ChangePlan, PersistError> {
    require_schema_v1(p.schema_version)?;
    let timestamp = Utc
        .timestamp_opt(p.timestamp_secs, 0)
        .single()
        .ok_or_else(|| PersistError::DeserializationFailed {
            reason: format!("invalid unix timestamp: {}", p.timestamp_secs),
        })?;
    let changes = p
        .changes
        .iter()
        .map(persisted_page_change_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    let summary = persisted_change_summary_to_runtime(&p.summary)?;
    let pending_snapshot = persisted_snapshot_to_runtime(&p.pending_snapshot)?;
    Ok(ChangePlan {
        target_url: p.target_url.clone(),
        timestamp,
        changes,
        summary,
        pending_snapshot,
    })
}
