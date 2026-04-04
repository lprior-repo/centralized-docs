# Contract Specification

bead_id: cdocs-rwm
bead_title: action: migrate watch/apply snapshot persistence from `DocCache` to `StateDb`
phase: state-1-contract
updated_at: 2026-04-04T00:00:00Z

## Context

- **Feature**: Migrate the watch/apply snapshot persistence layer from the legacy `DocCache` (serde_json-based per-key get/put) to the unified `StateDb` (rkyv-based bulk-load + atomic batch commit).
- **Domain terms**:
  - `Snapshot` — point-in-time record of all scraped pages for a target URL (defined in `watch.rs`).
  - `DocCache` — legacy redb-backed cache with per-key serde_json get/put, including `get_snapshot`/`put_snapshot`.
  - `StateDb` — unified state database with two-transaction architecture: read session for bulk loading, atomic `commit_changes` for writes.
  - `StateReadSession` — scoped read transaction that borrows `StateDb`, provides `load_snapshots`.
  - `StateChanges` — batch struct consumed by `StateDb::commit_changes`; has `new_snapshots: Vec<([u8; 32], Vec<u8>)>`.
  - `ArchivedRaw` — owned wrapper for raw rkyv-archived bytes; has `deserialize<T>()`.
  - `url_hash` — SHA-256 of URL string, used as the 32-byte key for snapshots in both legacy and new code.
- **Assumptions**:
  - `serialize_snapshot` and `StateReadSession::load_snapshots` / `ArchivedRaw::deserialize` stubs will be implemented (or this bead depends on their completion).
  - The `Snapshot` type in `watch.rs` already derives `rkyv::Archive, rkyv::Serialize, rkyv::Deserialize` (or will be added as part of this migration).
  - The watch/apply snapshot key is always derived from `url_hash(url).as_bytes()` which is `[u8; 32]`.
- **Open questions**:
  - Does `Snapshot` already have the required `rkyv` derives? If not, adding them is in scope for this bead.

## Preconditions

### PRE-1: Database Path Validity
- `cache_path` (the redb file path) must point to a writable directory (or one that can be created).
- Parent directories must be creatable via `std::fs::create_dir_all`.

### PRE-2: Snapshot Key Derivation
- `url_hash(url)` must produce a valid 32-byte key.
- The URL string must be non-empty.

### PRE-3: Read-Session Lifetime
- `StateReadSession` must be dropped before `StateDb::commit_changes` is called (redb borrow rule).

### PRE-4: Serialization Round-Trip
- `Snapshot` must be serializable to rkyv bytes and deserializable back to an equal `Snapshot`.

## Postconditions

### POST-1: Snapshot Load Returns Previously Stored Snapshot
- After storing a `Snapshot` for URL `u`, loading by the same URL key must return a `Snapshot` that is `PartialEq`-equal to the stored one.

### POST-2: Missing Snapshot Returns Empty Default
- When no snapshot exists for a URL key, the load function must return an empty `Snapshot` with:
  - `target_url` == the requested URL string
  - `timestamp` == current UTC time (or a sentinel)
  - `pages` == empty `BTreeMap`

### POST-3: Watch Does Not Mutate Stored State
- `run_watch` must never call `StateDb::commit_changes`. The watch command is read-only.

### POST-4: Apply Commits Exactly One Snapshot
- `run_apply` must call `StateDb::commit_changes` exactly once, with `new_snapshots` containing exactly one entry (the URL key → serialized snapshot bytes).

### POST-5: Apply Is Idempotent
- Running `run_apply` twice with identical scrape content produces an empty change plan on the second run and `commit_changes` is either skipped or is a no-op (unchanged bytes).

### POST-6: Transaction Isolation
- `run_watch` opens one read session, uses it, drops it.
- `run_apply` opens one read session, drops it, then opens one write transaction via `commit_changes`.

## Invariants

### INV-1: Pure Calculation Layer Untouched
- All pure functions in `watch.rs` (`compute_plan`, `diff_snapshots`, `snapshot_from_scrape`, `format_plan_*`, `count_by_kind`, `write_plan_reports`) must remain byte-identical after migration. No changes to signatures, behavior, or test coverage.

### INV-2: Key Identity Stability
- The 32-byte key for a given URL must be identical whether derived through the legacy `DocCache` path or the new `StateDb` path. `url_hash(url).as_bytes()` == `url_hash(url).as_bytes()` (SHA-256 is deterministic).

### INV-3: No `DocCache` Imports in `cmd/watch.rs`
- After migration, `cmd/watch.rs` must not import `DocCache`, `CacheConfig`, or any type from `doc_transformer::cache` except `url_hash` and `content_hash` (which remain in `cache` module as pure functions).

### INV-4: Railway-Oriented Error Handling
- Every fallible operation returns `Result<T, E>`. No `unwrap()`, `expect()`, `panic!`, or `todo!` in production paths.

### INV-5: Single Source of Truth
- Snapshots flow through `StateDb` only. No parallel writes to `DocCache.snapshots` table.

### INV-6: Migration Is Confined to Command I/O Boundary
- Changes are restricted to the "Actions" section of `cmd/watch.rs` (lines 131-260 in the current file). The `run_watch`, `run_apply`, and their I/O helpers are the only functions modified.

## Error Taxonomy

All errors from this migration are already covered by existing error types. No new error variants are needed.

### Existing Errors (used directly)

