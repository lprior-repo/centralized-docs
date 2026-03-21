# Black-Hat Review R8 — Cache Module
## Target: centralized-docs/src/cache/ + src/errors/
## Date: 2026-03-21
## Auditor: black-hat-reviewer (5-phase, zero prior context, every line read)

---

## R6 Resolution Status

| R6 ID | Status | Notes |
|-------|--------|-------|
| 001 (in_flight.remove) | FIXED | No remove() anywhere; lazy cleanup via clear_all only |
| 002 (serialization divergence) | **NOT FIXED** | Owner gets Ok(value), waiters get Err at dedup.rs:140-143 + store/mod.rs:226 |
| 003 (errors.rs >300L) | FIXED | Split into errors/ directory; all files under 300L |
| 004 (silent cache write failure) | **NOT FIXED** | TODO comment at dedup.rs:153; no eprintln! emitted |
| 005 (#[non_exhaustive] on error enums) | MOSTLY FIXED | 7 of 8 now have it; DocTransformerError (errors/mod.rs:35) missing |
| 006 (dual serialization paths) | **NOT FIXED** | put_raw still skips validate_value_size |
| 007 (spin_loop) | **NOT FIXED** | Still at dedup.rs:63; violates hard constraint |
| 008 (#[non_exhaustive] on InflightDecision) | FIXED | Present at dedup.rs:170 |

---

```
STATUS: REJECTED

## DEFECT-001: `spin_loop()` violates hard constraint "yield_now only, no spin_loop"
**Severity:** MAJOR
**Phase:** PERFORMANCE
**File:** cache/store/dedup.rs:63
**Constraint violated:** "30s timeout on wait_once_lock (yield_now only, no spin_loop)"
**Evidence:**
```rust
std::hint::spin_loop();
std::thread::yield_now();
```
`spin_loop()` emits the x86 `PAUSE` instruction designed for tight spin-loops with nanosecond waits. `yield_now()` immediately deschedules the thread for a full scheduler quantum (~1-10ms). The `PAUSE` is wasted work — the CPU deschedules regardless. Under extreme contention (100+ threads per RQ attack 1), this adds N useless instructions per polling cycle with zero benefit.
**Fix:** Delete line 63 (`std::hint::spin_loop()`). Keep only `std::thread::yield_now()` at line 64.

---

## DEFECT-002: Owner/waiter result divergence on serialization failure
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:140-143, cache/store/mod.rs:226
**Constraint violated:** "Owner and waiter get same result on serialization failure"
**Evidence:**
In `finalize_compute` (dedup.rs:140-143):
```rust
let slot_result: Result<Vec<u8>> = match compute_result {
    Ok(value) => serde_json::to_vec(value).map_err(Into::into),
    Err(e) => Err(anyhow::anyhow!("{e:#}")),
};
```
In `get_or_compute` (store/mod.rs:226):
```rust
compute_result  // returns ORIGINAL Ok(value) to owner
```
**Failure mode:** When `compute_result` is `Ok(value)` but `serde_json::to_vec(value)` fails:
- `slot_result` = `Err(serialization_error)` → published to OnceLock
- Owner returns `Ok(value)` (the original, never-serialized result)
- Waiters call `wait_once_lock` → get `Err(serialization_error)` from OnceLock
- Owner succeeds silently; all waiters receive an error
This breaks the exact-once deduplication invariant. Triggerable with a type that has `Serialize` in its bounds but whose `Serialize` impl returns an error (e.g., a type containing `Rc<T>` with a custom `serde` impl).
**Fix:** If `serde_json::to_vec` fails, propagate the error to the owner too:
```rust
let slot_result: Result<Vec<u8>> = match compute_result {
    Ok(value) => serde_json::to_vec(value).map_err(Into::into),
    Err(e) => Err(anyhow::anyhow!("{e:#}")),
};
if let Err(ref e) = slot_result {
    let _ = slot.set(Err(anyhow::anyhow!("{e:#}")));
    return Err(anyhow::anyhow!("compute succeeded but serialization failed: {e:#}"));
}
```

---

## DEFECT-003: `put_raw` bypasses `validate_value_size` — values >10MB can be cached
**Severity:** MAJOR
**Phase:** SECURITY
**File:** cache/store/mod.rs:244-259, cache/store/dedup.rs:149
**Constraint violated:** "Single validate_and_insert function for value validation"
**Evidence:**
The `get_or_compute` path flows through:
1. `finalize_compute` (dedup.rs:140): serializes via `serde_json::to_vec(value)` — no size check
2. `finalize_compute` (dedup.rs:150): calls `put_raw(cache_type, key, bytes)` — no size check
3. `put_raw` (store/mod.rs:255): `table.insert(key, bytes)?` — direct insert

Meanwhile, the `put_document`/`put_scrape`/`put_transform` paths go through `put_cached_value_with_limit` (hash.rs:67-68) which calls `validate_value_size`. Two separate insertion paths with different validation behavior.
**Attack scenario:** A `get_or_compute` call with a value whose serialized form exceeds 10MB bypasses the value size limit entirely. The constraint "value max 10MB" is silently violated.
**Fix:** Add `validate_value_size(bytes)?` as the first line of `put_raw`, OR create a single `validate_and_insert` function as specified in the constraint.

---

## DEFECT-004: Missing `eprintln!` on cache write failure
**Severity:** MINOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:149-154
**Constraint violated:** "eprintln! on cache write failure"
**Evidence:**
```rust
if put_raw(cache_type, key, bytes).is_err() {
    // Cache write failed but compute succeeded — return value anyway.
    // The slot still gets set with Ok(bytes) so waiters get the value.
    // TODO(production): wire up log::warn! for observability.
}
```
The error is silently swallowed. No `eprintln!`, no logging, no metric. Future `get_or_compute` calls will recompute every time since the value is never persisted, silently defeating caching with zero observability.
**Fix:** Replace the TODO comment with `eprintln!`:
```rust
if put_raw(cache_type, key, bytes).is_err() {
    eprintln!(
        "WARN: cache write failed for key (cache_type={cache_type:?}, key_len={}), value not persisted",
        key.len()
    );
}
```

---

## DEFECT-005: Missing `#[non_exhaustive]` on `DocCache` (public struct)
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/mod.rs:36-37
**Constraint violated:** "#[non_exhaustive] on ALL public enums AND structs (including pub(super))"
**Evidence:**
```rust
#[derive(Debug, Clone)]
pub struct DocCache {
    db: Arc<Database>,
    config: CacheConfig,
    in_flight: Arc<DashMap<...>>,
}
```
No `#[non_exhaustive]` attribute. `DocCache` is the primary public API type. Adding fields (e.g., a metrics counter) is a breaking semver change.
**Fix:** Add `#[non_exhaustive]` above `#[derive(Debug, Clone)]`.

---

## DEFECT-006: Missing `#[non_exhaustive]` on `InFlightKey` (pub(super) struct)
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/dedup.rs:34-35
**Constraint violated:** "#[non_exhaustive] on ALL public enums AND structs (including pub(super))"
**Evidence:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct InFlightKey {
    pub cache_type: CacheType,
    pub key_hash: [u8; 32],
}
```
No `#[non_exhaustive]` attribute.
**Fix:** Add `#[non_exhaustive]` above `#[derive(...)]`.

---

## DEFECT-007: Missing `#[non_exhaustive]` on `DocTransformerError` (public enum)
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** src/errors/mod.rs:35
**Constraint violated:** "#[non_exhaustive] on ALL public enums AND structs"
**Evidence:**
```rust
#[derive(Debug, Error)]
#[allow(clippy::large_enum_variant)]
#[allow(dead_code)]
pub enum DocTransformerError {
```
This is the only error enum in the entire errors/ module missing `#[non_exhaustive]`. All 7 others (CacheError, ConfigError, ValidationError, DocumentError, IndexError, IoError, EmbeddingError) now have it. This is the top-level error type for the entire library — the most important one to have `#[non_exhaustive]`.
**Fix:** Add `#[non_exhaustive]` above `#[derive(Debug, Error)]`.

---

## Notes (no defect filed)

1. **R6 CRITICAL fixes all verified:** `in_flight.remove()` is gone (DEFECT-004 fix comment at dedup.rs:160-164). `catch_unwind` is correct (store/mod.rs:203-216). 30s timeout is correct (dedup.rs:48). Empty key rejection is correct (hash.rs:40). Early key validation in `get_or_compute` is correct (store/mod.rs:188).

2. **All files under 300 lines:** Largest is `store/mod.rs` at 260 lines. The errors/ split is complete — largest error file is `transformer.rs` at 89 lines.

3. **No Mutex, no channels, no unwrap/expect in production:** Confirmed across all production code. Test code correctly uses `unwrap_err()` and `expect()` for assertions.

4. **`#[cfg(unix)]` on `path_hash` and re-export:** Correctly applied at hash.rs:103 and mod.rs:25-26.

5. **DashMap + OnceLock + catch_unwind for dedup:** Correctly implemented. DashMap at store/mod.rs:40, OnceLock at dedup.rs:41, catch_unwind at store/mod.rs:203.

6. **DocCache derives Clone (via Arc<Database>):** Correct at store/mod.rs:36-38.

7. **SHA-256 hashing:** Correct at hash.rs:87-95 using `sha2::Sha256`.

8. **Key 1..=256 bytes, value max 10MB:** Constants correct at config.rs:11,15. Validation correct at hash.rs:38-46 (rejects 0 and >256). Value validation is correct in the `put_*` methods but **missing in the `put_raw` path** (DEFECT-003).

9. **Test coverage is comprehensive:** 15 adversarial tests covering extreme concurrency, error propagation (50 waiters), disk corruption, in-memory backend, empty inputs, double-open, concurrent clear, special bytes, type mismatch, stats accuracy, disabled-cache, and boundary limits. No gaps for the current API surface.

10. **Gap in test coverage for DEFECT-002 and DEFECT-003:** There is no test that triggers a serialization failure in `get_or_compute` (to prove owner/waiter divergence), and no test that inserts a value >10MB through the `get_or_compute` path (to prove the bypass). These should be added as adversarial tests.

---

## Summary

| Severity | Count | IDs |
|----------|-------|-----|
| CRITICAL | 0     | -- |
| MAJOR    | 3     | 001, 002, 003 |
| MINOR    | 4     | 004, 005, 006, 007 |

**Verdict: REJECTED** — Three MAJOR defects. DEFECT-001 is a direct hard-constraint violation (`spin_loop()` must not exist per "yield_now only, no spin_loop"). DEFECT-002 breaks the exact-once dedup invariant on serialization failure. DEFECT-003 allows values >10MB to bypass the size limit via `get_or_compute`.

**Progress from R6:** `in_flight.remove()` eliminated (R6-001). File-length issue resolved (R6-003). `#[non_exhaustive]` added to `InflightDecision` (R6-008). 6 of 8 error enums now have `#[non_exhaustive]`. All R6 CRITICAL defects remain fixed.
