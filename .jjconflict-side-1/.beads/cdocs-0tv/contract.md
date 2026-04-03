---
bead_id: cdocs-0tv
bead_title: "data: add explicit snapshot APIs on `StateReadSession` and `StateDb`"
phase: state-1-contract
updated_at: 2026-04-02T14:00:00Z
---

# Contract Specification

## Context

- **Feature**: Add explicit snapshot read and write APIs to the `StateDb` / `StateReadSession` state database layer, enabling the watch/apply subsystem to persist and retrieve `Snapshot` values through the same two-transaction architecture used by the rest of the pipeline.
- **Domain terms**:
  - `StateDb` -- Wrapper around a redb `Database`; owns the file handle and table definitions. Entry point for opening the database and committing changes.
  - `StateReadSession<'db>` -- Borrows a redb `ReadTransaction` from `StateDb`. All reads happen within this session. One per command invocation.
  - `StateChanges` -- Batch of mutations committed atomically by `StateDb::commit_changes`. Includes `new_snapshots` and `deleted_snapshots` fields.
  - `OwnedArchive<T>` -- Owned wrapper over rkyv-serialized bytes. Provides `archived()` for zero-copy access and `deserialize()` for full ownership.
  - `Snapshot` -- Point-in-time snapshot of all scraped pages for a target URL. Contains `target_url: String`, `timestamp: DateTime<Utc>`, `pages: BTreeMap<String, PageHash>`.
  - Snapshot key -- `[u8; 32]` SHA-256 hash of the target URL (stable identity, same as `url_hash` in current `DocCache` usage).
  - redb `snapshots` table -- `TableDefinition<[u8; 32], &[u8]>` storing rkyv-serialized `Snapshot` values.
- **Assumptions**:
  - `StateDb::open`, `StateDb::begin_read`, `StateDb::commit_changes`, and `StateReadSession` already exist as part of earlier beads.
  - The redb `snapshots` table is already defined and initialized during `StateDb::open`.
  - `StateChanges` already has `new_snapshots: Vec<([u8; 32], Vec<u8>)>` and `deleted_snapshots: Vec<[u8; 32]>` fields.
  - `OwnedArchive<T>` is already implemented with `archived()` and `deserialize()` methods.
  - `Snapshot` already has rkyv derives (`rkyv::Archive`, `rkyv::Serialize`, `rkyv::Deserialize`).
  - rkyv serialization uses `rkyv::to_bytes::<rkyv::rancor::Error>(&value)` for writing.
- **Open questions**: None -- all types and table definitions are specified in the architecture spec.

## Preconditions

1. `StateDb` is open and all tables (including `snapshots`) are initialized via `StateDb::open`.
2. `StateReadSession` is obtained from `StateDb::begin_read()` and borrows the underlying redb read transaction for the lifetime of the session.
3. The `snapshots` redb table is defined as `TableDefinition<&[u8], &[u8]>` keyed by 32-byte SHA-256 hash.
4. The `StateChanges` struct has mutable `new_snapshots` and `deleted_snapshots` fields accessible to the caller.
5. Snapshot keys are `[u8; 32]` values computed by `url_hash(url)` -- the same hashing function used by the existing `DocCache` snapshot persistence, ensuring key stability across migration.
6. `Snapshot` implements `rkyv::Archive` so that `OwnedArchive<Snapshot>` is constructible.

## Postconditions

1. `StateReadSession::load_snapshots(hashes: &[[u8; 32]]) -> Result<HashMap<[u8; 32], OwnedArchive<Snapshot>>>` returns an `OwnedArchive<Snapshot>` for every requested hash that has a persisted entry in the `snapshots` table.
2. Hashes not found in the `snapshots` table are absent from the returned `HashMap` (no error for missing keys -- the caller interprets absence as "first run").
3. All returned `OwnedArchive<Snapshot>` values own their bytes independently of the redb read transaction lifetime; callers can hold them after the `StateReadSession` is dropped.
4. `StateChanges::new_snapshots` entries are committed to the `snapshots` table by `StateDb::commit_changes` using a single redb write transaction.
5. `StateChanges::deleted_snapshots` entries are removed from the `snapshots` table by `StateDb::commit_changes` in the same write transaction.
6. Snapshot writes (new) and deletes are applied atomically within the same write transaction as all other state changes -- ACID semantics.

## Invariants

