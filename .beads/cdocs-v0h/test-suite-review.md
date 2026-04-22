# Test Suite Review — cdocs-v0h

**Bead**: cdocs-v0h "feat(state): Add Database::compact() for state.redb garbage collection"
**Scope**: `compact_state_db`, `should_suggest_compaction`, `log_compaction_suggestion`, `run_compact`
**Date**: 2026-04-07
**Reviewer**: Test Inquisitor (Mode 2 — Suite Inquisition)

---

## VERDICT: REJECTED

---

### Tier 0 — Static

**[PASS]** Banned pattern scan — No bare `assert!(result.is_ok())` or `assert!(result.is_err())` found. All `is_ok()` assertions include descriptive messages. The two `prop_assert!(result.is_ok())` hits are in proptest happy-path branches with explicit sad-path assertions in the alternate branch — acceptable.

**[FAIL]** Holzmann rule scan —
- `commit.rs:4136` — `for i in 1..100u8` loop in `test_compact_after_deletes_preserves_remaining_data` test body. Holzmann Rule 2: **LETHAL**. This is a setup loop creating 99 entries to generate garbage. The loop itself is not testing logic, but it is inside the test function body. A loop is a loop is a loop.
- `commit.rs:3086` — `for i in 0..100u8` loop in `commit_changes_persists_batch_with_100_entries_per_vec` — pre-existing, not part of this bead, but noted.

**[PASS]** Mock interrogation — No mocks found. All tests use real redb databases (tempfile-backed). No mockall, no Mock::new(), no .expect_.

**[PASS]** Integration test purity — No `use crate::` in `/tests/` for compact-related code. The compact tests live in `#[cfg(test)] mod tests` within the source file.

**[PASS]** Error variant completeness — `CompactFailed` variant is tested at line 4204 (`commit_error_compact_failed_display_contains_path_and_reason`). All 12 other CommitError variants have tests. No untested variants.

**[PASS]** Density audit — 114 tests / 20 public functions = **5.7x** (target ≥5x). Meets threshold.

**[PASS]** Naming violations — 3 compact tests use `fn test_*` naming pattern (lines 4062, 4083, 4120). However, these are within an existing `mod tests` block that already uses mixed naming (many tests use descriptive names, some use `fn test_*`). The compact tests are the ONLY new violations in this bead. **MINOR** — not LETHAL since the existing codebase has mixed conventions and the naming is descriptive after the `test_` prefix.

---

### Tier 1 — Execution

**[PASS]** Compilation — `cargo test -p centralized-docs --lib -- compact` compiles and runs cleanly.

**[PASS]** Test execution — 7 compact tests, 7 passed, 0 failed, 0 flaky.

**[PASS]** Ordering probe — All tests use per-test `TempDir` with isolated paths. No shared state. Ordering independence verified by design.

**[N/A]** Insta — Not present in this project.

---

### Tier 2 — Coverage

Manual analysis of branch coverage for the 3 new functions:

**`should_suggest_compaction` (lines 699-706)** — Pure function, 3 branches:
| Branch | Test | Status |
|--------|------|--------|
| `logical_data_size == 0` | `should_suggest_compaction_returns_false_for_zero_sizes` (1000, 0) | COVERED |
| `file_size == 0` | `should_suggest_compaction_returns_false_for_zero_sizes` (0, 1000) | COVERED |
| `ratio > COMPACTION_THRESHOLD_RATIO` true | `should_suggest_compaction_returns_true_when_ratio_exceeded` | COVERED |
| `ratio > COMPACTION_THRESHOLD_RATIO` false | `should_suggest_compaction_returns_false_when_ratio_ok` | COVERED |
| `ratio == COMPACTION_THRESHOLD_RATIO` (exact boundary) | **NO TEST** | **UNCOVERED** |
| `file_size > u32::MAX` (truncation path) | **NO TEST** | **UNCOVERED** |

