# Contract Specification: watch.rs — Terraform-style Plan/Apply Workflow

## Context

- **Feature:** Documentation change tracking with plan/apply workflow for `centralized-docs`
- **Module:** `centralized-docs/src/watch.rs`
- **Domain terms:**
  - `Snapshot` — a point-in-time capture of all scraped pages as URL→PageHash map
  - `PageHash` — xxh3_128 content hash + metadata for a single page
  - `ChangeKind` — enum: `Added | Removed | Modified`
  - `PageChange` — a single page-level diff entry between two snapshots
  - `ChangePlan` — the complete diff (the "terraform plan") showing what *would* change
  - `ChangeSummary` — aggregate counts of added/removed/modified/unchanged
  - `ScrapeResult` — the raw output from the scraper (external dependency from `crate::scrape::validation`)
  - `apply` — commit a pending snapshot to storage (outside this module's scope; this module only builds the plan)
- **Assumptions:**
  - `ScrapeResult.pages` uses canonical URLs as unique keys (no duplicate URLs per scrape)
  - `crate::cache::hash::content_hash` is deterministic: same input bytes → same u128 output
  - Stored snapshots are persisted externally; this module only computes plans and writes reports
- **Open questions:**
  - None — the API surface is fully defined in watch.rs

---

## Preconditions

| ID | Function | Precondition |
|----|----------|--------------|
| P1 | `snapshot_from_scrape` | `result.pages` must contain no duplicate URLs (caller invariant) |
| P2 | `compute_plan` | `previous` is a valid snapshot previously produced by `snapshot_from_scrape` |
| P3 | `compute_plan` | `current_scrape` is a valid `ScrapeResult` (pages have non-empty URL fields) |
| P4 | `diff_directories` | Both `dir_a` and `dir_b` exist and contain readable `manifest.json` files |
| P5 | `diff_directories` | Each `manifest.json` is a valid `ScrapeResult` (deserializable) |
| P6 | `write_plan_reports` | `output_dir` is writable (parent exists and has permissions) |
| P7 | `format_plan_json` | `plan` is a valid `ChangePlan` (no panicking fields) |

## Postconditions

| ID | Function | Postcondition |
|----|----------|---------------|
| Q1 | `snapshot_from_scrape` | Returned `Snapshot.pages` has exactly one entry per URL in `result.pages` |
| Q2 | `snapshot_from_scrape` | Every `PageHash.content_hash` is the xxh3_128 of its page's markdown bytes |
| Q3 | `snapshot_from_scrape` | `Snapshot.target_url` equals the input `target_url` |
| Q4 | `compute_plan` | Every URL present in `current_scrape` but absent in `previous` appears as `ChangeKind::Added` |
| Q5 | `compute_plan` | Every URL present in `previous` but absent in `current_scrape` appears as `ChangeKind::Removed` |
| Q6 | `compute_plan` | Every URL present in both with differing hashes appears as `ChangeKind::Modified` |
| Q7 | `compute_plan` | Every URL present in both with identical hashes has NO entry in `changes` |
| Q8 | `compute_plan` | `summary.added` + `summary.removed` + `summary.modified` + `summary.unchanged` = `total_current` |
| Q9 | `compute_plan` | `pending_snapshot` contains exactly the pages from `current_scrape` |
| Q10 | `compute_plan` | `plan.changes` is sorted: first by `ChangeKind` (added, removed, modified), then by URL |
| Q11 | `diff_directories` | Returns `Ok(ChangePlan)` when both manifests are valid, `Err` otherwise |
| Q12 | `write_plan_reports` | Creates `change-plan.json` and `change-plan.md` in `output_dir` |
| Q13 | `write_plan_reports` | `change-plan.json` is a valid JSON representation of the plan |
| Q14 | `format_plan_markdown` | Output contains "# Documentation Change Plan" header |
| Q15 | `format_plan_json` | Output deserializes back to `ChangePlan` (roundtrip) |

## Invariants

| ID | Invariant |
|----|-----------|
| I1 | **Hash determinism:** `snapshot_from_scrape` called twice with the same `ScrapeResult` produces snapshots with identical page hashes for every URL |
| I2 | **Idempotency:** `compute_plan` called twice with the same `previous` and `current_scrape` produces identical plans |
| I3 | **Pure calculation:** `snapshot_from_scrape`, `compute_plan`, and `diff_snapshots` perform zero I/O — they are referentially transparent |
| I4 | **No mutation:** `compute_plan` does not modify `previous` or `current_scrape`; it only reads them |
| I5 | **Empty plan idempotency:** If `current_scrape` pages are identical to `previous` pages (same URLs, same hashes), `plan.changes` is empty and `plan.summary.is_empty()` is `true` |
| I6 | **First scrape completeness:** When `previous.pages` is empty, every page in `current_scrape` is `ChangeKind::Added` and `summary.removed == 0` |
| I7 | **Total removal completeness:** When `current_scrape.pages` is empty, every page in `previous` is `ChangeKind::Removed` and `summary.added == 0` |
| I8 | **Summary conservation:** `summary.total_current == pending_snapshot.pages.len()` always |
| I9 | **Change consistency:** For every `PageChange`:
  - `Added` → `old_hash == None`, `new_hash == Some(_)`
  - `Removed` → `old_hash == Some(_)`, `new_hash == None`
  - `Modified` → `old_hash != None`, `new_hash != None`, `old_hash != new_hash` |
| I10 | **BTreeMap ordering:** `Snapshot.pages` is a `BTreeMap`, so iteration order is deterministic (sorted by URL) |
| I11 | **Serialization roundtrip:** `Snapshot`, `ChangePlan`, and `ChangeSummary` survive serde serialize→deserialize→compare |

---

## Error Taxonomy

| Variant | Source Function | Condition |
|---------|----------------|-----------|
| `Error::Io(path, reason)` | `diff_directories`, `write_plan_reports` | File not found, permission denied, disk full |
| `Error::Deserialization(path, detail)` | `diff_directories` | `manifest.json` is not valid JSON or schema mismatch with `ScrapeResult` |
| `Error::Serialization(detail)` | `format_plan_json` | serde serialization fails (theoretically impossible for valid `ChangePlan`) |

**Note:** `snapshot_from_scrape`, `compute_plan`, and `diff_snapshots` are infallible (`#[must_use]`, no `Result`). All errors are I/O boundary errors.

---

## Contract Signatures

```rust
// Pure calculation — infallible
#[must_use]
pub fn snapshot_from_scrape(target_url: &str, result: &ScrapeResult) -> Snapshot;

#[must_use]
pub fn compute_plan(target_url: &str, previous: &Snapshot, current_scrape: &ScrapeResult) -> ChangePlan;

// Pure diff — infallible
fn diff_snapshots(previous: &Snapshot, current: &Snapshot) -> Vec<PageChange>;

// Formatting — serialization can fail
pub fn format_plan_json(plan: &ChangePlan) -> Result<String, serde_json::Error>;

// Pure formatting — infallible
#[must_use]
pub fn format_plan_markdown(plan: &ChangePlan) -> String;

// I/O boundary — can fail
pub fn diff_directories(dir_a: &Path, dir_b: &Path) -> Result<ChangePlan, anyhow::Error>;

// I/O boundary — can fail
pub fn write_plan_reports(plan: &ChangePlan, output_dir: &Path) -> Result<(), anyhow::Error>;
```

### Method signatures (on types)

```rust
impl ChangeSummary {
    #[must_use]
    pub fn is_empty(&self) -> bool;
}

impl std::fmt::Display for ChangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
```

---

## Non-goals

- This module does NOT perform scraping (that is `crate::scrape`)
- This module does NOT persist snapshots to disk (that is the caller / `apply` command)
- This module does NOT perform network I/O
- This module does NOT manage a snapshot database or version history
- Change detection is URL-keyed: if a page moves to a new URL with the same content, it appears as Remove+Add, not as a rename
