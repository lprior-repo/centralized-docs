# Test Plan: StateReadSession (cdocs-ej0)

## Summary

- **Behaviors identified**: 30
- **Trophy allocation**: 4 static / 14 unit / 16 integration / 1 E2E
- **Proptest invariants**: 5
- **Fuzz targets**: 1
- **Kani harnesses**: 2
- **Total test count**: 43 (31 BDD named + 5 proptest + 1 fuzz + 2 kani + 4 static)
- **Density ratio**: 43 / 8 pub fns = **5.375×**
- **Mutation kill target**: ≥90%

---

## 1. Behavior Inventory

Every system guarantee expressed as `[Subject] [action] [outcome] when [condition]`:

| # | Behavior |
|---|----------|
| B01 | `DocCache::begin_read` returns `StateReadSession` when no session is active (redb backend) |
| B02 | `DocCache::begin_read` returns `StateReadSession` when no session is active (LRU backend) |
| B03 | `DocCache::begin_read` returns `Err(SessionError::AlreadyOpen)` when a session is already active |
| B04 | `DocCache::begin_read` succeeds after previous session is dropped (flag reset) |
| B05 | `DocCache::begin_read` returns `Err(SessionError::BackendError)` when redb fails to start a read transaction |
| B06 | `StateReadSession::get` returns `Ok(Some(V))` when key exists and cache type is enabled (redb) |
| B07 | `StateReadSession::get` returns `Ok(Some(V))` when key exists and cache type is enabled (LRU) |
| B08 | `StateReadSession::get` returns `Ok(None)` when key does not exist |
| B09 | `StateReadSession::get` returns `Ok(None)` when cache type is disabled in session's enabled mask |
| B10 | `StateReadSession::get` returns `Err(SessionError::KeyTooLarge)` when key exceeds 256 bytes |
| B11 | `StateReadSession::get` returns `Err(SessionError::DeserializationError)` when stored bytes are corrupt |
| B12 | `StateReadSession::get` returns `Err(SessionError::BackendError)` when redb read fails |
| B13 | `StateReadSession::get_document` delegates to `get(CacheType::Document, key)` and returns identical results |
| B14 | `StateReadSession::get_scrape` delegates to `get(CacheType::Scrape, key)` and returns identical results |
| B15 | `StateReadSession::get_transform` delegates to `get(CacheType::Transform, key)` and returns identical results |
| B16 | `StateReadSession::get_snapshot` delegates to `get(CacheType::Snapshot, key)` and returns identical results |
| B17 | `StateReadSession::stats` returns entry counts from a single consistent snapshot |
| B18 | `StateReadSession` drop clears `session_open` flag on `DocCache` |
| B19 | `StateReadSession` drop releases the redb `ReadTransaction` (no resource leak) |
| B20 | Multiple concurrent readers sharing one session see the same snapshot (snapshot isolation) |
| B21 | Writes committed after session creation are invisible to the session (MVCC snapshot) |
| B22 | `DocCache::get()` continues to work unchanged during and without a session (backward compat) |
| B23 | `DocCache::put()` succeeds while a session is active (write path unblocked) |
| B24 | `StateReadSession::enabled` mask matches the `DocCache` config at session creation time |
| B25 | `StateReadSession::stats` returns all-zero counts when no data has been written to any table |
| B26 | `StateReadSession::get_document` returns `Ok(None)` when the requested key does not exist |
| B27 | `StateReadSession::get_scrape` returns `Err(SessionError::KeyTooLarge)` when called with a 257-byte key through the convenience method |
| B28 | `StateReadSession::get` returns `Ok(None)` when key is zero-length empty bytes `b""` |
| B29 | `StateReadSession::get` returns `Err(SessionError::KeyTooLarge)` when key is 10000 bytes |
| B30 | `StateReadSession::get` returns `Ok(None)` for all CacheTypes when every type is disabled in the enabled mask |

---

## 2. Trophy Allocation

