# Contract Specification

## Bead Metadata

- **bead_id:** cdocs-bvh
- **bead_title:** data: add archive-safe persisted output records and rkyv derives
- **phase:** 1 (contract-only, no implementation)

## Context

### Feature

Add append-only persisted record types that mirror the runtime domain types across all five
pipeline phases. These records derive `rkyv::Archive`, `rkyv::Serialize`, and
`rkyv::Deserialize` for zero-copy deserialization from mmapped files. Conversion helpers
translate between runtime domain values and persisted record values.

### Domain Terms (source-verified)

| Term | Source File | Runtime Type | Description |
|------|-------------|--------------|-------------|
| Analysis | `analyze.rs` | `Analysis` | Per-file metadata extraction: title, headings, links, word count, category, frontmatter |
| Heading | `analyze.rs` | `Heading` | Markdown heading: level (1-6), text, line number |
| Link | `analyze.rs` | `Link` | Extracted link: text, target, kind (Internal/External) |
| LinkKind | `analyze.rs` | `LinkKind` | Internal or External link classification |
| AnalyzeResult | `analyze.rs` | `AnalyzeResult` | Batch analysis: analyses + failed_files + total_discovered |
| FailedFile | `analyze.rs` | `FailedFile` | source_path + error message for failed analysis |
| IdMapping | `assign.rs` | `IdMapping` | Assigned document identity: id, filename, subcategory, slug |
| TransformResult | `transform.rs` | `TransformResult` | Batch transform: success_count, total_count, error_count, errors |
| TransformError | `transform.rs` | `TransformError` | source_path + error from a failed transform |
| Chunk (ctd) | `chunking_adapter.rs` | `Chunk` | Extended chunk with related_chunk_ids, context_prefix, chunk_level |
| ChunksResult | `chunking_adapter.rs` | `ChunksResult` | Batch chunking: total_chunks, counts by level, chunks_metadata |
| ChunkType | `contextual_chunker::chunk` | `ChunkType` | Code / Table / Prose classification |
| ChunkLevel | `contextual_chunker::chunk` | `ChunkLevel` | Summary / Standard / Detailed hierarchy |
| ScrapedPage | `scrape/validation.rs` | `ScrapedPage` | url, markdown, title, links, headers, word_count, slug, filter_status, elements_removed, density_score |
| Header | `scrape/validation.rs` | `Header` | level (u8) + text |
| PageFilterStatus | `scrape/validation.rs` | `PageFilterStatus` | Filtered / Unfiltered |
| ScrapeResult | `scrape/validation.rs` | `ScrapeResult` | pages, total_urls, success_count, error_count, errors, base_url |
| Snapshot | `watch.rs` | `Snapshot` | target_url, timestamp, pages (BTreeMap<String, PageHash>) |
| PageHash | `watch.rs` | `PageHash` | url, content_hash ([u8; 32]), title |
| ChangePlan | `watch.rs` | `ChangePlan` | target_url, timestamp, changes, summary, pending_snapshot |
| PageChange | `watch.rs` | `PageChange` | url, kind (Added/Modified/Removed), old_hash, new_hash, title |
| ChangeKind | `watch.rs` | `ChangeKind` | Added / Modified / Removed |
| ChangeSummary | `watch.rs` | `ChangeSummary` | added, removed, modified, unchanged, total_current, total_previous |
| DiscoveryFile | `discover.rs` | `DiscoveryFile` | source_path, size_bytes |
| DocumentId | `types/mod.rs` | `DocumentId` | Validated newtype for document IDs |
| ChunkId | `types/mod.rs` | `ChunkId` | Validated newtype for chunk IDs |
| Category | `types/mod.rs` | `Category` | Validated newtype for categories |

### Assumptions

1. rkyv will be added as a dependency (`rkyv = "0.8"`). All record types derive `Archive`,
   `Serialize`, `Deserialize` via rkyv's derive macros.
