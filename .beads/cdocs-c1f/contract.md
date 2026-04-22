# Contract Specification

## Context

- **Feature**: Add redb Builder durability tuning with Paranoid mode to `StateDb`
- **Bead**: cdocs-c1f
- **Domain terms**:
  - `DurabilityConfig` — our domain enum controlling crash-safety behaviour per write transaction
  - `StateDbBuilder` — builder pattern for `StateDb` construction with configurable cache and durability
  - `cache_size` — redb page cache in bytes (default: 64 MiB = 67108864)
  - `two_phase_commit` — redb's non-deprecated replacement for `Durability::Paranoid`; enables extra fsync after every commit for crash safety
- **Assumptions**:
  - `redb = "2"` (currently resolves to v2.6.3) is the active dependency
  - `redb::Durability::Paranoid` is deprecated since redb 2.3.0; we use `WriteTransaction::set_two_phase_commit(true)` instead
  - All existing tests in `commit.rs` (lines 790-3127) and `mod.rs` must continue to pass unchanged
  - `StateDb::open(path)` remains the public API with backward-compatible defaults
  - The existing fallback pattern (`try open, then create`) is preserved inside the builder
- **Open questions**: None

## Preconditions

- **PRE-1**: `StateDbBuilder::new()` must be called before `open()`. The builder is consumed on `open()`.
- **PRE-2**: `StateDbBuilder::open(path)` requires `path` to be a valid filesystem path. Parent directories are created if absent.
- **PRE-3**: `cache_size` must be > 0. A value of 0 means "use redb's default" (1 GiB).
- **PRE-4**: Only one `StateDb` instance may hold the underlying `.redb` file open at a time (redb file lock).
- **PRE-5**: `commit_changes` requires `active_read_sessions == 0` (existing invariant, unchanged).
- **PRE-6**: `DurabilityConfig::Paranoid` is only applied to write transactions. Read transactions are unaffected.

## Postconditions

- **POST-1** (`StateDb::open(path)`): Returns `Ok(StateDb)` with `cache_size = 64 MiB`, `DurabilityConfig::Default`. Behaviour is identical to the current code.
- **POST-2** (`StateDbBuilder::open(path)`): Returns `Ok(StateDb)` with user-specified `cache_size` and `DurabilityConfig`.
- **POST-3** (`StateDbBuilder::cache_size(n)`): The redb `Builder::set_cache_size(n)` is called before `open`/`create`. The cache is split 90/10 between read/write by redb internally.
- **POST-4** (`commit_changes` with `DurabilityConfig::Paranoid`): `write_tx.set_two_phase_commit(true)` is called on the `WriteTransaction` before any writes. After `write_tx.commit()`, data is guaranteed durable on disk (extra fsync).
- **POST-5** (`commit_changes` with `DurabilityConfig::Default`): No `set_two_phase_commit` call; redb's default `Immediate` durability applies (single fsync on commit).
- **POST-6**: All 8 redb tables are initialized on every `open()` (existing behaviour preserved).

## Invariants

- **INV-1**: `StateDb::open(path)` is exactly equivalent to `StateDbBuilder::new().open(path)`.
- **INV-2**: `DurabilityConfig` is stored inside `StateDb` and applied to every write transaction opened by `commit_changes`.
- **INV-3**: The fallback open pattern is preserved: `builder.open(path).or_else(|_| builder.create(path))`.
- **INV-4**: `StateDb` struct layout remains `{ db: Database, active_read_sessions: AtomicUsize, durability_config: DurabilityConfig }`. The new field is appended (no reordering of existing fields).
- **INV-5**: `DurabilityConfig` implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` (value type semantics).
- **INV-6**: `StateDbBuilder` is consumed on `open()` — not `&mut self` but `self` — preventing double-open.
- **INV-7**: `#[non_exhaustive]` on `DurabilityConfig` to allow future variants (e.g., `Eventual`) without breaking changes.
- **INV-8**: `redb::Durability::Paranoid` is never used (it is deprecated). All paranoid-mode functionality goes through `set_two_phase_commit(true)`.

## Error Taxonomy

No new error variants are needed. All errors map to existing `CommitError` variants:

| Scenario | Error Variant | New? |
|---|---|---|
| Builder `open` fails, `create` also fails | `CommitError::DatabaseOpen { path, reason }` | No |
| Parent directory creation fails | `CommitError::DatabaseOpen { path, reason }` | No |
| Table initialization fails | `CommitError::TableInit { reason }` | No |
| `begin_write` fails | `CommitError::WriteTransaction { reason }` | No |
| `commit` fails | `CommitError::CommitFailed { reason }` | No |