| Layer | Count | Behaviors | Rationale |
|-------|-------|-----------|-----------|
| **Static** | 4 | INV-4, type-level checks | Compile-time: `StateReadSession` has no write methods; `Send + Sync` bounds; `#[non_exhaustive]` on `SessionError`; `SessionGuard` drop impl exists |
| **Unit** | 14 | B08, B09, B10 (×2 tests), B13, B14, B15, B16, B25, B26, B27, B28, B29, B30 | Pure logic: key validation boundaries, enabled-mask checks, convenience delegation (happy + error), stats on empty tables. No external I/O dependencies. |
| **Integration** | 16 | B01, B02, B03, B04, B05, B06, B07, B11, B12, B17, B18, B20, B21, B22, B23, B24 | Component boundary: singleton enforcement across real redb, snapshot isolation via real MVCC, RAII cleanup verified by state inspection, backward compat, corrupt-data injection, enabled mask with real config |
| **E2E** | 1 | B19 | Resource leak detection requires external observation of open file descriptors via `/proc/self/fd`. Platform-specific, process-level observation. |

**Ratio**: ~11% static / ~40% unit / ~46% integration / ~3% E2E — heavier on integration because the core value of this feature is correct interaction between `DocCache`, `SessionGuard`, `AtomicBool`, and redb's `ReadTransaction`. The singleton enforcement and snapshot isolation are inherently cross-component.

---

## 3. BDD Scenarios

### Behavior B01: `begin_read` succeeds on redb backend

```
Given: a DocCache backed by a redb file database where initialize_tables() has been
       called, creating DOCUMENT_TABLE, SCRAPE_TABLE, TRANSFORM_TABLE, and SNAPSHOT_TABLE
  And: CacheConfig with all CacheTypes enabled
 When: begin_read() is called with no active session
 Then: returns Ok(StateReadSession)
  And: session.get::<String>(CacheType::Document, b"nonexistent") returns Ok(None) —
       proving the session holds a valid, functional ReadTransaction
  And: the returned session's enabled mask includes all CacheType variants (all 6 bits set)
```

**Test name**: `fn begin_read_returns_session_when_no_session_active_redb()`

---

### Behavior B02: `begin_read` succeeds on LRU backend

```
Given: a DocCache backed by in-memory LRU with value "lru_val" stored at
       CacheType::Document under key b"stored_key" via put_document()
 When: begin_read() is called with no active session
 Then: returns Ok(StateReadSession)
  And: DocCache::get::<String>(CacheType::Document, b"stored_key") returns Ok(Some("lru_val"))
  And: session.get::<String>(CacheType::Document, b"stored_key") returns Ok(Some("lru_val")) —
       identical value from both paths
```

**Test name**: `fn begin_read_returns_session_when_no_session_active_lru()`

---

### Behavior B03: `begin_read` rejects when session already active

```
Given: a DocCache with an active StateReadSession (begin_read succeeded, session still in scope)
 When: begin_read() is called again on the same DocCache
 Then: returns Err(SessionError::AlreadyOpen)
```

Error variant:
```
Given: first begin_read() returned Ok(session) and session is still in scope
 When: second begin_read() is called
 Then: Err(SessionError::AlreadyOpen) — exact variant match via matches!(err, SessionError::AlreadyOpen)
```

**Test name**: `fn begin_read_returns_already_open_when_session_active()`

---

### Behavior B04: `begin_read` succeeds after session dropped

```
Given: a DocCache (redb) with value "survivor" stored at CacheType::Document under key b"prestored"
  And: a StateReadSession was created via begin_read() and then dropped (goes out of scope)
 When: begin_read() is called again
 Then: returns Ok(StateReadSession)
  And: new_session.get::<String>(CacheType::Document, b"prestored") returns Ok(Some("survivor")) —
       confirming a fully functional new session that reads the same data
```

**Test name**: `fn begin_read_succeeds_after_previous_session_dropped()`

---

### Behavior B05: `begin_read` returns backend error on redb failure

```
Given: a DocCache backed by redb where the underlying Database handle has been dropped
       via a test-only method DocCache::test_invalidate_backend() (sets inner to a
       closed/invalid state), simulating a backend failure
 When: begin_read() is called
 Then: returns Err(SessionError::BackendError { operation, message })
  And: operation == "begin_read"
  And: message.len() > 0 — non-empty error description
```

**Test name**: `fn begin_read_returns_backend_error_when_redb_fails()`

---

