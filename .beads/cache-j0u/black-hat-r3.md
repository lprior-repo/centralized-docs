# BLACK HAT CODE REVIEW - redb Cache Implementation

**Reviewer:** black-hat-r3 (fresh audit, zero prior context)
**Date:** 2026-03-21
**Target:** `src/cache/mod.rs` (626 lines), `src/errors.rs:277-304` (CacheError enum)

---

```
STATUS: REJECTED
```

5 CRITICAL, 3 MAJOR, 5 MINOR defects found.

---

## DEFECT-001: Race condition in `get_or_compute` — duplicate computation and write
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** `mod.rs:226-238`
**Evidence:**
```rust
pub fn get_or_compute<V, F>(&self, cache_type: CacheType, key: &[u8], compute: F) -> Result<V>
where
    V: Serialize + DeserializeOwned,
    F: FnOnce() -> Result<V>,
{
    if let Some(cached) = self.get::<V>(cache_type, key)? {
        return Ok(cached);
    }

    let value = compute()?;
    self.put(cache_type, key, &value)?;
    Ok(value)
}
```
**Impact:** Two threads calling `get_or_compute` with the same key simultaneously will both observe a cache miss, both invoke `compute()`, and both write. For expensive operations (scraping, embedding, LLM calls), this doubles cost. For non-idempotent side-effects in the compute closure, it causes corruption. The doc comment on `DocCache` claims "Thread Safety" via MVCC but MVCC only guarantees read consistency — it does NOT prevent this TOCTOU race.
**Fix:** Use `std::sync::Mutex` or `DashMap` keyed on `(CacheType, key)` to serialize the check-then-act. Alternatively, document that `get_or_compute` is NOT safe for concurrent use with the same key and remove the thread-safety claim.

---

## DEFECT-002: `clear_all` has a window where tables don't exist
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** `mod.rs:200-210`
**Evidence:**
```rust
pub fn clear_all(&self) -> Result<()> {
    let write_tx = self.db.begin_write()?;
    {
        write_tx.delete_table(DOCUMENT_CACHE_TABLE)?;
        write_tx.delete_table(SCRAPE_CACHE_TABLE)?;
        write_tx.delete_table(TRANSFORM_CACHE_TABLE)?;
        write_tx.delete_table(METADATA_TABLE)?;
    }
    write_tx.commit()?;
    self.initialize_tables()
}
```
**Impact:** Between `commit()` and `initialize_tables()`, if another thread calls `get_document`, `stats`, or any method that begins a read transaction and opens a table, redb will return an error because the table does not exist. `initialize_tables` starts a NEW write transaction — there is no atomic "delete + recreate" operation. This is a race window that causes spurious `BackendError` crashes under concurrent load.
**Fix:** Delete and recreate tables within the SAME write transaction, or use a `RwLock` to serialize `clear_all` against all reads.

---

## DEFECT-003: `put_cached_value` bypasses value size validation — dead code trap
**Severity:** CRITICAL
**Phase:** SECURITY
**File:** `mod.rs:297-308`
**Evidence:**
```rust
/// Stores a cached value without size validation.
// TODO: Kept for future batch API that may need unchecked inserts
#[allow(dead_code)]
fn put_cached_value<V: Serialize>(
    table: &mut redb::Table<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}
```
**Impact:** This function is reachable code that completely bypasses `MAX_VALUE_SIZE` (10MB) validation. The TODO comment says "future batch API" — if a developer enables this, they silently reintroduce the DoS vector that the size limits were designed to prevent. The comment itself is a `TODO` in source code, violating the project's AGENTS.md rule: "No markdown TODOs. Use bd exclusively." While this is a code TODO, it indicates the author is tracking future work outside the bead system.
**Fix:** Delete the function entirely. If a batch API is needed later, create it with explicit opt-out validation and a bead tracking the decision. The `#[allow(dead_code)]` annotation is a red flag — dead code with security implications should not linger.

---

## DEFECT-004: No key validation on `get` operations — wasted read work on oversized keys
**Severity:** MAJOR
**Phase:** PERFORMANCE
**File:** `mod.rs:134-140, 156-162, 178-184`
**Evidence:**
```rust
pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
    if !self.config.cache_document_content {
        return Ok(None);
    }
    let read_tx = self.db.begin_read()?;
    get_cached_value(&read_tx, DOCUMENT_CACHE_TABLE, key)
}
```
**Impact:** A key that exceeds `MAX_KEY_SIZE` can never have been stored (put would reject it), so the get will always return `None`. But the code still opens a read transaction, opens the table, and performs a B-tree lookup — all wasted I/O. An attacker can amplify this by sending many oversized keys, causing transaction overhead without any cache benefit.
**Fix:** Add `validate_key_size(key)?` to all `get_*` methods (or a cheap early-return: `if key.len() > MAX_KEY_SIZE { return Ok(None); }`).

---