1. **Key stability**: Snapshot load semantics remain keyed by the existing stable snapshot identity -- SHA-256 of the target URL. No change in key derivation.
2. **One-read, one-write**: Snapshot persistence obeys the one-read-transaction and one-write-transaction invariant per run. `load_snapshots` uses the shared `StateReadSession` read transaction; snapshot writes go through `commit_changes` which opens the single write transaction after the read session is dropped.
3. **Owned-bytes independence**: No `OwnedArchive<Snapshot>` returned from `load_snapshots` borrows bytes from a dropped redb transaction. The bytes are copied into the `OwnedArchive` box at load time.
4. **rkyv byte validity**: All bytes written to the `snapshots` table are valid rkyv archives of `Snapshot`. Invalid bytes are never persisted; validation happens at serialization time.
5. **Empty-hash input**: Passing an empty `hashes` slice to `load_snapshots` returns an empty `HashMap` without error and without any table access.
6. **Duplicate key handling**: If `new_snapshots` contains duplicate keys, the last entry wins (consistent with redb insert semantics). If a key appears in both `new_snapshots` and `deleted_snapshots`, the delete takes precedence (applied after insert in the write transaction).

## Error Taxonomy

All errors use `Result<T, StateError>` where `StateError` is defined as:

```rust
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// The redb database could not be opened (I/O, corruption, lock).
    #[error("state database open failed: {message}")]
    DatabaseOpenFailed { message: String },

    /// A redb read transaction could not be started.
    #[error("failed to begin read transaction: {message}")]
    ReadTransactionFailed { message: String },

    /// A redb write transaction could not be started.
    #[error("failed to begin write transaction: {message}")]
    WriteTransactionFailed { message: String },

    /// The redb table could not be opened.
    #[error("failed to open redb table '{table}': {message}")]
    TableOpenFailed { table: &'static str, message: String },

    /// A redb storage operation failed (generic backend error).
    #[error("redb storage error during {operation}: {message}")]
    StorageError { operation: &'static str, message: String },

    /// rkyv serialization failed when preparing bytes for write.
    #[error("snapshot serialization failed: {message}")]
    SerializationFailed { message: String },

    /// rkyv deserialization failed -- archived bytes are corrupt or invalid.
    #[error("snapshot deserialization failed for key {key_hex}: {message}")]
    DeserializationFailed { key_hex: String, message: String },

    /// rkyv bytecheck validation failed -- bytes do not represent a valid archive.
    #[error("snapshot archive validation failed for key {key_hex}: {message}")]
    ArchiveValidationFailed { key_hex: String, message: String },

    /// A redb commit operation failed after writes.
    #[error("failed to commit state changes: {message}")]
    CommitFailed { message: String },

    /// An I/O error occurred (e.g., creating parent directories).
    #[error("I/O error: {message}")]
    Io { message: String },
}
```

### Error mapping to calling code

| Situation | Error variant | When |
|-----------|---------------|------|
| `StateDb::open` fails to create/open redb file | `DatabaseOpenFailed` | Database creation |
| `begin_read()` fails | `ReadTransactionFailed` | Read session start |
| `snapshots` table cannot be opened for read | `TableOpenFailed { table: "snapshots" }` | Load |
| rkyv `to_bytes` fails for a `Snapshot` value | `SerializationFailed` | Write preparation |
| Stored bytes fail rkyv validation during load | `ArchiveValidationFailed` | Load |
| Stored bytes fail rkyv deserialization | `DeserializationFailed` | Load |
| `begin_write()` fails | `WriteTransactionFailed` | Commit |
| `snapshots` table cannot be opened for write | `TableOpenFailed { table: "snapshots" }` | Commit |
| `write_tx.commit()` fails | `CommitFailed` | Commit |
| Filesystem error creating parent dirs | `Io` | Database open |

## Contract Signatures

### `StateReadSession` -- Snapshot Read

```rust
impl<'db> StateReadSession<'db> {
    /// Bulk load archived snapshots for the requested hashes.
    ///
    /// Returns a HashMap keyed by the hashes that were found. Hashes with no
    /// persisted entry are simply absent from the map (no error).
    ///
    /// # Preconditions
    /// - `self` holds a valid redb read transaction.
    /// - The `snapshots` table is initialized.
    ///
    /// # Postconditions
    /// - Every entry in the returned map contains an `OwnedArchive<Snapshot>`
    ///   whose bytes are owned independently of the redb transaction.
    /// - Empty `hashes` input returns an empty HashMap without table access.
    ///
    /// # Errors
    /// - `StateError::TableOpenFailed` if the snapshots table cannot be opened.
    /// - `StateError::ArchiveValidationFailed` if stored bytes fail rkyv check.
    /// - `StateError::DeserializationFailed` if rkyv deserialize fails.
    /// - `StateError::StorageError` if redb read fails.
    fn load_snapshots(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Snapshot>>, StateError>;
}
```