| Error Type | Variant | When |
|---|---|---|
| `CommitError` | `DatabaseOpen` | `StateDb::open` fails (invalid path, permission denied) |
| `CommitError` | `TableInit` | Table initialization fails after database open |
| `CommitError` | `ReadTransaction` | `StateDb::begin_read` fails |
| `CommitError` | `WriteTransaction` | `commit_changes` fails to begin write tx |
| `CommitError` | `CommitFailed` | `commit_changes` fails to commit write tx |
| `CommitError` | `ZeroHashKey` | Snapshot key is `[0u8; 32]` (should not happen with `url_hash`) |
| `CommitError` | `PayloadTooLarge` | Serialized snapshot exceeds 50 MiB |
| `StateError` | `SerializationFailed` | `serialize_snapshot` fails (rkyv error) |
| `StateError` | `DeserializationFailed` | `ArchivedRaw::deserialize` fails (rkyv error) |
| `StateError` | `InvalidArchive` | Stored bytes are not valid rkyv archive |
| `StateError` | `ArchiveValidationFailed` | Stored bytes fail rkyv validation for a specific key |
| `StateError` | `TableOpenFailed` | Snapshots table cannot be opened in read session |
| `StateError` | `StorageError` | redb read/write fails during table operation |

### Error Propagation

All errors are converted to `anyhow::Error` at the command boundary (`run_watch`, `run_apply`) via `?` operator. No error swallowing.

## Contract Signatures

### Replaced Functions (cmd/watch.rs I/O helpers)

```rust
// BEFORE (legacy DocCache):
fn open_cache(cache_path: &Path) -> Result<DocCache>;
fn load_snapshot(cache: &DocCache, url: &str) -> Result<Snapshot>;
fn store_snapshot(cache: &DocCache, url: &str, snapshot: &Snapshot) -> Result<()>;

// AFTER (new StateDb):
fn open_state_db(state_db_path: &Path) -> Result<StateDb>;
fn load_snapshot(state_db: &StateDb, url: &str) -> Result<Snapshot>;
fn store_snapshot(state_db: &StateDb, url: &str, snapshot: &Snapshot) -> Result<()>;
```

### Affected Command Signatures (unchanged externally)

```rust
// cmd/watch.rs — public API unchanged
pub async fn run_watch(
    url: &str,
    output: &Path,
    cache_path: &Path,  // NOTE: will point to StateDb path, not DocCache path
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
    json_output: bool,
) -> Result<()>;

pub async fn run_apply(
    url: &str,
    cache_path: &Path,  // NOTE: will point to StateDb path, not DocCache path
    scrape_dir: &Path,
    yes: bool,
) -> Result<()>;
```

### StateDb APIs Used (already exist)

```rust
// state/commit.rs — existing APIs
impl StateDb {
    pub fn open(path: &Path) -> Result<Self, CommitError>;
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, CommitError>;
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError>;
}

impl StateReadSession<'_> {
    pub fn load_snapshots(
        &self,
        keys: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], ArchivedRaw>, StateError>;
}

impl ArchivedRaw {
    pub fn deserialize<T>(&self) -> Result<T, StateError>;
}

// state/mod.rs — existing (stub)
pub fn serialize_snapshot(snapshot: &Snapshot) -> Result<Vec<u8>, StateError>;
```

### Internal Helper Behavior Contracts

#### `open_state_db(cache_path: &Path) -> Result<StateDb>`
- **Precondition**: `cache_path` is a valid file path; parent directory is writable or creatable.
- **Postcondition**: Returns an open `StateDb` with all 8 tables initialized.
- **Errors**: `CommitError::DatabaseOpen`, `CommitError::TableInit`.

#### `load_snapshot(state_db: &StateDb, url: &str) -> Result<Snapshot>`
- **Precondition**: `url` is non-empty.
- **Postcondition**:
  - If a snapshot exists for `url`, returns it deserialized from rkyv bytes.
  - If no snapshot exists, returns an empty default `Snapshot` with `target_url == url`.
- **Transaction model**: Opens a `StateReadSession`, calls `load_snapshots` with a single key, drops session.
- **Errors**: `CommitError::ReadTransaction`, `StateError::TableOpenFailed`, `StateError::DeserializationFailed`, `StateError::InvalidArchive`.

#### `store_snapshot(state_db: &StateDb, url: &str, snapshot: &Snapshot) -> Result<()>`
- **Precondition**: `url` is non-empty; `snapshot` is a valid `Snapshot`.
- **Postcondition**: The serialized snapshot is committed to the `snapshots` table under the URL's hash key.
- **Transaction model**: Calls `commit_changes` with `StateChanges { new_snapshots: vec![(url_key, rkyv_bytes)] }`.
- **Errors**: `CommitError::WriteTransaction`, `CommitError::CommitFailed`, `StateError::SerializationFailed`, `CommitError::PayloadTooLarge`.

## Non-goals

- Migrating `DocCache` consumers outside of `cmd/watch.rs` (other commands, scrape caching, etc.).
- Changing the `Snapshot` struct definition or its `serde` derives.
- Adding rkyv derives to `Snapshot` if they already exist (verify first; add only if missing).
- Changing the CLI argument names or the `cache_path` parameter semantics for callers.
- Modifying `run_diff` (it does not use snapshots).
- Removing `DocCache::get_snapshot`/`put_snapshot` methods from `cache/mod.rs`.
- Performance optimization or benchmarking (out of scope for this P3 task).
