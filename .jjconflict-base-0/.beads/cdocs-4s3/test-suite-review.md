# Test Suite Inquisition Report — `state::bulk_load`

**Scope**: `centralized-docs/src/state/bulk_load.rs` + `centralized-docs/src/state/mod.rs` + `centralized-docs/tests/bulk_load/`
**Date**: 2026-04-02
**Inquisitor**: test-reviewer (Mode 2 — Suite Inquisition)
**STATUS: REJECTED**

---

## VERDICT: REJECTED

9 LETHAL findings. 3 MAJOR findings. 4 MINOR findings. The suite is not ship-worthy.

---

## Tier 0 — Static

### [FAIL] Banned pattern scan — 9 LETHAL hits

`assert!(result.is_err())` used as guard assertions before the real `matches!` check.
While the `matches!` assertions below ARE concrete, the `is_err()` guard is a banned
pattern per protocol. Every one of these is redundant (the `unwrap_err()` below would
panic on `Ok`), but banned means banned.

| File | Line | Pattern |
|------|------|---------|
| `src/state/bulk_load.rs` | 672 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 699 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 726 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 804 | `assert!(result.is_err());` before BackendError match |
| `src/state/bulk_load.rs` | 1035 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 1062 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 1089 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 1116 | `assert!(result.is_err());` before MalformedRow match |
| `src/state/bulk_load.rs` | 1186 | `assert!(result.is_err());` before BackendError match |

**Fix**: Remove all `assert!(result.is_err())` lines. The `result.unwrap_err()` on the
next line already panics on `Ok`. The `matches!` assertion is the real proof.

### [FAIL] Holzmann Rule 2 — Loops in test bodies — 4 LETHAL hits

| File | Line | Context |
|------|------|---------|
| `tests/bulk_load/boundary_tests.rs` | 26 | `for i in 0..256u16 {` — inserts 256 entries in test body |
| `tests/bulk_load/boundary_tests.rs` | 60 | `for (i, hash) in hashes.iter().enumerate() {` — inserts 500 entries in test body |
| `src/state/bulk_load.rs` | 1454 | `for n in [0, 1, 5, 20] {` — parameterized test using loop |
| `src/state/bulk_load.rs` | 1484 | `for n in [0, 1, 5, 20] {` — parameterized test using loop |

**Fix**: Replace with `rstest` parameterized tests or individual named tests per case.

### [PASS] Silent error discard — production code only

- `src/state/bulk_load.rs:54` — `let _ = write!(acc, "{b:02x}");` in production `hex_encode()`.
  The `write!` to a `String` never fails. This is NOT in test code. **Not a test defect.**
- `tests/bulk_load/common.rs:83` — `let _ = write_tx.open_table(*def).unwrap();` in test
  helper. The `.unwrap()` still panics on error; `let _ =` discards the table handle after
  creation. Side-effectful setup, not assertion context. **MINOR.**

### [PASS] Ignored tests — none found
### [PASS] Sleep in tests — none found
### [PASS] Naming violations — none found
### [PASS] Shared mutable state — none found
### [PASS] Mock interrogation — no mocks found
### [PASS] Integration test purity — `use doc_transformer::` (public API) only

### [FAIL] Error variant completeness — 2 LETHAL hits

**BulkLoadError::StorageError** (src/state/bulk_load.rs:81):
- Returned by `StateReadSession::new()` when `db.begin_read()` fails.
- The ONLY test for this path is `session_new_returns_storage_error_on_read_failure`
  (line 1403), which tests the HAPPY PATH (`assert!(result.is_ok())`) and never
  exercises the error.
- **No test asserts `BulkLoadError::StorageError` with exact variant match.**
- This is a zombie variant: defined, returned, never verified.

**StateLoadError::Utf8KeyError** (src/state/mod.rs:449):
- Comment at line 783 says "impossible with current schema" and skips the test.
- A variant that exists in the public API but has zero test coverage is a maintenance
  trap. If the schema changes to `&[u8]` keys, this path activates with zero proof it works.
- **No test asserts `StateLoadError::Utf8KeyError` with exact variant match.**

### [FAIL] Density audit — LETHAL

```
Public functions:  32  (11 in bulk_load.rs + 21 in mod.rs)
Test functions:   112  (30 unit + 34 integration + 48 mod.rs unit)
Ratio:            3.5x (target ≥ 5x)
```

