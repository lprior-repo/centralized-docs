# Contract Specification: `StateDb::open` and redb Table Initialization

## Context

- **Feature**: cdocs-uk1 -- `StateDb` wrapper with `open(path)` and internal table initialization
- **Module**: `centralized-docs/src/state/` (`mod.rs` for schema/tables, `commit.rs` for `StateDb`)
- **Domain terms**:
  - **StateDb**: Newtype wrapper over `redb::Database`. Owns the state database lifecycle.
  - **State path**: A `&Path` to a `.redb` file on disk. Parent directories are created if absent.
  - **Table initialization**: A single write transaction that calls `open_table` for all 8 tables
    (idempotent -- redb creates if absent, succeeds silently if present).
  - **StateChanges**: Batch of mutations consumed by `commit_changes`. Moved (not Clone).
  - **StateReadSession**: Scoped read transaction. Must be dropped before `commit_changes`.
  - **CommitError**: Error taxonomy for the commit pipeline (used by `StateDb::open` for
    `DatabaseOpen` and `TableInit` variants).
  - **StateError**: Error taxonomy for schema-level operations (Pod read/write, table ops,
    validation). Used by `initialize_tables`, `bulk_load`, and Pod helpers.
- **Assumptions**:
  - The crate uses `#![forbid(unsafe_code)]`, `#![deny(clippy::unwrap_used)]`,
    `#![deny(clippy::expect_used)]`, `#![deny(clippy::panic)]`.
  - redb 2.x is the backend. `Database::create()` creates or opens an existing database.
  - `open_table` on a `WriteTransaction` is idempotent (creates table if absent, no-op if present).
  - The database file uses a dedicated path separate from the legacy `DocCache` path.
  - Coexistence with legacy `DocCache` tables in the same `.redb` file is supported but not
    required by this contract (tested separately in `mod.rs` tests).
- **Open questions**: None. Implementation is complete and tests pass.

---

## Types (Contract Signatures)

### `StateDb` (newtype wrapper)

```rust
/// State database providing the two-transaction architecture.
/// Wraps a `redb::Database`. Does NOT support in-memory LRU mode.
#[derive(Debug)]
pub struct StateDb {
    db: Database, // private field -- not constructible outside StateDb::open
}
```

**Ownership contract**: `StateDb` owns the `redb::Database`. Callers receive an owned `StateDb`
instance. The inner `db` field is private; access only through `StateDb::database()`.

### `StateDb::open`

```rust
impl StateDb {
    /// Open the state database at the given path.
    ///
    /// Creates the database and all required tables if they do not exist.
    /// Parent directories are created automatically.
    pub fn open(path: &Path) -> Result<StateDb, CommitError>;
}
```

### `StateDb::begin_read`

```rust
impl StateDb {
    /// Open a single shared read transaction for the command's lifetime.
    ///
    /// The caller MUST drop the session before calling commit_changes.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, CommitError>;
}
```

### `StateDb::commit_changes`

```rust
impl StateDb {
    /// Commit all state changes in exactly one redb write transaction.
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError>;
}
```

### `StateDb::database`

```rust
impl StateDb {
    /// Get a reference to the underlying redb database.
    #[must_use]
    pub fn database(&self) -> &Database;
}
```

### `StateReadSession`

```rust
/// A scoped read transaction. One per command run.
/// Must be dropped before calling StateDb::commit_changes.
pub struct StateReadSession<'db> {
    read_txn: redb::ReadTransaction,
    _phantom: std::marker::PhantomData<&'db ()>,
}
```

### `initialize_tables` (free function in `mod.rs`)

```rust
/// Create all 8 tables in a single write transaction.
/// Idempotent: redb open_table creates if absent, succeeds silently if present.
pub fn initialize_tables(db: &Database) -> Result<(), StateError>;
```

### `StateChanges` (batch type in `commit.rs`)

```rust
/// Batch of state mutations to commit atomically.
/// Moved into commit_changes (consumed). Not Clone by design.
pub struct StateChanges {
    pub updated_files: Vec<(String, FileStateRaw)>,
    pub deleted_files: Vec<String>,
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,
    pub updated_urls: Vec<(String, UrlStateRaw)>,
    pub deleted_urls: Vec<String>,
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    pub deleted_snapshots: Vec<[u8; 32]>,
}
```

---

## Preconditions

### `StateDb::open(path)`

- [P-O1] `path` may be any valid filesystem path. Empty path `Path::new("")` is permitted
  by the type system but will fail at the `Database::create` call.
- [P-O2] The filesystem must be writable at `path` or its parent directory.
  If `path` has a non-empty parent, `create_dir_all` is called before opening the database.
