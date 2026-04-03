# Contract Specification

## Metadata

| Field | Value |
|-------|-------|
| bead_id | cdocs-bg3 |
| bead_title | Create redb table definitions for raw state and archived outputs |
| phase | contract |

## Context

### Feature

Define all 8 redb table definitions (`TableDefinition` constants) for the new state database
schema described in `architecture-spec.md` section 3. These definitions are the **single source
of truth** for key/value type alignment across the raw-state (Pod/bytemuck) and archived-output
(rkyv) storage strategies.

### Domain Terms

- **Pod table** -- a redb table whose value type is a fixed-size `#[repr(C)]` struct that
  implements `bytemuck::Pod`. Read via `bytemuck::pod_read_unaligned`, written via
  `bytemuck::bytes_of`. Zero deserialize, zero allocate.
- **rkyv table** -- a redb table whose value bytes are `rkyv::to_bytes` output. Read by
  copying bytes into `OwnedArchive<T>` then calling `.archived()` (zero-copy pointer cast) or
  `.deserialize()` (full materialization). Key is always `[u8; 32]` (SHA-256 hash).
- **Raw state** -- `FileStateRaw` (200 bytes) and `UrlStateRaw` (120 bytes). Fixed-size Pod
  structs stored directly in redb. No serialization format.
- **Archived output** -- `rkyv(Analysis)`, `rkyv(String)`, `rkyv(Vec<Chunk>)`,
  `rkyv(ScrapedPage)`, `rkyv(Snapshot)`. Variable-size byte blobs stored in redb.
- **StateDb** -- the new `StateDb` struct (replacing `DocCache` for state operations) that
  opens these tables and provides bulk read/write sessions.
- **OwnedArchive<T>** -- owned wrapper around `Box<[u8]>` that provides `archived()` and
  `deserialize()` without exposing redb transaction lifetimes.

### Assumptions

1. redb `2.x` `TableDefinition` API: `TableDefinition::new("name")` with explicit
   key/value type parameters. The existing `cache/mod.rs` uses `TableDefinition<&[u8], &[u8]>`
   for binary tables and `TableDefinition<&str, &str>` for metadata.
2. redb requires that table definitions be declared as `const` and that the same definition
   (same name, same types) is used for both reads and writes.
3. `bytemuck::Pod` requires `#[repr(C)]`, no padding with undefined values, and all fields
   are themselves Pod. Both `FileStateRaw` and `UrlStateRaw` use only `[u8; N]`, `u64`, and
   `u16` fields with explicit `_reserved` padding to eliminate undefined padding bytes.
4. `rkyv 0.8` with `bytecheck` feature. All domain types (`Analysis`, `Chunk`, `Heading`,
   `Link`, `LinkKind`, `ScrapedPage`, `Snapshot`, `PageHash`, `Header`, `PageFilterStatus`,
   `ChunkType`, `ChunkLevel`) must eventually derive `rkyv::Archive`, `rkyv::Serialize`, and
   `rkyv::Deserialize`. Those derives are **out of scope** for this bead (this bead defines
   the table constants only).
5. The existing `cache/mod.rs` table names (`documents`, `scrape`, `transforms`, `snapshots`,
   `analysis`, `chunks`, `metadata`) are **legacy**. The new table names from the architecture
   spec are **different** and will coexist during migration.

### Open Questions

1. ~~Should legacy `DocCache` table definitions be removed in this bead?~~
   **No.** This bead defines new table definitions only. Legacy table removal is a separate bead.
2. ~~Should the Pod tables use `TableDefinition<&str, &[u8]>` (typed key) or
   `TableDefinition<&[u8], &[u8]>` (uniform binary)?~~
   **`&str` key** for `file_state` and `url_state` (source_path/url is human-readable and
   enables redb range scans). **`&[u8]` key** for all rkyv output tables (hash keys are opaque
   32-byte blobs). See architecture spec section 3.
3. ~~Should the metadata table key/value type change from `&str`/`&str`?~~
   **No.** The architecture spec says `metadata  key: &str  ->  &str`. Same as existing.

---

## Table Definitions

### Table 1: `file_state`

| Property | Value |
|----------|-------|
| **Name** | `"file_state"` |
| **Key type** | `&str` (source_path, e.g. `"concept/general/test.md"`) |
| **Value type** | `&[u8]` (raw bytes of `FileStateRaw`, exactly 200 bytes) |
| **Serialization** | bytemuck Pod cast -- `bytes_of(&state)` to write, `pod_read_unaligned` to read |
| **Value size** | Fixed 200 bytes |

