bead_id: cdocs-90e
bead_title: action: load archived scrape outputs for unchanged pages and skip downstream stages
phase: state-1-contract
updated_at: 2026-04-03T00:00:00Z

# Contract Specification

## Context

### Feature
Load archived scrape outputs for unchanged pages from the redb state database and
skip CPU-heavy downstream stages (validation, analysis, transform, chunking) for those
pages. Changed and new pages continue through the full processing pipeline.

### Domain Terms

| Term | Definition |
|------|-----------|
| `UrlStateRaw` | 120-byte Pod struct storing `content_hash`, `url_hash` (FK to `scrape_outputs`), `last_fetched_secs`, `status_code`. Keyed by canonical URL string in the `url_state` redb table. |
| `scrape_outputs` | redb table: `[u8; 32]` hash key -> rkyv-archived `PersistedScrapeResult` bytes. |
| `PersistedScrapeResult` | Archived batch scrape result containing `Vec<PersistedScrapedPage>`, `total_urls`, `success_count`, `error_count`, `errors`, `base_url`, `schema_version`. |
| `ScrapedPage` | Runtime scraped page: `url`, `markdown`, `title`, `links`, `headers`, `word_count`, `slug`, `filter_status`, `elements_removed`, `density_score`. |
| `StateReadSession` | RAII guard holding one shared `redb::ReadTransaction`. All bulk loaders operate within this single transaction. |
| `OwnedArchive<T>` | Owned wrapper around rkyv-archived bytes, validated via bytecheck at construction. Decouples the archived view from the redb transaction lifetime. |
| `PageHash` | Content fingerprint: `{ url, content_hash: [u8; 32], title }`. Used by the watch module for snapshot comparison. |
| `ChangeKind` | Page classification: `Added`, `Modified`, `Removed`. Computed by comparing current scrape content hashes against the stored snapshot. |
| `ScrapeReuseStats` | Statistics struct for reuse accounting: `reused: usize`, `scraped: usize`. |

### Assumptions

1. The network fetch still occurs for all pages (current spider-rs limitation). The
   classification into unchanged/changed happens *after* the scrape, by comparing
   scraped content hashes against stored `UrlStateRaw.content_hash` values.
2. Unchanged-page handling is driven by stored scrape hashes in `UrlStateRaw`.
   The `url_hash` field is the FK into the `scrape_outputs` table.
3. The existing `analyze_reuse` module (bead cdocs-b5h) provides the canonical
   pattern for archive loading, partitioning, and merge-in-order. This bead
   mirrors that architecture for the scrape stage.
4. `PersistedScrapeResult` stores a batch of pages. The archive is keyed by the
   scrape batch hash (the `url_hash` in `UrlStateRaw`), NOT by individual page
   content hashes. A single batch may contain many pages.
5. The `ingest` command (`cmd/ingest.rs`) is the primary integration point: it
   runs `scrape_site`, then `run_index`. The classification + reuse logic inserts
   between the scrape and the downstream stages.

### Open Questions

None resolved. The contract proceeds based on the above assumptions.

## Preconditions

- **PRE-1**: The state database (`state.redb`) exists and is openable at the
  configured output path.
- **PRE-2**: A `StateReadSession` has been successfully created (read transaction
  is live).
- **PRE-3**: `url_state` table exists and is readable (initialized by a previous
  run or by `initialize_tables`).
- **PRE-4**: `scrape_outputs` table exists and is readable.
- **PRE-5**: A fresh `ScrapeResult` has been obtained from `scrape_site` (the
  network fetch has completed for all pages).
- **PRE-6**: For each page in the fresh `ScrapeResult`, the current content hash
  has been computed (SHA-256 of the scraped markdown content).
- **PRE-7**: The stored `UrlStateRaw` rows are valid (120-byte Pod structs with
  correct layout -- guaranteed by `StateReadSession::load_url_states`).

## Postconditions

- **POST-1**: Every page in the fresh scrape result appears in the final output
  exactly once -- either loaded from archive (unchanged) or from the fresh scrape
  (changed/new).
- **POST-2**: Unchanged pages are loaded from `scrape_outputs` via `url_hash` FK
  lookup and are *never* re-processed through validation, analysis, transform, or
  chunking stages.
- **POST-3**: Changed and new pages pass through the full downstream pipeline
  (validation, analysis, transform, chunking) using the freshly scraped content.
- **POST-4**: The final page list preserves the order of the fresh scrape result
  (discovery/crawl order).
