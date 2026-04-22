# Test Suite Review v2 — cdocs-v0h

**Bead:** cdocs-v0h "feat(state): Add Database::compact() for state.redb garbage collection"
**Reviewer:** Test Inquisitor (Mode 2 — Suite Inquisition)
**Date:** 2026-04-07
**Previous verdict:** REJECTED (v1 — tautological tests, no size assertion, missing boundary)
**Scope:** `compact_state_db()`, `should_suggest_compaction()`, `CommitError::CompactFailed`

---

## VERDICT: REJECTED

---

### Tier 0 — Static

| Check | Result | Notes |
|-------|--------|-------|
| Banned patterns (`is_ok()`/`is_err()`) | **PASS** | Grep returns zero hits. Three multiline `assert!(result.is_ok(), "msg")` assertions at lines 4073, 4100, 4169 bypass the regex. Functionally weak but not pattern-matched. |
| Silent error discard (`let _ =` / `.ok()`) | **PASS** | Zero hits in compact test code. `_size_before` at line 4163 is an underscore-prefixed binding, not `let _ =`. |
| Ignored tests | **PASS** | Zero `#[ignore]` in compact tests. (16 ignored tests in `state_db_integration_tests.rs` are abandoned-API, unrelated.) |
| Sleep in tests | **PASS** | Zero hits in compact tests. |
| Naming violations (`fn test_`) | **PASS** | 5 of 10 tests use `test_` prefix. **However** — this is the established codebase convention (2340 tests, majority use `test_` prefix). Not a bead-specific defect. The `should_suggest_compaction_*` and `commit_error_compact_failed_*` tests use descriptive names. |
| Holzmann Rule 2 (loops) | **PASS** | All loops in compact tests are compile-time bounded: `1..100u8`, `1u8..=200`, `11u8..=200`, `1u8..=10`, `1u8..=50`. Deterministic upper bounds known at compile time. Compliant. |
| Shared mutable state | **PASS** | Zero hits. Each test creates its own `TempDir`. |
| Mock interrogation | **PASS** | Zero mocks. Tests use real `redb` instances. |
| Integration purity | **PASS** | All compact tests are module-level (`#[cfg(test)]` in `src/state/commit.rs`), not in `tests/`. No `use crate::` violation. |
| Error variant completeness | **PASS** | `CompactFailed { path, reason }` at line 184 has Display test at line 4330 asserting exact variant fields. All other `CommitError` variants tested throughout the file. |
| Density audit | **PASS** | 2340 tests / 393 pub fns = **5.95x** (target ≥5x) |

### Tier 1 — Execution

| Check | Result | Notes |
|-------|--------|-------|
| Clippy | **FAIL** | **2 warnings, 1 from this bead:** |
| | | 1. `src/state/commit.rs:4206` — `unused variable: size_after_insert` (THIS BEAD) |
| | | 2. Pre-existing `clippy::uninlined-format-args` in another module (NOT this bead) |
| nextest | **PASS** | 10/10 compact tests pass. Cannot run full nextest gate because clippy already failed. |
| Ordering probe | **SKIP** | Blocked by clippy failure. |
| Insta | **N/A** | No insta dependency in Cargo.toml. |

### Tier 2 — Coverage

**SKIP** — Tier 1 failed. Not compute-worthy.

### Tier 3 — Mutation

**SKIP** — Tier 1 failed. Not compute-worthy.

---

## Previous Rejection — Fix Assessment

### LETHAL (v1): Tautological compact tests — pass when body replaced with Ok(true)

**PARTIALLY FIXED.**

The new `test_compact_after_churn_recovers_space` (line 4255) catches the no-op mutation:

```
Mutation: Replace compact_state_db body with Ok(true)
Result:   test_compact_after_churn_recovers_space FAILS
Why:      50 write/delete cycles leave stale pages. No-op compact means
          size_after_compact == size_before_compact, violating the strict
          size_after_compact < size_before_compact assertion.
```

However, 4 of 5 `compact_state_db` integration tests are STILL individually tautological for the return value:

| Test | is_ok() only? | Catches no-op? | Why |
|------|---------------|----------------|-----|
| `test_compact_on_empty_db_succeeds` (4062) | Yes (line 4073) | No | Empty DB, no garbage. No-op is indistinguishable from real compact. |
| `test_compact_on_fresh_db_is_noop` (4083) | Yes (line 4100) | No | Fresh DB, no deletes. Data survives either way. |
| `test_compact_after_deletes_preserves_remaining_data` (4120) | Yes (line 4169) | No | Only checks data survival, not space reclamation. |
| `test_compact_reduces_file_size_after_bulk_delete` (4187) | No (asserts `compacted` bool) | **No** | Records `size_after_insert` at line 4206 but NEVER COMPARES IT. Name claims "reduces file size" but test only checks `compacted == true` + data survival. |
| `test_compact_after_churn_recovers_space` (4255) | No (asserts bool + size) | **Yes** | Asserts `size_after_compact < size_before_compact`. This is the only test that proves compact actually does work. |