**redb definition:**
```rust
const FILE_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("file_state");
```

**Key invariant:** Every key in this table is a relative source path (no leading `/`, no
trailing whitespace). The key is identical to `DiscoveryFile::source_path`.

**Value layout** (`FileStateRaw`, 200 bytes, `#[repr(C)]`):
```
offset   size  field
0        32    content_hash: [u8; 32]    // SHA-256 of file bytes
32       32    config_hash: [u8; 32]     // SHA-256 of category config (or zeroed)
64       32    analysis_hash: [u8; 32]   // FK -> analysis_outputs key
96       32    transform_hash: [u8; 32]  // FK -> transform_outputs key
128      32    chunk_hash: [u8; 32]      // FK -> chunk_outputs key
160       8    last_processed_secs: u64   // unix timestamp
168      32    _reserved: [u8; 32]       // future-proof padding
Total: 200 bytes
```

### Table 2: `analysis_outputs`

| Property | Value |
|----------|-------|
| **Name** | `"analysis_outputs"` |
| **Key type** | `&[u8]` (32-byte SHA-256 hash) |
| **Value type** | `&[u8]` (rkyv-archived `Analysis` bytes) |
| **Serialization** | rkyv `to_bytes` to write, `OwnedArchive<Analysis>` to read |
| **Value size** | Variable |

**redb definition:**
```rust
const ANALYSIS_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("analysis_outputs");
```

**Key invariant:** The key is always exactly 32 bytes. It is the `analysis_hash` field from
`FileStateRaw`, which is `SHA-256(source_path || file_content || config_hash)`.

### Table 3: `transform_outputs`

| Property | Value |
|----------|-------|
| **Name** | `"transform_outputs"` |
| **Key type** | `&[u8]` (32-byte SHA-256 hash) |
| **Value type** | `&[u8]` (rkyv-archived `String` bytes -- transformed markdown) |
| **Serialization** | rkyv `to_bytes` to write, `OwnedArchive<String>` to read |
| **Value size** | Variable |

**redb definition:**
```rust
const TRANSFORM_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("transform_outputs");
```

**Key invariant:** The key is always exactly 32 bytes. It is the `transform_hash` field from
`FileStateRaw`.

### Table 4: `chunk_outputs`

| Property | Value |
|----------|-------|
| **Name** | `"chunk_outputs"` |
| **Key type** | `&[u8]` (32-byte SHA-256 hash) |
| **Value type** | `&[u8]` (rkyv-archived `Vec<Chunk>` bytes) |
| **Serialization** | rkyv `to_bytes` to write, `OwnedArchive<Vec<Chunk>>` to read |
| **Value size** | Variable |

**redb definition:**
```rust
const CHUNK_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("chunk_outputs");
```

**Key invariant:** The key is always exactly 32 bytes. It is the `chunk_hash` field from
`FileStateRaw`.

### Table 5: `url_state`

| Property | Value |
|----------|-------|
| **Name** | `"url_state"` |
| **Key type** | `&str` (canonical URL, e.g. `"https://docs.example.com/api"`) |
| **Value type** | `&[u8]` (raw bytes of `UrlStateRaw`, exactly 120 bytes) |
| **Serialization** | bytemuck Pod cast -- `bytes_of(&state)` to write, `pod_read_unaligned` to read |
| **Value size** | Fixed 120 bytes |

**redb definition:**
```rust
const URL_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("url_state");
```

**Value layout** (`UrlStateRaw`, 120 bytes, `#[repr(C)]`):
```
offset   size  field
0        32    content_hash: [u8; 32]    // SHA-256 of scraped markdown content
32       32    url_hash: [u8; 32]        // FK -> scrape_outputs key
64        8    last_fetched_secs: u64    // unix timestamp
72        2    status_code: u16          // last HTTP status (200, 304, etc.)
74       46    _reserved: [u8; 46]       // future ETag/Last-Modified slot
Total: 120 bytes
```

### Table 6: `scrape_outputs`

| Property | Value |
|----------|-------|
| **Name** | `"scrape_outputs"` |
| **Key type** | `&[u8]` (32-byte SHA-256 hash of URL) |
| **Value type** | `&[u8]` (rkyv-archived `ScrapedPage` bytes) |
| **Serialization** | rkyv `to_bytes` to write, `OwnedArchive<ScrapedPage>` to read |
| **Value size** | Variable |

