# Test Suite Inquisition Report — bulk_load Module

**Date:** 2026-04-02  
**Scope:** `src/state/bulk_load.rs` (production) + `src/state/bulk_load.rs::tests` (30 unit) + `tests/bulk_load/` (34 integration)  
**Total tests under review:** 64  
**Inquisitor mode:** Mode 2 — Suite Inquisition  

---

## VERDICT: REJECTED

2 LETHAL findings at Tier 0. Execution stops. Suite must be rewritten before resubmission.

---

## Tier 0 — Static Analysis

### [FAIL] Banned Pattern Scan

**LETHAL — Hollow `assert!(result.is_ok())` with no concrete value check:**

| File:Line | Pattern | Context |
|---|---|---|
| `src/state/bulk_load.rs:1409` | `assert!(result.is_ok(), ...)` | `session_new_returns_storage_error_on_read_failure` — SOLE assertion in test. No concrete value verified. Test name promises error-path coverage but only checks success. **Hollow test.** |

**12 additional `assert!(result.is_err())` / `assert!(result.is_ok())` instances (MAJOR):**

All 11 `assert!(result.is_err())` instances in `src/state/bulk_load.rs` (lines 672, 699, 726, 765, 804, 1035, 1062, 1089, 1116, 1151, 1186) are **redundant precondition guards** followed by concrete `matches!` assertions on exact error variants. They are not hollow — the real assertion is the `matches!` macro that checks exact variants and field values. However, the banned pattern grep still matches. Remove the redundant guards; the `unwrap_err()` already panics on `Ok`.

| File:Line | Pattern | Verdict |
|---|---|---|
| `src/state/bulk_load.rs:672` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:699` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:726` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:765` | `assert!(result.is_err(), ...)` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:804` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1035` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1062` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1089` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1116` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1151` | `assert!(result.is_err(), ...)` | Redundant guard, concrete variant match follows |
| `src/state/bulk_load.rs:1186` | `assert!(result.is_err())` | Redundant guard, concrete variant match follows |
| `tests/bulk_load/load_analyses_tests.rs:240` | `assert!(result.is_ok(), ...)` | Redundant guard, `assert_eq!(result.unwrap().len(), 0)` follows |

### [PASS] Holzmann Rule Scan — No Shared Mutable State

- `static mut`: No matches
- `lazy_static!`: No matches
- `once_cell`: No matches

**Loops in test bodies (bounded, MAJOR — not LETHAL per Holzmann Rule 2 spirit):**

| File:Line | Loop | Bound |
|---|---|---|
| `tests/bulk_load/boundary_tests.rs:26` | `for i in 0..256u16` | 256 (setup loop) |
| `tests/bulk_load/boundary_tests.rs:60` | `for (i, hash) in hashes.iter().enumerate()` | 500 (setup loop) |
| `src/state/bulk_load.rs:1454` | `for n in [0, 1, 5, 20]` | 4 (parameterized test) |
| `src/state/bulk_load.rs:1484` | `for n in [0, 1, 5, 20]` | 4 (parameterized test) |

All loops are bounded with fixed iteration counts. They are setup/data-insertion loops, not assertion loops. Holzmann Rule 2 requires "an upper bound on iterations" — satisfied. Cited as MAJOR for violating the strict letter of the grep rule.

### [PASS] Mock Interrogation

No mocks found. No `mockall`, no `Mock::new()`, no `.expect_`.

### [PASS] Integration Test Purity

No `use crate::` found in `tests/`. All integration tests use the public API via `doc_transformer::` prefix.

### [FAIL] Error Variant Completeness

**BulkLoadError (3 variants):**

| Variant | Tested? | Evidence |
|---|---|---|
| `TableOpen` | YES | `load_analyses_tests.rs:142` — exact match on `{ table, message }` |
| `StorageError` | **NO** | **LETHAL: No test forces `BulkLoadError::StorageError` from `load_entries` or `StateReadSession::new`.** The `session_new_returns_storage_error_on_read_failure` test at line 1403 only checks `is_ok()` (hollow). No test exercises the `StorageError` path from `load_entries` line 414. |
| `CorruptPayload` | YES | Tested in all 4 loader test files + `owned_archive_tests.rs` with exact field matching |

**StateLoadError (3 variants):**

| Variant | Tested? | Evidence |
|---|---|---|
| `MalformedRow` | YES | 13 unit tests across B6-B9, B18-B22 with exact field matching |
| `Utf8KeyError` | **NO** | Documented as "structurally impossible with &str keys." MINOR — defensive variant, but no test exists. |
| `BackendError` | YES | B11, B24 with exact `operation` field matching |

**LETHAL: `BulkLoadError::StorageError` has no test asserting the exact variant.**

### [PASS] Density Audit

```
Public functions:  11  (try_from_bytes, as_bytes, archived, deserialize, new,
                        load_analyses, load_transforms, load_chunks, load_scrapes,
                        load_file_states, load_url_states)