### Behavior B06: session get returns value on redb

```
Given: a DocCache (redb) with a document stored via put_document(b"key_alpha", &"stored_value")
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Document, b"key_alpha") is called
 Then: returns Ok(Some("stored_value")) — exact string match
```

**Test name**: `fn session_get_returns_stored_value_when_key_exists_redb()`

---

### Behavior B07: session get returns value on LRU

```
Given: a DocCache (LRU) with a value stored via put(CacheType::Scrape, b"key_beta", &"scrape_result")
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Scrape, b"key_beta") is called
 Then: returns Ok(Some("scrape_result")) — exact string match
```

**Test name**: `fn session_get_returns_stored_value_when_key_exists_lru()`

---

### Behavior B08: session get returns None for missing key

```
Given: a DocCache (either backend) with a StateReadSession
 When: session.get::<String>(CacheType::Document, b"nonexistent_key") is called
 Then: returns Ok(None)
```

**Test name**: `fn session_get_returns_none_when_key_missing()`

---

### Behavior B09: session get returns None for disabled cache type

```
Given: a DocCache with CacheConfig where CacheType::Document is disabled
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Document, b"any_key") is called
 Then: returns Ok(None) — even when data exists for that CacheType (verified in B24)
```

**Test name**: `fn session_get_returns_none_when_cache_type_disabled()`

---

### Behavior B10: session get rejects oversized key

```
Given: a StateReadSession on any backend
 When: session.get::<String>(CacheType::Document, &[0u8; 257]) is called
 Then: returns Err(SessionError::KeyTooLarge { size: 257, max: 256 })
   And: size == 257 and max == 256
```

Boundary:
```
 When: session.get() is called with key of exactly 256 bytes (vec![0u8; 256])
 Then: returns Ok(None) (or Ok(Some(..)) if populated) — not KeyTooLarge
```

**Test name**: `fn session_get_returns_key_too_large_when_key_exceeds_256_bytes()`
**Test name**: `fn session_get_accepts_key_at_exactly_256_bytes()`

---

### Behavior B11: session get rejects corrupt deserialization

```
Given: a DocCache (redb) with raw bytes b"not valid json" stored under key b"corrupt_key"
       in DOCUMENT_TABLE (injected via raw redb write transaction, bypassing serde)
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Document, b"corrupt_key") is called
 Then: returns Err(SessionError::DeserializationError { message })
  And: message.len() > 0
```

**Test name**: `fn session_get_returns_deserialization_error_when_stored_bytes_corrupt()`

---

### Behavior B12: session get returns backend error on redb read failure

```
Given: a StateReadSession opened on a redb backend where the underlying Database handle
       is dropped mid-session via DocCache::test_invalidate_backend(), rendering the
       ReadTransaction's table references invalid
 When: session.get() triggers a table read that fails
 Then: returns Err(SessionError::BackendError { operation, message })
  And: operation is a &'static str (e.g., "open_table" or "get")
  And: message.len() > 0
```

**Test name**: `fn session_get_returns_backend_error_when_redb_read_fails()`

---

### Behavior B13: `get_document` delegates correctly

```
Given: a DocCache with value "doc_val" stored at CacheType::Document under key b"dk"
  And: a StateReadSession opened on that cache
 When: session.get_document::<String>(b"dk") is called
 Then: returns Ok(Some("doc_val")) — identical to session.get(CacheType::Document, b"dk")
```

**Test name**: `fn get_document_returns_same_as_get_with_document_type()`

---

### Behavior B14: `get_scrape` delegates correctly

```
Given: a DocCache with value "scrape_val" stored at CacheType::Scrape under key b"sk"
  And: a StateReadSession opened on that cache
 When: session.get_scrape::<String>(b"sk") is called
 Then: returns Ok(Some("scrape_val")) — identical to session.get(CacheType::Scrape, b"sk")
```

**Test name**: `fn get_scrape_returns_same_as_get_with_scrape_type()`

---

### Behavior B15: `get_transform` delegates correctly

```
Given: a DocCache with value "transform_val" stored at CacheType::Transform under key b"tk"
  And: a StateReadSession opened on that cache
 When: session.get_transform::<String>(b"tk") is called
 Then: returns Ok(Some("transform_val")) — identical to session.get(CacheType::Transform, b"tk")
```