**redb definition:**
```rust
const SCRAPE_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("scrape_outputs");
```

**Key invariant:** The key is always exactly 32 bytes. It is the `url_hash` field from
`UrlStateRaw`, which is `SHA-256(url_bytes)`.

### Table 7: `snapshots`

| Property | Value |
|----------|-------|
| **Name** | `"snapshots"` |
| **Key type** | `&[u8]` (32-byte SHA-256 hash of `(target_url, timestamp)`) |
| **Value type** | `&[u8]` (rkyv-archived `Snapshot` bytes) |
| **Serialization** | rkyv `to_bytes` to write, `OwnedArchive<Snapshot>` to read |
| **Value size** | Variable |

**redb definition:**
```rust
const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("snapshots");
```

**Key invariant:** The key is always exactly 32 bytes. It uniquely identifies a snapshot
by hashing `(target_url, timestamp)`.

### Table 8: `metadata`

| Property | Value |
|----------|-------|
| **Name** | `"metadata"` |
| **Key type** | `&str` (well-known key, e.g. `"schema_version"`) |
| **Value type** | `&str` (string value) |
| **Serialization** | None -- redb handles `&str` natively |
| **Value size** | Variable |

**redb definition:**
```rust
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");
```

**Key invariant:** Key names are ASCII, non-empty. Known keys: `"schema_version"`.

---

## Table Classification Summary

| Category | Tables | Key Type | Value Type | Read Path | Write Path |
|----------|--------|----------|------------|-----------|------------|
| Pod state | `file_state`, `url_state` | `&str` | `&[u8]` (fixed) | `bytemuck::pod_read_unaligned` | `bytemuck::bytes_of` |
| rkyv output | `analysis_outputs`, `transform_outputs`, `chunk_outputs`, `scrape_outputs`, `snapshots` | `&[u8]` (32B) | `&[u8]` (variable) | `OwnedArchive<T>::from_bytes` then `.archived()` or `.deserialize()` | `rkyv::to_bytes::<rkyv::rancor::Error>(&value)` |
| Metadata | `metadata` | `&str` | `&str` | Direct redb `AccessGuard::value()` | Direct redb `table.insert()` |

---

## Preconditions

### P-01: Table name uniqueness
All 8 table names MUST be unique within the redb database. The names `"file_state"`,
`"analysis_outputs"`, `"transform_outputs"`, `"chunk_outputs"`, `"url_state"`,
`"scrape_outputs"`, `"snapshots"`, `"metadata"` MUST NOT collide with each other or with
legacy `DocCache` table names (`"documents"`, `"scrape"`, `"transforms"`, `"snapshots"`,
`"analysis"`, `"chunks"`).

### P-02: Table name matches architecture spec exactly
Each table name MUST match the name given in architecture-spec.md section 3 verbatim.

### P-03: Key/value types align with storage strategy
- Pod tables (`file_state`, `url_state`) MUST use `TableDefinition<&str, &[u8]>`.
- rkyv output tables MUST use `TableDefinition<&[u8], &[u8]>`.
- metadata MUST use `TableDefinition<&str, &str>`.

### P-04: FileStateRaw is exactly 200 bytes
`std::mem::size_of::<FileStateRaw>() == 200`. Enforced by `#[repr(C)]` layout and explicit
`_reserved: [u8; 32]` field.

### P-05: UrlStateRaw is exactly 120 bytes
`std::mem::size_of::<UrlStateRaw>() == 120`. Enforced by `#[repr(C)]` layout and explicit
`_reserved: [u8; 46]` field.

### P-06: Pod structs have no undefined padding
Both `FileStateRaw` and `UrlStateRaw` use explicit `_reserved` byte arrays to fill all
padding. All other fields are `[u8; N]`, `u64`, or `u16` -- all fully defined bit patterns.

### P-07: All hash keys are exactly 32 bytes
Every `[u8; 32]` key written to rkyv output tables and referenced as FK fields in Pod structs
is a SHA-256 digest -- always exactly 32 bytes.

### P-08: bytemuck safety requirements met
`FileStateRaw` and `UrlStateRaw` MUST implement `bytemuck::Pod` and `bytemuck::Zeroable`.
This requires:
- `#[repr(C)]` or `#[repr(transparent)]`
- No padding bytes with undefined values (satisfied by explicit `_reserved`)
- All field types are themselves `Pod` (`[u8; N]`, `u64`, `u16`)

