# Contract Specification

## Phase Metadata

| Field       | Value                                                              |
|-------------|--------------------------------------------------------------------|
| bead_id     | cdocs-b3v                                                          |
| bead_title  | data: implement raw state bulk loaders on `StateReadSession`       |
| phase       | contract (Design-by-Contract)                                      |
| status      | ready for test-plan                                                |
| schema_id   | centralized-docs-20260402083758-yhtgoxwp                           |

## Context

### Feature

Implement `load_file_states` and `load_url_states` as read-only methods on a
`StateReadSession` struct.  These methods bulk-scan redb table rows that hold
raw tracked-state bytes (file-state and URL-state), decode each value through
bytemuck-safe `Pod` reads (zero-copy cast), and return typed `HashMap` results.
Malformed rows are rejected with a semantic error rather than silently producing
partially-decoded state.

### Domain Terms

| Term                | Meaning                                                                  |
|---------------------|--------------------------------------------------------------------------|
| `StateReadSession`  | Session struct that borrows the shared `redb::ReadTransaction` for a run |
| `FileStateRaw`      | Pod-compatible `#[repr(C)]` struct representing one file's tracked state |
| `UrlStateRaw`       | Pod-compatible `#[repr(C)]` struct representing one URL's tracked state  |
| `Pod`               | bytemuck trait: safe for zero-copy `&[u8] -> &T` re-interpretation      |
| Bulk scan           | Full-table iteration within an existing read transaction (no new tx)     |
| `SNAPSHOTS_TABLE`   | Existing redb table (`&[u8] -> &[u8]`) that stores file state rows       |
| `SCRAPE_TABLE`      | Existing redb table (`&[u8] -> &[u8]`) that stores URL state rows        |

### Assumptions

1. `bytemuck` will be added as a dependency (not yet in `Cargo.toml`).
2. `FileStateRaw` and `UrlStateRaw` are `#[repr(C)]` structs that implement
   `bytemuck::Pod` and `bytemuck::Zeroable`. Their exact field layout is
   defined by the implementer but **must** contain at minimum:
   - A SHA-256 content hash (`[u8; 32]`).
   - A last-seen timestamp (`u64` epoch seconds).
3. The existing `SNAPSHOTS_TABLE` and `SCRAPE_TABLE` table definitions in
   `cache/mod.rs` are reused as the storage for raw state rows.
4. `StateReadSession` wraps a `redb::ReadTransaction` and is constructed
   once per pipeline run; it is **not** `Clone` or `Send` (transaction-bound).
5. Keys in redb tables are canonical paths (file) or canonical URLs (URL)
   encoded as UTF-8 bytes.

### Open Questions

_None -- bead clarification_status is RESOLVED._

---

## Type Definitions

### Raw State Structs (Pod-compatible)

```rust
/// Raw file-state row stored in `SNAPSHOTS_TABLE`.
/// `#[repr(C)]` guarantees bytemuck-safe layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStateRaw {
    /// SHA-256 content hash of the file.
    pub content_hash: [u8; 32],
    /// Last-seen timestamp (seconds since Unix epoch).
    pub last_seen_epoch_s: u64,
    // Total size: 32 + 8 = 40 bytes
}

// SAFETY: FileStateRaw is #[repr(C)], no padding bytes, all fields Pod.
unsafe impl bytemuck::Pod for FileStateRaw {}
unsafe impl bytemuck::Zeroable for FileStateRaw {}

/// Raw URL-state row stored in `SCRAPE_TABLE`.
/// `#[repr(C)]` guarantees bytemuck-safe layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlStateRaw {
    /// SHA-256 content hash of the scraped page.
    pub content_hash: [u8; 32],
    /// Last-seen timestamp (seconds since Unix epoch).
    pub last_seen_epoch_s: u64,
    // Total size: 32 + 8 = 40 bytes
}