Unit tests:        30  (src/state/bulk_load.rs #[cfg(test)])
Integration tests: 34  (tests/bulk_load/*.rs)
Total tests:       64
Ratio:             64 / 11 = 5.8x (target: ≥5x) ✓
```

### [PASS] Silent Error Discard (production-scoped)

- `src/state/bulk_load.rs:54` — `let _ = write!(acc, "{b:02x}")` — `fmt::Write` to `String` is infallible. False positive. MINOR.
- `tests/bulk_load/common.rs:83` — `let _ = expr.unwrap()` — discarding Ok value after unwrap. Error handled by panic. False positive. MINOR.

---

## Tier 1 — Compilation + Execution

### [PASS] Tests Pass: 64/64

```
Unit tests:       30 passed, 0 failed, 0 ignored (0.45s)
Integration:      34 passed, 0 failed, 0 ignored (0.22s)
```

### [PASS] Ordering Probe: Consistent

```
Unit (1 thread):     30 passed (4.15s)
Unit (8 threads):    30 passed (1.23s)
Integration (1 thr): 34 passed (3.52s)
Integration (8 thr): 34 passed (1.01s)
```

No divergence. No hidden shared state.

### [SKIPPED] Clippy

Workspace-level `deny(unwrap_used)` triggers 320+ errors in test code across other modules. The `bulk_load` unit tests inside `src/state/bulk_load.rs` also trigger this (`.unwrap()` in test helpers without `#![allow(clippy::unwrap_used)]` inside the `#[cfg(test)]` block). The integration tests in `tests/bulk_load/mod.rs` have the correct `#![allow(...)]`. **Recommendation:** Add `#![allow(clippy::unwrap_used)]` to the `#[cfg(test)] mod tests` block in `bulk_load.rs`.

### [SKIPPED] Insta

No `insta` dependency in Cargo.toml.

---

## Tier 2 — Coverage

### [SKIPPED] Fresh Run Blocked

`cargo llvm-cov nextest` fails with double-spawn errors (target-dir mismatch in workspace). `cargo llvm-cov` (without nextest) fails to collect object files. Infrastructure issue prevents fresh coverage data.

**Stale report (from previous full run):**

| File | Lines | Missed | Coverage |
|---|---|---|---|
| `state/bulk_load.rs` | 749 | 149 | **80.11%** |
| TOTAL | 17432 | 3081 | 82.33% |

**NOTE:** The 80.11% includes ~1000 lines of `#[cfg(test)]` code which should be ~100% covered. The production code (~350 lines) likely has significantly lower coverage. The 149 missed lines include the untested `StorageError` path and the `hex_encode` function (only tested indirectly).

**Estimated production-code coverage: ~65%.** Below the 90% threshold. But cannot be verified with fresh data.

---

## Tier 3 — Mutation

### [SKIPPED] Execution Blocked

`cargo mutants` fails to build due to pre-existing compilation errors in `tests/index_adversarial.rs` (references deleted function `build_and_write_compass`). Unrelated to bulk_load but blocks the entire test runner.

### Manual Mutant Analysis (63 mutants listed)

| Category | Count | Kill Rate | Analysis |
|---|---|---|---|
| `hex_encode` replacements | 2 | 100% | Caught by CorruptPayload `key_hex` field comparison |
| `try_from_bytes` → `Ok(Default)` | 1 | 100% | Caught by `owned_archive_returns_corrupt_payload` (garbage accepted) |
| `as_bytes` → empty/single byte | 3 | 100% | Caught by `owned_archive_preserves_exact_input_bytes` |
| `archived` → `Ok(Default)` | 1 | 100% | Caught by `owned_archive_archived_returns_matching_field_values` |
| `deserialize` → `Ok(Default)` | 1 | 100% | Caught by `owned_archive_deserialize_returns_owned_value` |
| `load_analyses` → `Ok(HashMap::new/from_iter)` | 9 | 100% | Caught by key identity + field value assertions |
| `load_transforms` → same | 9 | 100% | Caught by `success_count` field assertion |
| `load_chunks` → same | 9 | 100% | Caught by `chunks_metadata.len()` assertion |
| `load_scrapes` → same | 9 | 100% | Caught by `pages.len()` assertion |
| `load_file_states` → same | 3 | 100% | Caught by exact key + struct equality assertions |
| `load_url_states` → same | 3 | 100% | Caught by same |
| `load_entries` → same | 9 | 100% | Caught by all 4 loader tests (delegate to load_entries) |
| `scan_pod_table` → same | 3 | 100% | Caught by `load_file_states`/`load_url_states` tests |
| `scan_pod_table: != → ==` | 1 | 100% | Caught: valid rows rejected, all happy-path tests fail |

**Estimated kill rate: 63/63 = 100%** (all generated mutants killed).

**Untested mutations (not generated by cargo-mutants):**
- Removing `.unique()` from `load_entries` — correctness unaffected (HashMap deduplicates by key), performance regression only. No test verifies single table lookup per unique hash.
- `StateReadSession::new` `StorageError` path — no test forces `db.begin_read()` to fail.

---

## LETHAL FINDINGS (2)

1. **`src/state/bulk_load.rs:1409`** — `assert!(result.is_ok(), "fresh db should create session successfully")` is the SOLE assertion in `session_new_returns_storage_error_on_read_failure`. No concrete value verified. Test is hollow — it would pass even if `StateReadSession::new` returned a session that does nothing.

2. **`BulkLoadError::StorageError` variant** — No test anywhere in the suite asserts the exact `StorageError` variant. The variant is produced at `bulk_load.rs:251` (from `StateReadSession::new`) and `bulk_load.rs:414` (from `load_entries` table.get). Neither path is exercised by a test that matches on `BulkLoadError::StorageError { table, message }`.

---

## MAJOR FINDINGS (14)

1. `src/state/bulk_load.rs:672` — `assert!(result.is_err())` redundant guard before concrete `matches!`
2. `src/state/bulk_load.rs:699` — same
3. `src/state/bulk_load.rs:726` — same
4. `src/state/bulk_load.rs:765` — same
5. `src/state/bulk_load.rs:804` — same
6. `src/state/bulk_load.rs:1035` — same
7. `src/state/bulk_load.rs:1062` — same
8. `src/state/bulk_load.rs:1089` — same
9. `src/state/bulk_load.rs:1116` — same
10. `src/state/bulk_load.rs:1151` — same
11. `src/state/bulk_load.rs:1186` — same
12. `tests/bulk_load/load_analyses_tests.rs:240` — `assert!(result.is_ok())` redundant guard before concrete `assert_eq`
13. `tests/bulk_load/boundary_tests.rs:26,60` — loops in test bodies (bounded setup, Holzmann compliant)
14. `src/state/bulk_load.rs:1454,1484` — loops in test bodies (bounded parameterized, Holzmann compliant)

---

## MINOR FINDINGS (4)

1. `src/state/bulk_load.rs:54` — `let _ = write!(acc, "{b:02x}")` in `hex_encode` — infallible `fmt::Write` to `String`. False positive for "silent error discard."
2. `tests/bulk_load/common.rs:83` — `let _ = expr.unwrap()` — discarding Ok value after unwrap handles errors via panic. False positive.
3. `StateLoadError::Utf8KeyError` — no test. Documented as structurally impossible with `&str` keys. Defensive variant only.
4. `.unique()` deduplication in `load_entries` — no test verifies single lookup per unique hash (performance contract, not correctness).

---

## MANDATE

Before resubmission, ALL of the following must exist:

### Must Fix (LETHAL)

1. **Replace or remove `session_new_returns_storage_error_on_read_failure`** (`bulk_load.rs:1403-1413`). This test currently only asserts `is_ok()`. Either:
   - **(A)** Delete it — the success path is already covered by `session_new_holds_database_reference_when_constructed` (line 597).
   - **(B)** Rewrite it to actually force a `StorageError` from `db.begin_read()` and assert the exact variant:
     ```rust
     // Must match: BulkLoadError::StorageError { table: "<begin_read>", message: _ }
     ```

2. **Add a test asserting `BulkLoadError::StorageError` exact variant.** Required test name: `load_entries_returns_storage_error_when_redb_get_fails`. This requires finding a way to inject a `StorageError` from `table.get()` or proving it structurally impossible with a comment + naming convention (like Utf8KeyError).

   **RECOMMENDED TEST:**
   ```
   GIVEN a database with analysis_outputs table
    AND a hash key h1
    AND the table is corrupted/inaccessible
   WHEN session.load_analyses(&[h1])
   THEN Err(BulkLoadError::StorageError { table: "analysis_outputs", message: <non-empty> })
   ```

### Should Fix (MAJOR)

3. **Remove all 11 redundant `assert!(result.is_err())` guards** in `src/state/bulk_load.rs`. The `result.unwrap_err()` on the next line already panics on `Ok`. The `matches!` macro is the real assertion.

4. **Remove `assert!(result.is_ok(), ...)` guard** in `tests/bulk_load/load_analyses_tests.rs:239-242`. The `assert_eq!(result.unwrap().len(), 0)` on line 243 already asserts the concrete value.

5. **Add `#![allow(clippy::unwrap_used)]`** at the top of the `#[cfg(test)] mod tests` block in `src/state/bulk_load.rs` to fix workspace-level clippy lint violations.

### Resubmission Protocol

After fixing: **re-run ALL tiers from Tier 0.** Full re-run. Always. Fixing one thing can break another.

---

## Summary Statistics

| Metric | Value |
|---|---|
| Production LOC | ~350 lines |
| Test LOC (unit) | ~1007 lines |
| Test LOC (integration) | ~825 lines |
| Public functions | 11 |
| Total tests | 64 (30 unit + 34 integration) |
| Test/function ratio | 5.8x |
| Error variants | 6 (BulkLoadError 3, StateLoadError 3) |
| Tested variants | 4/6 (StorageError + Utf8KeyError untested) |
| Estimated mutant kill rate | 100% (63/63, manual analysis) |
| Stale line coverage | 80.11% (includes test code) |
| Ordering consistency | PASS (1-thread ≡ 8-thread) |
| STATUS | **REJECTED** |
