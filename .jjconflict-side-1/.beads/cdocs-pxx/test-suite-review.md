# Test Suite Inquisition Report

**Module:** `state::commit` (`centralized-docs/src/state/commit.rs`)
**Date:** 2026-04-02
**Scope:** 49 unit/integration tests in `commit::tests`, plus coverage + mutation analysis
**Reviewer:** Test Inquisitor (Mode 2 — Suite Inquisition)

---

## VERDICT: STATUS: REJECTED

**11 LETHAL | 4 MAJOR | 3 MINOR**

The commit module test suite was stopped at Tier 0 with 7 LETHAL findings.
Tier 1 added 2 more LETHAL. Tier 2 added 1 LETHAL. Tier 3 confirmed 1 LETHAL.
≥3 LETHAL at Tier 0 alone mandates immediate rejection per protocol.
Full four-tier results provided below per request.

---

## Tier 0 — Static Analysis

### [PASS] Banned pattern: `assert!(result.is_ok())` / `assert!(result.is_err())`

No hits for the literal `result.is_ok()` / `result.is_err()` pattern with bare
assert. However, **see LETHAL #2** for the semantic equivalent.

### [FAIL] Banned pattern: `let _ =` in test code

**LETHAL #1** — `centralized-docs/src/state/commit.rs:2211`

```rust
let _ = state_db.commit_changes(invalid);
```

Inside `proptest_atomicity_mixed_batches` test. The error from `commit_changes`
is silently discarded. The test's intent is to verify data survival after a
failed commit — the real assertion is at line 2216 — but per Holzmann Rule 6,
any `let _ =` in test code is a silent discard. The error type and variant go
unchecked.

**Fix:** Replace with:
```rust
let err = state_db.commit_changes(invalid);
assert!(
    matches!(err, Err(CommitError::ZeroHashKey { .. })),
    "invalid commit should fail with ZeroHashKey: {err:?}"
);
```

### [FAIL] Banned pattern: `is_err()` without specific variant

**LETHAL #2** — `centralized-docs/src/state/commit.rs:1930`

```rust
assert!(result.is_err(), "should fail with ZeroHashKey");
```

The message SAYS "ZeroHashKey" but the assertion doesn't VERIFY it. This test
passes even if `commit_changes` returns `CommitError::WriteTransaction` or any
other error variant. The test's real value is the rollback verification at
lines 1934-1937, but the error assertion is hollow.

**Fix:**
```rust
assert!(
    matches!(result, Err(CommitError::ZeroHashKey { .. })),
    "should fail with ZeroHashKey: {result:?}"
);
```

### [PASS] Banned pattern: `#[ignore]`

No hits.

### [PASS] Banned pattern: sleep in tests

No hits.

### [PASS] Banned pattern: naming violations (`fn test_`, `fn it_works`, etc.)

No hits. All test names follow descriptive behavior-naming convention.

### [PASS] Holzmann Rule 2: Loops in test bodies (commit module)

No loops found inside `#[cfg(test)] mod tests` in commit.rs. All `for` loops in
commit.rs are in production code (validation + write helpers). This is correct.

**Note:** `state::mod.rs` tests have loops at lines 1080, 1091, 1170, 1200, 1531
but those are outside the commit module scope.

### [PASS] Holzmann Rule 7: Shared mutable state

No hits. No `static mut`, `lazy_static!`, or `once_cell` with mutable interior.

### [PASS] Mock interrogation

No mocks found. The commit module uses real `redb::Database` instances via
`tempfile::TempDir`. This is correct — integration tests against the real
storage engine, not mocks.

### [PASS] Integration test purity

No `use crate::` in `/tests/` for commit module. The commit module has no
separate integration test files — all tests are inline `#[cfg(test)]`.

### [FAIL] Error variant completeness

**LETHAL #3–#7** — 5 CommitError variants have zero test coverage:

| Variant | Line | Test Status |
|---------|------|-------------|
| `ZeroHashKey` | 137 | TESTED with `matches!` on exact fields |
| `EmptyStringKey` | 140 | TESTED with `matches!` on exact fields |
| `DuplicateStateKey` | 143 | TESTED with `matches!` on exact fields |
| `MissingReference` | 146 | TESTED with `matches!` + hex verification |
| `PayloadTooLarge` | 153 | TESTED with exact size values |
| `DatabaseOpen` | 161 | TESTED via error message content |
| **`TableInit`** | 165 | **NOT TESTED** — comment: "Hard to trigger deterministically" |
| **`ReadTransaction`** | 168 | **NOT TESTED** — comment: "redb 2.x makes it difficult" |
| **`WriteTransaction`** | 171 | **NOT TESTED** |
| `WriteFailed` | 174 | TESTED (behavior 49 match arm) |
| **`CommitFailed`** | 177 | **NOT TESTED** |
| **`ReadFailed`** | 180 | **NOT TESTED** |

