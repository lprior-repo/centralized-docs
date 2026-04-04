# Contract Specification

```
bead_id: cdocs-r4f
bead_title: calc: build URL-state and scrape-output commit batches from scrape results
phase: state-1-contract
updated_at: 2026-04-03T23:40:00Z
```

## Context

- **Feature**: cdocs-r4f -- Build `StateChanges` batches from scrape classification results and
  processed scrape outputs. Mirrors the existing file-state batch builder (`build_file_state_changes`
  in `calc/build_state_changes.rs`) but for the URL-state domain.
- **Module**: `centralized-docs/src/calc/` (new file `build_scrape_state_changes.rs`, re-exported from
  `mod.rs`)
- **Domain terms**:
  - **ScrapeDiff**: Partition of scraped URLs into unchanged, changed, new, and deleted buckets.
    Analogous to `FileDiff` in `diff.rs`. Every URL appears in exactly one bucket.
  - **UrlStateRaw**: 120-byte `#[repr(C)]` fixed-size Pod struct (defined in `state/mod.rs`).
    Fields: `content_hash: [u8; 32]`, `url_hash: [u8; 32]` (FK -> `scrape_outputs` key),
    `last_fetched_secs: u64`, `status_code: u16`, `reserved: [u8; 46]`.
  - **StateChanges**: Batch of mutations consumed by `StateDb::commit_changes` (defined in
    `state/commit.rs`). Fields relevant to this bead: `updated_urls`, `deleted_urls`,
    `new_scrapes`. All other fields left empty (`vec![]`).
  - **PersistedScrapedPage**: rkyv-archivable scrape output (defined in `persisted.rs`).
    Stored in `scrape_outputs` table keyed by SHA-256 of rkyv bytes.
  - **content_hash (URL context)**: SHA-256 of a page's markdown content bytes. Used in
    `UrlStateRaw.content_hash` and in classification to detect unchanged pages.
  - **url_hash**: SHA-256 of a URL string. Used as the FK in `UrlStateRaw.url_hash` pointing
    to the `scrape_outputs` table key (which is the SHA-256 of the rkyv-serialized payload).
  - **ScrapeOutputs**: HashMap of processed scrape artifacts keyed by URL string. Contains the
    serialized scrape payload bytes for each changed/new page.
- **Assumptions**:
  - The crate uses `#![forbid(unsafe_code)]`, `#![deny(clippy::unwrap_used)]`,
    `#![deny(clippy::expect_used)]`, `#![deny(clippy::panic)]`.
  - `UrlStateRaw` (120 bytes) is already defined in `state/mod.rs` with `from_bytes`/`to_bytes`.
  - `StateChanges` is already defined in `state/commit.rs` with `empty()` constructor.
  - `commit_changes` in `state/commit.rs` enforces reference integrity: every non-zero
    `url_hash` in `updated_urls` must have a corresponding entry in `new_scrapes`.
  - Scrape classification (unchanged/changed/new/deleted) is computed upstream and passed in
    as a `ScrapeDiff` input. This bead only builds the batch -- it does not classify.
  - `hash_payload` and `serialize_and_hash` from `calc/build_state_changes.rs` can be reused.
  - Serialization uses `serde_json::to_vec` (consistent with existing `build_file_state_changes`).
- **Open questions**: None. All types and patterns are established in the codebase.

---

## Types (Contract Signatures)

### `ScrapeDiff`

```rust
/// Partition of scraped URLs into unchanged, changed, new, and deleted buckets.
/// Every URL appears in exactly one bucket.
/// Union of all buckets == union of current-scrape URLs and stored-URL keys.
/// Intersection of any two distinct buckets == empty set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapeDiff {
    /// URLs whose markdown content hash matches the stored state (no rewrite needed).
    pub unchanged: Vec<String>,
    /// URLs present in both current scrape and stored state, but with different content hash.
    pub changed: Vec<String>,
    /// URLs present in current scrape but not in stored state.
    pub new_urls: Vec<String>,
    /// URLs present in stored state but not in current scrape.
    pub deleted: Vec<String>,
}
```

