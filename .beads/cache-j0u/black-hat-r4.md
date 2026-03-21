# BLACK-HAT CODE REVIEW — Round 4

**Auditor:** black-hat-reviewer (fresh audit, zero prior context)
**Date:** 2026-03-21
**Target:** `centralized-docs/src/cache/` (mod.rs, config.rs, store.rs, hash.rs, tests.rs) + `errors.rs` CacheError
**Methodology:** 5-phase adversarial review (Correctness / Security / Performance / Maintainability / Testing)

---

```
STATUS: REJECTED

Summary: 2 CRITICAL, 4 MAJOR, 5 MINOR
Line counts: store.rs=317 (EXCEEDS 300), tests.rs=362 (EXCEEDS 300), errors.rs=320 (EXCEEDS 300)
```

---

## DEFECT-001: Owner panic deadlocks all waiters forever
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** `store.rs:56-67` + `store.rs:259-299`
**Evidence:**
```rust
fn wait_once_lock(slot: &ComputeSlot) -> Result<Vec<u8>> {
    loop {
        if let Some(result) = slot.get() {
            return match result.as_ref() { ... };
        }
        std::hint::spin_loop();
        std::thread::yield_now();
    }
}
```
The `wait_once_lock` function is an infinite loop with no timeout, no cancellation, and no panic guard. If the owner thread (the one that won the `DashMap::entry` race at line 261) panics **after** inserting the `Arc<OnceLock>` into the DashMap but **before** calling `slot.set()` (line 294), the OnceLock is never resolved. Every waiter enters `wait_once_lock` and spins forever, burning CPU and never returning.

The panic window spans lines 262–294 (33 lines of code including `compute()`, two `serde_json::to_vec` calls, and a `self.put()` write transaction). Any panic within `compute()` (stack overflow, index OOB, third-party panic, integer overflow in debug) triggers this.

**Impact:** Thread leak → CPU burn → cascade deadlock if callers hold other locks. Production outage under any panic in `compute`.

**Fix:** Wrap the owner path in `std::panic::catch_unwind(AssertUnwindSafe(...))` and always call `slot.set(Err(...))` in a destructor or in the catch_unwind fallback, ensuring waiters are always released. Alternatively, replace the raw OnceLock with a `tokio::sync::Notify` or use a scoped cancellation token.

---

## DEFECT-002: `path_hash` re-export fails compilation on non-unix
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** `mod.rs:25` + `hash.rs:102-106`
**Evidence:**
```rust
// mod.rs:25
pub use hash::{content_hash, path_hash, url_hash};

// hash.rs:102-106
#[must_use]
#[cfg(unix)]
pub fn path_hash(path: &Path) -> [u8; 32] {
    use std::os::unix::ffi::OsStrExt;
    content_hash(path.as_os_str().as_bytes())
}
```
`path_hash` is gated with `#[cfg(unix)]` but re-exported unconditionally from `mod.rs`. On Windows (or any non-unix target), this fails to compile: `use of undeclared type or module `hash::path_hash``.

**Impact:** The entire crate fails to build on non-unix platforms. CI on Windows/macOS broken.

**Fix:** Either gate the re-export (`#[cfg(unix)] pub use hash::path_hash;`) or provide a `#[cfg(windows)]` fallback using `std::os::windows::ffi::OsStrExt`.

---

## DEFECT-003: File exceeds 300-line constraint — store.rs
**Severity:** MAJOR
**Phase:** MAINTAINABILITY
**File:** `store.rs` (317 lines)
**Evidence:** File is 317 lines, exceeding the stated 300-line maximum from the known constraints.

**Impact:** Violates project-level architectural constraint. Signals the file is doing too much.

**Fix:** Extract `get_or_compute` and its supporting types (`InFlightKey`, `ComputeSlot`, `wait_once_lock`) into a dedicated `store/inflight.rs` or `store/dedup.rs` submodule.

---

## DEFECT-004: File exceeds 300-line constraint — tests.rs
**Severity:** MAJOR
**Phase:** MAINTAINABILITY
**File:** `tests.rs` (362 lines)
**Evidence:** File is 362 lines, exceeding the 300-line maximum.

**Impact:** Same as DEFECT-003.

**Fix:** Split into `tests/basic.rs`, `tests/dedup.rs`, `tests/limits.rs` using submodules, or inline tests into the modules they exercise.

---

