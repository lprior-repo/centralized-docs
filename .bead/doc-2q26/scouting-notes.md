---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:45:40Z
---

# Scouting Notes

## Codebase Structure Analysis

### Relevant Files

| File | Purpose | Modification Type |
|------|---------|-------------------|
| `release-gate/src/domain.rs` | Domain types | ADD: `WarningBudget`, `WarningCount`, `BudgetCheckResult` |
| `release-gate/src/gate.rs` | Gate logic | ADD: `check_warning_budget()` function |
| `release-gate/src/main.rs` | Binary entry point | MODIFY: Add warning budget to exit logic |
| `release-gate/Cargo.toml` | Dependencies | ADD: `regex` for parsing clippy output |
| `.moon/tasks.yml` | CI tasks | CONSIDER: Add warning baseline file as input |
| `.clippy-baseline` | NEW FILE | Store baseline warning count |

### Existing Patterns Found

1. **Domain Types** (`domain.rs`):
   - Sum types for results: `P0CheckResult::Passed | Failed(Vec<Bead>)`
   - Struct for aggregated results: `GateResult`
   - `is_open()` helper method pattern
   - `Default` implementations

2. **Error Handling** (`gate.rs`):
   - `thiserror` for error taxonomy: `BeadError` enum
   - Explicit error variants: `Io`, `Parse`, `Precondition`, `CiFailed`
   - `Result<T, BeadError>` everywhere

3. **Gate Logic** (`gate.rs`):
   - `ReleaseGate` struct with `execute()` method
   - Sequential checks with fail-fast on blocking conditions
   - Command execution via `std::process::Command`

4. **Main Binary** (`main.rs`):
   - Exit code constants: `EXIT_GATE_PASSED`, `EXIT_P0_BLOCKED`, etc.
   - Pattern matching on results for exit codes
   - Clear error messages to stderr

### What Exists

- Release gate already validates P0 beads and CI status
- Exit code taxonomy established (0-3)
- Domain types pattern established
- Error handling pattern established
- Moon task for `release-gate` already exists

### What Needs Creation

1. **New Domain Types**:
   - `WarningCount` (newtype for type safety)
   - `WarningBudget` (config struct with baseline)
   - `BudgetCheckResult` (Passed | Exceeded)

2. **New Functions**:
   - `run_clippy_and_count_warnings()` - capture clippy output
   - `parse_warning_count()` - extract count from clippy JSON
   - `load_baseline()` - read baseline from file
   - `check_warning_budget()` - compare and report

3. **New Files**:
   - `.clippy-baseline` - single integer baseline count
   - `release-gate/src/warning_budget.rs` - new module

4. **Modifications**:
   - `GateResult` to include `budget_check: BudgetCheckResult`
   - `main.rs` exit logic to handle budget failures
   - New exit code: `EXIT_WARNING_BUDGET_EXCEEDED = 4`

---

## Research Question: Baseline Policy

**Question:** Should baseline be absolute count or allow small delta?

### Answer: **Absolute Count (Recommended)**

#### Rationale

1. **Determinism**: Absolute count is deterministic and easy to reason about
   - "0 warnings allowed" or "N warnings allowed" is unambiguous
   - Delta approach requires additional configuration (threshold) and edge case handling

2. **Simplicity**: Baseline file contains a single integer
   ```
   0
   ```
   vs delta approach requiring:
   ```yaml
   baseline: 5
   max_delta: 2
   ```

3. **Goal Alignment**: The goal is "prevent warning regressions"
   - Absolute count enforces "never worse than N"
   - If you want 0 warnings, set baseline to 0
   - If you have legacy warnings, set baseline to current count

4. **CI Behavior**: 
   - Pass: `current_warnings <= baseline`
   - Fail: `current_warnings > baseline`
   - Clear binary outcome

5. **Anti-Pattern Prevention**:
   - Delta approach allows "drift" over time (each PR adds 1 warning)
   - Absolute count forces explicit baseline updates when intentional

#### Implementation

```rust
// Baseline file: .clippy-baseline (single line, single integer)
// Example content:
5

// Budget check:
fn check_warning_budget(current: WarningCount, baseline: WarningCount) -> BudgetCheckResult {
    if current.value() <= baseline.value() {
        BudgetCheckResult::Passed
    } else {
        BudgetCheckResult::Exceeded { 
            current, 
            baseline,
            delta: current.value() - baseline.value(),
        }
    }
}
```

#### Workflow for Updates

When warnings are intentionally reduced:
1. Run `cargo clippy --workspace --all-targets --all-features -- -W clippy::all 2>&1 | grep "warning:" | wc -l`
2. Update `.clippy-baseline` with new count
3. Commit the baseline file update

---

## Impact Scope

### Files Modified
- `release-gate/src/domain.rs` - Add warning budget types
- `release-gate/src/gate.rs` - Add budget check logic
- `release-gate/src/main.rs` - Add exit code handling
- `release-gate/Cargo.toml` - Add `regex` dependency

### Files Created
- `release-gate/src/warning_budget.rs` - New module for budget logic
- `.clippy-baseline` - Baseline warning count file

### No Breaking Changes
- Existing P0 and CI checks continue unchanged
- New check is additive
- Exit codes 0-3 preserved, new code 4 for budget exceeded

---

## Open Questions Resolved

| Question | Resolution |
|----------|------------|
| Absolute vs delta baseline? | **Absolute count** - simpler, deterministic, prevents drift |
| Where to store baseline? | **`.clippy-baseline`** at repository root |
| Baseline file format? | **Single integer** on first line |
| What clippy flags to use? | **Same as moon `:clippy` task** - workspace, all-targets, all-features |
| How to parse clippy output? | **Count lines containing "warning:"** from stderr |
| When to fail? | **Current > Baseline** (strictly greater) |