The density ratio of 3.5× is below the 5× target. This is partly due to the large number
of `const fn` table accessors (8 trivial functions that each have 1 test). But the rule is
the rule: 5× is the floor.

**Fix**: Add tests for uncovered error paths (StorageError, Utf8KeyError), add property
tests for `hex_encode` and `OwnedArchive` round-trip, add boundary tests for each `load_*`
method with empty + single + many hashes.

### Insta: ABSENT (no insta gate needed)

---

## Tier 1 — Execution

### [FAIL] Clippy — crate-wide compilation errors block full lint

```
error[E0560]: struct `LlmsConfig` has no field named `generate_full` (validation_atomicity_tests.rs:112)
error[E0425]:  cannot find function `build_and_write_compass` in module `index` (index_adversarial.rs:83)
error:         `.as_ref().map(String::as_str)` clippy lint (persisted_tests.rs)
error:         redundant field names, long literal lacking separators (multiple)
```

These are NOT in bulk_load code but prevent `cargo clippy --tests --all-features` from
passing. The full crate clippy gate is BLOCKED.

**However**, bulk_load-specific compilation succeeds. Bulk_load unit + integration tests
all pass.

### [PASS] nextest (cargo test): 64 passed, 0 failed, 0 flaky

```
Unit tests (--lib -- state::bulk_load):      30 passed, 0 failed
Integration tests (--test lib -- bulk_load):  34 passed, 0 failed
```

### [PASS] Ordering probe: CONSISTENT

```
Single-thread (--test-threads=1): 30 passed (3.49s) / 34 passed (2.87s)
Multi-thread  (--test-threads=8): 30 passed (0.65s) / 34 passed (0.47s)
No divergence. No hidden shared state.
```

### Insta: N/A (absent)

---

## Tier 2 — Coverage

### [FAIL] Line coverage: 80.11% unit-only for bulk_load.rs (target ≥ 90%)

```
state/bulk_load.rs:  80.11% line (749 lines, 149 missed) — unit tests only
state/mod.rs:        11.99% line (901 lines, 793 missed) — unit tests only (most untested by bulk_load filter)
```

The 80.11% figure is from unit tests only. The 20% gap includes the `load_analyses`,
`load_transforms`, `load_chunks`, `load_scrapes` methods which are exercised by integration
tests. However, the combined measurement is BLOCKED by broken tests elsewhere in the crate
preventing `cargo llvm-cov nextest` from building the full test suite.

**Estimated combined coverage**: ~92–95% based on analysis of integration test coverage of
the rkyv loader methods. But estimation is not evidence. The coverage gate cannot be
definitively passed.

**Function coverage**: 69.70% (46/66 functions hit by unit tests alone). The 20 missed
functions are the `load_*` methods (covered by integration tests) plus the `scan_pod_table`
error branches for `decode_fn` failure.

### Branch coverage: N/A (0 branch instrumentation reported)

---

## Tier 3 — Mutation

### [FAIL] Kill rate: INCONCLUSIVE — build failure blocks mutation testing

```
ERROR cargo build failed in an unmutated tree, so no mutants were tested
```

Root cause: stale tests in `index_adversarial.rs` and `validation_atomicity_tests.rs`
reference deleted/renamed API functions (`build_and_write_compass`, `generate_full` field).
These prevent the full test suite from compiling, which blocks `cargo mutants`.

Existing `mutants.out/` contains empty results: `[]` in `mutants.json`.

**Manual mutation analysis** (thought experiment per protocol):

The following mutations would survive because the tests don't exercise these paths:

1. **Delete `StateReadSession::new` StorageError branch** (line 251): No test triggers this
   error path. `session_new_returns_storage_error_on_read_failure` tests the SUCCESS path.
   - REQUIRED TEST: `session_new_returns_storage_error_when_db_corrupted`