### `ScrapeOutputs`

```rust
/// Processed scrape artifacts keyed by URL string.
/// Contains the rkyv-serializable scrape payload for each changed/new page.
#[derive(Debug, Clone)]
pub struct ScrapeOutputs {
    /// Map of URL -> (content_hash, status_code, scrape_payload_bytes).
    /// `content_hash` is SHA-256 of the page's markdown bytes.
    /// `status_code` is the HTTP status code from the scrape.
    /// `scrape_payload_bytes` is the serialized `PersistedScrapedPage`.
    pub artifacts: HashMap<String, ScrapeArtifact>,
}

/// Single page's processed scrape artifact.
#[derive(Debug, Clone)]
pub struct ScrapeArtifact {
    /// SHA-256 of the page's markdown content.
    pub content_hash: [u8; 32],
    /// HTTP status code from the scrape (e.g., 200).
    pub status_code: u16,
    /// Serialized scrape payload bytes (ready for storage in scrape_outputs table).
    pub payload_bytes: Vec<u8>,
}
```

### `ScrapeBatchConfig`

```rust
/// Configuration for the scrape batch builder.
#[derive(Debug, Clone)]
pub struct ScrapeBatchConfig {
    /// Unix timestamp (seconds) for `last_fetched_secs` in `UrlStateRaw`.
    pub now_secs: u64,
}
```

### `ScrapeBatchBuildError`

```rust
/// Exhaustive error taxonomy for `build_scrape_state_changes`.
#[derive(Debug, thiserror::Error)]
pub enum ScrapeBatchBuildError {
    /// A changed or new URL has no corresponding scrape artifact.
    #[error("missing scrape artifact for URL: {url}")]
    MissingScrapeArtifact { url: String },

    /// A changed or new URL's artifact has zero-length payload bytes.
    #[error("empty scrape payload for URL: {url}")]
    EmptyScrapePayload { url: String },

    /// Serialization of a scrape payload failed (should not happen since bytes are pre-serialized,
    /// but defensive check for hash computation).
    #[error("scrape payload processing failed for URL {url}: {reason}")]
    PayloadProcessingFailed { url: String, reason: String },

    /// A URL appears in more than one diff category.
    #[error("duplicate URL in scrape diff: {url} appears in multiple categories")]
    DuplicateUrl { url: String },

    /// The input ScrapeDiff was empty (no URLs in any category).
    #[error("scrape diff is empty: no unchanged, changed, new, or deleted URLs")]
    EmptyDiff,
}
```

### `build_scrape_state_changes`

```rust
/// Build a deterministic URL-state change batch from scrape classification results
/// and processed scrape outputs.
///
/// Changed and new URLs produce updated `UrlStateRaw` rows and scrape payload blobs.
/// Deleted URLs produce only delete entries. Unchanged URLs are not rewritten.
///
/// # Errors
///
/// Returns `Err(ScrapeBatchBuildError)` if preconditions are violated:
/// - `EmptyDiff` when all four categories are empty
/// - `DuplicateUrl` when a URL appears in multiple categories
/// - `MissingScrapeArtifact` when a changed or new URL has no artifact
/// - `EmptyScrapePayload` when an artifact's payload is zero-length
/// - `PayloadProcessingFailed` when payload processing fails
pub fn build_scrape_state_changes(
    diff: &ScrapeDiff,
    outputs: &ScrapeOutputs,
    config: &ScrapeBatchConfig,
) -> Result<StateChanges, ScrapeBatchBuildError>;
```

### `build_url_state_raw`

```rust
/// Construct a `UrlStateRaw` from individual hash and timestamp components.
///
/// All fields are set to the provided values. `reserved` is zeroed.
/// Total struct size is exactly 120 bytes.
#[must_use]
pub fn build_url_state_raw(
    content_hash: [u8; 32],
    url_hash: [u8; 32],
    last_fetched_secs: u64,
    status_code: u16,
) -> UrlStateRaw;
```

