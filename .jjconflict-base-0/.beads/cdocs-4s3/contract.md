# Contract Specification

## Bead Metadata

| Field        | Value                                                        |
|--------------|--------------------------------------------------------------|
| bead_id      | cdocs-4s3                                                    |
| bead_title   | data: implement archived output bulk loaders with transform reuse coverage |
| phase        | 2 - CONTRACT                                                 |
| created      | 2026-04-02                                                   |
| status       | DRAFT                                                        |

## Context

### Feature

Add `StateReadSession` bulk loader methods for the four variable-size output
tables in the redb state database.  Each loader accepts a slice of `[u8; 32]`
hash keys, reads every matching value inside the shared read transaction,
copies the raw bytes into an `OwnedArchive<T>`, and returns a `HashMap` keyed
by the same hashes.  Callers can then reuse cached outputs for unchanged
content without re-running any pipeline stage.

### Domain Terms

| Term                 | Meaning                                                      |
|----------------------|--------------------------------------------------------------|
| `StateReadSession`   | RAII guard holding one shared `redb::ReadTransaction` for the entire command run |
| `OwnedArchive<T>`    | Owns a `Box<[u8]>` of rkyv-serialised bytes; provides `archived()` (zero-copy) and `deserialize()` (owned) access |
| `analysis_outputs`   | redb table: key `[u8; 32]` -> rkyv-serialised `Analysis`     |
| `transform_outputs`  | redb table: key `[u8; 32]` -> rkyv-serialised `String`       |
| `chunk_outputs`      | redb table: key `[u8; 32]` -> rkyv-serialised `Vec<Chunk>`   |
| `scrape_outputs`     | redb table: key `[u8; 32]` -> rkyv-serialised `ScrapedPage`  |
| hash key             | SHA-256 content hash that uniquely identifies a stored output |
| bulk load            | Read N entries in one shared read transaction; zero per-entry transaction overhead |

### Assumptions

1. The redb database and its tables (`analysis_outputs`, `transform_outputs`,
   `chunk_outputs`, `scrape_outputs`) already exist (created by `StateDb::open`
   / `initialize_tables`).
2. All domain types (`Analysis`, `Chunk`, `ScrapedPage`, and their nested
   types) already derive `rkyv::Archive`, `rkyv::Serialize`, and
   `rkyv::Deserialize` (done in prior beads).
3. `OwnedArchive<T>` is already defined with `archived()` and `deserialize()`
   methods (done in prior bead).
4. `StateReadSession` struct already exists and wraps a `redb::ReadTransaction<'db>`.
5. The caller is responsible for providing only well-formed `[u8; 32]` hash
   keys that correspond to legitimate stored outputs.  A key with no stored
   value is **not an error**; it simply does not appear in the returned map.

### Open Questions

None.  All constraints are resolved from the architecture spec.

---

## Preconditions

| ID   | Precondition                                                                                   |
|------|------------------------------------------------------------------------------------------------|
| P-01 | `StateReadSession` holds a live, non-dropped `redb::ReadTransaction`.                          |
| P-02 | The target redb table (analysis, transform, chunk, or scrape) has been created by `initialize_tables`. |
| P-03 | Each hash in the input slice `&[[u8; 32]]` is a well-formed SHA-256 digest.                    |
| P-04 | The read transaction has not been invalidated by a concurrent write conflict.                   |

---

## Postconditions

| ID   | Postcondition                                                                                                   |
|------|------------------------------------------------------------------------------------------------------------------|
| Q-01 | The returned `HashMap` contains exactly those hashes from the input that had a matching stored value.            |
| Q-02 | Each value in the returned map is a valid `OwnedArchive<T>` whose bytes passed rkyv `bytecheck` validation.      |
| Q-03 | The `OwnedArchive<T>` bytes are fully independent of the `redb::AccessGuard` lifetime (heap-owned `Box<[u8]>`).  |
| Q-04 | All reads occur within the single shared `ReadTransaction` held by the session; no additional transactions opened. |
| Q-05 | The read transaction remains alive and usable after the call returns (no consumption or invalidation).            |
| Q-06 | Hashes absent from the table are silently omitted; no error, no default, no placeholder.                         |
| Q-07 | Duplicate hashes in the input produce a single entry in the output map (idempotent deduplication).               |