5 of 12 variants are untested. Per protocol: every Error variant must have a
test asserting the exact variant. The comments acknowledging the difficulty of
triggering these via redb do not exempt them. Create the variant directly and
test its Display output, or use a wrapper function that returns the error.

### [PASS] Density audit (commit module)

```
Public functions in commit.rs: 6
  - StateChanges::empty()
  - should_skip_write()
  - StateDb::open()
  - StateDb::begin_read()
  - StateDb::commit_changes()
  - StateDb::database()

Tests in commit.rs: 49

Ratio: 49 / 6 = 8.2x (target ≥5x)
```

### [PASS] Insta dependency

Not present. N/A.

---

## Tier 1 — Compilation + Execution

### [FAIL] Clippy: 320+ warnings/errors

**LETHAL #8** — `cargo clippy --tests -p centralized-docs -- -D warnings` fails
with 320+ errors. Primary causes:

1. **`unwrap_used` denied in test code** — The workspace config denies
   `clippy::unwrap_used` globally, but test helpers use `.unwrap()` for setup.
   Examples in commit.rs:
   - `TempDir::new().unwrap()` (line 2199)
   - `StateDb::open(&db_path).unwrap()` (line 2201)
   - `state_db.commit_changes(setup).unwrap()` (line 2206)

   Fix: Add `#[allow(clippy::unwrap_used)]` at the test module level, or use
   `expect("setup invariant")` with an allow annotation.

2. **`cast_possible_truncation`** — Line 2134: `content_hash: [i as u8; 32]`
   in proptest. The `i` is a `usize` index cast to `u8`.

3. **Unreadable literal** — `52428800` in test assertions (lines 1416, 1442,
   1468, 1494, 1520). Fix: `52_428_800`.

### [FAIL] Project-wide build broken

**LETHAL #9** — `cargo-mutants` (and `--all-features` builds) fail because:

- `tests/validation_atomicity_tests.rs:202` — references non-existent field
  `generate_full` on `LlmsConfig`
- `tests/index_adversarial.rs:57,83` — calls non-existent function
  `build_and_write_compass` (should be `build_and_write_index`)

These are stale integration tests that haven't been updated after refactoring.
They prevent mutation testing and `--all-features` builds.

### [PASS] nextest: 49 passed, 0 failed, 0 flaky

```
cargo test --lib -p centralized-docs -- state::commit
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 818 filtered out
```

All 49 commit module tests pass cleanly.

### [PASS] Ordering probe: consistent

```
--test-threads=1: 49 passed, 0 failed (35.06s)
--test-threads=8: 49 passed, 0 failed (31.34s)
```

No divergence between serial and parallel execution. No hidden shared state.

### N/A Insta: clean

No insta dependency in project.

---

## Tier 2 — Coverage

### [FAIL] Line coverage: 83.89% overall (< 90% target)

**LETHAL #10** — Project-wide line coverage is 83.89%, below the 90% threshold.

| File | Lines | Uncovered | Line% | Branches | Uncovered | Branch% |
|------|-------|-----------|-------|----------|-----------|---------|
| `state/commit.rs` | 1978 | 148 | **92.52%** | 112 | 13 | **88.39%** |
| `state/mod.rs` | 1371 | 70 | **94.89%** | 87 | 13 | **85.06%** |
| **TOTAL** | 27048 | 4357 | **83.89%** | 2055 | 445 | **78.35%** |

The commit module itself passes the line coverage threshold (92.52%). The
overall project drags the average below 90%.

### [FAIL] Branch coverage: 88.39% on commit.rs (< 90% target)

**MAJOR #1** — `state/commit.rs` branch coverage is 88.39%. The 13 uncovered
branches correspond to:

- The 5 untested error paths (TableInit, ReadTransaction, WriteTransaction,
  CommitFailed, ReadFailed) — each has a `map_err` branch
- The `create_dir_all` branch when parent is empty
- Possible boundary branches in `check_payload_size`

### [MAJOR] Branch coverage: 85.06% on state/mod.rs

**MAJOR #2** — Below 90% threshold.

### [PASS] Calc layer coverage

