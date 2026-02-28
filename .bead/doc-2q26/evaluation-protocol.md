---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:45:40Z
---

# Evaluation Protocol

## Gate Criteria

This bead is considered **PASSED** when ALL of the following criteria are met.

---

## Mandatory Checks (Must Pass)

### M1: Moon CI Pipeline

```bash
moon run :ci
```

**Expected Result:** Exit code 0

**What it validates:**
- `:fmt` - Code formatting is correct
- `:clippy` - No clippy errors (warnings allowed per baseline)
- `:test` - All tests pass
- `:test-doc` - Documentation tests pass
- `:build` - Release build succeeds

**Failure action:** Fix all issues before proceeding.

---

### M2: Release Gate Binary

```bash
cargo run --release -p release-gate
```

**Expected Result:** Exit code 0 (if all gates pass)

**What it validates:**
- P0 check passes (no open P0 beads)
- CI check passes (moon run :ci)
- Budget check passes (warnings <= baseline)

**Failure action:** Address failing gate.

---

### M3: Unit Tests for Warning Budget Module

```bash
cargo test -p release-gate warning_budget
```

**Expected Result:** All tests pass

**Required test coverage:**
- `WarningCount::new()` creates valid count
- `WarningCount::from_clippy_output()` counts correctly
- `WarningCount::from_baseline()` parses valid input
- `WarningCount::from_baseline()` errors on invalid input
- `BudgetCheckResult::is_passed()` returns correct bool
- `check_budget()` returns Passed when current <= baseline
- `check_budget()` returns Exceeded when current > baseline
- `load_baseline()` reads file successfully
- `load_baseline()` errors on missing file

---

### M4: Exit Code Verification

```bash
# Test pass scenario
echo "100" > .clippy-baseline  # High baseline
cargo run --release -p release-gate
echo $?  # Should be 0

# Test fail scenario
echo "0" > .clippy-baseline  # Strict baseline
# (Ensure warnings exist, then run)
cargo run --release -p release-gate
echo $?  # Should be 4 if warnings exist
```

**Expected Results:**
- Exit code 0 when budget check passes
- Exit code 4 when budget exceeded

---

## Quality Gates

### Q1: No Clippy Warnings in New Code

```bash
moon run :clippy
```

**Policy:** New code MUST have 0 warnings. Baseline applies to legacy code only.

---

### Q2: No Unwrap/Expect in Production Code

```bash
grep -r "unwrap()" release-gate/src/*.rs | grep -v test
grep -r "expect(" release-gate/src/*.rs | grep -v test
```

**Expected Result:** No matches (enforced by clippy lints)

---

### Q3: Documentation

```bash
cargo doc -p release-gate --no-deps
```

**Expected Result:** Documentation builds without warnings.

**Required documentation:**
- Module-level doc comment in `warning_budget.rs`
- Public function doc comments
- Public type doc comments

---

## Evidence Required

### E1: Baseline File Exists

```bash
ls -la .clippy-baseline
cat .clippy-baseline
```

**Expected:** File exists at repository root with single integer content.

---

### E2: Test Output

```bash
cargo test -p release-gate -- --nocapture 2>&1 | head -50
```

**Expected:** All tests pass with green output.

---

### E3: Gate Execution Output

```bash
cargo run --release -p release-gate 2>&1
```

**Expected:**
```
Running Moon CI checks...
...
✅ Moon CI passed
✅ Release gate PASSED - all checks passed
```

---

## Exit Code Reference

| Scenario | Expected Exit Code |
|----------|-------------------|
| All gates pass | 0 |
| P0 beads open | 1 |
| CI failed | 2 |
| Error/precondition failure | 3 |
| **Warning budget exceeded** | **4** |

---

## Grading Rubric

| Criterion | Weight | Pass Condition |
|-----------|--------|----------------|
| M1: Moon CI | 30% | Exit code 0 |
| M2: Release Gate | 20% | Exit code 0 (or 4 if expected) |
| M3: Unit Tests | 30% | All tests pass |
| M4: Exit Codes | 10% | Correct codes 0 and 4 |
| Q1-Q3: Quality | 10% | No violations |

**Passing Score:** 100% (all criteria must pass)

---

## Failure Recovery

### If M1 Fails (Moon CI)

1. Run `moon run :fmt-fix` to fix formatting
2. Run `moon run :clippy` to see remaining issues
3. Fix clippy warnings
4. Run `moon run :test` to fix failing tests
5. Re-run `moon run :ci`

### If M2 Fails (Release Gate)

1. Check which gate failed (P0, CI, or Budget)
2. Address the specific failure
3. Re-run release gate

### If M3 Fails (Tests)

1. Run `cargo test -p release-gate -- --nocapture` for details
2. Fix failing tests
3. Re-run tests

### If M4 Fails (Exit Codes)

1. Verify exit code logic in `main.rs`
2. Check `BudgetCheckResult` matching
3. Verify error handling

---

## Regression Prevention

### What Constitutes a Regression

| Condition | Status |
|-----------|--------|
| Warning count increases beyond baseline | **FAIL** |
| New clippy warnings introduced | **FAIL** |
| Baseline file deleted | **FAIL** |
| Baseline contains non-integer | **FAIL** |
| Exit code 4 not returned on budget exceeded | **FAIL** |

### What Does NOT Constitute a Regression

| Condition | Status |
|-----------|--------|
| Warning count decreases (fewer warnings) | **PASS** |
| Warning count equals baseline | **PASS** |
| Manual baseline update with intentional reduction | **PASS** |

---

## Final Verification Checklist

Before marking bead as complete:

- [ ] `moon run :ci` exits with code 0
- [ ] `.clippy-baseline` file exists with valid content
- [ ] `release-gate` binary runs successfully
- [ ] Exit code 0 when all checks pass
- [ ] Exit code 4 when budget exceeded
- [ ] All unit tests pass
- [ ] No new clippy warnings in changed files
- [ ] No `unwrap()` or `expect()` in production code
- [ ] Documentation builds without warnings
- [ ] Changes committed and pushed

---

## Troubleshooting

### Issue: Exit code is not 4 when warnings exceed baseline

**Check:**
1. Is `execute_budget_check()` being called?
2. Is `BudgetCheckResult::Exceeded` being matched in `main.rs`?
3. Is `EXIT_WARNING_BUDGET_EXCEEDED = 4` defined?

### Issue: Baseline file not found

**Check:**
1. Does `.clippy-baseline` exist at repository root?
2. Is path correct in `execute_budget_check()`?
3. Is working directory correct when running?

### Issue: Warning count is wrong

**Check:**
1. Are clippy flags correct?
2. Is stderr being captured?
3. Is `WarningCount::from_clippy_output()` counting correctly?

### Issue: Tests fail

**Check:**
1. Are test functions in correct module?
2. Are test dependencies added to `Cargo.toml`?
3. Are test names correct in `cargo test` command?

---

## Sign-Off

**Implementing Agent:** _[Fill after implementation]_

**Date:** _[Fill after implementation]_

**Moon CI Status:** [ ] Pass

**Test Status:** [ ] Pass

**Gate Status:** [ ] Pass

**Overall:** [ ] PASS / [ ] FAIL