---

## Invariants

| ID   | Invariant                                                                                                      |
|------|------------------------------------------------------------------------------------------------------------------|
| I-01 | **Transaction scope**: all table reads happen inside the `StateReadSession`'s `ReadTransaction`. No write transaction is opened. |
| I-02 | **No serde / no bincode**: all (de)serialization uses rkyv exclusively. No `serde_json`, no `bincode` anywhere in the load path. |
| I-03 | **Ownership transfer**: bytes are copied out of the `redb::AccessGuard` into a `Box<[u8]>` before the guard is dropped. The `OwnedArchive<T>` is fully self-contained. |
| I-04 | **Deterministic**: given the same database state and the same input hashes, the same output map is always produced. |
| I-05 | **Fail-fast on corruption**: if `bytecheck` validation fails for any entry, the entire bulk load returns an error immediately. No partial results. |
| I-06 | **Key identity**: the `[u8; 32]` used as the `HashMap` key is the exact same bytes as the redb table key and the original input hash. No hashing of hashes. |

---

## Error Taxonomy

### `BulkLoadError` enum

```rust
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum BulkLoadError {
    /// The redb table could not be opened for reading.
    #[error("bulk load failed: cannot open table '{table}': {message}")]
    TableOpen {
        table: &'static str,
        message: String,
    },

    /// A redb storage-level error occurred while reading a value.
    #[error("bulk load failed: storage error reading table '{table}': {message}")]
    StorageError {
        table: &'static str,
        message: String,
    },

    /// A stored value's bytes failed rkyv bytecheck validation.
    /// The `key_hex` field identifies which entry is corrupt.
    #[error("bulk load failed: corrupt archived payload for key {key_hex} in table '{table}': {message}")]
    CorruptPayload {
        table: &'static str,
        key_hex: String,
        message: String,
    },
}
```

### Error-condition mapping

| Error Variant       | When it occurs                                                              |
|---------------------|-----------------------------------------------------------------------------|
| `TableOpen`         | `read_txn.open_table(table_def)` returns a redb error (table missing, I/O)  |
| `StorageError`      | `table.get(&key)` or table iteration returns a redb error (I/O, storage)   |
| `CorruptPayload`    | Stored bytes fail rkyv `bytecheck` validation when constructing `OwnedArchive` |

### What is NOT an error

| Situation                        | Behaviour                                    |
|----------------------------------|----------------------------------------------|
| Hash not found in table          | Silently omitted from returned `HashMap`     |
| Empty input hash slice `&[]`     | Returns empty `HashMap` (zero entries)       |
| Duplicate hashes in input slice  | Single entry in returned `HashMap`           |

---

## Contract Signatures

### `StateReadSession` bulk loader methods