**Net:** The suite catches the mutation via ONE test. 4 out of 5 integration tests pass with a no-op. This is a fragile defense-in-depth posture.

### MAJOR (v1): No file size reduction assertion

**FIXED** — `test_compact_after_churn_recovers_space` asserts `size_after_compact < size_before_compact`.

### MAJOR (v1): Missing threshold boundary test

**FIXED** — `should_suggest_compaction_at_exact_threshold_boundary` (line 4311) tests both `10_000 / 1_000` (exactly 10x, must return false) and `10_001 / 1_000` (exceeds 10x, must return true).

---

## LETHAL FINDINGS

### L1 — `commit.rs:4206` — Unused variable `size_after_insert` fails clippy

```rust
let size_after_insert = std::fs::metadata(&db_path)
    .expect("db file should exist")
    .len();
```

`size_after_insert` is recorded after bulk insert but never compared against anything. The variable name implies it should be used to verify compaction reduced the file, but no such assertion exists. This is not just a clippy failure — it's a test that claims to verify size reduction but doesn't.

**Impact:** `cargo clippy --tests -- -D warnings` returns non-zero. CI would fail.

### L2 — `commit.rs:4187` — `test_compact_reduces_file_size_after_bulk_delete` is falsely named

The test name says "reduces file size." The test body:
1. Inserts 200 entries
2. Records `size_after_insert` (then ignores it)
3. Deletes 190 entries
4. Calls compact, asserts `compacted == true`
5. Verifies 10 entries survived

If `compact_state_db` is replaced with `fn compact_state_db(_: &Path) -> Result<bool, CommitError> { Ok(true) }`, this test **PASSES**:
- `compacted` is `true` ✓
- Entries 1-10 were never deleted, so they survive ✓
- No size comparison ✓

The test proves data survives compaction. It does NOT prove file size reduction. The name is a lie.

---

## MAJOR FINDINGS (2/3 threshold)

### M1 — `commit.rs:4163` — `_size_before` recorded but unused (dead code)

```rust
let _size_before = std::fs::metadata(&db_path)
    .expect("db file should exist before compact")
    .len();
```

Underscore-prefixed, never used. If it's "for documentation," a comment would suffice. A binding that's never read is dead code that confuses readers into thinking the size is being asserted.

### M2 — No error-path integration test for `compact_state_db`

`CompactFailed` is tested via Display construction (line 4330). But no test actually triggers the error through the function — e.g., compacting a corrupted file, a file with active transactions, or a read-only path. The error handling path has zero integration coverage.

---

## MINOR FINDINGS (2/5 threshold)

### m1 — Lines 4073, 4100, 4169 — Multiline `is_ok()` assertions bypass banned-pattern grep

Three tests use `assert!(result.is_ok(), "msg")` spread across multiple lines. The grep pattern catches only `assert!(result.is_ok())` (single-line). These assertions verify "doesn't crash" but not the bool return value. Acceptable for empty/fresh DB tests where the return value is semantically unimportant, but worth noting.

### m2 — `test_compact_on_fresh_db_is_noop` (line 4083) — Name claims "noop" but doesn't verify

The test name says the compact is a noop, but the test doesn't verify:
- Return value is `Ok(false)` (no compaction performed)
- File size unchanged after compact

It only checks `is_ok()` and data survival. For a fresh DB with no deletes, compact returning `Ok(true)` (indicating compaction WAS performed) would also pass this test. The "noop" claim is unverified.

---

## MANDATE

Before resubmission for v3 review:

1. **FIX L1** — Either use `size_after_insert` in an assertion (e.g., compare with size after delete + compact), or prefix it with `_` to silence clippy. If you use it, the test's name becomes accurate.

2. **FIX L2** — `test_compact_reduces_file_size_after_bulk_delete` must assert actual file size reduction:
   ```rust
   let size_after_compact = std::fs::metadata(&db_path).expect("db should exist").len();
   assert!(
       size_after_compact < size_after_insert,
       "compacted ({size_after_compact}) should be smaller than after insert ({size_after_insert})"
   );
   ```
   This would make the test name truthful AND kill the no-op mutation independently of the churn test.

3. **FIX M1** — Remove `_size_before` or use it. Dead bindings in tests are noise.

4. **CONSIDER M2** — Add a test that triggers `CompactFailed` through the function (e.g., create a file that's not a valid redb database, then try compact). This is not blocking but the error path has zero integration coverage.

After fixing: re-run `cargo clippy --tests -- -D warnings` must return 0 (excluding pre-existing unrelated warnings). Then resubmit for full v3 review from Tier 0.