2. Record types live in a new module `src/persisted.rs` under the `centralized-docs` crate.
3. Persisted records are **structurally identical** to their runtime counterparts in terms of
   data fields, but replace `Arc<str>` with `String`, `HashMap` with sorted `Vec<(K, V)>`
   pairs for deterministic serialization, and `chrono::DateTime<Utc>` with `i64` (unix epoch
   seconds) for rkyv safety.
4. `BTreeMap` is used instead of `HashMap` where ordering matters for determinism.
5. Persisted records are append-only: once serialized, a record is never mutated in place.
6. Conversion helpers are pure functions: `fn from_runtime(T) -> P` and
   `fn into_runtime(P) -> Result<T, PersistError>`.
7. The rkyv `Archive` types use the default `rkyv::Archive` derive without custom resolvers.

### Open Questions

1. Should we version persisted records (e.g., `record_version: u32`) for forward
   compatibility? **Recommended: yes, add `schema_version: u32` to each top-level record.**
2. Should `content: Arc<str>` in `Analysis` be stored in full or as a content hash?
   **Assumption: stored in full, as rkyv serialization handles it efficiently.**
3. Should persisted records use `#[rkyv(compare)]` for `PartialEq` on archived types?
   **Assumption: no, keep it simple; add only if needed later.**

---

## Record Type Specifications

### 1. PersistedHeading

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedHeading {
    pub level: u32,        // 1-6, validated at conversion time
    pub text: String,      // non-empty after trim
    pub line: usize,       // 0-based line number in source
}
```

### 2. PersistedLinkKind

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedLinkKind {
    Internal,
    External,
}
```

### 3. PersistedLink

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedLink {
    pub text: String,
    pub target: String,       // non-empty
    pub kind: PersistedLinkKind,
}
```

### 4. PersistedAnalysis

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedAnalysis {
    pub schema_version: u32,                              // always 1
    pub source_path: String,                              // non-empty
    pub title: String,                                    // non-empty
    pub frontmatter: Option<Vec<(String, String)>>,       // sorted by key, deterministic
    pub headings: Vec<PersistedHeading>,
    pub links: Vec<PersistedLink>,
    pub first_paragraph: String,
    pub word_count: usize,
    pub has_code: bool,
    pub has_tables: bool,
    pub category: String,                                 // non-empty, lowercase
    pub content: String,                                  // full cleaned content
}
```

### 5. PersistedFailedFile

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedFailedFile {
    pub source_path: String,   // non-empty
    pub error: String,         // non-empty
}
```

### 6. PersistedAnalyzeResult

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedAnalyzeResult {
    pub schema_version: u32,
    pub analyses: Vec<PersistedAnalysis>,
    pub failed_files: Vec<PersistedFailedFile>,
    pub total_discovered: usize,
}
```

### 7. PersistedIdMapping

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedIdMapping {
    pub source_path: String,    // key into link_map
    pub id: String,             // e.g., "concept/general/my-doc"
    pub filename: String,       // e.g., "concept-general-my-doc.md"
    pub subcategory: String,
    pub slug: String,
}
```

### 8. PersistedTransformError

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformError {
    pub source_path: String,
    pub error: String,
}
```

### 9. PersistedTransformResult

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformResult {
    pub schema_version: u32,
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
    pub errors: Vec<PersistedTransformError>,
}
```

### 10. PersistedChunkType

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkType {
    Code,
    Table,
    Prose,
}
```

### 11. PersistedChunkLevel

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkLevel {
    Summary,
    Standard,
    Detailed,
}
```

### 12. PersistedChunk

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunk {
    pub schema_version: u32,
    pub chunk_id: String,                        // format: "{doc_id}#{index}"
    pub doc_id: String,
    pub doc_title: String,
    pub chunk_index: usize,
    pub content: String,                         // non-empty
    pub token_count: usize,                      // > 0
    pub heading: Option<String>,
    pub heading_path: Vec<String>,
    pub chunk_type: PersistedChunkType,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    pub related_chunk_ids: Vec<String>,
    pub summary: String,
    pub chunk_level: PersistedChunkLevel,
    pub parent_chunk_id: Option<String>,
    pub child_chunk_ids: Vec<String>,
    pub context_prefix: Option<String>,
}
```