## DEFECT-005: Double serialization in `get_or_compute` — value serialized twice per successful cache
**Severity:** MAJOR
**Phase:** PERFORMANCE
**File:** `store.rs:279-286`
**Evidence:**
```rust
let slot_result: Result<Vec<u8>> = match &compute_result {
    Ok(value) => serde_json::to_vec(value).map_err(Into::into),  // serialize #1
    Err(e) => Err(anyhow::anyhow!("{e:#}")),
};

if let (Ok(_bytes), Ok(ref value)) = (&slot_result, &compute_result) {
    if let Err(e) = self.put(cache_type, key, value) {  // calls put_cached_value_with_limit
                                                      // which does serde_json::to_vec #2
```
The value is serialized to `Vec<u8>` at line 280 (producing `_bytes`), then `_bytes` is discarded and `self.put()` at line 286 re-serializes the same value inside `put_cached_value_with_limit` (hash.rs:66). For a 10MB value, this allocates 20MB total and doubles CPU time on the critical path.

**Impact:** 2x memory allocation and CPU for serialization on every successful cache write through `get_or_compute`. For values near the 10MB limit, this causes transient memory spikes and GC pressure.

**Fix:** Use `_bytes` from `slot_result` directly. Replace `self.put()` with a lower-level method that accepts pre-serialized `&[u8]`, e.g., `put_raw(cache_type, key, &bytes)`. This eliminates the second serialization and makes the OnceLock bytes and redb bytes identical.

---

## DEFECT-006: Owner returns `Err` on successful compute when `put` to redb fails
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** `store.rs:286-289`
**Evidence:**
```rust
if let Err(e) = self.put(cache_type, key, value) {
    let _ = slot.set(Err(anyhow::anyhow!("{e:#}")));
    self.in_flight.remove(&in_flight_key);
    return Err(e);  // <-- owner had compute_result = Ok(value) but returns Err
}
```
When `compute()` succeeds (`compute_result = Ok(value)`) but `self.put()` fails (e.g., disk full, write transaction conflict), the owner returns `Err(e)` even though the computation succeeded. The caller invoked `get_or_compute` — the contract is "get cached or compute." The computation succeeded; the cache write is an implementation detail.

The waiters also receive this `Err`, so all N threads get an error despite the value being available.

**Impact:** False error propagation. Callers retry, wasting compute resources. The successfully computed value is silently discarded. Under transient redb write contention, this causes unnecessary retries.

**Fix:** On `put` failure, still return `compute_result` (the successful value) to the owner. Set the OnceLock to `Ok(serialized_bytes)` so waiters also get the value. Log or surface the cache-write failure as a warning, not as a blocking error. Alternatively, store the serialized bytes directly from `slot_result` to redb to avoid the second `put` call entirely (see DEFECT-005).

---

## DEFECT-007: Spin-wait loop contradicts documentation — no `OnceLock::wait()` usage
**Severity:** MINOR
**Phase:** PERFORMANCE
**File:** `store.rs:22-24` + `store.rs:56-67`
**Evidence:**
```rust
// Doc comment (line 22-24):
// `OnceLock::wait()` was stabilised in Rust 1.87. On older compilers the
// method may not exist; in that case, fall back to a spin-wait loop...

// Actual implementation (line 56-67):
fn wait_once_lock(slot: &ComputeSlot) -> Result<Vec<u8>> {
    loop {                                        // ALWAYS spins
        if let Some(result) = slot.get() {
            return match result.as_ref() { ... };
        }
        std::hint::spin_loop();
        std::thread::yield_now();
    }
}
```
The documentation claims the implementation uses `OnceLock::wait()` (OS-level parking, zero CPU) on Rust 1.87+. The actual code is **always** a spin-wait loop with no `#[cfg]` guard and no `OnceLock::wait()` call anywhere.

**Impact:** Waiters burn CPU instead of parking. Under high concurrency with expensive `compute()` functions, this wastes significant CPU cycles. Documentation is actively misleading.

**Fix:** Either (a) implement the `OnceLock::wait()` path with a `#[cfg]` version check or feature flag, or (b) rewrite the doc comment to accurately describe the spin-wait behavior.

---

## DEFECT-008: `CacheConfig` and `CacheStats` missing `#[non_exhaustive]`
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** `config.rs:35` + `config.rs:88`
**Evidence:**
```rust
#[derive(Debug, Clone)]
pub struct CacheConfig { ... }    // no #[non_exhaustive]

#[derive(Debug, Clone)]
pub struct CacheStats { ... }     // no #[non_exhaustive]
```
Meanwhile, `CacheBackend` (line 26) and `CacheType` (line 81) correctly have `#[non_exhaustive]`. Adding a field to `CacheConfig` or `CacheStats` is a breaking change.