Pure functions in commit.rs have effectively 100% coverage:
- `should_skip_write()` — tested by 2 unit tests + proptest
- `validate_no_zero_hashes()` — tested by 5 unit tests + proptest
- `validate_no_empty_string_keys()` — tested by 4 unit tests
- `validate_no_duplicate_keys()` — tested by 2 unit tests + proptest
- `validate_payload_sizes()` — tested by 5 unit tests
- `validate_reference_integrity()` — tested by 4 unit tests + proptest
- `hash_to_hex()` — tested via MissingReference hex assertion
- `check_zero_hash()` — tested indirectly via validate_no_zero_hashes tests
- `check_ref()` — tested indirectly via validate_reference_integrity tests

---

## Tier 3 — Mutation

### [FAIL] Mutation testing: BLOCKED by broken integration tests

**LETHAL #11** — `cargo mutants` cannot complete because the project has broken
integration tests (`validation_atomicity_tests.rs`, `index_adversarial.rs`)
that prevent compilation. This means **no mutation kill rate can be computed**.

Per protocol: inability to run mutation testing = kill rate 0% = below 90%.

### Manual Mutation Analysis (estimated)

Since cargo-mutants is blocked, here is a manual analysis of expected surviving
mutations in `state/commit.rs` production code:

#### Expected Surviving Mutations

| # | Mutation | Why It Survives | Required Test |
|---|----------|----------------|---------------|
| 1 | `check_payload_size`: `>` to `>=` (line 304) | No test has a payload of exactly `MAX_VALUE_SIZE` bytes (50 MiB). Only `MAX_VALUE_SIZE + 1` is tested. | `commit_changes_accepts_payload_exactly_at_max_value_size_boundary` |
| 2 | Remove `create_dir_all` in `StateDb::open` (line 643) | `TempDir` always creates parent directory. No test exercises the code path where parent doesn't exist. | `state_db_open_creates_parent_directories_when_missing` |
| 3 | Swap `CommitError::TableInit` variant (line 655) | No test triggers TableInit error. | `state_db_open_returns_table_init_error_when_tables_corrupt` |
| 4 | Swap `CommitError::ReadTransaction` variant (line 673) | No test triggers ReadTransaction error. | `state_db_begin_read_returns_read_transaction_error_on_failure` |
| 5 | Swap `CommitError::WriteTransaction` variant (line 715) | No test triggers WriteTransaction error. | `commit_changes_returns_write_transaction_error_on_failure` |
| 6 | Swap `CommitError::CommitFailed` variant (line 723) | No test triggers commit failure. | `commit_changes_returns_commit_failed_when_redb_commit_fails` |
| 7 | Swap `CommitError::ReadFailed` variant (line 608) | No test triggers read failure in `read_and_compare`. | `commit_changes_returns_read_failed_when_redb_read_fails` |

**Estimated kill rate:** ~43 of 50 mutations = **86%** (below 90% target)

#### Well-Killed Mutations (test suite catches these)

These mutations are correctly killed by existing tests:

- `should_skip_write`: change `==` to `!=` → caught by 3 tests + proptest
- `check_zero_hash`: remove ZERO_HASH check → caught by 5 tests + proptest
- `check_empty_string_keys`: remove trim check → caught by 4 tests
- `check_duplicate_keys`: remove seen.insert → caught by 2 tests + proptest
- `check_ref`: remove zero-hash bypass → caught by behavior 23 test
- `write_payload_entries`: remove dedup → caught by behavior 39 test
- All delete operations: remove deletion → caught by behaviors 30, 35, 38
- `apply_all_writes`: remove any table write → caught by behavior 45 mixed test
- `validate_all`: reorder checks → caught by priority-specific tests

---

## LETHAL FINDINGS (11)

1. **commit.rs:2211** — `let _ = state_db.commit_changes(invalid);` in test. Silent
   error discard violates Holzmann Rule 6.

2. **commit.rs:1930** — `assert!(result.is_err(), "should fail with ZeroHashKey")`.
   Banned `is_err()` pattern without specific variant check.

3. **commit.rs:165** — `CommitError::TableInit` has zero tests asserting the exact
   variant. Error variant completeness violation.

4. **commit.rs:168** — `CommitError::ReadTransaction` has zero tests asserting the
   exact variant.

5. **commit.rs:171** — `CommitError::WriteTransaction` has zero tests asserting the
   exact variant.

6. **commit.rs:177** — `CommitError::CommitFailed` has zero tests asserting the
   exact variant.

7. **commit.rs:180** — `CommitError::ReadFailed` has zero tests asserting the exact
   variant.

