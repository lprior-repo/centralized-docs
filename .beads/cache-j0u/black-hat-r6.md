# Black-Hat Review R6 — Cache Module
## Target: centralized-docs/src/cache/ + errors.rs
## Date: 2026-03-21
## Auditor: black-hat-reviewer (5-phase, zero prior context, every line read)

---

## R5 Resolution Status

| R5 ID | Status | Notes |
|-------|--------|-------|
| 001 (errors.rs >300L) | **NOT FIXED** | Still 321 lines |
| 002 (unbounded spin) | FIXED | 30s timeout added at dedup.rs:42 |
| 003 (missing catch_unwind) | FIXED | Proper catch_unwind at store/mod.rs:204-218 |
| 004 (in_flight.remove race) | **NOT FIXED** | Still at dedup.rs:142; violates hard constraint |
| 005 (empty key allowed) | FIXED | validate_key_size rejects len==0 at hash.rs:40 |
| 006 (get_or_compute no early validation) | FIXED | validate_key_size at store/mod.rs:190 |
| 007 (silent cache write failure) | **NOT FIXED** | Still at dedup.rs:131-136 |
| 008 (double validation paths) | **NOT FIXED** | Still two separate paths |
| 009 (public CacheConfig fields) | NOT FIXED | Design choice, low priority |
| 010 (DocCache not Clone) | NOT FIXED | Design choice, low priority |
| 011 (SHA-256 collision theoretical) | NOT FIXED | Accepted limitation |
| 012 (clear_all TOCTOU) | NOT FIXED | Inherent to MVCC, documented |
| 013 (#[non_exhaustive] on InflightDecision) | **NOT FIXED** | Still missing |
| 014 (adversarial_stress.rs near limit) | NOT FIXED | Still 242 lines |
| 015 (O(n^2) collision test) | FIXED | Uses HashSet at adversarial.rs:63-68 |
| 016 (#[non_exhaustive] on error enums) | **NOT FIXED** | 7 of 8 still missing |

---

```
STATUS: REJECTED

## DEFECT-001: `in_flight.remove()` violates hard constraint "No in_flight.remove()"
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:142
**Evidence:**
```rust
// Step 7: clean up — future callers hit redb cache
in_flight.remove(&in_flight_key);
```
**Constraint violation:** The stated constraints explicitly say "No in_flight.remove() (lazy cleanup via clear_all)." This line directly violates that constraint.
**Analysis:** The ordering inside `finalize_compute` is: redb commit (via `put_raw`) → OnceLock.set → DashMap.remove. When `put_raw` succeeds, the redb write is committed BEFORE the remove, so a new thread arriving after the remove hits the redb cache and is fine. However, when `put_raw` fails (best-effort at line 132), the OnceLock is set with serialized bytes but redb has nothing. After the remove, a new thread: (a) misses redb (write failed), (b) misses DashMap (removed), (c) becomes a new owner and recomputes. The OnceLock is orphaned — waiters who cloned the Arc before the remove can still read it, but new arrivals cannot. Dedup guarantee degrades to "at most N computes" where N depends on redb failure rate, not "exactly one."
**Fix:** Delete line 142. Let entries accumulate in the DashMap and be cleaned only by `clear_all()` (which already calls `self.in_flight.clear()` at store/mod.rs:147). Accept the memory trade-off: each unique computed key retains a dead `Arc<OnceLock>` (~40 bytes) until `clear_all` is called.

---

## DEFECT-002: Serialization failure causes owner/waiter result divergence
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:125-128
**Evidence:**
```rust
let slot_result: Result<Vec<u8>> = match compute_result {
    Ok(value) => serde_json::to_vec(value).map_err(Into::into),
    Err(e) => Err(anyhow::anyhow!("{e:#}")),
};
```
Then at store/mod.rs:228: `compute_result` (the original `Result<V>`) is returned to the owner. Meanwhile, `slot_result` (the serialized bytes) is published to the OnceLock for waiters.
**Failure mode:** If `compute_result` is `Ok(value)` but `serde_json::to_vec(value)` fails (e.g., custom Serialize impl that errors, cyclic references, non-serializable fields like `Rc`):
- `slot_result` = `Err(serialization error)` → OnceLock set with Err
- Owner returns `Ok(value)` at store/mod.rs:228
- Waiters call `wait_once_lock` → get `Err(serialization error)` from OnceLock
- Waiters receive an error; owner receives the value

This breaks the "all threads get the same result" invariant. Owner succeeds silently while waiters fail.
**Fix:** If `serde_json::to_vec` fails, do NOT set the slot_result with the error. Instead, set it with the original error context that indicates the value was computed but could not be serialized. Better yet, attempt serialization BEFORE calling compute (serialize a sentinel type to verify the type is serializable), or propagate the serialization error back to the owner as well:
```rust
let slot_result: Result<Vec<u8>> = match compute_result {
    Ok(value) => serde_json::to_vec(value).map_err(Into::into),
    Err(e) => Err(anyhow::anyhow!("{e:#}")),
};
// If serialization failed, the owner should also fail
if let Err(ref e) = slot_result {
    return Err(anyhow::anyhow!("compute succeeded but serialization failed: {e:#}"));
}
```

---

## DEFECT-003: `errors.rs` exceeds 300-line file limit (321 lines)
**Severity:** MAJOR
**Phase:** MAINTAINABILITY
**File:** errors.rs (321 lines)
**Evidence:** File is 21 lines over the 300-line hard limit. This was flagged in R5 (DEFECT-001) and remains unfixed.
**Impact:** Violates stated architectural constraint. The file contains 8 error enums + tests + re-exports.
**Fix:** Extract into `src/errors/` directory:
- `errors/mod.rs` (re-exports + DocTransformerError + tests) ~80 lines
- `errors/config.rs` (ConfigError) ~20 lines
- `errors/validation.rs` (ValidationError) ~45 lines
- `errors/document.rs` (DocumentError) ~30 lines
- `errors/index.rs` (IndexError) ~30 lines
- `errors/io.rs` (IoError) ~20 lines
- `errors/embedding.rs` (EmbeddingError) ~30 lines
- `errors/cache.rs` (CacheError) ~20 lines

---

## DEFECT-004: Silent cache write failure swallows errors without observability
**Severity:** MINOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:131-136
**Evidence:**
```rust
if let Ok(ref bytes) = slot_result {
    if put_raw(cache_type, key, bytes).is_err() {
        // Cache write failed but compute succeeded — return value anyway.
        // The slot still gets set with Ok(bytes) so waiters get the value.
    }
}
```
**Impact:** If the redb write fails (disk full, corruption, value exceeds limit after serialization), the error is silently discarded. No logging, no metric, no indication to the caller. Future `get_or_compute` calls will recompute every time since the value is never persisted, defeating caching.
**Fix:** At minimum, use `eprintln!` or `tracing::warn!` to log the failure. Ideally, return a result type that distinguishes "value computed but not persisted" from "value computed and persisted." Given the zero-dependency stance, `eprintln!` is the minimum viable fix:
```rust
if put_raw(cache_type, key, bytes).is_err() {
    eprintln!("WARN: cache write failed for key (cache_type={cache_type:?}), value not persisted");
}
```

---

## DEFECT-005: `#[non_exhaustive]` missing on 7 of 8 public error enums in `errors.rs`
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** errors.rs:23, 99, 119, 164, 193, 222, 247
**Evidence:** Only `CacheError` (line 276) has `#[non_exhaustive]`. The following public enums do NOT:
- `DocTransformerError` (line 23) — `pub enum`
- `ConfigError` (line 99) — `pub enum`
- `ValidationError` (line 119) — `pub enum`
- `DocumentError` (line 164) — `pub enum`
- `IndexError` (line 193) — `pub enum`
- `IoError` (line 222) — `pub enum`
- `EmbeddingError` (line 247) — `pub enum`

**Constraint:** "#[non_exhaustive] on all public enums/structs."
**Impact:** Adding any variant to these enums is a breaking semver change for downstream crates. Exhaustive `match` statements in consumer code will fail to compile.
**Fix:** Add `#[non_exhaustive]` to all 7 enums. This is a one-line change per enum.

---

## DEFECT-006: Dual serialization/validation paths create maintenance divergence risk
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/hash.rs:62-71 (`put_cached_value_with_limit`), cache/store/mod.rs:242-256 (`put_raw`)
**Evidence:**
- `put_cached_value_with_limit`: serializes via `serde_json::to_vec`, then validates size, then inserts
- `put_raw`: takes pre-serialized bytes, validates size, then inserts

Two separate code paths enforce the same value-size invariant. If MAX_VALUE_SIZE validation logic changes (e.g., adding a header check), both paths must be updated in lockstep.
**Fix:** Extract a single `validate_and_insert(table, key, bytes)` function used by both paths. The `put_*` methods call `put_cached_value_with_limit` (serialize → validate → insert). The `get_or_compute` owner path already has pre-serialized bytes and calls `put_raw` directly. Unify:
```rust
pub(crate) fn validate_and_insert(
    table: &mut Table<&[u8], &[u8]>,
    key: &[u8],
    bytes: &[u8],
) -> Result<()> {
    validate_value_size(bytes)?;
    table.insert(key, bytes)?;
    Ok(())
}
```

---

## DEFECT-007: `spin_loop()` is ineffective when paired with `yield_now()`
**Severity:** MINOR
**Phase:** PERFORMANCE
**File:** cache/store/dedup.rs:57-58
**Evidence:**
```rust
std::hint::spin_loop();
std::thread::yield_now();
```
`spin_loop()` emits the x86 `PAUSE` instruction, which is designed for tight spin-wait loops where the expected wait is nanoseconds to microseconds. `yield_now()` immediately yields the thread's time slice, sleeping for at least one scheduler quantum (~1-10ms). The `spin_loop()` hint is wasted because the thread is about to sleep — the CPU will deschedule it regardless.
**Impact:** Zero functional impact. The `spin_loop()` compiles to a single no-op `PAUSE` instruction that costs nothing. Purely a code clarity issue.
**Fix:** Remove `std::hint::spin_loop()` from line 57. Keep only `std::thread::yield_now()`. Alternatively, keep both with a comment explaining the intent (yield for I/O-bound waits, spin_loop as belt-and-suspenders).

---

## DEFECT-008: `#[non_exhaustive]` missing on `InflightDecision` (pub(super))
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/dedup.rs:148
**Evidence:**
```rust
pub(super) enum InflightDecision<V> {
```
No `#[non_exhaustive]` attribute. While `pub(super)` limits visibility to the parent module, the stated constraint is "#[non_exhaustive] on all public enums/structs" and `pub(super)` is a form of public visibility.
**Impact:** Negligible — only visible within `store/mod.rs`. But violates the blanket convention.
**Fix:** Add `#[non_exhaustive]` for consistency.

---

## Notes (no defect filed)

1. **R5 CRITICAL fixes verified:** Both `catch_unwind` (store/mod.rs:204-218) and the 30s timeout (dedup.rs:42) are correctly implemented. The catch_unwind properly downcasts panic payloads and converts to anyhow errors. The timeout uses `Instant` arithmetic with no overflow risk for reasonable timeouts.

2. **`panic = "abort"` caveat:** If the crate is compiled with `panic = "abort"` in Cargo.toml, `catch_unwind` is a no-op and the process will abort on any compute panic, re-introducing the waiter deadlock. This is a Cargo.toml configuration concern, not a code defect, but worth documenting.

3. **`clear_all` + `in_flight.clear()` ordering (store/mod.rs:147-148):** The in_flight map is cleared before the redb transaction begins. This is safe because: (a) waiters holding Arc clones to OnceLock slots are unaffected — their Arc keeps the slot alive, (b) new threads will miss the DashMap and check redb, which still has data until the delete+commit completes. The worst case is a brief window where a new thread becomes a second owner and recomputes, which is acceptable.

4. **Test coverage is strong:** 15 adversarial tests covering extreme concurrency (100 threads), error propagation (50 waiters), disk corruption, in-memory backend, empty inputs, double-open, concurrent clear, special bytes, type mismatch, stats accuracy, and disabled-cache behavior. No gaps in coverage for the current API surface.

---

## Summary

| Severity | Count | IDs |
|----------|-------|-----|
| CRITICAL | 0     | — |
| MAJOR    | 3     | 001, 002, 003 |
| MINOR    | 5     | 004, 005, 006, 007, 008 |

**Verdict: REJECTED** — Three MAJOR defects remain. DEFECT-001 is a direct violation of a stated hard constraint (in_flight.remove must not exist). DEFECT-002 introduces owner/waiter result divergence on serialization failure. DEFECT-003 is a recurring file-length violation from R5.

**Positive progress from R5:** Both CRITICAL defects (unbounded spin, missing catch_unwind) are correctly fixed. Empty-key validation and early key-size check in get_or_compute are implemented. The O(n^2) collision test is now O(n) via HashSet.