**`compact_state_db` (lines 726-742)** — IO action, branches:
| Branch | Test | Status |
|--------|------|--------|
| `builder.open(path)` succeeds | `test_compact_on_fresh_db_is_noop` | COVERED |
| `builder.open(path)` fails, `builder.create(path)` succeeds | `test_compact_on_empty_db_succeeds` (empty dir) | COVERED |
| `builder.open(path)` fails, `builder.create(path)` fails → CompactFailed | **NO TEST** | **UNCOVERED** |
| `db.compact()` succeeds, returns true | No test distinguishes true from false | **UNCOVERED** |
| `db.compact()` succeeds, returns false | No test distinguishes true from false | **UNCOVERED** |
| `db.compact()` fails → CompactFailed | **NO TEST** | **UNCOVERED** |

**`log_compaction_suggestion` (lines 897-961)** — Private action:
| Branch | Test | Status |
|--------|------|--------|
| `db_path` is None (in-memory) → early return | No test | **UNCOVERED** |
| `db_path` is Some, metadata fails → early return | No test | **UNCOVERED** |
| `db_path` is Some, metadata succeeds, ratio > threshold → warn logged | No test | **UNCOVERED** |
| `db_path` is Some, metadata succeeds, ratio <= threshold → no warn | No test | **UNCOVERED** |

**`run_compact` (cmd/compact.rs, lines 18-27)** — CLI handler:
| Branch | Test | Status |
|--------|------|--------|
| `compact_state_db` returns Ok(true) → "completed successfully" | **NO TEST** | **UNCOVERED** |
| `compact_state_db` returns Ok(false) → "already compact" | **NO TEST** | **UNCOVERED** |
| `compact_state_db` returns Err → error propagated | **NO TEST** | **UNCOVERED** |

**[FAIL]** Coverage — Multiple uncovered branches across all new functions. The integration tests verify that compact doesn't corrupt data, but they do NOT verify that compact actually did anything, or that the return value is checked.

---

### Tier 3 — Mutation

**Kill rate: ~40% (3 of 7 mutations caught)**

| # | Mutation | Caught? | By which test |
|---|----------|---------|---------------|
| M1 | Stub `compact_state_db` body → `Ok(true)` | **NO** | All 3 tests pass — data survives because it was written BEFORE compact |
| M2 | Stub `compact_state_db` body → `Ok(false)` | **NO** | All 3 tests pass — `is_ok()` doesn't check inner bool |
| M3 | Stub `compact_state_db` body → `Err(CompactFailed{...})` | **YES** | All 3 tests fail the `assert!(result.is_ok(), ...)` |
| M4 | Change `>` to `>=` in `should_suggest_compaction` | **NO** | No test at exact boundary (ratio = 10.0) |
| M5 | Remove early return for `file_size == 0` | **PARTIAL** | (1000, 0) would panic on division → caught. (0, 1000) would return 0/1000=0 < 10 → not caught |
| M6 | Remove `set_cache_size(DEFAULT_CACHE_SIZE)` | **NO** | No observable difference from tests |
| M7 | Delete `run_compact` body entirely → `Ok(())` | **NO** | No tests for run_compact at all |

### Surviving Mutants

1. **M1: Stub compact_state_db → Ok(true)** — The tests verify that data written BEFORE compact is still readable AFTER compact. But they never verify that compact actually compacted anything. If compact is a no-op stub, all tests pass. The tests prove that compact doesn't destroy data, but they DON'T prove that compact does its job.
   - **REQUIRED TEST**: `compact_reduces_file_size_after_bulk_delete` — Write 100 large entries, delete 99, compact, verify `fs::metadata(path).len()` decreased.

2. **M2: Stub compact_state_db → Ok(false)** — The bool return value is never checked by any test. The CLI handler `run_compact` branches on this value to print different messages, but no test verifies which message.
   - **REQUIRED TEST**: `compact_returns_true_when_compaction_performed` or verify the bool return explicitly.

3. **M4: Change > to >= in should_suggest_compaction** — No test at the exact threshold boundary.
   - **REQUIRED TEST**: `should_suggest_compaction_returns_false_at_exact_threshold_ratio` — `should_suggest_compaction(10000, 1000)` should be false (ratio exactly 10.0, not > 10.0).