- **POST-5**: `ScrapeReuseStats.reused + ScrapeReuseStats.scraped ==
  fresh_scrape_result.pages.len()` (every page accounted for).
- **POST-6**: For each reused page, the loaded `ScrapedPage.url` matches the
  corresponding fresh page's URL (FK integrity verified).
- **POST-7**: For each reused page, the stored `content_hash` in `UrlStateRaw`
  matches the freshly computed content hash (hash consistency verified).

## Invariants

- **INV-1**: No `.unwrap()` or `.expect()` on production paths. All fallible
  operations return `Result<T, Error>`.
- **INV-2**: The `StateReadSession` is the sole I/O boundary. All database reads
  flow through its bulk loader methods (`load_scrapes`, `load_url_states`).
- **INV-3**: Archive loading is fail-fast: the first corrupt `PersistedScrapeResult`
  stops the entire load for that batch (matching `I-05` from `bulk_load.rs`).
- **INV-4**: Individual page deserialization failures within a successfully loaded
  batch are non-fatal: the affected page falls back to fresh processing.
- **INV-5**: The classification partition is mutually exclusive and collectively
  exhaustive: every page is either `Unchanged` or `ChangedOrNew`.
- **INV-6**: Zero hash (`[0u8; 32]`) in `UrlStateRaw.url_hash` means "never
  archived" -- such pages are always treated as changed/new (fallback to fresh).
- **INV-7**: Missing `url_state` entry for a URL means the page was never seen
  before -- treated as new (fallback to fresh).
- **INV-8**: Hash mismatch between fresh content hash and stored `content_hash`
  means the page changed -- treated as changed (fresh processing).
- **INV-9**: The module-level lint directives (`deny(unwrap_used)`,
  `deny(expect_used)`, `deny(panic)`, `forbid(unsafe_code)`) are preserved.

## Error Taxonomy

```rust
/// Error type for the scrape-reuse pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ScrapeReuseError {
    /// Failed to load URL states from the state database.
    #[error("failed to load url states: {0}")]
    StateLoad(#[from] StateLoadError),

    /// Failed to load archived scrape outputs from the state database.
    #[error("failed to load archived scrape outputs: {0}")]
    BulkLoad(#[from] BulkLoadError),

    /// A loaded PersistedScrapeResult failed schema validation or deserialization.
    #[error("failed to deserialize archived scrape output for url_hash {key_hex}: {message}")]
    DeserializationFailed {
        /// Hex-encoded key of the corrupt archive.
        key_hex: String,
        /// Error description from rkyv or schema validation.
        message: String,
    },

    /// Hash integrity violation: loaded page's content_hash does not match
    /// the stored UrlStateRaw.content_hash for the same URL.
    #[error("hash mismatch for '{url}': stored={stored_hex}, loaded={loaded_hex}")]
    HashMismatch {
        /// URL of the affected page.
        url: String,
        /// Hex-encoded stored content hash from UrlStateRaw.
        stored_hex: String,
        /// Hex-encoded content hash from the loaded scrape page.
        loaded_hex: String,
    },

    /// No url_state entry exists for a URL that was expected to be unchanged.
    /// This should not occur if classification is correct; indicates a logic bug.
    #[error("missing url_state for expected-unchanged URL '{url}'")]
    MissingUrlState {
        /// URL with no url_state entry.
        url: String,
    },
}
```

### Error Variant Semantics

| Variant | Severity | Recovery |
|---------|----------|----------|
| `StateLoad` | Fatal | Propagate -- database is unreadable, cannot proceed. |
| `BulkLoad` | Fatal | Propagate -- archive table is unreadable, cannot proceed. |
| `DeserializationFailed` | Per-batch fatal | The entire batch containing the corrupt archive is treated as fallback. Individual pages from that batch are re-processed fresh. |
| `HashMismatch` | Per-page error | The affected page falls back to fresh processing. Logged as warning. |
| `MissingUrlState` | Logic bug | The affected page falls back to fresh processing. Logged as warning. Should never occur if classification is correct. |

## Contract Signatures

### Classification (Pure Calculation)