```rust
impl<'db> StateReadSession<'db> {
    /// Bulk load archived `Analysis` outputs for the requested hashes.
    ///
    /// Reads every matching entry from the `analysis_outputs` table inside
    /// the shared read transaction. Returns a `HashMap` mapping each found
    /// hash to an `OwnedArchive<Analysis>`.
    ///
    /// # Preconditions
    /// - P-01: Read transaction is live.
    /// - P-02: `analysis_outputs` table exists.
    /// - P-03: Each hash is a well-formed SHA-256 digest.
    /// - P-04: No concurrent write conflict.
    ///
    /// # Postconditions
    /// - Q-01 through Q-07 apply.
    ///
    /// # Errors
    /// - `BulkLoadError::TableOpen` if the table cannot be opened.
    /// - `BulkLoadError::StorageError` on redb I/O failure.
    /// - `BulkLoadError::CorruptPayload` if stored bytes fail rkyv validation.
    pub fn load_analyses(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Analysis>>, BulkLoadError>;

    /// Bulk load archived transform outputs (`String`) for the requested hashes.
    ///
    /// # Preconditions
    /// - P-01: Read transaction is live.
    /// - P-02: `transform_outputs` table exists.
    /// - P-03: Each hash is a well-formed SHA-256 digest.
    /// - P-04: No concurrent write conflict.
    ///
    /// # Postconditions
    /// - Q-01 through Q-07 apply.
    ///
    /// # Errors
    /// - `BulkLoadError::TableOpen` if the table cannot be opened.
    /// - `BulkLoadError::StorageError` on redb I/O failure.
    /// - `BulkLoadError::CorruptPayload` if stored bytes fail rkyv validation.
    pub fn load_transforms(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<String>>, BulkLoadError>;

    /// Bulk load archived chunk outputs (`Vec<Chunk>`) for the requested hashes.
    ///
    /// # Preconditions
    /// - P-01: Read transaction is live.
    /// - P-02: `chunk_outputs` table exists.
    /// - P-03: Each hash is a well-formed SHA-256 digest.
    /// - P-04: No concurrent write conflict.
    ///
    /// # Postconditions
    /// - Q-01 through Q-07 apply.
    ///
    /// # Errors
    /// - `BulkLoadError::TableOpen` if the table cannot be opened.
    /// - `BulkLoadError::StorageError` on redb I/O failure.
    /// - `BulkLoadError::CorruptPayload` if stored bytes fail rkyv validation.
    pub fn load_chunks(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Vec<Chunk>>>, BulkLoadError>;

    /// Bulk load archived scrape outputs (`ScrapedPage`) for the requested hashes.
    ///
    /// # Preconditions
    /// - P-01: Read transaction is live.
    /// - P-02: `scrape_outputs` table exists.
    /// - P-03: Each hash is a well-formed SHA-256 digest.
    /// - P-04: No concurrent write conflict.
    ///
    /// # Postconditions
    /// - Q-01 through Q-07 apply.
    ///
    /// # Errors
    /// - `BulkLoadError::TableOpen` if the table cannot be opened.
    /// - `BulkLoadError::StorageError` on redb I/O failure.
    /// - `BulkLoadError::CorruptPayload` if stored bytes fail rkyv validation.
    pub fn load_scrapes(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<ScrapedPage>>, BulkLoadError>;
}
```

### Supporting type: `OwnedArchive<T>` (already exists from prior beads, restated for completeness)

```rust
/// Owned wrapper around rkyv-archived bytes.
/// Decouples the archived view from the redb transaction lifetime.
#[derive(Debug)]
pub struct OwnedArchive<T: rkyv::Archive> {
    bytes: Box<[u8]>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: rkyv::Archive> OwnedArchive<T> {
    /// Construct from raw bytes. Validates rkyv bytecheck on construction.
    ///
    /// # Errors
    /// Returns `BulkLoadError::CorruptPayload` if bytecheck fails.
    pub fn try_from_bytes(
        table: &'static str,
        key: &[u8; 32],
        bytes: Box<[u8]>,
    ) -> Result<Self, BulkLoadError>;

    /// Zero-copy access to the archived root.
    /// Lifetime is tied to `&self`, NOT to any redb transaction.
    pub fn archived(&self) -> &T::Archived;
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive
        + rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
{
    /// Full deserialize into an owned value.
    pub fn deserialize(&self) -> Result<T, BulkLoadError>;
}
```

---

## Non-Goals

- Writing new output values to redb tables (handled by `StateDb::commit_changes`).
- Loading `FileStateRaw` / `UrlStateRaw` Pod types (handled by `load_file_states` / `load_url_states` via bytemuck, separate bead).
- Loading snapshots (handled by `load_snapshots`, separate bead for watch/apply).
- Migrating the existing `DocCache` serde-based code (separate migration bead).
- Providing individual single-key get methods (not needed; callers work in bulk).
- Streaming / lazy iteration over table entries (all loads are eager and complete).
