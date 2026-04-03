# Contract Specification: StateReadSession

## Phase Metadata

| Field        | Value                                                          |
|--------------|----------------------------------------------------------------|
| bead_id      | cdocs-ej0                                                      |
| bead_title   | data: add `StateReadSession` to enforce one shared read transaction per run |
| phase        | contract (pre-implementation)                                  |
| status       | STATE 0: ISOLATION COMPLETE                                    |

---

## Context

### Feature

Introduce a `StateReadSession` type that opens exactly one `redb::ReadTransaction`
per command run and scopes all cache reads through it. Loaders (index, scrape,
watch/apply) obtain a session via `DocCache::begin_read()` and use it for the
entire lifetime of the run. The write path (`DocCache::put()` etc.) remains
unchanged with its existing per-call write-transaction semantics.

### Current Problem

Today `DocCache::get()` on the redb backend calls `db.begin_read()` on **every
invocation**, creating and immediately discarding a `ReadTransaction`. During
`analyze_files_cached()` this means thousands of throwaway read transactions for
a single index run. redb's MVCC already supports long-lived read transactions
with snapshot isolation — the session pattern exploits this directly.

### Domain Terms

| Term                | Meaning                                                        |
|---------------------|----------------------------------------------------------------|
| `StateReadSession`  | RAII guard holding one shared read transaction for a run       |
| `SessionBackend`    | Internal enum: `Redb(ReadTransaction)` or `Lru(&RwLock<..>)`  |
| `SessionGuard`      | Drop-guard that clears the `open` flag on `DocCache`           |
| Read path           | All `get*` operations — go through `StateReadSession`          |
| Write path          | All `put*` operations — remain on `DocCache` directly          |

### Assumptions

1. The redb `Database` handle is `Clone` (it is — redb uses `Arc` internally)
   so sharing it between `DocCache` and `StateReadSession` has no ownership
   conflict.
2. `redb::ReadTransaction` is `Send + Sync` (it is) and can be safely held
   across `par_iter()` calls in `analyze_files_cached()`.
3. The LRU backend has no transaction concept — the session acts as a
   pass-through for reads, with the singleton constraint still enforced.
4. The `StateReadSession` borrows `&DocCache`, so Rust's borrow checker
   naturally prevents dropping `DocCache` while a session is live.
5. The write path (`put()`, `clear_all()`, `initialize_tables()`) is **not**
   blocked by an open read session. redb MVCC allows concurrent readers and a
   single writer. Writers see a snapshot that includes all committed writes at
   the time of `begin_write()`.

### Open Questions

_none_

---

## Type Specifications

### `SessionBackend<'a>`

```rust
/// Internal backend for a read session.
enum SessionBackend<'a> {
    /// Pass-through to the LRU cache (no transaction semantics).
    Lru(&'a RwLock<LruCache<Vec<u8>, Vec<u8>>>),
    /// Single long-lived redb read transaction.
    Redb(ReadTransaction),
}
```

### `SessionGuard<'a>`

```rust
/// RAII drop-guard that clears the `open` flag on DocCache.
///
/// Stored as a field of `StateReadSession` so it drops after the backend,
/// ensuring the flag is cleared even if the backend's drop panics.
struct SessionGuard<'a> {
    open_flag: &'a AtomicBool,
}

impl<'a> Drop for SessionGuard<'a> {
    fn drop(&mut self) {
        self.open_flag.store(false, Ordering::Release);
    }
}
```

### `StateReadSession<'a>`

```rust
/// Scoped read session — one per command run.
///
/// Holds a single `ReadTransaction` (redb) or a reference to the LRU cache.
/// All cache reads during a run flow through this type.
///
/// # Lifetime
///
/// Borrows `&DocCache` so the cache cannot be dropped while the session
/// is live. The `AtomicBool` in `DocCache` ensures only one session exists
/// at a time.
///
/// # Thread Safety
///
/// `StateReadSession` is `Send + Sync`. The inner `ReadTransaction` is
/// shareable across threads via `&self` methods.
pub struct StateReadSession<'a> {
    backend: SessionBackend<'a>,
    enabled: EnabledTypes,
    _guard: SessionGuard<'a>,
}
```

### Modifications to `DocCache`

```rust
pub struct DocCache {
    inner: CacheBackendInner,
    config: CacheConfig,
    /// Tracks whether a `StateReadSession` is currently open.
    /// `false` = no session, `true` = session active.
    session_open: AtomicBool,  // NEW FIELD
}
```

---

## Function Signatures

### `DocCache::begin_read`

```rust
impl DocCache {
    /// Open a read session for this command run.
    ///
    /// Returns `Err(SessionError::AlreadyOpen)` if a session is already active.
    /// Returns `Err(SessionError::BackendError)` if the redb read transaction
    /// cannot be started.
    ///
    /// The session borrows `&self` — it cannot outlive the cache.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, SessionError>;
}
```