## DEFECT-005: `path_hash` uses `to_string_lossy` — silent data corruption on non-UTF-8 paths
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** `mod.rs:370-372`
**Evidence:**
```rust
pub fn path_hash(path: &Path) -> [u8; 32] {
    content_hash(path.to_string_lossy().as_bytes())
}
```
**Impact:** On Linux, `Path` can contain arbitrary non-UTF-8 bytes. `to_string_lossy()` silently replaces invalid UTF-8 with U+FFFD (REPLACEMENT CHARACTER). Two different paths like `/tmp/\xFFoo` and `/tmp/\xEF\xBF\xBDoo` (if the latter were somehow constructed) could produce the same hash — a collision. More practically, a file `/tmp/test\x80file` and `/tmp/test\u{FFFD}file` hash identically, causing cache hits for the WRONG document. This is a correctness bug: the cache returns stale/wrong data.
**Fix:** Use `path.as_os_str().as_bytes()` (available on Unix) or `path.as_raw_bytes()` to hash the raw byte representation without lossy conversion. Add a `#[cfg(unix)]` guard or use `std::os::unix::ffi::OsStrExt`.

---

## DEFECT-006: `CacheError` variants `NotInitialized`, `AlreadyOpen`, `Io`, `Serialization`, `Deserialization` are never constructed
**Severity:** MAJOR
**Phase:** MAINTAINABILITY
**File:** `errors.rs:290-303`
**Evidence:**
```rust
#[error("cache not initialized")]
NotInitialized,

#[error("cache already open at path: {0}")]
AlreadyOpen(String),

#[error("cache I/O error: {0}")]
Io(String),

#[error("cache serialization error: {0}")]
Serialization(String),

#[error("cache deserialization error: {0}")]
Deserialization(String),
```
**Impact:** 5 of 7 `CacheError` variants are dead code. Grep confirms zero call sites. These will mislead anyone debugging — they see "NotInitialized" in the enum and assume there's initialization logic to audit. The error taxonomy is a lie. Meanwhile, `get_cached_value` maps real errors into `BackendError` with an `operation` string, which is less structured than having dedicated variants would be.
**Fix:** Delete all 5 unused variants. If needed later, add them with actual construction sites and a bead tracking the work.

---

## DEFECT-007: `content_hash` uses `sha2` directly instead of `Digest` trait from the broader ecosystem
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** `mod.rs:356-364`
**Evidence:**
```rust
pub fn content_hash(content: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&result);
    array
}
```
**Impact:** The `sha2::Digest` import is scoped inside the function (odd style — usually at module level). The manual `copy_from_slice` into a fixed array is unnecessary; `sha2::Digest::finalize_into` can write directly into a `GenericArray` or the result can use `.into()` if the return type matches. Minor: `result` is a `GenericArray<u8, U32>` — the explicit `[0u8; 32]` + `copy_from_slice` is boilerplate that a cleaner API would avoid.
**Fix:** Use `sha2::Sha256::digest(content).into()` if the output type is compatible, or `digest::Digest::finalize_into` to write directly into the array. Move the import to the module level.

---

## DEFECT-008: No test for `url_hash` or `path_hash`
**Severity:** MAJOR
**Phase:** TESTING
**File:** `mod.rs:366-372` (functions), `mod.rs:374-626` (tests)
**Evidence:**
```rust
pub fn url_hash(url: &str) -> [u8; 32] {
    content_hash(url.as_bytes())
}

pub fn path_hash(path: &Path) -> [u8; 32] {
    content_hash(path.to_string_lossy().as_bytes())
}
```
No test function exercises `url_hash` or `path_hash`. The only hash tests are `test_content_hash_consistency` and `test_content_hash_different_inputs`.
**Impact:** `url_hash` is a trivial wrapper but has no coverage confirming it returns a 32-byte hash. `path_hash` has the `to_string_lossy` bug (DEFECT-005) and NO test would have caught it because no test exists. A URL with special characters, an empty URL, a very long URL — none are tested.
**Fix:** Add `test_url_hash_returns_32_bytes`, `test_url_hash_deterministic`, `test_path_hash_deterministic`, `test_path_hash_preserves_identity` (same path bytes = same hash), and `test_path_hash_non_utf8_collision` (prove or fix DEFECT-005).

---

## DEFECT-009: `initialize_tables` discards table handles — unnecessary allocations
**Severity:** MINOR
**Phase:** PERFORMANCE
**File:** `mod.rs:122-132`
**Evidence:**
```rust
fn initialize_tables(&self) -> Result<()> {
    let write_tx = self.db.begin_write()?;
    {
        let _ = write_tx.open_table(DOCUMENT_CACHE_TABLE)?;
        let _ = write_tx.open_table(SCRAPE_CACHE_TABLE)?;
        let _ = write_tx.open_table(TRANSFORM_CACHE_TABLE)?;
        let _ = write_tx.open_table(METADATA_TABLE)?;
    }
    write_tx.commit()?;
    Ok(())
}
```
**Impact:** Four `let _ =` bindings open four tables and immediately drop the handles. In redb, `open_table` is idempotent (creates if not exists), so the logic is correct, but each `_` allocation is dropped. The curly braces create a scope that ensures all handles are dropped before commit — this is correct and necessary for redb. However, the `_` pattern obscures intent. A reader might wonder "why are we opening tables we don't use?" — the intent (idempotent creation) deserves a comment or a named `_init` pattern.
**Fix:** Rename to `let _doc =`, `let _scrape =`, etc., or add a one-line comment: `// open_table is idempotent: creates if not exists`.

