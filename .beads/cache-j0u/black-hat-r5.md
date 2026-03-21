# Black-Hat Review R5 — Cache Module
## Target: centralized-docs/src/cache/
## Date: 2026-03-21
## Auditor: black-hat-reviewer (5-phase, zero prior context, every line read)

---

```
STATUS: REJECTED

## DEFECT-001: `errors.rs` exceeds 300-line file limit
**Severity:** MAJOR
**Phase:** MAINTAINABILITY
**File:** errors.rs (321 lines)
**Evidence:** `wc -l` reports 321 lines. Constraint states all files must be under 300 lines.
**Impact:** Violates stated architectural constraint. Signals module bloat and reduces navigability.
**Fix:** Extract non-cache error variants (ConfigError, ValidationError, DocumentError, IndexError, IoError, EmbeddingError) into separate files under `src/errors/` directory, leaving `errors.rs` as a re-export facade.

---

## DEFECT-002: `wait_once_lock` is an unbounded spin loop — no timeout
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:40-51
**Evidence:**
```rust
pub(super) fn wait_once_lock(slot: &ComputeSlot) -> Result<Vec<u8>> {
    loop {
        if let Some(result) = slot.get() {
            return match result.as_ref() {
                Ok(bytes) => Ok(bytes.clone()),
                Err(e) => Err(anyhow::anyhow!("{e:#}")),
            };
        }
        std::hint::spin_loop();
        std::thread::yield_now();
    }
}
```
**Impact:** If the owner thread panics (not caught by catch_unwind, which is NOT present), or is killed, or deadlocks on redb write, all waiters spin forever. There is no timeout, no cancellation token, no panic propagation. This is a liveness deadlock hazard. The docstring says "catch_unwind around compute() to prevent waiter deadlock" but no `catch_unwind` exists anywhere in the codebase.
**Fix:** Wrap `compute()` in `std::panic::catch_unwind(AssumeUnwindSafe(compute))` inside `get_or_compute` in `store/mod.rs:202`. On panic, publish the panic payload as `Err` into the OnceLock so waiters unblock. Additionally, add a configurable timeout (e.g., 30s) to `wait_once_lock` that returns `CacheError::ComputeTimeout` if exceeded.

---

## DEFECT-003: `catch_unwind` documented but never implemented
**Severity:** CRITICAL
**Phase:** CORRECTNESS
**File:** cache/store/mod.rs:177-215 (get_or_compute method)
**Evidence:** The docstring at line 35 says "No Mutex is used anywhere in this type." The requirements state "catch_unwind around compute() to prevent waiter deadlock." Searching all files: zero instances of `catch_unwind`. The `compute()` call at line 202 is bare:
```rust
let compute_result = compute();
```
**Impact:** If `compute()` panics, the OnceLock is never set, and all waiters in `wait_once_lock` spin forever. This is a production livelock.
**Fix:**
```rust
let compute_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(compute));
let compute_result = compute_result.unwrap_or_else(|_| {
    Err(anyhow::anyhow!("compute closure panicked"))
});
```
Then proceed to `finalize_compute` which will publish the error to waiters.

---

## DEFECT-004: `in_flight.remove` races with late waiters — lost wake-up
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:133
**Evidence:**
```rust
// Step 7: clean up — future callers hit redb cache
in_flight.remove(&in_flight_key);
```
This runs after `slot.set()` but there is a TOCTOU gap: a new waiter could check the redb cache (miss because the write might not be committed yet due to redb transaction timing), then check the DashMap (already removed), and become a second owner, running compute again.
**Impact:** Under high contention with slow redb commits, a second duplicate compute can be triggered. The "exact once" guarantee is probabilistic, not guaranteed.
**Fix:** Either (a) do not remove the in-flight entry (let it be cleared on next `clear_all` or use an LRU eviction), or (b) ensure the redb write is fully committed and visible BEFORE removing the DashMap entry by adding a memory fence / ensuring the write transaction commit happens-before the remove. The current ordering is correct for `OnceLock` visibility but does not guarantee redb read visibility for a thread that just missed the DashMap entry.

---

## DEFECT-005: Empty-key caching allows cache poisoning
**Severity:** MINOR
**Phase:** SECURITY
**File:** cache/tests/adversarial.rs:156-186 (test passes), all `put_*` methods
**Evidence:** The adversarial test `rq_attack_5_empty_inputs` validates that empty keys work, and the production code allows them. Empty string `b""` is a valid key for all three cache types.
**Impact:** An empty key collides between all three cache types conceptually (though separated by table). More importantly, `content_hash(b"")` is used as the `InFlightKey.key_hash` for all empty keys of the same `CacheType`, which is correct. The real risk is low since the empty key is unlikely to be accidental, but allowing it means a malicious or buggy caller can overwrite a shared empty-key slot.
**Fix:** Reject zero-length keys in `validate_key_size()` (add `key.is_empty()` check). This makes the invariant explicit: keys must be 1..=256 bytes.

---

## DEFECT-006: `get_or_compute` does not validate key size before computing
**Severity:** MAJOR
**Phase:** SECURITY
**File:** cache/store/mod.rs:185-215
**Evidence:** The `get_or_compute` method passes the raw key through to `get()` and `put_raw()`. The `get()` path does not validate key size. Only `put_raw()` at line 227 calls `validate_key_size(key)`. If an attacker provides a 257-byte key, the compute closure runs (potentially expensive), THEN `put_raw` rejects it. The value is lost — the compute ran for nothing and cannot be retried.
**Impact:** Wasted compute resources for oversized keys. The result is computed but never cached, violating the "compute once" invariant (it will recompute on every call with that oversized key).
**Fix:** Add `validate_key_size(key)?` at the top of `get_or_compute` before the cache check at line 190-193.

---

## DEFECT-007: `finalize_compute` silently swallows cache write failures
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/dedup.rs:121-127
**Evidence:**
```rust
if let Ok(ref bytes) = slot_result {
    if put_raw(cache_type, key, bytes).is_err() {
        // Cache write failed but compute succeeded — return value anyway.
        // The slot still gets set with Ok(bytes) so waiters get the value.
    }
}
```
**Impact:** If the redb write fails (disk full, corruption, value too large), the error is silently discarded. The caller gets `Ok(value)` but has no way to know the cache was not persisted. Future calls will re-compute every time since the value is never in redb, defeating the purpose of caching. The owner succeeds, waiters succeed, but the cache is never populated — a silent degradation.
**Fix:** Log the cache write failure at `warn!` level. Consider returning a `Result<V, CacheError>` that includes a warning, or using a dedicated `CacheResult<V>` type that indicates whether the value was persisted.

---

## DEFECT-008: `put_raw` double-validates value but `put_*` methods validate via serialization
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/mod.rs:226-241, cache/hash.rs:61-69
**Evidence:** `put_document`, `put_scrape`, `put_transform` all call `put_cached_value_with_limit` which serializes then validates. `put_raw` validates separately at line 236. Two different validation paths for the same invariant. If the value format changes, both paths must be updated.
**Impact:** Maintenance risk — if validation logic diverges, one path could accept what the other rejects.
**Fix:** Extract a single `validate_and_serialize<V: Serialize>(value: &V) -> Result<Vec<u8>>` function that both paths use.

---

## DEFECT-009: `CacheConfig` fields are public with no validation
**Severity:** MINOR
**Phase:** SECURITY
**File:** cache/config.rs:37-42
**Evidence:**
```rust
pub struct CacheConfig {
    pub backend: CacheBackend,
    pub cache_document_content: bool,
    pub cache_scrape_results: bool,
    pub cache_transforms: bool,
}
```
All fields are public `bool`. Any caller can set all three to `false` (test `rq_attack_15` does exactly this). While `#[non_exhaustive]` prevents structural matching from outside the crate, the fields are directly writable.
**Impact:** A misconfigured `CacheConfig` makes the entire cache a no-op silently. This is documented behavior per the test, but it's a foot-gun for API consumers.
**Fix:** Consider a builder pattern or `with_*` methods that make the configuration intentional. Low priority since the current API is simple.