**Preconditions:**
- `self` is a valid, open `DocCache`.
- No other `StateReadSession` is currently active on this cache instance.

**Postconditions:**
- On success: returns `StateReadSession` with an active read transaction.
- `self.session_open` is set to `true` (via `Ordering::AcqRel`).
- The session's `enabled` mask is copied from `self.config.enabled`.
- On error: no state mutation; `session_open` remains `false`.

### `StateReadSession::get`

```rust
impl<'a> StateReadSession<'a> {
    /// Retrieve a cached value by table type and key.
    ///
    /// Uses the session's single read transaction (redb) or LRU pass-through.
    /// Returns `Ok(None)` if the key is not found or the cache type is disabled.
    pub fn get<V: DeserializeOwned>(
        &self,
        cache_type: CacheType,
        key: &[u8],
    ) -> Result<Option<V>, SessionError>;
}
```

**Preconditions:**
- `self` is a live session (not dropped).
- `key.len() <= MAX_KEY_SIZE` (256 bytes).

**Postconditions:**
- On `Ok(Some(v))`: `v` is the deserialized value stored at `(cache_type, key)`.
- On `Ok(None)`: no entry exists for that key, or the cache type is disabled.
- On `Err(SessionError::KeyTooLarge { .. })`: key exceeded `MAX_KEY_SIZE`.
- On `Err(SessionError::DeserializationError { .. })`: stored bytes are corrupt.
- On `Err(SessionError::BackendError { .. })`: redb I/O failure.

### `StateReadSession::get_document`

```rust
pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>, SessionError>;
```

Delegates to `self.get(CacheType::Document, key)`.

### `StateReadSession::get_scrape`

```rust
pub fn get_scrape<V: DeserializeOwned>(&self, url_hash: &[u8]) -> Result<Option<V>, SessionError>;
```

Delegates to `self.get(CacheType::Scrape, url_hash)`.

### `StateReadSession::get_transform`

```rust
pub fn get_transform<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>, SessionError>;
```

Delegates to `self.get(CacheType::Transform, key)`.

### `StateReadSession::get_snapshot`

```rust
pub fn get_snapshot<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>, SessionError>;
```

Delegates to `self.get(CacheType::Snapshot, key)`.

### `StateReadSession::stats`

```rust
/// Return entry counts for each table using the session's read transaction.
pub fn stats(&self) -> Result<CacheStats, SessionError>;
```

**Preconditions:**
- `self` is a live session.

**Postconditions:**
- All counts are computed from the same snapshot (same `ReadTransaction`).
- Counts are consistent with each other (no interleaved writes can skew).

### Drop (implicit)

```rust
impl<'a> Drop for StateReadSession<'a> {
    /// Releases the read transaction and clears the open flag.
    fn drop(&mut self);
}
```

**Postconditions:**
- `DocCache::session_open` is set to `false` (via `SessionGuard` drop).
- The redb `ReadTransaction` is released.
- A subsequent `begin_read()` will succeed.

### Unchanged Write Path

The following methods on `DocCache` remain **unchanged** and do NOT require a
session:

- `DocCache::put()`
- `DocCache::put_document()`
- `DocCache::put_scrape()`
- `DocCache::put_transform()`
- `DocCache::put_snapshot()`
- `DocCache::clear_all()`
- `DocCache::initialize_tables()`

These continue to create their own write transactions per call.

---

## Invariants

### INV-1: Singleton Session

At most one `StateReadSession` exists per `DocCache` instance at any time.

**Enforcement:** `DocCache::session_open: AtomicBool` is set to `true` on
`begin_read()` and cleared to `false` on `StateReadSession` drop. A second
call to `begin_read()` while a session is active returns
`Err(SessionError::AlreadyOpen)`.

### INV-2: Read Transaction Reuse

All reads through a `StateReadSession` use the same `ReadTransaction` for the
redb backend. No new `db.begin_read()` calls occur during the session's
lifetime.

**Enforcement:** `SessionBackend::Redb(ReadTransaction)` is created once in
`begin_read()` and stored in the session. All `get()` calls reference it.

### INV-3: Snapshot Isolation

A `StateReadSession` sees a consistent snapshot of the database as of the
moment `begin_read()` was called. Writes committed after session creation are
not visible to the session.

**Enforcement:** redb's MVCC guarantees this — a `ReadTransaction` sees the
state at the time it was opened.

### INV-4: No Write Capability on Session

`StateReadSession` exposes no `put*`, `clear*`, or mutation methods.

**Enforcement:** The type system — no such methods exist on `StateReadSession`.