- [P-O3] If `path` has an empty parent (e.g., just a filename like `"state.redb"`), directory
  creation is skipped (no `create_dir_all("")` call).
- [P-O4] No other `redb::Database` handle may be open on the same file path concurrently
  (redb enforces this via file locking).

### `StateDb::begin_read()`

- [P-R1] `self` must be a valid, open `StateDb` (i.e., `open()` returned `Ok`).
- [P-R2] Only one `StateReadSession` should be alive at a time per `StateDb` (protocol
  contract, not enforced by types).

### `StateDb::commit_changes(changes)`

- [P-C1] `changes.updated_files` must contain no duplicate `source_path` keys.
- [P-C2] `changes.updated_urls` must contain no duplicate URL keys.
- [P-C3] No `source_path` in `updated_files` may be empty or whitespace-only after trimming.
- [P-C4] No URL in `updated_urls` may be empty or whitespace-only after trimming.
- [P-C5] Every hash key in `new_analyses`, `new_transforms`, `new_chunks`, `new_scrapes`,
  `new_snapshots` must NOT be the zero hash `[0u8; 32]`.
- [P-C6] Every payload value must be <= `MAX_VALUE_SIZE` (50 MiB).
- [P-C7] Reference integrity: every non-zero `analysis_hash`, `transform_hash`, `chunk_hash`
  in `updated_files` must have a matching entry in `new_analyses`/`new_transforms`/`new_chunks`.
  Every non-zero `url_hash` in `updated_urls` must have a matching entry in `new_scrapes`.
- [P-C8] Zero hashes (`[0u8; 32]`) in `FileStateRaw`/`UrlStateRaw` fields represent "no output yet"
  and are accepted without requiring a matching payload entry.
- [P-C9] All precondition validation is performed BEFORE opening the write transaction
  (pure computation, no side effects).

### `initialize_tables(db)`

- [P-I1] `db` must be a valid, open `redb::Database`.
- [P-I2] No concurrent write transaction may be active on `db` (redb serializes writes).

---

## Postconditions

### `StateDb::open(path)` -- on `Ok`