8. **Project-wide** — `cargo clippy --tests -- -D warnings` fails with 320+ errors.
   Test code triggers denied lints (`unwrap_used`, `cast_possible_truncation`).

9. **Project-wide** — Broken integration tests prevent `--all-features` builds and
   block mutation testing:
   - `tests/validation_atomicity_tests.rs:202` — `generate_full` field removed
   - `tests/index_adversarial.rs:57,83` — `build_and_write_compass` renamed

10. **Project-wide** — Line coverage 83.89% overall, below 90% threshold.

11. **Project-wide** — Mutation testing blocked (kill rate = 0% computable).
    Manual estimate: 86% (below 90%).

---

## MAJOR FINDINGS (4)

1. **commit.rs** — Branch coverage 88.39%, below 90% threshold. 13 uncovered
   branches correspond to untested error paths and boundary conditions.

2. **state/mod.rs** — Branch coverage 85.06%, below 90% threshold.

3. **commit.rs:304** — Boundary gap: no test with payload at exactly
   `MAX_VALUE_SIZE` (50 MiB). The `>` vs `>=` mutation would survive.

4. **commit.rs:643** — `create_dir_all` branch untested. No test exercises
   database open where parent directory doesn't already exist.

---

## MINOR FINDINGS (3)

1. **commit.rs:886** — `assert!(session.is_ok(), ...)` on `StateReadSession`.
   The Ok type has no public inspection methods, making `is_ok()` the only
   meaningful assertion. Weak but unavoidable without changing the API.

2. **commit.rs:2134** — `[i as u8; 32]` in proptest triggers
   `cast_possible_truncation`. The `i` is a `usize` index. Should use
   `(i % 256) as u8` or generate `u8` directly.

3. **commit.rs:1416,1442,1468,1494,1520** — Magic number `52428800` (50 MiB)
   should be written as `52_428_800` for readability per clippy suggestion.

---

## MANDATE

Before resubmission, ALL of the following must exist. This is not optional.
Every item listed below is a hard blocker.

### Must-Fix Before Resubmission

1. **Fix `let _ =` at commit.rs:2211.** Replace with explicit error variant
   assertion using `matches!`.

2. **Fix `is_err()` at commit.rs:1930.** Replace with `matches!` checking for
   `CommitError::ZeroHashKey`.

3. **Write 5 missing error variant tests** — one for each untested variant.
   Even if the variant can't be triggered naturally via redb, create a unit
   test that constructs the variant directly and asserts Display output:
   ```rust
   #[test]
   fn commit_error_table_init_display_contains_reason() {
       let err = CommitError::TableInit { reason: "corrupt".to_string() };
       let msg = format!("{err}");
       assert!(msg.contains("corrupt"), "TableInit display: {msg}");
   }
   ```
   Repeat for `ReadTransaction`, `WriteTransaction`, `CommitFailed`, `ReadFailed`.

4. **Add `#[allow(clippy::unwrap_used)]` at test module level** in commit.rs to
   resolve the 320+ clippy errors. Test setup code using `.unwrap()` is
   acceptable per Holzmann Rule 6 (setup, not assertion).

5. **Fix broken integration tests:**
   - `tests/validation_atomicity_tests.rs:202` — update `LlmsConfig` usage
   - `tests/index_adversarial.rs:57,83` — rename `build_and_write_compass` to
     `build_and_write_index`

6. **Add boundary test for `MAX_VALUE_SIZE`:**
   ```rust
   #[test]
   fn commit_changes_accepts_payload_exactly_at_max_value_size() {
       let (state_db, _temp_dir) = create_temp_state_db();
       let mut changes = StateChanges::empty();
       changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE])];
       let result = state_db.commit_changes(changes);
       assert!(result.is_ok(), "payload at exactly MAX_VALUE_SIZE should be accepted: {result:?}");
   }
   ```

7. **Add parent directory creation test for `StateDb::open`:**
   ```rust
   #[test]
   fn state_db_open_creates_parent_directories() {
       let temp_dir = TempDir::new().unwrap();
       let nested = temp_dir.path().join("a/b/c/state.redb");
       let state_db = StateDb::open(&nested);
       assert!(state_db.is_ok(), "should create nested parent dirs: {state_db:?}");
   }
   ```

8. **After all fixes: re-run ALL four tiers from Tier 0.** Not just the
   failing tier. Full re-run. Always.

---

*End of report. STATUS: REJECTED. 11 LETHAL findings. Do not ship.*