4. **M7: Delete run_compact body** — No tests at all for the CLI handler.
   - **REQUIRED TEST**: `run_compact_prints_success_on_compaction` — Integration test that verifies stdout/stderr output.

---

### LETHAL FINDINGS

1. **commit.rs:4136** — Loop (`for i in 1..100u8`) in test body `test_compact_after_deletes_preserves_remaining_data`. Holzmann Rule 2 violation. A loop in a test is a program, not a proof. Replace with a setup helper or pre-built fixture.

### MAJOR FINDINGS (3)

1. **commit.rs:4182-4198** — `should_suggest_compaction` has NO test at the exact threshold boundary (`ratio == 10.0`). Mutation `>` to `>=` survives undetected. This is a pure function — the threshold boundary is the single most important test case.

2. **commit.rs:4062-4113** — All three `compact_state_db` integration tests are **tautological**: they verify data written before compact exists after compact. They pass when `compact_state_db` is replaced with `Ok(true)`. None verify that compaction actually happened (file size reduction, garbage removal).

3. **cmd/compact.rs:18-27** — `run_compact` has ZERO tests. This is a `pub fn` in the CLI layer that branches on the `bool` return from `compact_state_db`. Neither branch is tested. The function prints different messages to stderr depending on the result — no test verifies which message is printed.

### MINOR FINDINGS (2)

1. **commit.rs:4062, 4083, 4120** — Three compact tests use `fn test_*` naming prefix. The existing test suite in this file uses descriptive behavior names (e.g., `commit_changes_rejects_zero_hash_key_in_analysis_outputs`). The new compact tests should follow the established convention: `compact_state_db_succeeds_on_empty_database`, `compact_state_db_preserves_data_on_fresh_database`, `compact_state_db_preserves_remaining_data_after_deletes`.

2. **commit.rs:699-706** — `should_suggest_compaction` truncates `u64` to `u32` via `u32::try_from(file_size).unwrap_or(u32::MAX)`. No test exercises values > 4 GiB. For a database compaction function, this is realistic. A test with `file_size = u32::MAX as u64 + 1` should verify the truncation behavior.

---

### MANDATE

Before resubmission, the following MUST exist:

1. **LETHAL fix**: Remove the loop from `test_compact_after_deletes_preserves_remaining_data` (line 4136). Extract the 99-entry fixture into a test helper function (e.g., `create_garbage_entries()`) that is called once. The test body itself must be loop-free.

2. **REQUIRED TEST: `should_suggest_compaction_returns_false_at_exact_threshold_ratio`**
   ```rust
   #[test]
   fn should_suggest_compaction_returns_false_at_exact_threshold_ratio() {
       // file_size / logical_data_size = exactly 10.0 (threshold)
       // ratio > 10.0 is false; ratio >= 10.0 would be true
       assert!(!should_suggest_compaction(10000, 1000));
   }
   ```

3. **REQUIRED TEST: `compact_state_db_actually_reduces_file_size_after_deletes`**
   - Write 100+ large entries to create a sizable database file
   - Delete 99% of entries
   - Record file size before and after `compact_state_db`
   - Assert `file_size_after < file_size_before`
   - This test kills mutation M1 (stub body → Ok(true))

4. **REQUIRED TEST: `compact_state_db_return_value_is_checked`**
   - Call `compact_state_db` on a fresh database
   - Assert the return value is `Ok(false)` OR `Ok(true)` explicitly (not just `is_ok()`)
   - This test kills mutation M2 (stub body → Ok(false))

5. **REQUIRED TEST: `compact_state_db_returns_error_on_invalid_path`**
   - Call `compact_state_db` with a path that cannot be opened or created
   - Assert the error is `CommitError::CompactFailed`
   - This covers the error branch in `compact_state_db`

6. **REQUIRED TEST**: At least one test for `run_compact` verifying it produces output on success and propagates errors. This is a `pub fn` with zero test coverage.

7. **Rename tests**: Remove `test_` prefix from the 3 compact test function names to match the existing naming convention.

After fixes: **re-run ALL tiers from Tier 0**. Full re-run. Always.