// SAFETY: UrlStateRaw is #[repr(C)], no padding bytes, all fields Pod.
unsafe impl bytemuck::Pod for UrlStateRaw {}
unsafe impl bytemuck::Zeroable for UrlStateRaw {}
```

### Session Struct

```rust
/// Read-only session that borrows the shared `redb::ReadTransaction`
/// for a single pipeline run. All bulk loaders operate within this
/// transaction -- no additional transactions are opened.
pub struct StateReadSession<'tx> {
    read_tx: &'tx redb::ReadTransaction,
}
```

---

## Contract Signatures

```rust
impl<'tx> StateReadSession<'tx> {
    /// Construct a new read session borrowing the given transaction.
    ///
    /// # Preconditions
    /// - `read_tx` is a valid, live `redb::ReadTransaction`.
    ///
    /// # Postconditions
    /// - Returned session borrows `read_tx` for lifetime `'tx`.
    /// - No I/O is performed during construction.
    pub fn new(read_tx: &'tx redb::ReadTransaction) -> Self;

    /// Bulk-load all file-state rows from `SNAPSHOTS_TABLE`.
    ///
    /// Scans every row in the snapshots table, decodes each value
    /// through bytemuck-safe `Pod` cast, and returns a map from
    /// canonical path (String) to `FileStateRaw`.
    ///
    /// # Preconditions
    /// - `self.read_tx` is a valid, live read transaction.
    /// - `SNAPSHOTS_TABLE` exists (guaranteed by `DocCache::initialize_tables`).
    ///
    /// # Postconditions
    /// - Every row whose value length == `size_of::<FileStateRaw>()` (40 bytes)
    ///   appears in the returned map.
    /// - No new database transaction is opened.
    /// - The returned map is complete for all well-formed rows at the
    ///   transaction's snapshot timestamp.
    ///
    /// # Errors
    /// - `StateLoadError::MalformedRow` if any row's value length != 40 bytes.
    ///   The entire load is aborted; no partial map is returned.
    /// - `StateLoadError::BackendError` if the redb table cannot be opened.
    /// - `StateLoadError::Utf8KeyError` if a key is not valid UTF-8.
    pub fn load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, StateLoadError>;

    /// Bulk-load all URL-state rows from `SCRAPE_TABLE`.
    ///
    /// Scans every row in the scrape table, decodes each value
    /// through bytemuck-safe `Pod` cast, and returns a map from
    /// canonical URL (String) to `UrlStateRaw`.
    ///
    /// # Preconditions
    /// - `self.read_tx` is a valid, live read transaction.
    /// - `SCRAPE_TABLE` exists (guaranteed by `DocCache::initialize_tables`).
    ///
    /// # Postconditions
    /// - Every row whose value length == `size_of::<UrlStateRaw>()` (40 bytes)
    ///   appears in the returned map.
    /// - No new database transaction is opened.
    /// - The returned map is complete for all well-formed rows at the
    ///   transaction's snapshot timestamp.
    ///
    /// # Errors
    /// - `StateLoadError::MalformedRow` if any row's value length != 40 bytes.
    ///   The entire load is aborted; no partial map is returned.
    /// - `StateLoadError::BackendError` if the redb table cannot be opened.
    /// - `StateLoadError::Utf8KeyError` if a key is not valid UTF-8.
    pub fn load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>, StateLoadError>;
}
```

---

## Preconditions

| #  | Precondition                                                                                   | Enforced by               |
|----|------------------------------------------------------------------------------------------------|---------------------------|
| P1 | `StateReadSession` exists and owns (borrows) the shared `ReadTransaction`.                     | `new()` signature + `&'tx` lifetime |
| P2 | `FileStateRaw` and `UrlStateRaw` are `#[repr(C)]` and implement `bytemuck::Pod`.              | Compile-time (trait bounds) |
| P3 | `SNAPSHOTS_TABLE` and `SCRAPE_TABLE` have been initialized via `DocCache::initialize_tables`. | Caller responsibility (`DocCache::open`) |
| P4 | The `ReadTransaction` is live (not cancelled/closed) for the duration of the call.            | Borrow checker (`'tx`)   |
| P5 | redb table keys are valid UTF-8 byte sequences.                                                | Runtime checked; error on violation |

---

## Postconditions

| #  | Postcondition                                                                                                       | Verifiable by                         |
|----|---------------------------------------------------------------------------------------------------------------------|---------------------------------------|
| Q1 | `load_file_states` and `load_url_states` scan their respective tables **without** opening additional transactions.  | Test: same-tx assertion / no second `begin_read` |
| Q2 | Malformed raw values (byte length != `size_of::<FileStateRaw>()` or `size_of::<UrlStateRaw>()`) return an **error** -- no partial map is returned. | Test: corrupt row -> `Err`           |
| Q3 | All well-formed rows present in the table at the transaction snapshot are present in the returned `HashMap`.       | Test: write N rows -> load returns N entries |
| Q4 | Keys in the returned `HashMap` are lossless UTF-8 `String` representations of the raw key bytes.                   | Test: roundtrip key identity         |
| Q5 | Decoded `FileStateRaw` / `UrlStateRaw` values are bitwise-identical to the bytes written.                         | Test: write known struct -> load -> assert eq |
| Q6 | Empty tables produce empty `HashMap`s (no error).                                                                  | Test: empty table -> `Ok(HashMap::new())` |

---

## Invariants

| #  | Invariant                                                                                                  | Category            |
|----|------------------------------------------------------------------------------------------------------------|---------------------|
| I1 | Methods are **read-only** -- no writes to any redb table, no mutations to session state.                   | Purity              |
| I2 | Methods **do not deserialize through serde** -- decoding is via bytemuck `Pod` cast only.                  | Performance/Correctness |
| I3 | `size_of::<FileStateRaw>()` == 40 bytes and `size_of::<UrlStateRaw>()` == 40 bytes.                       | Layout contract     |
| I4 | A single call either returns `Ok(HashMap)` with all well-formed rows, or `Err(StateLoadError)`.           | Atomicity (all-or-nothing per call) |
| I5 | No `unwrap`, `expect`, or `panic` in production code paths.                                               | Holzmann Rule 7     |
| I6 | No `unsafe` beyond the `bytemuck::Pod` / `bytemuck::Zeroable` impls (which are audited).                  | Memory safety       |

---

## Error Taxonomy

```rust
/// Errors that can occur during bulk state loading.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum StateLoadError {
    /// A raw value in the table has an unexpected byte length.
    /// Carries the key, actual byte count, and expected byte count.
    /// The entire load is aborted; no partial map is returned.
    #[error("malformed raw state row for key {key:?}: got {actual} bytes, expected {expected}")]
    MalformedRow {
        key: String,
        actual: usize,
        expected: usize,
    },

    /// A table key is not valid UTF-8.
    #[error("non-UTF-8 key in state table: {bytes_lossy:?}")]
    Utf8KeyError {
        bytes_lossy: String,
    },

    /// The underlying redb backend failed during a table operation.
    #[error("cache backend error during {operation}: {message}")]
    BackendError {
        operation: &'static str,
        message: String,
    },
}
```

### Error-to-Failure-Mode Mapping

| Error variant         | Trigger                                          | Failure mode                    |
|-----------------------|--------------------------------------------------|---------------------------------|
| `MalformedRow`        | Value byte count != `size_of::<T>()`             | Corrupt storage / version skew  |
| `Utf8KeyError`        | Key bytes are not valid UTF-8                    | Data integrity violation        |
| `BackendError`        | `redb` returns an error on `open_table` / `range`| Database I/O failure            |

### Railway-Oriented Error Handling

All fallible operations use `Result<T, StateLoadError>` and the `?` operator.
Errors propagate immediately -- the first malformed row aborts the entire load.
This satisfies the bead contract: "_diff correctness depends on trusted raw hashes_".

---

## Non-goals

1. This contract does **not** define how `FileStateRaw` / `UrlStateRaw` rows are
   written to the tables. That is the responsibility of a separate bead.
2. This contract does **not** add `bytemuck` to `Cargo.toml`. That is a
   prerequisite step for the implementer.
3. This contract does **not** define `StateReadSession` integration with the
   existing `DocCache` public API surface beyond the scope of these two methods.
4. No serde-based deserialization is performed or specified.
5. No concurrent write-during-read guarantees are specified beyond what redb's
   MVCC already provides natively.