**Test name**: `fn get_transform_returns_same_as_get_with_transform_type()`

---

### Behavior B16: `get_snapshot` delegates correctly

```
Given: a DocCache with value "snap_val" stored at CacheType::Snapshot under key b"snk"
  And: a StateReadSession opened on that cache
 When: session.get_snapshot::<String>(b"snk") is called
 Then: returns Ok(Some("snap_val")) — identical to session.get(CacheType::Snapshot, b"snk")
```

**Test name**: `fn get_snapshot_returns_same_as_get_with_snapshot_type()`

---

### Behavior B17: session stats returns consistent snapshot counts

```
Given: a DocCache (redb) with 3 documents, 2 scrape entries, 1 transform, 0 snapshots
       (inserted via put_document/put_scrape/put_transform before session creation)
  And: a StateReadSession opened on that cache
 When: session.stats() is called
 Then: returns Ok(CacheStats { document_entries: 3, scrape_entries: 2,
       transform_entries: 1, snapshot_entries: 0, analysis_entries: 0, chunk_entries: 0 })
  And: all counts are from the same ReadTransaction (verified by confirming stats match
       DocCache::stats() called immediately before session creation)
```

**Test name**: `fn session_stats_returns_correct_counts_from_single_snapshot()`

---

### Behavior B18: session drop clears the open flag

```
Given: a DocCache where begin_read() succeeded and returned a StateReadSession
 When: the StateReadSession is dropped (goes out of scope)
 Then: DocCache::begin_read() succeeds on the next call, returning Ok(StateReadSession)
```

**Test name**: `fn session_drop_clears_open_flag_allowing_new_session()`

---

### Behavior B19: session drop releases redb ReadTransaction (no leak)

```
Given: a DocCache (redb) on Linux (#[cfg(target_os = "linux")])
  And: fd_count_before = count of entries in /proc/self/fd (captured before any session)
 When: a StateReadSession is created and dropped 10 times in a loop
  And: fd_count_after = count of entries in /proc/self/fd (captured after all 10 cycles)
 Then: fd_count_after == fd_count_before (delta == 0)
  And: no file descriptor accumulation after repeated create/drop cycles
```

**Test name**: `fn session_drop_releases_redb_read_transaction_no_fd_leak()`

**Layer**: E2E — requires external observation of file descriptors via `/proc/self/fd`.

---

### Behavior B20: multiple readers share one session snapshot (concurrent)

```
Given: a DocCache (redb) with a document stored at key b"shared_key" with value "original"
  And: a StateReadSession opened on that cache
  And: a concurrent writer thread that calls DocCache::put(CacheType::Document, b"shared_key", "modified")
 When: session.get::<String>(CacheType::Document, b"shared_key") is called AFTER the writer commits
 Then: returns Ok(Some("original")) — the session snapshot predates the write
```

**Test name**: `fn session_sees_original_snapshot_after_concurrent_write()`

---

### Behavior B21: writes after session open are invisible

```
Given: a DocCache (redb) with empty DOCUMENT_TABLE
  And: a StateReadSession opened on that cache
 When: DocCache::put_document(b"late_key", &"late_value") is called and commits
  And: session.get::<String>(CacheType::Document, b"late_key") is called
 Then: returns Ok(None) — write not visible through the session's snapshot
```

**Test name**: `fn session_does_not_see_writes_committed_after_session_creation()`

---

### Behavior B22: DocCache::get() works unchanged alongside session

```
Given: a DocCache (redb) with value "compat_val" stored under key b"compat"
  And: a StateReadSession currently active on that cache
 When: DocCache::get::<String>(CacheType::Document, b"compat") is called
 Then: returns Ok(Some("compat_val")) — existing API unaffected by session
```

**Test name**: `fn doc_cache_get_still_works_when_session_active()`

---

### Behavior B23: DocCache::put() works while session is active

```
Given: a DocCache (redb) with an active StateReadSession
 When: DocCache::put_document(b"during_session", &"written_during_session") is called
 Then: returns Ok(()) — write succeeds, not blocked by session
  And: DocCache::get::<String>(CacheType::Document, b"during_session") returns
       Ok(Some("written_during_session")) after commit
```

