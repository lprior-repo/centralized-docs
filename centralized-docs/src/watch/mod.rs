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

mod diff;
mod format;

pub use diff::{
    compute_plan, diff_directories, resolve_manifest_dir, snapshot_from_scrape,
    ManifestResolveError,
};
pub use format::{format_plan_json, format_plan_markdown, write_plan_reports};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A content hash for a single page.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct PageHash {
    /// The canonical URL of the page.
    pub url: String,
    /// SHA-256 hash of the page's markdown content.
    pub content_hash: [u8; 32],
    /// The page title (for display in diffs).
    pub title: String,
}

/// rkyv wrapper that serializes [`chrono::DateTime<Utc>`] as an ISO 8601 string.
///
/// This bridges the gap between `chrono` (which does not implement rkyv traits)
/// and the rkyv serialization framework.
pub struct DateTimeWrap;

impl rkyv::with::ArchiveWith<DateTime<Utc>> for DateTimeWrap {
    type Archived = rkyv::string::ArchivedString;
    type Resolver = rkyv::string::StringResolver;

    fn resolve_with(
        field: &DateTime<Utc>,
        resolver: Self::Resolver,
        out: rkyv::Place<Self::Archived>,
    ) {
        let iso = field.to_rfc3339();
        rkyv::string::ArchivedString::resolve_from_str(&iso, resolver, out);
    }
}

impl<S> rkyv::with::SerializeWith<DateTime<Utc>, S> for DateTimeWrap
where
    S: rkyv::rancor::Fallible + ?Sized,
    <S as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    S: rkyv::ser::Writer,
{
    fn serialize_with(
        field: &DateTime<Utc>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as rkyv::rancor::Fallible>::Error> {
        rkyv::string::ArchivedString::serialize_from_str(&field.to_rfc3339(), serializer)
    }
}

impl<D> rkyv::with::DeserializeWith<rkyv::string::ArchivedString, DateTime<Utc>, D> for DateTimeWrap
where
    D: rkyv::rancor::Fallible + ?Sized,
    <D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
{
    fn deserialize_with(
        field: &rkyv::string::ArchivedString,
        _deserializer: &mut D,
    ) -> Result<DateTime<Utc>, <D as rkyv::rancor::Fallible>::Error> {
        let iso = field.as_str();
        chrono::DateTime::parse_from_rfc3339(iso)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                #[derive(Debug)]
                struct ParseError(chrono::ParseError);
                impl std::fmt::Display for ParseError {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(&self.0, f)
                    }
                }
                impl std::error::Error for ParseError {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(&self.0)
                    }
                }
                rkyv::rancor::Source::new(ParseError(e))
            })
    }
}

/// A point-in-time snapshot of all scraped pages.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct Snapshot {
    /// The target URL that was scraped.
    pub target_url: String,
    /// When this snapshot was taken.
    #[rkyv(with = DateTimeWrap)]
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
    pub old_hash: Option<[u8; 32]>,
    /// New content hash (None for Removed).
    pub new_hash: Option<[u8; 32]>,
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

#[cfg(test)]
mod tests_diff;
#[cfg(test)]
mod tests_format;
#[cfg(test)]
mod tests_resolve_manifest;
