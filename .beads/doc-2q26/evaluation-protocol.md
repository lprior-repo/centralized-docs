---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:00:00Z
---

# Evaluation Protocol

## Definition of Done

The bead is considered **DONE** when ALL of the following conditions are met:

### 1. Code Quality Gates (Must Pass)

| Gate | Command | Expected Result |
|------|---------|-----------------|
| Format check | `moon run :fmt` | No output (passes) |
| Clippy check | `moon run :clippy` | No warnings (or baseline-acceptable count) |
| Type check | `moon run :check` | No errors |
| Tests | `moon run :test` | All tests pass |
| Build | `moon run :build` | Binary compiles successfully |

### 2. Functional Verification

#### 2.1 Baseline File Exists

```bash
# Verify baseline file exists at repository root
cat .clippy-baseline
# Expected: single integer (e.g., "5")
```

#### 2.2 Release Gate Binary Works

```bash
# Build release-gate
cargo build --package release-gate

# Run release gate
cargo run --package release-gate

# Expected (if warnings <= baseline):
# ✅ Release gate passed - all checks OK

# Expected (if warnings > baseline):
# Release blocked: Warning budget exceeded (current: X, baseline: Y, delta: Z)
```

#### 2.3 Exit Code Verification

| Scenario | Expected Exit Code |
|----------|-------------------|
| All gates pass | 0 |
| P0 beads open | 1 |
| CI failed | 2 |
| Error/precondition failure | 3 |
| **Warning budget exceeded** | **4** |

```bash
# Test exit code 4 (warning budget exceeded)
# 1. Create baseline with 0 warnings
echo "0" > .clippy-baseline

# 2. Add code that produces clippy warning
# (e.g., unused variable)

# 3. Run release gate
cargo run --package release-gate

# 4. Check exit code
echo $?
# Expected: 4
```

### 3. Test Suite Verification

All tests defined in `martin-fowler-tests.md` must pass:

```bash
# Run warning budget tests
cargo test --package release-gate -- warning_budget

# Run all release-gate tests
cargo test --package release-gate
```

| Test Category | Must Pass |
|--------------|-----------|
| Happy Path Tests | 6/6 |
| Error Path Tests | 3/3 |
| Edge Case Tests | 5/5 |
| Contract Verification Tests | 6/6 |
| Contract Violation Tests | 6/6 |
| Integration Tests | 3/3 |

---

## Validation Commands

### Quick Validation (Fast Iteration)

```bash
# Format check only
moon run :fmt

# Clippy only
moon run :clippy

# Type check only
moon run :check

# Quick check all three
moon run :quick
```

### Full Validation (Before Commit)

```bash
# Full CI pipeline
moon run :ci

# This runs in parallel:
# - fmt
# - clippy  
# - check
# - test
# - build
```

### Manual Functional Testing

```bash
# 1. Check current warning count
cargo clippy --workspace --all-targets --all-features 2>&1 | grep -c "warning:"

# 2. Update baseline if needed
echo "X" > .clippy-baseline

# 3. Run release gate
cargo run --package release-gate

# 4. Verify exit code
echo $?

# 5. Restore baseline to original value
```

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

## Quality Checklist

Before marking bead as complete, verify:

- [ ] All Moon quality gates pass (`moon run :ci`)
- [ ] `.clippy-baseline` file exists at repository root
- [ ] `release-gate` binary compiles without errors
- [ ] `release-gate` runs without panics
- [ ] Exit code 0 returned when warnings <= baseline
- [ ] Exit code 4 returned when warnings > baseline
- [ ] Error message visible in output when budget exceeded
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] No new clippy warnings introduced by this change

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

## Commit Requirements

Before marking bead as complete, ensure:

1. **Code changes committed:**
   ```bash
   git add -A
   git commit -m "feat(release-gate): add clippy warning budget enforcement"
   ```

2. **Baseline file committed:**
   ```bash
   git add .clippy-baseline
   git commit -m "chore: add initial clippy warning baseline"
   ```

3. **Quality gates pass:**
   ```bash
   moon run :ci
   ```

4. **Push successful:**
   ```bash
   git push
   git status  # Must show "up to date with origin"
   ```

---

## Handoff Context

When handing off to next session, provide:

1. Current warning count: `X`
2. Baseline value: `Y`
3. Exit code behavior verified: Yes/No
4. Tests passing: Yes/No
5. Any blockers or follow-up needed