**Test name**: `fn doc_cache_put_succeeds_while_session_active()`

---

### Behavior B24: session enabled mask matches DocCache config

```
Given: a DocCache with CacheConfig where CacheType::Scrape is disabled
  And: CacheType::Document is enabled
  And: value "exists" is stored at CacheType::Scrape under key b"sc_key" via put_scrape()
  And: value "doc" is stored at CacheType::Document under key b"doc_key" via put_document()
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Document, b"doc_key") is called
 Then: returns Ok(Some("doc"))
 When: session.get::<String>(CacheType::Scrape, b"sc_key") is called
 Then: returns Ok(None) — despite data existing for the disabled type
```

**Test name**: `fn session_enabled_mask_matches_cache_config_at_creation_time()`

---

### Behavior B25: stats returns all zeros when tables empty

```
Given: a DocCache (redb) with all tables initialized via initialize_tables()
  And: no data written to any table
  And: a StateReadSession opened on that cache
 When: session.stats() is called
 Then: returns Ok(CacheStats { document_entries: 0, scrape_entries: 0,
       transform_entries: 0, snapshot_entries: 0, analysis_entries: 0, chunk_entries: 0 })
```

**Test name**: `fn stats_returns_all_zeros_when_tables_empty()`

---

### Behavior B26: `get_document` returns None when key missing

```
Given: a DocCache (redb) with a StateReadSession
  And: no data stored at CacheType::Document under key b"no_such_doc"
 When: session.get_document::<String>(b"no_such_doc") is called
 Then: returns Ok(None)
```

**Test name**: `fn get_document_returns_none_when_key_missing()`

---

### Behavior B27: `get_scrape` returns KeyTooLarge through convenience method

```
Given: a StateReadSession on any backend
 When: session.get_scrape::<String>(&vec![0u8; 257].as_slice()) is called
 Then: returns Err(SessionError::KeyTooLarge { size: 257, max: 256 })
  And: size == 257 and max == 256 — key validation fires through the delegation path
```

**Test name**: `fn get_scrape_returns_key_too_large_through_convenience_method()`

---

### Behavior B28: session get returns None for empty (zero-length) key

```
Given: a DocCache (redb) with a StateReadSession
 When: session.get::<String>(CacheType::Document, b"") is called with a zero-length key
 Then: returns Ok(None) — empty key is valid but matches no entry
```

**Test name**: `fn session_get_returns_none_when_key_is_zero_length()`

---

### Behavior B29: session get returns KeyTooLarge for very large key (10000 bytes)

```
Given: a StateReadSession on any backend
 When: session.get::<String>(CacheType::Document, &vec![0xAAu8; 10000]) is called
 Then: returns Err(SessionError::KeyTooLarge { size: 10000, max: 256 })
  And: size == 10000 and max == 256
```

**Test name**: `fn session_get_returns_key_too_large_when_key_is_10000_bytes()`

---

### Behavior B30: session get returns None when all CacheTypes disabled

```
Given: a DocCache with CacheConfig where EnabledTypes is empty (all CacheTypes disabled)
  And: value "hidden" stored at CacheType::Document under key b"any_key" via put_document()
  And: a StateReadSession opened on that cache
 When: session.get::<String>(CacheType::Document, b"any_key") is called
 Then: returns Ok(None) — despite data existing, all types are disabled
 When: session.get::<String>(CacheType::Scrape, b"any_key") is called
 Then: returns Ok(None)
 When: session.get::<String>(CacheType::Transform, b"any_key") is called
 Then: returns Ok(None)
 When: session.get::<String>(CacheType::Snapshot, b"any_key") is called
 Then: returns Ok(None)
```

**Test name**: `fn session_get_returns_none_when_all_cache_types_disabled()`

---

## 4. Proptest Invariants

### Proptest P1: Key validation boundary

```
Function: StateReadSession::get() — key size validation
Invariant: For any key K where K.len() <= 256, get() returns Ok(_) (not KeyTooLarge).
           For any key K where K.len() > 256, get() returns Err(SessionError::KeyTooLarge { size: K.len(), max: 256 }).
Strategy: proptest::collection::vec(any::<u8>(), 0..10001) — covers empty, boundary (256), oversized (257+), and large (10000).
Anti-invariant: key.len() == 0 → should return Ok(None) (empty key is valid but won't match).
```

