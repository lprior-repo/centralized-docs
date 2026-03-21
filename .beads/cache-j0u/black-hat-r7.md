# Black-Hat Audit Report — Cache Module (Round 7)

**Auditor**: black-hat-reviewer (manual, no prior context)  
**Date**: 2026-03-21  
**Target**: `centralized-docs/src/cache/` + `centralized-docs/src/errors/`  
**Lines audited**: ~1,750 (every line of every target file)

---

```
STATUS: REJECTED

9 defects found. 3 CRITICAL, 3 HIGH, 3 MEDIUM.
```

---

## PHASE 1: CORRECTNESS

### DEFECT-001 [CRITICAL]: `spin_loop()` present in `wait_once_lock` — violates "yield_now only" constraint

**File**: `store/dedup.rs:63`  
**Constraint**: "30s timeout on wait_once_lock (yield_now only, no spin_loop)"

```rust
std::hint::spin_loop();    // <-- VIOLATION
std::thread::yield_now();
```

`std::hint::spin_loop()` is a CPU-burning hint to the processor. The constraint explicitly requires yield_now only. Even if the impact is small when the owner is doing real I/O, the constraint is absolute. On a system with 100+ waiters all calling `spin_loop()`, this becomes measurable wasted CPU.

**Fix**: Remove `std::hint::spin_loop();` entirely. Keep only `std::thread::yield_now();`.

---

### DEFECT-002 [CRITICAL]: Owner/waiter result mismatch on serialization failure

**File**: `store/dedup.rs:140-158` + `store/mod.rs:202-228`  
**Constraint**: "Owner and waiter get same result on serialization failure"

Trace the failure path:

1. `compute` succeeds → `compute_result = Ok(value)`  
2. `finalize_compute` serializes: `serde_json::to_vec(value)` → **fails** (e.g. custom Serialize impl panics or returns error for certain types)  
3. `slot_result = Err(serialization_error)`  
4. `slot.set(slot_result)` → OnceLock now holds `Err`  
5. **Owner** returns `compute_result` → `Ok(value)` ← from `store/mod.rs:226`  
6. **Waiter** calls `wait_once_lock(&slot)` → gets `Err(serialization_error)` from the OnceLock  

**Owner gets `Ok(value)`, waiter gets `Err`.** They do NOT get the same result. This violates the explicit constraint.

Additionally, all *subsequent* callers with the same key will:
- Miss the redb cache (nothing was stored)  
- Find the stale OnceLock entry → get `Err(serialization_error)` forever  

The OnceLock entry is permanent (DEFECT-004 fix), so this poisons the key for the lifetime of the process.

**Fix**: When serialization fails, the slot should be set to a result that preserves the original compute outcome. One approach: set the slot to `Ok(value)` by avoiding the intermediate serialization, or use a different slot type that stores `Result<V>` instead of `Result<Vec<u8>>` (serializing only for the redb write path, not for the OnceLock).

---

### DEFECT-003 [CRITICAL]: Value size validation bypassed via `get_or_compute`

**File**: `store/dedup.rs:140-155` + `store/mod.rs:244-258`  
**Constraint**: "value max 10MB" + "Single validate_and_insert for value validation"

The `get_or_compute` → `finalize_compute` → `put_raw` path:

1. `finalize_compute` serializes: `serde_json::to_vec(value)` → `bytes`  
2. **No call to `validate_value_size(&bytes)`**  
3. `put_raw` explicitly skips validation (comment at `store/mod.rs:253`: "Value size already validated during serialization in get_or_compute")  
4. **But the validation never happened.** The comment is lying.  

A compute closure that returns a value serializing to >10MB will be stored in redb, bypassing the size limit.

This also violates "Single validate_and_insert" — there are now TWO paths: `put_cached_value_with_limit` (validates) and `put_raw` (does not).

**Fix**: Add `validate_value_size(&bytes)?` in `finalize_compute` before calling `put_raw`, or merge `put_raw` into a function that always validates. If validation fails, the slot should be set with the error so owner and waiters both receive it (see DEFECT-002).

---

## PHASE 2: SECURITY

### DEFECT-004 [HIGH]: `DocCache` struct missing `#[non_exhaustive]`

**File**: `store/mod.rs:37`  
**Constraint**: "#[non_exhaustive] on ALL public enums AND structs"

```rust
#[derive(Debug, Clone)]
pub struct DocCache {  // <-- missing #[non_exhaustive]
```

All four other public types in config.rs have it. `DocCache` is the primary public API surface and can have fields added in future versions. Without `#[non_exhaustive]`, downstream code can construct `DocCache { db, config, in_flight }` directly and will break on field additions.

**Fix**: Add `#[non_exhaustive]` above `#[derive(Debug, Clone)]`.

---

### DEFECT-005 [HIGH]: `DocTransformerError` enum missing `#[non_exhaustive]`

**File**: `errors/mod.rs:36`  
**Constraint**: "#[non_exhaustive] on ALL public enums AND structs"

```rust
#[derive(Debug, Error)]
pub enum DocTransformerError {  // <-- missing #[non_exhaustive]
```

This is the top-level error enum. All sub-error types (CacheError, ValidationError, etc.) have `#[non_exhaustive]`. The parent does not. Adding a new variant will break downstream exhaustive matches.

**Fix**: Add `#[non_exhaustive]` above `#[derive(Debug, Error)]`.