**Impact:** Any downstream crate constructing `CacheConfig { ... }` or pattern-matching `CacheStats { ... }` will break on field additions.

**Fix:** Add `#[non_exhaustive]` to both structs and provide constructor methods.

---

## DEFECT-009: Error propagation test uses `thread::sleep` for ordering — inherently racy
**Severity:** MINOR
**Phase:** TESTING
**File:** `tests.rs:326`
**Evidence:**
```rust
// Let the owner register in the DashMap before spawning waiters
thread::sleep(std::time::Duration::from_millis(20));
```
The test assumes 20ms is sufficient for the owner to call `get_or_compute`, miss the cache, create the `InFlightKey`, and insert into the DashMap. Under load (CI, containerized environments), this race can fail, producing intermittent test failures or false positives.

**Impact:** Flaky test in CI. False confidence in error propagation.

**Fix:** Use a synchronization primitive (e.g., `std::sync::Barrier` or `Arc<AtomicBool>`) to ensure the owner has registered in the DashMap before waiters proceed.

---

## DEFECT-010: No test for panic-in-compute deadlock (DEFECT-001)
**Severity:** MINOR
**Phase:** TESTING
**File:** `tests.rs` (missing coverage)
**Evidence:** No test exercises the scenario where `compute()` panics while waiters are parked in `wait_once_lock`. This is the most dangerous failure mode of the entire `get_or_compute` design (DEFECT-001), yet it is completely untested.

**Impact:** The CRITICAL deadlock defect has no regression test. A fix could silently regress.

**Fix:** Add a test that spawns an owner (whose compute closure panics) and a waiter with a timeout (e.g., `std::thread::spawn` + `join` with a `std::time::Instant` guard). Assert the waiter does NOT hang indefinitely. Once DEFECT-001 is fixed, this test validates the fix.

---

## DEFECT-011: `CacheError` missing `#[non_exhaustive]`
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** `errors.rs:277`
**Evidence:**
```rust
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CacheError { ... }    // no #[non_exhaustive]
```
Same issue as DEFECT-008. `CacheBackend` and `CacheType` in config.rs have `#[non_exhaustive]` but `CacheError` does not, creating inconsistency.

**Impact:** Adding a variant to `CacheError` is a breaking change for downstream `match` statements.

**Fix:** Add `#[non_exhaustive]`.

---

## PASS ITEMS (what was done right)

| Check | Status |
|---|---|
| Zero `Mutex` in production code | PASS |
| Zero `unwrap`/`expect` in production code | PASS |
| SHA-256 content hashing (stable across Rust versions) | PASS |
| `CacheBackend` enum (Memory / File) | PASS |
| Key max 256 bytes enforced | PASS |
| Value max 10MB enforced | PASS |
| `#[non_exhaustive]` on `CacheBackend` and `CacheType` | PASS |
| `#[cfg(test)]` gate on test module | PASS |
| DashMap + OnceLock dedup architecture | PASS (logic correct when no panic) |
| In-flight cleanup after compute | PASS |
| Error propagation to waiters | PASS |
| Thread safety (Send + Sync) | PASS |
| redb MVCC for concurrent reads | PASS |
| `#[must_use]` on pure functions | PASS |
| Clippy lint compliance in production code | PASS |
| `content_hash` returns fixed `[u8; 32]` | PASS |
| No unsafe code | PASS |
| Serde round-trip in basic tests | PASS |
| Config validation on open | PASS |

---

## FIX PRIORITY ORDER

| Priority | Defect | Effort |
|---|---|---|
| P0 | DEFECT-001: Panic deadlock | Medium — `catch_unwind` + destructor |
| P0 | DEFECT-002: Non-unix compile | Trivial — `#[cfg(unix)]` on re-export |
| P1 | DEFECT-006: False error on put failure | Low — return `compute_result` instead of `Err(e)` |
| P1 | DEFECT-005: Double serialization | Low — use `_bytes` directly |
| P2 | DEFECT-003: store.rs 300-line | Low — extract `inflight.rs` |
| P2 | DEFECT-004: tests.rs 300-line | Low — split test modules |
| P2 | DEFECT-007: Doc vs impl mismatch | Trivial — fix doc or add `OnceLock::wait` |
| P3 | DEFECT-008/011: Missing `#[non_exhaustive]` | Trivial |
| P3 | DEFECT-009: Racy test | Low — use `Barrier` |
| P3 | DEFECT-010: Missing panic test | Low — add with timeout |