### Proptest P2: Serialization round-trip through session

```
Function: put() then session.get() round-trip
Invariant: For any serializable value V: serde_json::to_vec(&V) then put(CacheType, key, &V)
           then session.get::<V>(CacheType, key) == Ok(Some(V)) — byte-exact round-trip.
Strategy: Generate random String, Vec<u8>, HashMap<String, i32>, Vec<Vec<String>> via proptest::arbitrary().
Anti-invariant: Values exceeding MAX_VALUE_SIZE (50MB) should be rejected by put(), not by session.get().
```

### Proptest P3: Enabled mask isolation

```
Function: StateReadSession with partially-enabled mask
Invariant: For any subset S of {Document, Scrape, Transform, Snapshot, Analysis, Chunk},
           if CacheType T ∉ S is disabled, session.get(T, any_key) always returns Ok(None)
           regardless of what data exists at that key.
Strategy: proptest::bits::u8::ANY — interpret bits as enabled/disabled for 6 cache types.
Anti-invariant: None — all valid u8 bitmasks are valid configurations.
```

### Proptest P4: Session open/drop cycle idempotency

```
Function: begin_read() → drop → begin_read() cycle
Invariant: For any number of cycles N (1..100), each begin_read() succeeds and returns
           a session that can read previously written data. The Nth session sees the
           same data as the 1st session (assuming no intervening writes).
Strategy: proptest::num::usize::ANY filtered to 1..=50.
Anti-invariant: Calling begin_read() twice without dropping returns AlreadyOpen — always.
```

### Proptest P5: Snapshot consistency across reads

```
Function: Multiple session.get() calls on the same session
Invariant: For any sequence of get() calls on the same session, all reads return data
           from the same MVCC snapshot. If value V was written BEFORE session creation,
           session.get() always returns Some(V). If V was written AFTER session creation,
           session.get() always returns None for that key.
Strategy: Generate random (key, value) pairs via proptest::collection::vec(any::<u8>(), 1..50) paired with arbitrary strings.
Anti-invariant: None — redb MVCC guarantees this; we're verifying our session wrapper preserves it.
```

---

## 5. Fuzz Targets

### Fuzz Target F1: `session_get_deserialize`

```
Function: StateReadSession::get::<V>() — deserialization of stored bytes
Input type: Arbitrary &[u8] (raw bytes stored in redb table)
Risk: Panic in serde_json::from_slice, OOM on maliciously large allocation,
      logic error in error-path mapping (CacheError → SessionError).
Corpus seeds:
  - b"" (empty)
  - b"null"
  - b"\"\"" (empty string)
  - b"[1,2,3]" (valid JSON array)
  - b"\x00\x00\x00" (binary garbage)
  - b"{\"a\":" (truncated JSON)
  - 64KB random bytes (stress allocation)

Implementation note: Fuzz target opens a redb database, writes arbitrary bytes via raw
write transaction (bypassing serde), then calls session.get::<String>() to exercise
the deserialization + error-mapping path.
```

---

## 6. Kani Harnesses

### Kani Harness K1: AtomicBool ordering correctness

```
Property: SessionGuard::drop stores false with Ordering::Release, and
          DocCache::begin_read loads with Ordering::Acquire. Under Kani's
          memory model, this guarantees that if begin_read() observes false,
          the previous session's guard has fully dropped.
Bound: Single-threaded — Kani explores all interleavings of store/load orderings.
Rationale: AtomicBool ordering bugs are subtle and can allow two concurrent sessions
           under thread interleaving. Formal verification is warranted because
           INV-1 (singleton) is the core safety guarantee.
```

### Kani Harness K2: EnabledTypes bitmask correctness

```
Property: For all u8 values E and all CacheType variants C (0..=5):
          EnabledTypes(E).is_enabled(C) == ((E & (1 << C as u8)) != 0)
Bound: 256 values of E × 6 variants of C = 1,536 cases — trivial for Kani.
Rationale: Bitwise logic is easy to get wrong. A single flipped bit means a disabled
           type leaks reads or an enabled type is silently skipped. Exhaustive proof
           over the entire input space is cheap and eliminates an entire error class.
```