---

### DEFECT-006 [MEDIUM]: Misleading comment — in_flight entries NOT bounded by "concurrent" keys

**File**: `store/dedup.rs:21`  
**Code**: `In-flight entries accumulate but are bounded by the number of concurrent keys.`

This is **false**. Entries are bounded by the number of **unique** keys ever computed, not concurrent keys. Once a key's OnceLock is set, the entry persists forever (DEFECT-004 fix). Only `clear_all()` removes entries.

For a long-running process computing millions of unique keys, the DashMap grows unboundedly: each entry is `InFlightKey` (40 bytes) + `Arc<OnceLock<Result<Vec<u8>>>>` (16 bytes) + DashMap overhead (~64 bytes) ≈ 120 bytes/key. At 1M unique keys = ~120MB.

**Fix**: Change comment to say "unique keys" not "concurrent keys". Consider a periodic eviction strategy (e.g. LRU or time-based) for entries with completed OnceLocks.

---

## PHASE 3: PERFORMANCE

(No defects beyond DEFECT-001 which covers the `spin_loop` CPU waste.)

---

## PHASE 4: MAINTAINABILITY

### DEFECT-007 [MEDIUM]: Missing `eprintln!` on cache write failure

**File**: `store/dedup.rs:150-154`  
**Constraint**: "eprintln on cache write failure"

```rust
if put_raw(cache_type, key, bytes).is_err() {
    // Cache write failed but compute succeeded — return value anyway.
    // The slot still gets set with Ok(bytes) so waiters get the value.
    // TODO(production): wire up log::warn! for observability.
}
```

The failure is silently swallowed. The constraint requires `eprintln!`. The comment acknowledges this should be logged. No logging of any kind occurs.

**Fix**: Add `eprintln!("[cache] write failed for {:?}:{:?}: {:?}", cache_type, key, err);` inside the `if let Err(err) = put_raw(...)` block.

---

## PHASE 5: TESTING

### DEFECT-008 [HIGH]: No test for value size limit enforcement via `get_or_compute`

**Constraint**: "value max 10MB"

The 10MB value limit is tested via `put_document` (limits.rs:22-39) but never via `get_or_compute`. Given DEFECT-003 shows the limit is bypassed through `get_or_compute`, this missing test allowed the bug to exist undetected.

**Fix**: Add test:
```rust
#[test]
fn test_get_or_compute_rejects_overserialized_value() {
    let cache = DocCache::open(CacheConfig::in_memory())?;
    let big = vec![0u8; MAX_VALUE_SIZE + 1];
    let result = cache.get_or_compute(CacheType::Document, b"key", || Ok(big));
    assert!(result.is_err(), "oversized value should be rejected via get_or_compute");
}
```

---

### DEFECT-009 [MEDIUM]: No test for serialization failure owner/waiter consistency

**Constraint**: "Owner and waiter get same result on serialization failure"

No test exercises the case where compute succeeds but serialization fails, which is exactly the path with DEFECT-002. This missing test allowed the owner/waiter mismatch to exist undetected.

**Fix**: Create a type that serializes successfully for return but fails when re-serialized (e.g. using `serde_json::to_vec` with a custom `Serialize` impl that tracks call count and fails on the 2nd call). Verify owner and waiter get identical results.

---

## CONSTRAINT VERIFICATION SUMMARY

| Constraint | Status | Details |
|---|---|---|
| Zero Mutex/channels/unwrap/expect | PASS | No violations in production code |
| DashMap + OnceLock + catch_unwind | PASS | Correctly implemented |
| No in_flight.remove() | PASS | Confirmed absent |
| 30s timeout on wait_once_lock | FAIL | DEFECT-001: `spin_loop()` present |
| SHA-256 | PASS | `sha2::Sha256` used throughout |
| Key 1..=256 bytes | PASS | `validate_key_size` rejects 0 and >256 |
| Value max 10MB | FAIL | DEFECT-003: bypassed via get_or_compute |
| ALL files under 300 lines | PASS | Largest is store/mod.rs at 260 lines |
| #[non_exhaustive] on public types | FAIL | DEFECT-004, DEFECT-005 |
| #[cfg(unix)] on path_hash | PASS | Applied to both definition and re-export |
| Single validate_and_insert | FAIL | DEFECT-003: two paths, one unvalidated |
| eprintln on cache write failure | FAIL | DEFECT-007: silently swallowed |
| Owner/waiter same result on serialization failure | FAIL | DEFECT-002: mismatch |

**Score: 8/13 constraints pass. 5 violated.**

---

## PRIORITY REMEDIATION ORDER

1. **DEFECT-003** — Value size bypass (security: DoS via get_or_compute)  
2. **DEFECT-002** — Owner/waiter mismatch (correctness: data inconsistency)  
3. **DEFECT-001** — Remove `spin_loop()` (constraint violation)  
4. **DEFECT-004** — Add `#[non_exhaustive]` to `DocCache`  
5. **DEFECT-005** — Add `#[non_exhaustive]` to `DocTransformerError`  
6. **DEFECT-007** — Add `eprintln!` on cache write failure  
7. **DEFECT-008** — Add value limit test for get_or_compute  
8. **DEFECT-009** — Add serialization failure consistency test  
9. **DEFECT-006** — Fix misleading "concurrent keys" comment