### P-09: Database is open before table access
All table reads/writes require an active redb `Database` handle. No table operation may
proceed without `StateDb::open()` succeeding first.

### P-10: Read transaction held during bulk loads
`StateReadSession` methods (`load_file_states`, `load_url_states`, `load_analyses`, etc.)
MUST be called while the read transaction is alive. The read transaction MUST be dropped
before any write transaction begins (redb MVCC constraint).

---

## Postconditions

### POST-01: All 8 tables are created on first write
When `StateDb::open()` is called on a new database path, the first write transaction
MUST create all 8 tables via `write_tx.open_table(...)` for each definition.

### POST-02: Table definitions are const and `'static`
All `TableDefinition` constants have `'static` lifetime. They are compile-time constants
and incur zero runtime allocation.

### POST-03: Pod reads return exactly N bytes
- Reading from `file_state` returns exactly 200 bytes per value.
- Reading from `url_state` returns exactly 120 bytes per value.

### POST-04: rkyv bytes are valid archives
Bytes read from rkyv output tables MUST be valid `rkyv` archives for their respective types.
Validation occurs in `OwnedArchive<T>::from_bytes()` via `rkyv::access` (with `bytecheck`
feature). Invalid bytes produce `StateError::InvalidArchive`.

### POST-05: FK integrity across tables
Every `[u8; 32]` hash field in a Pod struct (`analysis_hash`, `transform_hash`, `chunk_hash`,
`url_hash`) either:
- Points to an existing key in the corresponding output table (normal case), OR
- Is all zeros (initial state before first processing of that output type).

### POST-06: Atomic write transaction
`StateDb::commit_changes(&StateChanges)` writes ALL changes (file state, url state, outputs,
deletions) in a **single** redb write transaction. If the commit fails, the database is
unchanged (ACID guarantee from redb).

---

## Invariants

### INV-01: Two transaction architecture
Every `ctd` command run uses exactly 2 redb transactions: 1 read (startup) + 1 write (shutdown).
No per-entry transactions.

### INV-02: Table name immutability
Table names are `const` string literals. They never change at runtime.

### INV-03: Pod value byte count
For any entry `(k, v)` in `file_state`: `v.len() == 200`.
For any entry `(k, v)` in `url_state`: `v.len() == 120`.

### INV-04: Hash key byte count
For any entry `(k, v)` in `analysis_outputs`, `transform_outputs`, `chunk_outputs`,
`scrape_outputs`, or `snapshots`: `k.len() == 32`.

### INV-05: Cross-table FK domain
`FileStateRaw.analysis_hash` values form a subset of keys in `analysis_outputs`.
`FileStateRaw.transform_hash` values form a subset of keys in `transform_outputs`.
`FileStateRaw.chunk_hash` values form a subset of keys in `chunk_outputs`.
`UrlStateRaw.url_hash` values form a subset of keys in `scrape_outputs`.
(Exception: all-zero hash `[0u8; 32]` indicates "not yet processed".)

### INV-06: Pod read/write symmetry
For any `FileStateRaw` value `s`: `pod_read_unaligned::<FileStateRaw>(bytes_of(&s)) == s`.
For any `UrlStateRaw` value `u`: `pod_read_unaligned::<UrlStateRaw>(bytes_of(&u)) == u`.

### INV-07: rkyv read/write symmetry
For any `rkyv`-serializable value `v` of type `T`:
`OwnedArchive::<T>::from_bytes(rkyv::to_bytes::<rkyv::rancor::Error>(&v).as_slice())`
succeeds and `.deserialize()` returns a value equal to `v`.

### INV-08: Metadata table is string-only
The `metadata` table accepts only `&str` keys and `&str` values. No binary data.

### INV-09: No overlapping table names with legacy cache
The 8 new table names (`file_state`, `analysis_outputs`, `transform_outputs`,
`chunk_outputs`, `url_state`, `scrape_outputs`, `snapshots`, `metadata`) are a
disjoint set from the 7 legacy table names (`documents`, `scrape`, `transforms`,
`snapshots`, `analysis`, `chunks`, `metadata`).

**Note:** `"metadata"` appears in both sets. This is intentional -- the new `metadata`
table replaces the legacy one. During migration, both `DocCache` and `StateDb` may be
open on the same database file. The metadata table definition is identical in both
(`TableDefinition<&str, &str>`), so they share the same redb table safely.