---

## DEFECT-010: `CacheConfig::default()` uses a relative path — fragile in production
**Severity:** MINOR
**Phase:** CORRECTNESS
**File:** `mod.rs:76-85`
**Evidence:**
```rust
impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            backend: CacheBackend::File(std::path::PathBuf::from(".cache/ctd_cache.redb")),
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }
}
```
**Impact:** The default path `.cache/ctd_cache.redb` is relative to the process working directory. If the process is started from different directories (systemd unit, cron, different user), the cache file ends up in different locations, silently creating multiple independent caches. This is a latent bug in any deployment that doesn't explicitly set the path.
**Fix:** Use `dirs::cache_dir()` or `$XDG_CACHE_HOME` to construct an absolute default path. At minimum, resolve to an absolute path at `open()` time.

---

## DEFECT-011: `put_cached_value_with_limit` serializes BEFORE checking size — allocation wasted on oversized values
**Severity:** MINOR
**Phase:** PERFORMANCE
**File:** `mod.rs:335-344`
**Evidence:**
```rust
fn put_cached_value_with_limit<V: Serialize>(
    table: &mut redb::Table<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    validate_value_size(&bytes)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}
```
**Impact:** For a value that serializes to 50MB (over the 10MB limit), the full 50MB `Vec<u8>` is allocated, THEN the size check rejects it, and the Vec is dropped. This is a transient memory spike. With concurrent requests, an attacker can amplify memory usage by sending payloads just over the limit.
**Fix:** Use `serde_json::to_vec(value)` with a size-limiting serializer, or check a pre-serialization size hint if the type provides one. For unbounded types, the current approach is acceptable but should be documented as a known trade-off.

---

## DEFECT-012: No concurrency stress tests
**Severity:** MINOR
**Phase:** TESTING
**File:** `mod.rs:374-626` (entire test module)
**Evidence:** All 13 tests are single-threaded. No `#[test]` uses `std::thread::spawn`, `rayon`, or `tokio` to exercise concurrent access — despite the module header claiming "Thread Safety" and "MVCC" guarantees.
**Impact:** DEFECT-001 (race in `get_or_compute`) and DEFECT-002 (race in `clear_all`) would be caught by even a basic concurrent test. The thread-safety claim is untested and, as shown above, incorrect for `get_or_compute`.
**Fix:** Add `test_concurrent_get_or_compute_same_key` spawning 10 threads, each calling `get_or_compute` with the same key and a side-effecting closure. Assert the compute function is called exactly once. Add `test_concurrent_clear_all_reads` that reads while another thread clears.

---

## DEFECT-013: `CacheStats` is missing metadata table entry count
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** `mod.rs:212-224, 269-274`
**Evidence:**
```rust
pub fn stats(&self) -> Result<CacheStats> {
    let read_tx = self.db.begin_read()?;
    let doc_count = table_len(&read_tx, DOCUMENT_CACHE_TABLE)?;
    let scrape_count = table_len(&read_tx, SCRAPE_CACHE_TABLE)?;
    let transform_count = table_len(&read_tx, TRANSFORM_CACHE_TABLE)?;
    // NOTE: metadata table count is not tracked
    Ok(CacheStats {
        document_entries: doc_count,
        scrape_entries: scrape_count,
        transform_entries: transform_count,
    })
}
```
`METADATA_TABLE` is created in `initialize_tables` and deleted in `clear_all`, but `CacheStats` has no field for it.
**Impact:** The metadata table exists but is invisible to monitoring. If metadata entries accumulate (e.g., cache versioning, timestamps), there's no way to observe it through `stats()`. Minor because metadata isn't currently written to, but the table exists and creates a blind spot.
**Fix:** Either add `metadata_entries: u64` to `CacheStats`, or remove `METADATA_TABLE` from `initialize_tables` / `clear_all` until it's actually used.

---

## Summary

| Severity | Count | IDs |
|----------|-------|-----|
| CRITICAL | 4 | 001, 002, 003, 005 |
| MAJOR | 3 | 004, 006, 008 |
| MINOR | 5 | 007, 009, 010, 011, 012, 013 |

**Verdict:** REJECTED. The cache has a thread-safety lie in its docs (DEFECT-001), a race window during clear (DEFECT-002), a silent data corruption bug via `to_string_lossy` (DEFECT-005), and a security-bypass dead-code function (DEFECT-003). No concurrent tests exist. 5 of 7 error variants are dead code. Fix all CRITICALs before re-review.