### INV-5: RAII Cleanup

When a `StateReadSession` is dropped (including via panic unwinding), the
`session_open` flag is cleared and the read transaction is released.

**Enforcement:** `SessionGuard` implements `Drop` with `AtomicBool::store`.
`StateReadSession`'s drop order ensures `_guard` drops last.

### INV-6: Backward Compatibility

Existing code that calls `DocCache::get()` directly continues to compile and
behave identically. The session API is additive.

**Enforcement:** `DocCache::get()` and all convenience methods remain public
and unchanged. `StateReadSession` is a new parallel path, not a replacement.

### INV-7: LRU Pass-Through Correctness

For the in-memory backend, `StateReadSession::get()` reads from the same
`RwLock<LruCache>` as `DocCache::get()`, producing identical results.

**Enforcement:** `SessionBackend::Lru(&RwLock<LruCache<..>>)` references the
same `RwLock` stored in `DocCache::inner`. The read path is identical to the
current `get_from_lru()` call.

---

## Preconditions

### PRE-1: DocCache must be open

`begin_read()` requires a valid `DocCache` that was successfully opened via
`DocCache::open()`.

### PRE-2: No existing session

`begin_read()` requires `session_open == false`. Violation yields
`SessionError::AlreadyOpen`.

### PRE-3: Key size within bounds

`StateReadSession::get()` requires `key.len() <= 256`. Violation yields
`SessionError::KeyTooLarge`.

### PRE-4: Deserializable stored values

`StateReadSession::get()` requires that stored bytes are valid
`serde_json` for the requested type `V`. Corruption yields
`SessionError::DeserializationError`.

---

## Postconditions

### POST-1: Session creation

On success, `begin_read()` returns a `StateReadSession` where:
- `session_open` is `true`
- The backend holds a live `ReadTransaction` (redb) or LRU reference (memory)
- `enabled` matches the `DocCache`'s current `EnabledTypes` mask

### POST-2: Read consistency

All `get()` calls on the same session return data from the same snapshot.
No interleaved writes between calls can change the returned data.

### POST-3: Session release

After `StateReadSession` is dropped:
- `session_open == false`
- `begin_read()` will succeed on the next call
- No redb resources are leaked (transaction is released)

### POST-4: Disabled type behavior

When `cache_type` is disabled in the session's `enabled` mask, `get()`
returns `Ok(None)` immediately — no I/O is performed.

---

## Error Taxonomy

### `SessionError` (new enum)

```rust
/// Errors specific to `StateReadSession` lifecycle and operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum SessionError {
    /// A read session is already active on this `DocCache`.
    /// Only one session may exist at a time.
    #[error("read session already open on this cache — only one session allowed")]
    AlreadyOpen,

    /// The provided key exceeds `MAX_KEY_SIZE` (256 bytes).
    #[error("session key too large: {size} bytes (max {max})")]
    KeyTooLarge {
        size: usize,
        max: usize,
    },

    /// Failed to deserialize a cached value.
    /// The stored bytes do not match the expected type `V`.
    #[error("session deserialization failed for key: {message}")]
    DeserializationError {
        message: String,
    },

    /// redb backend error during session creation or read.
    #[error("session backend error during {operation}: {message}")]
    BackendError {
        operation: &'static str,
        message: String,
    },
}
```

### Error Mapping

| Error Variant             | When It Occurs                                        |
|---------------------------|-------------------------------------------------------|
| `AlreadyOpen`             | `begin_read()` called while session is active         |
| `KeyTooLarge`             | `get()` key exceeds 256 bytes                         |
| `DeserializationError`    | Stored bytes are not valid `serde_json` for type `V`  |
| `BackendError`            | redb I/O failure (begin_read, open_table, get)        |

### Relationship to Existing `CacheError`

`SessionError` is a **separate** enum. It does not replace `CacheError`.
`CacheError` continues to be used by `DocCache::get()` and `DocCache::put()`.
`SessionError` is used exclusively by `StateReadSession` methods.

Callers that need to handle both can use `anyhow::Error` as a common boundary
or pattern-match on each individually.

---

## Non-Goals

1. **Write batching through sessions.** The write path remains per-call via
   `DocCache::put()`. A future bead may introduce `StateWriteSession` for
   batched writes with a single commit at shutdown.

2. **Blocking writers while a session is open.** redb MVCC allows concurrent
   reads and writes. Writers are not blocked.

3. **Removing `DocCache::get()`.** The direct `get()` path on `DocCache`
   remains for backward compatibility and internal use.

4. **Cross-process session coordination.** Sessions are per-process. No
   distributed locking or coordination is needed.

5. **Automatic session creation.** Callers must explicitly call
   `begin_read()`. No implicit session is created.