### 13. PersistedChunksResult

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunksResult {
    pub schema_version: u32,
    pub total_chunks: usize,
    pub document_count: usize,
    pub chunks_metadata: Vec<PersistedChunk>,
    pub summary_chunks: usize,
    pub standard_chunks: usize,
    pub detailed_chunks: usize,
}
```

### 14. PersistedHeader

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedHeader {
    pub level: u8,         // 1-6
    pub text: String,      // non-empty
}
```

### 15. PersistedPageFilterStatus

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedPageFilterStatus {
    Filtered,
    Unfiltered,
}
```

### 16. PersistedScrapedPage

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapedPage {
    pub url: String,
    pub markdown: String,
    pub title: String,
    pub links: Vec<String>,
    pub headers: Vec<PersistedHeader>,
    pub word_count: usize,
    pub slug: String,
    pub filter_status: PersistedPageFilterStatus,
    pub elements_removed: usize,
    pub density_score: f32,
}
```

### 17. PersistedScrapeResult

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapeResult {
    pub schema_version: u32,
    pub pages: Vec<PersistedScrapedPage>,
    pub total_urls: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<(String, String)>,     // (url, error_message)
    pub base_url: String,
}
```

### 18. PersistedPageHash

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageHash {
    pub url: String,
    pub content_hash: [u8; 32],            // SHA-256
    pub title: String,
}
```

### 19. PersistedChangeKind

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChangeKind {
    Added,
    Modified,
    Removed,
}
```

### 20. PersistedPageChange

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedPageChange {
    pub url: String,
    pub kind: PersistedChangeKind,
    pub old_hash: Option<[u8; 32]>,
    pub new_hash: Option<[u8; 32]>,
    pub title: String,
}
```

### 21. PersistedChangeSummary

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangeSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub unchanged: usize,
    pub total_current: usize,
    pub total_previous: usize,
}
```

### 22. PersistedSnapshot

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedSnapshot {
    pub schema_version: u32,
    pub target_url: String,
    pub timestamp_secs: i64,               // unix epoch seconds (replaces DateTime<Utc>)
    pub pages: Vec<(String, PersistedPageHash)>,  // sorted by URL key
}
```

### 23. PersistedChangePlan

```rust
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChangePlan {
    pub schema_version: u32,
    pub target_url: String,
    pub timestamp_secs: i64,               // unix epoch seconds
    pub changes: Vec<PersistedPageChange>,
    pub summary: PersistedChangeSummary,
    pub pending_snapshot: PersistedSnapshot,
}
```

---

## Conversion Helper Signatures

All conversions are fallible (runtime -> persisted is infallible, persisted -> runtime validates).

### Analysis Conversions

```rust
// Infallible: runtime -> persisted (all runtime data is already validated)
fn heading_to_persisted(h: &Heading) -> PersistedHeading;
fn link_kind_to_persisted(k: &LinkKind) -> PersistedLinkKind;
fn link_to_persisted(l: &Link) -> PersistedLink;
fn analysis_to_persisted(a: &Analysis) -> PersistedAnalysis;
fn analyze_result_to_persisted(r: &AnalyzeResult) -> PersistedAnalyzeResult;

// Fallible: persisted -> runtime (validates field constraints)
fn persisted_heading_to_runtime(p: &PersistedHeading) -> Result<Heading, PersistError>;
fn persisted_link_kind_to_runtime(p: PersistedLinkKind) -> Result<LinkKind, PersistError>;
fn persisted_link_to_runtime(p: &PersistedLink) -> Result<Link, PersistError>;
fn persisted_analysis_to_runtime(p: &PersistedAnalysis) -> Result<Analysis, PersistError>;
fn persisted_analyze_result_to_runtime(p: &PersistedAnalyzeResult) -> Result<AnalyzeResult, PersistError>;
```

### Transform Conversions