### `StateChanges` -- Snapshot Write Fields

```rust
/// Batch of state changes to commit atomically.
pub struct StateChanges {
    // ... existing fields (updated_files, deleted_files, new_analyses, etc.) ...

    /// New or updated snapshots to persist. Key = SHA-256 of target URL.
    /// Value = rkyv-serialized Snapshot bytes.
    /// Last entry wins on duplicate keys.
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,

    /// Snapshot keys to delete. Delete takes precedence over insert for the same key.
    pub deleted_snapshots: Vec<[u8; 32]>,
}
```

### `StateDb` -- Snapshot Write via commit_changes

Snapshot writes are committed as part of `StateDb::commit_changes`:

```rust
impl StateDb {
    /// Commit all state changes in one write transaction.
    ///
    /// For snapshots specifically:
    /// - Inserts each `new_snapshots` entry into the `snapshots` redb table.
    /// - Deletes each `deleted_snapshots` entry from the `snapshots` redb table.
    /// - Delete is applied after insert, so delete takes precedence on key collision.
    /// - All operations are atomic within a single write transaction.
    ///
    /// # Preconditions
    /// - The `StateReadSession` has been dropped (no concurrent read txn).
    /// - `new_snapshots` values are valid rkyv-serialized `Snapshot` bytes.
    ///
    /// # Postconditions
    /// - On success, all `new_snapshots` entries are durably persisted.
    /// - On success, all `deleted_snapshots` entries are removed.
    /// - On failure, no changes are persisted (ACID rollback).
    ///
    /// # Errors
    /// - `StateError::WriteTransactionFailed` if begin_write fails.
    /// - `StateError::TableOpenFailed` if snapshots table cannot be opened.
    /// - `StateError::StorageError` if redb insert/delete fails.
    /// - `StateError::CommitFailed` if commit fails.
    fn commit_changes(&self, changes: &StateChanges) -> Result<(), StateError>;
}
```

### Helper: Snapshot Serialization

```rust
/// Serialize a Snapshot to rkyv bytes for inclusion in StateChanges::new_snapshots.
///
/// Pure function -- no I/O. Callers use this to prepare bytes before building StateChanges.
///
/// # Errors
/// - `StateError::SerializationFailed` if rkyv::to_bytes fails.
fn serialize_snapshot(snapshot: &Snapshot) -> Result<Vec<u8>, StateError>;
```

## Type-Level Enforcement

1. **`OwnedArchive<Snapshot>`** -- The only way to get a `Snapshot` out of `load_snapshots` is through `OwnedArchive<Snapshot>::archived()` (zero-copy) or `OwnedArchive<Snapshot>::deserialize()` (owned). The archived bytes are owned, not borrowed from redb. This is enforced at the type level -- `OwnedArchive` contains `Box<[u8]>`, not a reference.

2. **`StateReadSession<'db>` borrows from `StateDb`** -- The lifetime `'db` ties the read session to the `StateDb`, preventing use-after-free. The `StateReadSession` must be dropped before `commit_changes` can succeed (redb MVCC constraint enforced by redb itself at runtime, but the lifetime makes it structurally harder to misuse).

3. **`StateChanges` is a plain data struct** -- No methods, no behavior. The caller constructs it with the correct bytes. The only way to write is through `StateDb::commit_changes`, which is the single write path.

4. **`#[non_exhaustive]` on `StateError`** -- Forward-compatible error enum. New variants can be added without breaking callers.

5. **Key type is `[u8; 32]`** -- Not `&[u8]` or `Vec<u8>`. Enforces exactly 32 bytes at the type level for snapshot keys, preventing accidentally passing a malformed key.

## Non-goals

1. Migration of `cmd/watch.rs` from `DocCache` to `StateDb` -- that is a separate bead (action layer).
2. Adding rkyv derives to `Snapshot` -- assumed already done in an earlier bead.
3. Implementing `StateDb::open`, `begin_read`, or `commit_changes` -- assumed done in earlier beads.
4. Changing the `Snapshot` domain type shape or semantics.
5. Implementing ETag or conditional GET support for scrapes.
6. Key rotation or snapshot versioning -- each key overwrites the previous value.