- [Q-O1] All parent directories of `path` exist (created if they didn't).
- [Q-O2] The redb database file exists at `path` (created if it didn't, opened if it did).
- [Q-O3] All 8 tables are initialized in the database:
  `file_state`, `url_state`, `analysis_outputs`, `transform_outputs`,
  `chunk_outputs`, `scrape_outputs`, `snapshots`, `metadata`.
- [Q-O4] `initialize_tables` was called exactly once during this `open` call.
- [Q-O5] If the database already existed with tables, no data was destroyed or modified
  (idempotent table initialization).
- [Q-O6] The returned `StateDb` is usable: `begin_read()` and `commit_changes()` will succeed
  (modulo external failures).

### `StateDb::open(path)` -- on `Err`

- [Q-O7] If `Err(CommitError::DatabaseOpen { .. })` is returned, the redb database was NOT
  opened. The cause may be:
  - `create_dir_all` failure for parent directories
  - `Database::create` failure (permissions, invalid path, concurrent lock)
- [Q-O8] If `Err(CommitError::TableInit { .. })` is returned, the database was opened but
  table initialization failed. The database file may exist in a partially initialized state.

### `StateDb::begin_read()` -- on `Ok`

- [Q-R1] A `StateReadSession` is returned holding an active read transaction.
- [Q-R2] The `StateReadSession` borrows `self` (`'db` lifetime), preventing `StateDb` from
  being dropped while the session is alive.

### `StateDb::commit_changes(changes)` -- on `Ok`

- [Q-C1] Exactly one write transaction was opened and committed.
- [Q-C2] All entries in `updated_files` are persisted to the `file_state` table.
- [Q-C3] All entries in `updated_urls` are persisted to the `url_state` table.
- [Q-C4] All entries in `new_analyses` are persisted to `analysis_outputs` (deduplicated:
  last-write-wins for duplicate hash keys within the batch).
- [Q-C5] All entries in `new_transforms` are persisted to `transform_outputs` (deduplicated).
- [Q-C6] All entries in `new_chunks` are persisted to `chunk_outputs` (deduplicated).
- [Q-C7] All entries in `new_scrapes` are persisted to `scrape_outputs` (deduplicated).
- [Q-C8] All entries in `new_snapshots` are persisted to `snapshots` (deduplicated).
- [Q-C9] All keys in `deleted_files` are removed from `file_state` (nonexistent silently skipped).
- [Q-C10] All keys in `deleted_urls` are removed from `url_state` (nonexistent silently skipped).
- [Q-C11] All keys in `deleted_snapshots` are removed from `snapshots` (nonexistent silently skipped).
- [Q-C12] Unchanged rows (byte-identical to existing value) are NOT rewritten (skip optimization).
- [Q-C13] `changes` is consumed (moved) and cannot be reused.

### `StateDb::commit_changes(changes)` -- on `Err`

- [Q-C14] NO writes are visible. If a write transaction was opened, it was aborted (dropped
  without commit). The database state is unchanged from before the call.
- [Q-C15] If validation fails (P-C1 through P-C8), no write transaction is opened at all.
- [Q-C16] If the write transaction commit fails after writes are applied, redb guarantees
  atomicity -- no partial writes are visible.

### `initialize_tables(db)` -- on `Ok`

- [Q-I1] All 8 tables exist in the database (created if absent, unchanged if present).
- [Q-I2] Exactly one write transaction was opened and committed.
- [Q-I3] Existing data in any of the 8 tables is preserved.

---

## Invariants

### Structural Invariants

- [INV-S1] `StateDb` wraps exactly one `redb::Database`. It is not clonable or copyable.
- [INV-S2] The `db` field of `StateDb` is always a valid, open `redb::Database` handle
  (established by `open()`, invariant holds until `StateDb` is dropped).
- [INV-S3] After `StateDb::open()` returns `Ok`, all 8 tables are guaranteed to exist.
- [INV-S4] `StateChanges` is not `Clone`. One batch per command run, one commit, then dropped.
- [INV-S5] `StateReadSession` borrows `StateDb` via `PhantomData<&'db ()>`, preventing
  concurrent drops. It does NOT prevent concurrent `commit_changes` (protocol contract).

### Table Schema Invariants

- [INV-T1] `file_state` table: `&str` keys (source paths), `&[u8]` values (200-byte `FileStateRaw`).
- [INV-T2] `url_state` table: `&str` keys (canonical URLs), `&[u8]` values (120-byte `UrlStateRaw`).
- [INV-T3] `analysis_outputs` table: `&[u8]` keys (32-byte hashes), `&[u8]` values (rkyv-archived bytes).
- [INV-T4] `transform_outputs` table: `&[u8]` keys (32-byte hashes), `&[u8]` values (rkyv bytes).
- [INV-T5] `chunk_outputs` table: `&[u8]` keys (32-byte hashes), `&[u8]` values (rkyv bytes).
- [INV-T6] `scrape_outputs` table: `&[u8]` keys (32-byte hashes), `&[u8]` values (rkyv bytes).
- [INV-T7] `snapshots` table: `&[u8]` keys (32-byte hashes), `&[u8]` values (rkyv bytes).
- [INV-T8] `metadata` table: `&str` keys, `&str` values.
- [INV-T9] All 8 table names are unique within the database.

### Data Integrity Invariants

- [INV-D1] Hash keys in payload tables (`analysis_outputs`, `transform_outputs`, `chunk_outputs`,
  `scrape_outputs`, `snapshots`) are always exactly 32 bytes.
- [INV-D2] The zero hash `[0u8; 32]` is never used as a key in any payload table.
- [INV-D3] `FileStateRaw` values are always exactly 200 bytes.
- [INV-D4] `UrlStateRaw` values are always exactly 120 bytes.
- [INV-D5] Reference integrity: every non-zero `analysis_hash` in `file_state` has a corresponding
  entry in `analysis_outputs` (enforced by `commit_changes` precondition P-C7).
- [INV-D6] Payload values never exceed `MAX_VALUE_SIZE` (50 MiB) (enforced by P-C6).

### Concurrency Invariants

- [INV-X1] redb serializes write transactions. At most one write transaction is active at a time.
- [INV-X2] Read transactions can proceed concurrently with each other.
- [INV-X3] Read transactions can proceed concurrently with a write transaction (readers see a
  snapshot from before the write).

---

## Error Taxonomy

### `CommitError` (used by `StateDb::open`, `begin_read`, `commit_changes`)

```
CommitError
  |
  +-- DatabaseOpen { path: String, reason: String }
  |     StateDb::open fails: create_dir_all error OR Database::create error
  |
  +-- TableInit { reason: String }
  |     StateDb::open fails: initialize_tables returns StateError
  |
  +-- ReadTransaction { reason: String }
  |     StateDb::begin_read fails: db.begin_read() error
  |
  +-- WriteTransaction { reason: String }
  |     StateDb::commit_changes fails: db.begin_write() error
  |
  +-- WriteFailed { table: &'static str, reason: String }
  |     commit_changes fails: individual table write error
  |
  +-- CommitFailed { reason: String }
  |     commit_changes fails: write transaction commit error
  |
  +-- ReadFailed { table: &'static str, reason: String }
  |     commit_changes fails: read within write transaction error
  |
  +-- ZeroHashKey { table: &'static str, index: usize }
  |     Precondition violation: zero hash in payload vec at index
  |
  +-- EmptyStringKey { table: &'static str, index: usize }
  |     Precondition violation: empty/whitespace string key at index
  |
  +-- DuplicateStateKey { table: &'static str, key: String }
  |     Precondition violation: duplicate string key in batch
  |
  +-- MissingReference { table, field, hash_hex, payload_table }
  |     Precondition violation: hash in state row has no matching payload
  |
  +-- PayloadTooLarge { table: &'static str, size: usize, max: usize }
        Precondition violation: payload exceeds MAX_VALUE_SIZE
```

### `StateError` (used by `initialize_tables`, Pod helpers, key validators)

```
StateError
  |
  +-- OpenFailed { path: PathBuf, detail: String }
  |     Database file could not be opened
  |
  +-- ReadTransactionFailed { message: String }
  |     Read transaction start failure
  |
  +-- WriteTransactionFailed { message: String }
  |     Write transaction start failure
  |
  +-- PodSizeMismatch { table: &'static str, expected: usize, actual: usize }
  |     Pod value has wrong byte count
  |
  +-- PodCastFailed { type_name: &'static str, message: String }
  |     Pod field extraction failure
  |
  +-- InvalidArchive { type_name: &'static str, message: String }
  |     rkyv archive validation failure
  |
  +-- DeserializationFailed { type_name: &'static str, message: String }
  |     rkyv deserialization failure
  |
  +-- SerializationFailed { type_name: &'static str, message: String }
  |     rkyv serialization failure
  |
  +-- TableOpenFailed { table: &'static str, message: String }
  |     Table open/create failure
  |
  +-- KeyNotFound { table: &'static str }
  |     Expected key missing
  |
  +-- StorageError { operation: &'static str, message: String }
  |     redb storage-level error
  |
  +-- CommitFailed { message: String }
  |     Write transaction commit failure
  |
  +-- InvalidHashKeyLength { actual: usize }
  |     Hash key != 32 bytes
  |
  +-- InvalidSourcePath { reason: String }
  |     Source path validation failure
  |
  +-- InvalidUrlKey { reason: String }
        URL key validation failure
```

### Error mapping: `StateDb::open` error flow

```
StateDb::open(path)
  |
  +-- create_dir_all(parent) fails
  |     => CommitError::DatabaseOpen { path, reason }
  |
  +-- Database::create(path) fails
  |     => CommitError::DatabaseOpen { path, reason }
  |
  +-- initialize_tables(&db) fails
  |     => StateError::* is caught and mapped to CommitError::TableInit { reason }
  |
  +-- All succeed => Ok(StateDb { db })
```

---

## Function Contracts (Summary Table)

| Function | Input | Output | Fail Modes |
|----------|-------|--------|------------|
| `StateDb::open(path)` | `&Path` | `Result<StateDb, CommitError>` | `DatabaseOpen`, `TableInit` |
| `StateDb::begin_read()` | `&self` | `Result<StateReadSession<'_>, CommitError>` | `ReadTransaction` |
| `StateDb::commit_changes(changes)` | `&self, StateChanges` (moved) | `Result<(), CommitError>` | All `CommitError` variants |
| `StateDb::database()` | `&self` | `&Database` (infallible) | None |
| `initialize_tables(db)` | `&Database` | `Result<(), StateError>` | `WriteTransactionFailed`, `TableOpenFailed`, `CommitFailed` |
| `StateChanges::empty()` | None | `StateChanges` (infallible) | None |
| `should_skip_write(existing, new)` | `&[u8], &[u8]` | `bool` (pure) | None |

---

## Ordering Guarantees

1. **Directory creation** happens before `Database::create` (sequential in `open`).
2. **Table initialization** happens after `Database::create` but before `open` returns.
3. **Precondition validation** in `commit_changes` happens before the write transaction is opened.
4. **Payload writes** happen before state upserts within the write transaction (so referenced
   data exists before FK references are written).
5. **Deletes** happen after all upserts within the same write transaction.
6. **Commit** is the final step. Any earlier error causes the transaction to be dropped (aborted).

---

## Non-goals

- This contract does NOT cover `StateReadSession` bulk-load methods (deferred to `bulk_load.rs`).
- This contract does NOT cover in-memory/LRU mode for state (StateDb is file-backed only).
- This contract does NOT cover migration from legacy `DocCache` tables (tested but not
  specified here).
- This contract does NOT cover CLI integration or argument parsing.
- This contract does NOT cover redb internals (crash recovery, WAL behavior, checksums).