### INV-10: Source path keys are relative and normalized
`file_state` keys MUST NOT start with `/`. They are relative paths from the source root
(e.g., `"concept/general/test.md"`). They MUST NOT contain `..` components.

### INV-11: URL keys are canonical
`url_state` keys MUST be absolute URLs with scheme (e.g., `"https://docs.example.com/api"`).
They MUST NOT contain trailing slashes (except for root `"/"`).

---

## Error Taxonomy

### `StateError` -- new error enum for state database operations

This replaces the use of `CacheError` for state operations. It is a separate enum because
the failure modes of the two-transaction bulk-load architecture are different from the
legacy per-entry cache.

```rust
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum StateError {
    // -- Database lifecycle errors --

    /// The state database could not be opened.
    #[error("failed to open state database at {path}: {source}")]
    OpenFailed {
        path: std::path::PathBuf,
        source: String,
    },

    /// A read transaction could not be started.
    #[error("failed to begin read transaction: {message}")]
    ReadTransactionFailed { message: String },

    /// A write transaction could not be started.
    #[error("failed to begin write transaction: {message}")]
    WriteTransactionFailed { message: String },

    // -- Pod read/write errors --

    /// A value read from a Pod table has the wrong byte count.
    #[error("pod value size mismatch for table {table}: expected {expected} bytes, got {actual}")]
    PodSizeMismatch {
        table: &'static str,
        expected: usize,
        actual: usize,
    },

    /// A Pod cast failed (alignment or size violation).
    #[error("pod cast failed for type {type_name}: {message}")]
    PodCastFailed {
        type_name: &'static str,
        message: String,
    },

    // -- rkyv archive errors --

    /// Bytes read from an rkyv table are not a valid archive.
    #[error("invalid rkyv archive for type {type_name}: {message}")]
    InvalidArchive {
        type_name: &'static str,
        message: String,
    },

    /// An rkyv deserialization failed.
    #[error("rkyv deserialization failed for type {type_name}: {message}")]
    DeserializationFailed {
        type_name: &'static str,
        message: String,
    },

    /// An rkyv serialization failed.
    #[error("rkyv serialization failed for type {type_name}: {message}")]
    SerializationFailed {
        type_name: &'static str,
        message: String,
    },

    // -- Table operation errors --

    /// A table could not be opened within a transaction.
    #[error("failed to open table {table}: {message}")]
    TableOpenFailed {
        table: &'static str,
        message: String,
    },

    /// A key was not found in the expected table.
    #[error("key not found in {table}")]
    KeyNotFound {
        table: &'static str,
    },

    /// A redb storage error occurred during {operation}.
    #[error("redb storage error during {operation}: {message}")]
    StorageError {
        operation: &'static str,
        message: String,
    },

    /// A write transaction commit failed.
    #[error("failed to commit state changes: {message}")]
    CommitFailed { message: String },

    // -- Constraint violations --

    /// A hash key has the wrong length (not 32 bytes).
    #[error("hash key has wrong length: expected 32 bytes, got {actual}")]
    InvalidHashKeyLength { actual: usize },

    /// A source path key violates the key format invariant.
    #[error("invalid source path key: {reason}")]
    InvalidSourcePath { reason: String },

    /// A URL key violates the URL format invariant.
    #[error("invalid URL key: {reason}")]
    InvalidUrlKey { reason: String },
}
```

### Error-to-failure-mode mapping

| Failure mode | Error variant | Triggering condition |
|---|---|---|
| Database file missing/corrupt | `OpenFailed` | `Database::create(path)` returns error |
| MVCC conflict | `ReadTransactionFailed` / `WriteTransactionFailed` | `db.begin_read()` / `db.begin_write()` returns error |
| Pod value truncated/wrong size | `PodSizeMismatch` | Value bytes `.len() != {200,120}` |
| Pod alignment violation | `PodCastFailed` | `pod_read_unaligned` on misaligned or short buffer |
| Corrupt rkyv bytes | `InvalidArchive` | `rkyv::access` fails (bytecheck) |
| rkyv type mismatch | `DeserializationFailed` | `rkyv::Deserialize` fails |
| rkyv serialization OOM or bug | `SerializationFailed` | `rkyv::to_bytes` fails |
| Table does not exist | `TableOpenFailed` | `open_table` on corrupted DB |
| FK points to missing output | `KeyNotFound` | Hash key present in Pod but absent in output table |
| Disk full / I/O error | `StorageError` | redb internal I/O error |
| Write commit failed | `CommitFailed` | `write_tx.commit()` returns error |
| Non-32-byte hash key | `InvalidHashKeyLength` | Key validation before table write |
| Absolute path or `..` in source_path | `InvalidSourcePath` | Key validation before `file_state` write |
| Non-absolute URL | `InvalidUrlKey` | Key validation before `url_state` write |