```rust
fn transform_error_to_persisted(e: &TransformError) -> PersistedTransformError;
fn transform_result_to_persisted(r: &TransformResult) -> PersistedTransformResult;

fn persisted_transform_error_to_runtime(p: &PersistedTransformError) -> Result<TransformError, PersistError>;
fn persisted_transform_result_to_runtime(p: &PersistedTransformResult) -> Result<TransformResult, PersistError>;
```

### Chunk Conversions

```rust
fn chunk_type_to_persisted(t: &ChunkType) -> PersistedChunkType;
fn chunk_level_to_persisted(l: &ChunkLevel) -> PersistedChunkLevel;
fn chunk_to_persisted(c: &Chunk) -> PersistedChunk;      // ctd Chunk
fn chunks_result_to_persisted(r: &ChunksResult) -> PersistedChunksResult;

fn persisted_chunk_type_to_runtime(p: PersistedChunkType) -> Result<ChunkType, PersistError>;
fn persisted_chunk_level_to_runtime(p: PersistedChunkLevel) -> Result<ChunkLevel, PersistError>;
fn persisted_chunk_to_runtime(p: &PersistedChunk) -> Result<Chunk, PersistError>;
fn persisted_chunks_result_to_runtime(p: &PersistedChunksResult) -> Result<ChunksResult, PersistError>;
```

### Scrape Conversions

```rust
fn header_to_persisted(h: &Header) -> PersistedHeader;
fn page_filter_status_to_persisted(s: &PageFilterStatus) -> PersistedPageFilterStatus;
fn scraped_page_to_persisted(p: &ScrapedPage) -> PersistedScrapedPage;
fn scrape_result_to_persisted(r: &ScrapeResult) -> PersistedScrapeResult;

fn persisted_header_to_runtime(p: &PersistedHeader) -> Result<Header, PersistError>;
fn persisted_page_filter_status_to_runtime(p: PersistedPageFilterStatus) -> Result<PageFilterStatus, PersistError>;
fn persisted_scraped_page_to_runtime(p: &PersistedScrapedPage) -> Result<ScrapedPage, PersistError>;
fn persisted_scrape_result_to_runtime(p: &PersistedScrapeResult) -> Result<ScrapeResult, PersistError>;
```

### Watch/Snapshot Conversions

```rust
fn page_hash_to_persisted(p: &PageHash) -> PersistedPageHash;
fn change_kind_to_persisted(k: &ChangeKind) -> PersistedChangeKind;
fn page_change_to_persisted(p: &PageChange) -> PersistedPageChange;
fn change_summary_to_persisted(s: &ChangeSummary) -> PersistedChangeSummary;
fn snapshot_to_persisted(s: &Snapshot) -> PersistedSnapshot;
fn change_plan_to_persisted(p: &ChangePlan) -> PersistedChangePlan;

fn persisted_page_hash_to_runtime(p: &PersistedPageHash) -> Result<PageHash, PersistError>;
fn persisted_change_kind_to_runtime(p: PersistedChangeKind) -> Result<ChangeKind, PersistError>;
fn persisted_page_change_to_runtime(p: &PersistedPageChange) -> Result<PageChange, PersistError>;
fn persisted_change_summary_to_runtime(p: &PersistedChangeSummary) -> Result<ChangeSummary, PersistError>;
fn persisted_snapshot_to_runtime(p: &PersistedSnapshot) -> Result<Snapshot, PersistError>;
fn persisted_change_plan_to_runtime(p: &PersistedChangePlan) -> Result<ChangePlan, PersistError>;
```

### Assign Conversions

```rust
fn id_mapping_to_persisted(source_path: &str, m: &IdMapping) -> PersistedIdMapping;
fn persisted_id_mapping_to_runtime(p: &PersistedIdMapping) -> Result<(String, IdMapping), PersistError>;
```

---

## Preconditions

1. **P-01:** All `to_persisted` conversion inputs must be valid runtime types (already
   constructed through their validated constructors or pipeline functions).
2. **P-02:** All `to_runtime` conversion inputs must have `schema_version == 1`. Any other
   value produces `PersistError::SchemaVersionMismatch`.