---

## Preconditions

1. **PRE-1 -- diff is non-empty**: At least one of `diff.unchanged`, `diff.changed`,
   `diff.new_urls`, or `diff.deleted` must be non-empty. Violation returns
   `ScrapeBatchBuildError::EmptyDiff`.

2. **PRE-2 -- no duplicate URLs across categories**: No URL string may appear in more than one
   of the four `ScrapeDiff` buckets. Violation returns `ScrapeBatchBuildError::DuplicateUrl`
   with the offending URL.

3. **PRE-3 -- all changed URLs have artifacts**: For every URL in `diff.changed` and
   `diff.new_urls`, `outputs.artifacts` must contain a corresponding `ScrapeArtifact` entry.
   Violation returns `ScrapeBatchBuildError::MissingScrapeArtifact`.

4. **PRE-4 -- all artifacts have non-empty payload**: Every `ScrapeArtifact` referenced by a
   changed or new URL must have `payload_bytes.len() > 0`. Violation returns
   `ScrapeBatchBuildError::EmptyScrapePayload`.

5. **PRE-5 -- now_secs is valid**: `config.now_secs` is a Unix timestamp (no further constraints;
   any `u64` is accepted).

---

## Postconditions

1. **POST-1 -- unchanged URLs produce no output**: No URL from `diff.unchanged` appears in
   `result.updated_urls`, `result.deleted_urls`, or `result.new_scrapes`.

2. **POST-2 -- changed URLs produce updated rows and payloads**: For every URL in `diff.changed`,
   `result.updated_urls` contains exactly one entry `(url, UrlStateRaw)` and `result.new_scrapes`
   contains exactly one entry `(hash_key, payload_bytes)`.

3. **POST-3 -- new URLs produce updated rows and payloads**: For every URL in `diff.new_urls`,
   `result.updated_urls` contains exactly one entry `(url, UrlStateRaw)` and `result.new_scrapes`
   contains exactly one entry `(hash_key, payload_bytes)`.

4. **POST-4 -- deleted URLs produce delete entries only**: For every URL in `diff.deleted`,
   `result.deleted_urls` contains exactly one entry. No corresponding payload entry is added
   to `result.new_scrapes`.

5. **POST-5 -- reference integrity**: For every `UrlStateRaw` in `result.updated_urls`, the
   `url_hash` field equals `hash_payload(artifact.payload_bytes)`. This hash must appear as a
   key in `result.new_scrapes`. This satisfies `commit_changes`'s reference integrity validation.

6. **POST-6 -- content_hash fidelity**: For every `UrlStateRaw` in `result.updated_urls`, the
   `content_hash` field equals `artifact.content_hash` from the corresponding `ScrapeArtifact`.

7. **POST-7 -- timestamp fidelity**: For every `UrlStateRaw` in `result.updated_urls`, the
   `last_fetched_secs` field equals `config.now_secs`.

8. **POST-8 -- status_code fidelity**: For every `UrlStateRaw` in `result.updated_urls`, the
   `status_code` field equals the `status_code` from the corresponding `ScrapeArtifact`.

9. **POST-9 -- non-URL fields are empty**: `result.updated_files`, `result.deleted_files`,
   `result.new_analyses`, `result.new_transforms`, `result.new_chunks`,
   `result.new_snapshots`, and `result.deleted_snapshots` are all empty vectors.

10. **POST-10 -- determinism**: Calling `build_scrape_state_changes` with the same inputs
    produces bit-identical `StateChanges` output. The order of entries within each vec is
    determined by the order of iteration over `diff.changed`, then `diff.new_urls`, then
    `diff.deleted`.

---

## Invariants

1. **INV-1 -- Zero-panic**: No `.unwrap()`, `.expect()`, `panic!`, or array indexing without
   bounds check. All fallible operations use `Result` propagation or explicit error mapping.