---

## DEFECT-010: `DocCache` does not implement `Clone` — shared usage requires `Arc`
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/mod.rs:38-43
**Evidence:** `DocCache` holds `Arc<DashMap<...>>` for the in_flight map but not `Arc<Database>`. The `db` field is `Database` (owned). Every test that shares the cache wraps it in `Arc`.
**Impact:** API friction. Users must manually wrap in `Arc` for multi-threaded usage, which is the primary use case (deduplication only makes sense with concurrency).
**Fix:** Implement `Clone` for `DocCache` (wrap `db` in `Arc<Database>` — redb's `Database` is `Clone` internally via reference counting, but wrapping in `Arc` makes it explicit). Alternatively, document that `Arc<DocCache>` is the intended usage pattern.

---

## DEFECT-011: Hash collision between `CacheType` + different keys
**Severity:** MINOR
**Phase:** SECURITY
**File:** cache/store/dedup.rs:27-31
**Evidence:**
```rust
pub(super) struct InFlightKey {
    pub cache_type: CacheType,
    pub key_hash: [u8; 32],
}
```
The `key_hash` is `content_hash(key)` — a SHA-256 of the raw key bytes. Two different keys that produce the same SHA-256 hash would collide in the `DashMap`. While SHA-256 collision resistance makes this astronomically unlikely, the `InFlightKey` does not store the original key, so verification is impossible.
**Impact:** Theoretical only. SHA-256 preimage resistance is unbroken. A collision here would be a cryptographic breakthrough. However, the dedup guarantee degrades from "same key" to "same key hash."
**Fix:** Accept as known limitation (document it). Full fix would require storing the original key, which increases memory per in-flight entry. Not worth it for SHA-256.

---

## DEFECT-012: `clear_all` is not atomic — TOCTOU between in_flight clear and table clear
**Severity:** MAJOR
**Phase:** CORRECTNESS
**File:** cache/store/mod.rs:146-161
**Evidence:**
```rust
pub fn clear_all(&self) -> Result<()> {
    self.in_flight.clear();      // ← step 1: clear in-flight map
    let write_tx = self.db.begin_write()?;  // ← step 2: begin write
    {
        write_tx.delete_table(DOCUMENT_CACHE_TABLE)?;  // ← step 3: delete tables
        ...
    }
    write_tx.commit()?;          // ← step 4: commit
    Ok(())
}
```
Between step 1 and step 4, a new `get_or_compute` call can: (a) miss the in-flight map (cleared), (b) miss the redb cache (about to be cleared or tables recreated empty), (c) become an owner, (d) compute, (e) write to the table that is then deleted by the still-pending write transaction (serialised by redb), or (f) write to the newly recreated empty table.
**Impact:** A compute result written between steps 3 and 4 could be lost. If the write transaction is still in progress when a new compute writes, redb serialises them — but the outcome depends on ordering. In the worst case, the clear succeeds after the compute write, losing the computed value.
**Fix:** Accept as inherent to MVCC architecture. Document that `clear_all` is best-effort during concurrent operations. The test `rq_attack_8` and `rq_attack_9` validate this doesn't panic, which is the correct safety property.

---

## DEFECT-013: `#[non_exhaustive]` missing on `InflightDecision` enum
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/store/dedup.rs:139
**Evidence:**
```rust
pub(super) enum InflightDecision<V> {
```
No `#[non_exhaustive]` attribute. While this is `pub(super)` (not public API), the stated constraint is "`#[non_exhaustive]` on all public enums/structs."
**Impact:** None functionally (it's internal). Violates the stated convention.
**Fix:** Add `#[non_exhaustive]` for consistency.

---

## DEFECT-014: Test file `adversarial_stress.rs` at exactly 242 lines — near limit
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** cache/tests/adversarial_stress.rs (242 lines)
**Evidence:** At 242 lines, adding more tests will breach the 300-line limit.
**Impact:** Adding new adversarial tests requires splitting the file.
**Fix:** Proactive split — move ATTACK 12-15 into `adversarial_edge_cases.rs` or similar.

---

## DEFECT-015: `rq_attack_2_sha256_no_collision_proximity` is O(n²) and slow
**Severity:** MINOR
**Phase:** PERFORMANCE
**File:** cache/tests/adversarial.rs:52-70
**Evidence:**
```rust
let inputs: Vec<Vec<u8>> = (0..10_000u16)
    .map(|i| format!("key_{i}").into_bytes())
    .collect();
...
for i in 0..hashes.len() {
    for j in (i + 1)..hashes.len() {
        assert_ne!(hashes[i], hashes[j], ...);
    }
}
```
10,000 × 9,999 / 2 = ~50M comparisons. Each compares 32 bytes. This test is slow.
**Impact:** CI slowdown. Not a correctness issue.
**Fix:** Use a `HashSet<[u8; 32]>` for O(n) collision detection:
```rust
let mut seen = HashSet::new();
for hash in hashes {
    assert!(seen.insert(hash), "COLLISION FOUND");
}
```

---

## DEFECT-016: `CacheError` missing `#[non_exhaustive]` on `DocTransformerError` and other error enums
**Severity:** MINOR
**Phase:** MAINTAINABILITY
**File:** errors.rs:23-74 (DocTransformerError), errors.rs:99-116 (ConfigError), errors.rs:119-161 (ValidationError), errors.rs:164-190 (DocumentError), errors.rs:193-219 (IndexError), errors.rs:222-236 (IoError), errors.rs:247-273 (EmbeddingError)
**Evidence:** `CacheError` at line 276 has `#[non_exhaustive]`. All other error enums in the same file do NOT. The stated constraint says "`#[non_exhaustive]` on all public enums/structs."
**Impact:** Adding variants to any of these enums is a breaking change for downstream consumers.
**Fix:** Add `#[non_exhaustive]` to all public error enums, or document why some are intentionally exhaustible.

---

## Summary

| Severity | Count | IDs |
|----------|-------|-----|
| CRITICAL | 2     | 002, 003 |
| MAJOR    | 4     | 001, 004, 006, 007 |
| MINOR    | 6     | 005, 008, 009, 010, 013, 014, 015, 016 |
| INFO     | 2     | 011, 012 |

**Verdict: REJECTED** — Two CRITICAL defects (unbounded spin loop with no timeout, missing catch_unwind) must be fixed before merge. The unbounded spin can hang production threads indefinitely if compute panics.