3. **P-03:** `rkyv::to_bytes::<rkyv::rancor::Error>(&record)` is only called on fully-populated
   records (no intentionally-empty required fields).
4. **P-04:** The `rkyv` dependency version is `0.8.x` with the `std` feature enabled.
5. **P-05:** Each persisted record module is behind `#[cfg(feature = "persist")]` or always
   compiled (decided at implementation time).

---

## Postconditions

1. **PO-01:** Every `to_persisted` conversion produces a record where
   `rkyv::to_bytes(&record).unwrap()` succeeds and the archived bytes roundtrip via
   `rkyv::from_bytes::<ArchivedT>(&bytes).unwrap()`.
2. **PO-02:** Every `to_runtime` conversion on a valid persisted record produces a runtime
   type that is `==` (where `PartialEq` exists) to the original value before it was converted
   to persisted form, with one exception: `Arc<str>` content is compared by string value.
3. **PO-03:** `frontmatter` in `PersistedAnalysis` is sorted by key (deterministic
   serialization) regardless of the original `HashMap` iteration order.
4. **PO-04:** `pages` in `PersistedSnapshot` is sorted by URL key (deterministic
   serialization) regardless of the original `BTreeMap` order (which is already sorted, so
   this is a no-op but explicitly stated).
5. **PO-05:** `DateTime<Utc>` values are stored as `i64` unix epoch seconds; conversion back
   produces a `DateTime<Utc>` with sub-second precision of 0 (lossy but acceptable per
   assumption).
6. **PO-06:** All top-level batch records (`PersistedAnalyzeResult`, `PersistedChunksResult`,
   `PersistedScrapeResult`, `PersistedChangePlan`) have `schema_version: 1`.

---

## Invariants

1. **INV-01:** Persisted records are **append-only**. Once serialized, no field of a
   persisted record is ever mutated. If data changes, a new record is created.
2. **INV-02:** All `String` fields that represent identifiers (`source_path`, `chunk_id`,
   `doc_id`, `url`, `slug`, `id`, `filename`) are non-empty after trimming.
3. **INV-03:** All `heading.level` values are in range `1..=6`.
4. **INV-04:** All `Header.level` values are in range `1..=6` (u8).
5. **INV-05:** `token_count` in `PersistedChunk` is always > 0.
6. **INV-06:** `content_hash` in `PersistedPageHash` is always exactly 32 bytes (SHA-256).
7. **INV-07:** Enum variants (`PersistedLinkKind`, `PersistedChangeKind`,
   `PersistedChunkType`, `PersistedChunkLevel`, `PersistedPageFilterStatus`) are exhaustive
   and match 1:1 with their runtime counterparts.
8. **INV-08:** Bidirectional conversion is lossless for all primitive fields. The only
   lossy conversion is `DateTime<Utc>` -> `i64` (seconds precision only).
9. **INV-09:** `density_score` in `PersistedScrapedPage` is a finite `f32` (not NaN, not Inf).
10. **INV-10:** `related_chunk_ids` in `PersistedChunk` may be empty (populated later by
    graph analysis) but never contains duplicates.

---

## Error Taxonomy

```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum PersistError {
    /// A required String field was empty or whitespace-only.
    #[error("field '{field}' must be non-empty")]
    EmptyField { field: String },

    /// A numeric field was outside its valid range.
    #[error("field '{field}' value {value} is out of range {min}..={max}")]
    OutOfRange { field: String, value: i64, min: i64, max: i64 },

    /// The persisted record's schema version does not match expected.
    #[error("schema version mismatch: expected {expected}, got {actual}")]
    SchemaVersionMismatch { expected: u32, actual: u32 },

    /// rkyv serialization failed (buffer allocation or write error).
    #[error("serialization failed: {reason}")]
    SerializationFailed { reason: String },

    /// rkyv deserialization / validation failed (corrupted bytes).
    #[error("deserialization failed: {reason}")]
    DeserializationFailed { reason: String },

    /// An enum variant that doesn't map to any known runtime value.
    #[error("unknown enum variant for '{type_name}'")]
    UnknownVariant { type_name: String },

    /// A float field was NaN or Infinite where finite was expected.
    #[error("field '{field}' must be a finite number, got {value}")]
    NonFiniteFloat { field: String, value: String },

    /// A content hash was not exactly 32 bytes.
    #[error("content hash must be exactly 32 bytes, got {actual_len}")]
    InvalidHashLength { actual_len: usize },
}
```