2. **INV-2 -- Railway-oriented error handling**: `build_scrape_state_changes` returns
   `Result<StateChanges, ScrapeBatchBuildError>`. No fallible operation escapes the `?` operator.

3. **INV-3 -- No side effects**: All functions are pure calculations. No I/O, no state mutation,
   no logging. Input references are borrowed; output is owned.

4. **INV-4 -- Unchanged URLs are never rewritten**: The function never produces state rows or
   payload blobs for URLs in `diff.unchanged`. This prevents churn on stable scrape state.

5. **INV-5 -- Batch construction is independent from commit boundary**: The returned `StateChanges`
   is a plain data structure. The caller decides when and whether to pass it to `commit_changes`.

6. **INV-6 -- One-to-one URL-to-row mapping**: Each changed or new URL produces exactly one
   `UrlStateRaw` entry in `updated_urls`. No URL produces zero or multiple rows.

7. **INV-7 -- Payload key matches FK reference**: The `url_hash` field in every produced
   `UrlStateRaw` is `hash_payload(&payload_bytes)`, and the same hash is used as the key in
   `new_scrapes`. This ensures `commit_changes`'s reference integrity check passes.

8. **INV-8 -- UrlStateRaw reserved field is zeroed**: Every produced `UrlStateRaw` has
   `reserved: [0u8; 46]`.

---

## Error Taxonomy

| Variant | Condition | Recovery |
|---------|-----------|----------|
| `EmptyDiff` | All four `ScrapeDiff` buckets are empty | Caller should not invoke batch builder when nothing changed |
| `DuplicateUrl` | Same URL appears in two or more diff categories | Caller must ensure classification produces disjoint partitions |
| `MissingScrapeArtifact` | Changed/new URL has no entry in `ScrapeOutputs` | Caller must ensure all changed/new pages were processed before batch build |
| `EmptyScrapePayload` | Artifact payload bytes are zero-length | Caller must ensure serialization produced non-empty output |
| `PayloadProcessingFailed` | Hash computation or payload processing fails | Caller should inspect the `reason` field; indicates a bug in pre-processing |

---

## Contract Signatures (Summary)

```rust
// Input types
pub struct ScrapeDiff { pub unchanged: Vec<String>, pub changed: Vec<String>, pub new_urls: Vec<String>, pub deleted: Vec<String> }
pub struct ScrapeArtifact { pub content_hash: [u8; 32], pub status_code: u16, pub payload_bytes: Vec<u8> }
pub struct ScrapeOutputs { pub artifacts: HashMap<String, ScrapeArtifact> }
pub struct ScrapeBatchConfig { pub now_secs: u64 }

// Error type
pub enum ScrapeBatchBuildError { MissingScrapeArtifact, EmptyScrapePayload, PayloadProcessingFailed, DuplicateUrl, EmptyDiff }

// Pure functions
pub fn build_scrape_state_changes(diff: &ScrapeDiff, outputs: &ScrapeOutputs, config: &ScrapeBatchConfig) -> Result<StateChanges, ScrapeBatchBuildError>;
pub fn build_url_state_raw(content_hash: [u8; 32], url_hash: [u8; 32], last_fetched_secs: u64, status_code: u16) -> UrlStateRaw;
```

---

## Non-goals

- This bead does NOT perform scrape classification (unchanged/changed/new/deleted detection).
  Classification is upstream; this bead receives a pre-classified `ScrapeDiff`.
- This bead does NOT perform HTTP requests, file I/O, or database writes.
- This bead does NOT define `UrlStateRaw` or `StateChanges` (those exist in `state/`).
- This bead does NOT handle the `scrape_outputs` table write (that is `commit_changes`'s job).
- This bead does NOT validate `UrlStateRaw` byte layout (that is `state/mod.rs`'s job).
- This bead does NOT handle snapshot persistence or `ChangePlan` generation (that is `watch.rs`).