```rust
/// Compute the content hash of a scraped page's markdown.
/// Pure function: deterministic, no I/O.
fn compute_page_content_hash(markdown: &str) -> [u8; 32];

/// Classify each scraped page as Unchanged or ChangedOrNew by comparing
/// its content hash against the stored UrlStateRaw.
///
/// Pure calculation: no I/O, no errors.
///
/// Returns a partition of page indices:
/// - `unchanged`: indices into `fresh_pages` where content hash matches stored state
/// - `changed_or_new`: all other indices (missing state, hash mismatch, zero hash)
fn classify_scraped_pages(
    fresh_pages: &[ScrapedPage],
    fresh_hashes: &[[u8; 32]],
    url_states: &HashMap<String, UrlStateRaw>,
) -> ScrapePageDiff;
```

### Archive Loading (I/O Action)

```rust
/// Load archived scrape outputs for unchanged pages.
///
/// For each unchanged page, looks up its UrlStateRaw.url_hash and loads the
/// corresponding PersistedScrapeResult from the scrape_outputs table.
/// Deserializes individual pages and verifies hash integrity.
///
/// # Arguments
/// * `page_diff` - Classification result with unchanged page indices.
/// * `fresh_pages` - The freshly scraped pages (for URL lookup).
/// * `url_states` - Loaded URL state entries (provides url_hash and content_hash).
/// * `session` - Shared read session for archive access.
///
/// # Returns
/// * `HashMap<usize, ScrapedPage>` - Index into fresh_pages -> loaded archived page
/// * `Vec<usize>` - Page indices that failed to load (fallback to fresh)
fn load_archived_scrape_pages(
    page_diff: &ScrapePageDiff,
    fresh_pages: &[ScrapedPage],
    url_states: &HashMap<String, UrlStateRaw>,
    session: &StateReadSession<'_>,
) -> Result<(HashMap<usize, ScrapedPage>, Vec<usize>), ScrapeReuseError>;
```

### Merge (Pure Calculation)

```rust
/// Merge reused archived pages and fresh pages into a single vec in crawl order.
///
/// For each position in the original fresh_pages list:
/// - If the index is in `archived_pages`, use the archived version.
/// - Otherwise, use the fresh version.
///
/// Pure calculation: no I/O, no errors.
///
/// # Postconditions
/// - Output vec length == fresh_pages length.
/// - Output order matches fresh_pages order.
fn merge_scrape_pages_in_order(
    fresh_pages: Vec<ScrapedPage>,
    archived_pages: HashMap<usize, ScrapedPage>,
) -> Vec<ScrapedPage>;
```

### Primary Entry Point

```rust
/// Classify scraped pages, load archived outputs for unchanged pages, and
/// merge into a final page list with reuse statistics.
///
/// # Arguments
/// * `fresh_result` - The freshly scraped result from scrape_site.
/// * `session` - Shared read session for state database access.
///
/// # Errors
/// Returns `ScrapeReuseError` for state database failures or archive corruption.
///
/// # Guarantees
/// - Every page in fresh_result appears in the output exactly once.
/// - Output order matches input order.
/// - Unchanged pages are loaded from archive, not re-processed.
fn scrape_with_reuse(
    fresh_result: ScrapeResult,
    session: &StateReadSession<'_>,
) -> Result<(ScrapeResult, ScrapeReuseStats), ScrapeReuseError>;
```

### Domain Types (Data Layer)

```rust
/// Partition of scraped page indices into unchanged vs changed-or-new.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapePageDiff {
    /// Indices into the original pages vec for unchanged pages.
    pub unchanged: Vec<usize>,
    /// Indices for changed or new pages.
    pub changed_or_new: Vec<usize>,
}

/// Statistics about scrape reuse within a single command invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapeReuseStats {
    /// Number of pages loaded from archived state (zero-cost reuse).
    pub reused: usize,
    /// Number of pages freshly scraped and processed through the pipeline.
    pub scraped: usize,
}
```

## Non-goals

- **NG-1**: Eliminating network fetches for unchanged pages. The current
  spider-rs architecture requires fetching all pages. This bead optimizes
  *downstream processing*, not network I/O.
- **NG-2**: Modifying the scrape classification logic in the `watch` module.
  The watch/snapshot/ChangePlan system remains unchanged.
- **NG-3**: Implementing the full downstream pipeline integration. This
  contract covers only the classification, archive loading, and merge steps.
  The caller (`ingest` or `index` command) is responsible for branching
  unchanged pages around the downstream stages.
- **NG-4**: Modifying `UrlStateRaw`, `PersistedScrapeResult`, or the redb
  table schemas. This bead works with existing types.
- **NG-5**: Per-page archive granularity. Currently `PersistedScrapeResult`
  stores a batch. This bead loads the full batch and extracts individual
  pages. A future bead may refactor to per-page archival.