`StateDbBuilder` reuses the same `CommitError` type. The builder's `open()` method signature is `fn open(self, path: &Path) -> Result<StateDb, CommitError>`.

## Contract Signatures

```rust
// ---------------------------------------------------------------------------
// DurabilityConfig — domain enum for write-transaction crash safety
// ---------------------------------------------------------------------------

/// Crash-safety configuration for `StateDb` write transactions.
///
/// Controls whether each `commit_changes` call performs an extra fsync
/// (two-phase commit) for maximum durability guarantees.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityConfig {
    /// Default redb durability (`Immediate`). Single fsync on commit.
    /// Fastest safe option. Suitable for most workloads.
    Default,
    /// Paranoid mode: enables redb two-phase commit (`set_two_phase_commit(true)`).
    /// Extra fsync after every commit for maximum crash safety.
    /// Use when data loss from a single commit is unacceptable.
    Paranoid,
}

impl Default for DurabilityConfig {
    fn default() -> Self {
        Self::Default
    }
}

// ---------------------------------------------------------------------------
// StateDbBuilder — builder pattern for StateDb construction
// ---------------------------------------------------------------------------

/// Builder for [`StateDb`] with configurable cache size and durability.
///
/// # Defaults
///
/// - `cache_size`: 64 MiB (67108864 bytes)
/// - `durability`: [`DurabilityConfig::Default`]
///
/// # Example
///
/// ```ignore
/// let db = StateDbBuilder::new()
///     .cache_size(128 * 1024 * 1024)
///     .durability(DurabilityConfig::Paranoid)
///     .open(path)?;
/// ```
pub struct StateDbBuilder {
    cache_size: usize,
    durability: DurabilityConfig,
}

impl StateDbBuilder {
    /// Create a new builder with defaults (64 MiB cache, Default durability).
    pub fn new() -> Self;

    /// Set the redb page cache size in bytes.
    ///
    /// Passing 0 uses redb's internal default (1 GiB).
    /// redb splits the cache 90/10 between read/write internally.
    pub fn cache_size(mut self, bytes: usize) -> Self;

    /// Set the durability configuration for write transactions.
    pub fn durability(mut self, config: DurabilityConfig) -> Self;

    /// Open or create the state database at `path` with configured settings.
    ///
    /// Preserves the fallback pattern: try `Builder::open`, then `Builder::create`.
    /// Creates parent directories if they do not exist.
    /// Initializes all 8 redb tables.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create/open the file.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open(self, path: &Path) -> Result<StateDb, CommitError>;
}

impl Default for StateDbBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// StateDb — modified struct and constructor
// ---------------------------------------------------------------------------

pub struct StateDb {
    db: Database,
    active_read_sessions: std::sync::atomic::AtomicUsize,
    durability_config: DurabilityConfig,
}

impl StateDb {
    /// Open the state database at the given path with default settings.
    ///
    /// Equivalent to `StateDbBuilder::new().open(path)`.
    /// Default: 64 MiB cache, `DurabilityConfig::Default`.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create/open the file.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open(path: &Path) -> Result<Self, CommitError>;

    /// Returns the active durability configuration.
    #[must_use]
    pub fn durability_config(&self) -> DurabilityConfig;

    /// Commit all changes atomically in a single write transaction.
    ///
    /// **Modified behaviour**: When `self.durability_config == DurabilityConfig::Paranoid`,
    /// calls `write_tx.set_two_phase_commit(true)` before applying writes.
    ///
    /// # Errors
    ///
    /// See [`CommitError`] variants.
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError>;

    // begin_read, database, drop_snapshots_table — signatures unchanged
}
```

## Non-goals

- This contract does NOT expose redb's `Durability::None` or `Durability::Eventual` variants. `DurabilityConfig` is intentionally limited to `Default` (= `Immediate`) and `Paranoid` (= `Immediate` + two-phase commit). Future expansion is possible via `#[non_exhaustive]`.
- This contract does NOT change `initialize_tables` or any read-path code.
- This contract does NOT add `cache_size` as a runtime-adjustable parameter after `StateDb` is opened (redb's cache is set at database creation time).
- This contract does NOT change the `StateDb::database()` accessor or its visibility.
- This contract does NOT address redb file lock contention or multi-process access patterns.