2. **Change `hex_encode` to return empty string**: No test directly asserts `hex_encode`
   output (it's a private function tested indirectly through CorruptPayload error messages).
   The `key_hex` field in CorruptPayload tests IS checked, providing indirect coverage.
   - VERDICT: Likely caught by `owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck`
     which asserts `key_hex == hex_encode_32(&hash)`.

3. **Delete `scan_pod_table` decode_fn error branch** (line 488): The `FileStateRaw::from_bytes`
   and `UrlStateRaw::from_bytes` return `StateError` on size mismatch, but `scan_pod_table`
   maps this to `StateLoadError::MalformedRow`. If the mapping were deleted (returning Ok
   with a default value), no current test would catch it because the size-check branch
   (line 480) catches the error BEFORE decode_fn is called.
   - REQUIRED TEST: `scan_pod_table_returns_malformed_row_when_decode_fn_fails_on_correct_size_bytes`

4. **Return `Ok(HashMap::new())` from `load_entries` for non-empty input**: Would be caught
   by `load_analyses_returns_all_entries_when_all_hashes_exist` which asserts `map.len() == 3`.

5. **Swap dedup: remove `.unique()` call**: Would be caught by
   `load_analyses_deduplicates_when_input_has_duplicate_hashes` which asserts `map.len() == 1`
   for 3 duplicate inputs.

6. **Remove early-return for empty input** (line 396–398): Would be caught by
   `load_analyses_returns_empty_map_when_input_empty_and_table_missing` which tests empty
   input against a missing table.

**Estimated kill rate**: ~85–90% for well-tested paths. Below 90% threshold due to
untestable `StorageError` path and `scan_pod_table` decode_fn mapping.

---

## LETHAL FINDINGS (9)

1. **L0091**: `src/state/bulk_load.rs:672` — `assert!(result.is_err())` banned assertion
2. **L0092**: `src/state/bulk_load.rs:699` — `assert!(result.is_err())` banned assertion
3. **L0093**: `src/state/bulk_load.rs:726` — `assert!(result.is_err())` banned assertion
4. **L0094**: `src/state/bulk_load.rs:804` — `assert!(result.is_err())` banned assertion
5. **L0095**: `src/state/bulk_load.rs:1035` — `assert!(result.is_err())` banned assertion
6. **L0096**: `src/state/bulk_load.rs:1062` — `assert!(result.is_err())` banned assertion
7. **L0097**: `src/state/bulk_load.rs:1089` — `assert!(result.is_err())` banned assertion
8. **L0098**: `src/state/bulk_load.rs:1116` — `assert!(result.is_err())` banned assertion
9. **L0099**: `src/state/bulk_load.rs:1186` — `assert!(result.is_err())` banned assertion
10. **L0101**: `tests/bulk_load/boundary_tests.rs:26` — loop in test body (Rule 2)
11. **L0102**: `tests/bulk_load/boundary_tests.rs:60` — loop in test body (Rule 2)
12. **L0103**: `src/state/bulk_load.rs:1454` — loop in test body (Rule 2)
13. **L0104**: `src/state/bulk_load.rs:1484` — loop in test body (Rule 2)
14. **L0105**: `src/state/bulk_load.rs:81` — `BulkLoadError::StorageError` has no test asserting exact variant
15. **L0106**: `src/state/mod.rs:449` — `StateLoadError::Utf8KeyError` has no test asserting exact variant
16. **L0107**: Density ratio 3.5× (112 tests / 32 functions) — below 5× target

---

## MAJOR FINDINGS (3)

1. **M001**: Tier 2 — Line coverage 80.11% from unit tests only; combined measurement
   blocked by broken tests in `index_adversarial.rs` and `validation_atomicity_tests.rs`.
   Cannot confirm ≥90% coverage.

2. **M002**: Tier 3 — Mutation testing blocked by crate-wide compilation failures.
   `index_adversarial.rs:83` calls deleted function `build_and_write_compass`.
   `validation_atomicity_tests.rs:112` references deleted field `generate_full`.
   Cannot confirm ≥90% kill rate.

3. **M003**: `src/state/bulk_load.rs:1403` — Test `session_new_returns_storage_error_on_read_failure`
   is named as if it tests a failure, but it ONLY tests the success path
   (`assert!(result.is_ok())`). This is a misleading test name that provides false confidence.

---

## MINOR FINDINGS (4)

1. **m001**: `tests/bulk_load/common.rs:83` — `let _ =` in test helper discards table handle
   after intentional side-effect creation. Harmless but technically a Rule 6 hit.

2. **m002**: `src/state/bulk_load.rs:54` — `let _ = write!(acc, "{b:02x}");` in production
   code `hex_encode`. The `write!` to `String` is infallible; this is idiomatic. Not a test
   defect but noted for completeness.

3. **m003**: Integration test helper `tests/bulk_load/common.rs:81` uses `for (name, def) in
   &rkyv_defs` loop inside `open_db_without_table`. This is a setup helper, not a test body.
   Holzmann Rule 2 technically applies but this is a gray area.

4. **m004**: `src/state/bulk_load.rs:1403-1413` — Test body is 10 lines long for a trivial
   assertion. Holzmann Rule 4 recommends ≤20 lines. This is fine but the test itself is
   misleading (see M003).

---

## MANDATE

The following must exist before resubmission. Every item requires a full re-run from Tier 0.

### Required Fixes (blocking)

1. **Delete all 9 `assert!(result.is_err())` guard lines** in `src/state/bulk_load.rs`.
   The `matches!` assertions that follow are the real proof. These are redundant noise.

2. **Replace 4 loops in test bodies** with `rstest` parameterized tests or individual named tests:
   - `boundary_tests.rs:26` → Extract to helper, call from named test
   - `boundary_tests.rs:60` → Extract to helper, call from named test
   - `bulk_load.rs:1454` → Replace with 4 individual `#[test]` functions or `#[rstest(n => [0, 1, 5, 20])]`
   - `bulk_load.rs:1484` → Same

3. **Add test for `BulkLoadError::StorageError`**:
   ```
   REQUIRED TEST: session_new_returns_storage_error_when_db_read_fails
   ```

4. **Add test for `StateLoadError::Utf8KeyError`** (or document with a const assertion
   that the table schema enforces UTF-8 at the type level, making this unreachable):
   ```
   REQUIRED TEST: load_file_states_handles_utf8_key_error (or static proof comment)
   ```

5. **Add tests to reach ≥5× density**:
   - `hex_encode_returns_lowercase_hex_for_all_byte_values` (property test)
   - `owned_archive_deserialize_roundtrip_matches_original` (proptest)
   - `load_analyses_returns_empty_map_when_all_hashes_missing` (boundary)
   - `load_transforms_preserves_key_identity` (key identity check)
   - `load_chunks_deduplicates_when_input_has_duplicate_hashes`
   - `load_scrapes_deduplicates_when_input_has_duplicate_hashes`
   - `scan_pod_table_returns_malformed_row_when_decode_fn_fails_on_correct_size_bytes`

6. **Fix broken tests blocking Tier 2/3**:
   - `tests/index_adversarial.rs:83` — rename `build_and_write_compass` → `build_and_write_index`
   - `tests/validation_atomicity_tests.rs:112,202` — remove `generate_full` field

### Required Tests for Mutation Survivors

| Survivor Behavior | Required Test Name |
|---|---|
| `StateReadSession::new` StorageError path never fires | `session_new_returns_storage_error_when_db_read_fails` |
| `scan_pod_table` decode_fn error mapping | `scan_pod_table_returns_malformed_row_when_decode_fn_fails_on_correct_size_bytes` |

---

## Summary Table

| Tier | Gate | Result | Detail |
|------|------|--------|--------|
| 0 | Banned patterns | **FAIL** | 9 hits: `assert!(result.is_err())` |
| 0 | Holzmann Rule 2 | **FAIL** | 4 loops in test bodies |
| 0 | Mock interrogation | PASS | No mocks |
| 0 | Integration purity | PASS | Public API only |
| 0 | Error variants | **FAIL** | StorageError, Utf8KeyError untested |
| 0 | Density | **FAIL** | 3.5× (target ≥5×) |
| 1 | Clippy | **FAIL** | Crate-wide compilation errors (not bulk_load) |
| 1 | Tests pass | PASS | 64 passed, 0 failed |
| 1 | Ordering | PASS | Consistent across thread counts |
| 2 | Line coverage | **FAIL** | 80.11% unit-only (blocked from combined measurement) |
| 2 | Branch coverage | N/A | No branch instrumentation |
| 3 | Mutation | **FAIL** | Blocked by crate compilation errors |

**0 LETHAL + < 3 MAJOR + < 5 MINOR = APPROVED** — NOT MET
**16 LETHAL + 3 MAJOR + 4 MINOR = REJECTED**

---

*The test suite has good structural coverage of happy paths and most error variants. The
integration tests in `tests/bulk_load/` are well-organized with concrete field-level
assertions. The defects are real but concentrated: banned guard assertions, loops in
boundary tests, and two untested error paths. Fix the mandate items and resubmit.*