---

## 7. Mutation Testing Checkpoints

Target: **≥90% mutation kill rate**

### Critical mutations that MUST be caught:

| Mutation | Caught by test |
|----------|---------------|
| `AtomicBool::compare_exchange` or store replaced with no-op | `begin_read_returns_already_open_when_session_active` — second begin_read would incorrectly succeed |
| `store(false, Release)` in SessionGuard::drop replaced with `store(true, Release)` | `begin_read_succeeds_after_previous_session_dropped` — subsequent begin_read would fail |
| `session_open` check removed (always pass) | `begin_read_returns_already_open_when_session_active` — would return Ok instead of Err |
| `key.len() > MAX_KEY_SIZE` changed to `>=` | `session_get_accepts_key_at_exactly_256_bytes` — boundary would fail |
| `key.len() > MAX_KEY_SIZE` changed to `+ 1` offset | `session_get_returns_key_too_large_when_key_exceeds_256_bytes` — 257-byte key would pass |
| `Ok(None)` returned for disabled type changed to proceed with I/O | `session_enabled_mask_matches_cache_config_at_creation_time` — would return data for disabled type |
| `enabled` mask not copied from config (uses default all-enabled) | `session_enabled_mask_matches_cache_config_at_creation_time` — disabled type would return data |
| Convenience method delegates to wrong CacheType (e.g., `get_document` uses `Scrape`) | `get_document_returns_same_as_get_with_document_type` — wrong table queried |
| `SessionGuard` drop removed entirely | `session_drop_clears_open_flag_allowing_new_session` — flag never clears |
| `_guard` field reordered to drop before `backend` | `session_drop_releases_redb_read_transaction_no_fd_leak` — fd leak detected |
| `Stats` method creates new read_tx instead of reusing session's | `session_does_not_see_writes_committed_after_session_creation` — stats would see newer data than session.get() |
| `BackendError` message field is empty | `begin_read_returns_backend_error_when_redb_fails` — asserts `message.len() > 0` |
| Key validation skipped in convenience method delegation | `get_scrape_returns_key_too_large_through_convenience_method` — 257-byte key would pass through |
| Stats count returns hardcoded values or 1 for empty tables | `stats_returns_all_zeros_when_tables_empty` — would return non-zero |
| `get` returns `Err(KeyTooLarge)` for zero-length key | `session_get_returns_none_when_key_is_zero_length` — would return Err instead of Ok(None) |

---

## 8. Combinatorial Coverage Matrix

### Group A: `begin_read` lifecycle

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| first open, redb | valid redb DocCache, no session | Ok(StateReadSession) | integration |
| first open, LRU | valid LRU DocCache, no session | Ok(StateReadSession) | integration |
| second open, session alive | valid DocCache, session in scope | Err(SessionError::AlreadyOpen) | integration |
| open after drop | valid DocCache, session dropped | Ok(StateReadSession) with functional reads | integration |
| redb failure | Database handle dropped via test_invalidate_backend() | Err(SessionError::BackendError { operation: "begin_read", .. }) | integration |
| enabled mask populated | DocCache with partial EnabledTypes | session reflects exact mask | integration |

### Group B: `StateReadSession::get` — happy path

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| existing key, redb | valid key, value pre-stored | Ok(Some(exact_value)) | integration |
| existing key, LRU | valid key, value pre-stored | Ok(Some(exact_value)) | integration |
| missing key | valid key, no data | Ok(None) | unit |
| disabled type | valid key, type disabled | Ok(None) | unit |
| key at boundary 256 | key.len() == 256 | Ok(None) or Ok(Some(..)) | unit |
| empty key (0 bytes) | key == b"" | Ok(None) | unit |
| all types disabled | any key, all types disabled | Ok(None) | unit |