---

## Contract Signatures

### Table definition constants

```rust
// Pod state tables (fixed-size value, human-readable key)
const FILE_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("file_state");
const URL_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("url_state");

// rkyv output tables (variable-size value, 32-byte hash key)
const ANALYSIS_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("analysis_outputs");
const TRANSFORM_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("transform_outputs");
const CHUNK_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("chunk_outputs");
const SCRAPE_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("scrape_outputs");
const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("snapshots");

// Metadata table (string key-value)
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");
```

### Table initialization function

```rust
/// Create all 8 tables in a single write transaction.
/// Called once during `StateDb::open()` on a new database.
///
/// # Preconditions
/// - `db` is a valid open redb `Database` handle
///
/// # Postconditions
/// - All 8 tables exist in the database
/// - Returns `Ok(())` on success
/// - Returns `StateError::TableOpenFailed` if any table creation fails
/// - Returns `StateError::CommitFailed` if the write transaction commit fails
fn initialize_tables(db: &Database) -> Result<(), StateError>;
```

### Table access dispatch

```rust
/// Returns the `FILE_STATE_TABLE` definition.
/// Used by `StateReadSession::load_file_states` and `StateDb::commit_changes`.
const fn file_state_table() -> TableDefinition<'static, &'static str, &'static [u8]>;

/// Returns the `URL_STATE_TABLE` definition.
const fn url_state_table() -> TableDefinition<'static, &'static str, &'static [u8]>;

/// Returns the `ANALYSIS_OUTPUTS_TABLE` definition.
const fn analysis_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]>;

/// Returns the `TRANSFORM_OUTPUTS_TABLE` definition.
const fn transform_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]>;

/// Returns the `CHUNK_OUTPUTS_TABLE` definition.
const fn chunk_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]>;

/// Returns the `SCRAPE_OUTPUTS_TABLE` definition.
const fn scrape_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]>;

/// Returns the `SNAPSHOTS_TABLE` definition.
const fn snapshots_table() -> TableDefinition<'static, &'static [u8], &'static [u8]>;

/// Returns the `METADATA_TABLE` definition.
const fn metadata_table() -> TableDefinition<'static, &'static str, &'static str>;
```

### Pod type definitions (defined alongside tables)

```rust
/// Fixed-size file state. 200 bytes. Pod. Zero-copy read from redb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct FileStateRaw {
    content_hash: [u8; 32],
    config_hash: [u8; 32],
    analysis_hash: [u8; 32],
    transform_hash: [u8; 32],
    chunk_hash: [u8; 32],
    last_processed_secs: u64,
    _reserved: [u8; 32],
}
// Static assert: size_of::<FileStateRaw>() == 200

/// Fixed-size URL state. 120 bytes. Pod. Zero-copy read from redb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct UrlStateRaw {
    content_hash: [u8; 32],
    url_hash: [u8; 32],
    last_fetched_secs: u64,
    status_code: u16,
    _reserved: [u8; 46],
}
// Static assert: size_of::<UrlStateRaw>() == 120
```

---

## Non-goals

1. **No rkyv derive additions.** Adding `rkyv::Archive`/`Serialize`/`Deserialize` derives to
   `Analysis`, `Chunk`, `Heading`, `Link`, `LinkKind`, `ScrapedPage`, `Snapshot`, `PageHash`,
   `Header`, `PageFilterStatus`, `ChunkType`, `ChunkLevel` is a separate bead.
2. **No `StateDb` implementation.** The `StateDb`, `StateReadSession`, `OwnedArchive<T>`, and
   `StateChanges` structs and their methods are a separate bead.
3. **No `compute_file_diff` implementation.** That is a separate bead.
4. **No legacy `DocCache` migration or removal.** That is a separate bead.
5. **No runtime table name validation.** Table names are compile-time constants enforced by
   `TableDefinition::new("literal")`.
6. **No concurrency primitives beyond redb's built-in MVCC.** redb handles multi-process
   locking. No additional mutex/rwlock needed for state tables.
7. **No migration tooling.** Migrating data from legacy `DocCache` tables to new state tables
   is a separate bead.