### Error-to-Failure-Mode Mapping

| Failure Mode | Error Variant | Trigger |
|---|---|---|
| Empty required field after deserialization | `EmptyField` | `PersistedAnalysis.source_path == ""` |
| Heading level outside 1-6 | `OutOfRange` | `PersistedHeading.level == 0` or `> 6` |
| Future schema version | `SchemaVersionMismatch` | `schema_version != 1` |
| Corrupted rkyv bytes | `DeserializationFailed` | `rkyv::from_bytes` fails validation |
| Buffer allocation failure | `SerializationFailed` | `rkyv::to_bytes` OOM |
| NaN density_score | `NonFiniteFloat` | `PersistedScrapedPage.density_score.is_nan()` |
| Wrong hash length | `InvalidHashLength` | `content_hash.len() != 32` |

---

## Anti-Hallucination Notes

**All types in this contract were derived from direct source file reads:**

1. `analyze.rs` (1371 lines) -- `Analysis`, `Heading`, `Link`, `LinkKind`, `AnalyzeResult`,
   `FailedFile` confirmed at lines 16-61. `content: Arc<str>` confirmed at line 47.
   `frontmatter: Option<HashMap<String, String>>` confirmed at line 39.

2. `chunking_adapter.rs` (514 lines) -- `Chunk` (ctd extended) confirmed at lines 44-63.
   `ChunksResult` confirmed at lines 67-74. `context_prefix: Option<String>` confirmed
   at line 62. `related_chunk_ids: Vec<String>` confirmed at line 56.

3. `scrape/validation.rs` (850 lines) -- `ScrapedPage` confirmed at lines 185-196.
   `Header` (level: u8) confirmed at line 201. `PageFilterStatus` confirmed at line 176.
   `ScrapeResult` confirmed at lines 206-214. `density_score: f32` at line 195.

4. `watch.rs` (702 lines) -- `Snapshot` confirmed at lines 42-49. `PageHash` with
   `content_hash: [u8; 32]` confirmed at line 35. `ChangePlan` confirmed at lines 88-99.
   `ChangeKind` confirmed at lines 53-66. `ChangeSummary` confirmed at lines 102-110.
   `DateTime<Utc>` at line 46.

5. `contextual-chunker/src/chunk.rs` (1051 lines) -- `Chunk` (upstream) confirmed at lines
   266-327. `ChunkLevel` confirmed at lines 177-183. `ChunkType` confirmed at lines
   209-218.

6. `types/mod.rs` (1079 lines) -- `DocumentId`, `ChunkId`, `Category`, etc. confirmed.

7. `assign.rs` -- `IdMapping` confirmed at lines 8-13.

8. `discover.rs` -- `DiscoveryFile` confirmed at lines 33-37.

9. `Cargo.toml` (212 lines) -- No existing `rkyv` dependency; must be added.

**No fields were invented. No types were assumed. All field names, types, and constraints
match the actual source code.**

---

## Non-goals

1. This contract does NOT specify the rkyv byte layout or alignment requirements (handled by
   rkyv's derive macros automatically).
2. This contract does NOT specify file I/O operations (read/write/mmap) -- only the in-memory
   record types and conversions.
3. This contract does NOT specify version migration (schema version > 1 handling) -- only
   version 1 is defined.
4. This contract does NOT add `serde::Serialize`/`Deserialize` derives on persisted records
   (rkyv replaces serde for binary persistence). JSON/serde persistence remains on the
   existing runtime types.
5. This contract does NOT change any existing runtime types or their serde derives.
6. This contract does NOT specify thread safety beyond what rkyv provides (archived refs are
   `Send + Sync` for `#[derive(Archive)]` types).