### Group C: `StateReadSession::get` — error paths

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| oversized key (257) | key.len() == 257 | Err(SessionError::KeyTooLarge { size: 257, max: 256 }) | unit |
| oversized key (10000) | key.len() == 10000 | Err(SessionError::KeyTooLarge { size: 10000, max: 256 }) | unit |
| corrupt bytes | raw invalid JSON in redb | Err(SessionError::DeserializationError { message }) | integration |
| redb I/O failure | Database handle dropped mid-session | Err(SessionError::BackendError { operation, message }) | integration |

### Group D: Convenience delegation — happy path

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| get_document | pre-stored Document value | Ok(Some("doc_val")) | unit |
| get_scrape | pre-stored Scrape value | Ok(Some("scrape_val")) | unit |
| get_transform | pre-stored Transform value | Ok(Some("transform_val")) | unit |
| get_snapshot | pre-stored Snapshot value | Ok(Some("snap_val")) | unit |

### Group D2: Convenience delegation — error paths

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| get_document missing key | no data at Document key | Ok(None) | unit |
| get_scrape oversized key | key.len() == 257 | Err(SessionError::KeyTooLarge { size: 257, max: 256 }) | unit |

### Group E: RAII and concurrency

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| drop then re-open | session dropped | begin_read() succeeds | integration |
| concurrent write invisible | write after session open | session.get() returns pre-write value | integration |
| put during session | active session | put returns Ok(()), DocCache::get sees new value | integration |
| DocCache::get during session | active session | DocCache::get returns current value | integration |

### Group F: `StateReadSession::stats`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| populated tables | 3 docs, 2 scrapes | CacheStats { document_entries: 3, scrape_entries: 2, .. } | integration |
| empty tables | no data written | CacheStats { document_entries: 0, scrape_entries: 0, transform_entries: 0, snapshot_entries: 0, analysis_entries: 0, chunk_entries: 0 } | unit |
| consistent with snapshot | data written before session | stats == DocCache::stats() at session-open time | integration |

### Group G: Proptest coverage

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| key size boundary | key.len() in 0..10001 | Ok if ≤256, KeyTooLarge if >256 | proptest |
| round-trip | any serializable V | put then get == Ok(Some(V)) | proptest |
| enabled mask | any u8 bitmask | disabled types return None | proptest |
| open/drop cycles | N in 1..50 | each cycle succeeds | proptest |
| snapshot consistency | any key set | session sees only pre-creation data | proptest |

---

## Static Analysis Checks

| Check | What it verifies |
|-------|-----------------|
| `clippy::all` with deny | No anti-patterns, no unwrap in session code |
| `#[non_exhaustive]` on `SessionError` | Adding new variants is not a breaking change |
| `StateReadSession` has no `put*` / `clear*` methods | INV-4 enforced at compile time |
| `Send + Sync` bounds on `StateReadSession` | Required for `par_iter()` in `analyze_files_cached()` |
| `SessionGuard` field `_guard` is last in `StateReadSession` struct | Drop order: backend drops first, then guard clears flag |
| `cargo-deny` | No duplicate dependency versions introduced |
| `#![deny(unused_must_use)]` | `begin_read()` return value cannot be silently discarded |

---

## Open Questions

1. **How to simulate redb backend failure (B05, B12)?** Options: (a) drop the Database handle before calling begin_read, (b) use a custom wrapper that injects failures, (c) close the file descriptor. Recommend option (a) — drop the `Database` while keeping a dangling `DocCache` reference (unsafe or via `ManuallyDrop`). May need a test-only method on `DocCache` to simulate backend failure. The B05 and B12 Given clauses assume a `DocCache::test_invalidate_backend()` method exists for this purpose.

2. **FD leak detection (B19)** — on Linux, read `/proc/self/fd` count before and after 10 session create/drop cycles. This is inherently platform-specific. The test is gated behind `#[cfg(target_os = "linux")]`. The assertion is exact: `fd_count_after == fd_count_before` (delta == 0).

3. **Should `StateReadSession::stats()` exist on the session at all?** The contract specifies it, but stats via session uses a single read transaction whereas `DocCache::stats()` creates a new one. If the contract changes, the snapshot-consistency test (B17) becomes irrelevant. This plan assumes the contract is final.

4. **Thread-safety test granularity** — B20 and B21 require concurrent writer threads. These should use `std::thread::spawn` with real redb. No mock or fake needed — redb handles real concurrency.
