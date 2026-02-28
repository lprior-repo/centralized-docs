---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:45:40Z
---

# Contract Specification

## Context

- **Feature:** Clippy warning budget enforcement for release validation
- **Domain terms:**
  - `WarningCount`: Non-negative integer representing number of clippy warnings
  - `WarningBudget`: Policy defining maximum allowed warnings (stored in baseline file)
  - `BudgetCheckResult`: Outcome of comparing current vs baseline warning count
- **Assumptions:**
  - Clippy is available in CI environment
  - `.clippy-baseline` file exists at repository root
  - Clippy output is captured via stderr
- **Open questions:** None - all resolved in scouting-notes.md

---

## Domain Types

### WarningCount (Newtype)

```rust
/// A non-negative count of clippy warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WarningCount(u32);

impl WarningCount {
    /// Create a new WarningCount, returns error if negative (impossible for u32)
    pub fn new(count: u32) -> Self {
        Self(count)
    }
    
    /// Get the raw count value
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
    
    /// Create from clippy output string (counts "warning:" occurrences)
    pub fn from_clippy_output(output: &str) -> Self {
        let count = output.lines()
            .filter(|line| line.contains("warning:"))
            .count() as u32;
        Self(count)
    }
    
    /// Parse from baseline file content
    pub fn from_baseline(content: &str) -> Result<Self, BudgetError> {
        let line = content.lines().next().ok_or(BudgetError::EmptyBaseline)?;
        let count: u32 = line.trim().parse().map_err(|_| BudgetError::InvalidBaseline {
            content: line.to_string(),
        })?;
        Ok(Self(count))
    }
}
```

### BudgetCheckResult (Sum Type)

```rust
/// Result of comparing current warning count to baseline budget
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetCheckResult {
    /// Current warnings <= baseline - release is clear
    Passed {
        current: WarningCount,
        baseline: WarningCount,
    },
    /// Current warnings > baseline - release blocked
    Exceeded {
        current: WarningCount,
        baseline: WarningCount,
        delta: u32,
    },
}

impl BudgetCheckResult {
    /// Returns true if budget check passed
    #[must_use]
    pub fn is_passed(&self) -> bool {
        matches!(self, Self::Passed { .. })
    }
}
```

### GateResult (Modified)

```rust
/// Result of the entire gate check (MODIFIED to include budget)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateResult {
    /// Result of the P0 beads check
    pub p0_check: P0CheckResult,
    /// Whether CI passed
    pub ci_passed: bool,
    /// Result of the warning budget check (NEW)
    pub budget_check: BudgetCheckResult,
}
```

---

## Error Taxonomy

```rust
/// Error types for warning budget operations
#[derive(Debug, thiserror::Error)]
pub enum BudgetError {
    #[error("baseline file is empty")]
    EmptyBaseline,
    
    #[error("invalid baseline content: '{content}' - expected non-negative integer")]
    InvalidBaseline { content: String },
    
    #[error("failed to read baseline file: {message}")]
    BaselineRead { message: String },
    
    #[error("failed to run clippy: {message}")]
    ClippyExecution { message: String },
    
    #[error("clippy process failed with exit code {code}")]
    ClippyFailed { code: i32 },
}
```

---

## Function Signatures

### Module: `warning_budget`

```rust
/// Load the warning budget baseline from file
/// 
/// # Errors
/// - `BudgetError::BaselineRead` if file cannot be read
/// - `BudgetError::EmptyBaseline` if file is empty
/// - `BudgetError::InvalidBaseline` if content is not a valid integer
pub fn load_baseline(path: &Path) -> Result<WarningCount, BudgetError>;

/// Run clippy and capture warning count from output
/// 
/// # Errors
/// - `BudgetError::ClippyExecution` if clippy command cannot be spawned
/// - `BudgetError::ClippyFailed` if clippy exits with non-zero (but we still count warnings)
pub fn run_clippy_and_count_warnings() -> Result<WarningCount, BudgetError>;

/// Compare current warnings to baseline budget
/// 
/// Returns `BudgetCheckResult::Passed` if current <= baseline
/// Returns `BudgetCheckResult::Exceeded` if current > baseline
#[must_use]
pub fn check_budget(current: WarningCount, baseline: WarningCount) -> BudgetCheckResult;

/// Execute full budget check workflow
/// 
/// # Errors
/// Propagates errors from `load_baseline` or `run_clippy_and_count_warnings`
pub fn execute_budget_check(baseline_path: &Path) -> Result<BudgetCheckResult, BudgetError>;
```

### Module: `gate` (modifications)

```rust
impl ReleaseGate {
    /// Execute the release gate checks (MODIFIED)
    /// 
    /// Checks in order:
    /// 1. P0 beads must be closed (fail-fast)
    /// 2. Moon CI must pass (fail-fast)
    /// 3. Warning budget must not be exceeded
    pub fn execute(&self) -> Result<GateResult, BeadError>;
}
```

---

## Preconditions

| ID | Precondition | Enforcement Level |
|----|--------------|-------------------|
| P1 | `.clippy-baseline` file exists at repository root | Result error: `BudgetError::BaselineRead` |
| P2 | `.clippy-baseline` contains valid non-negative integer | Result error: `BudgetError::InvalidBaseline` |
| P3 | `cargo` command is available in PATH | Result error: `BudgetError::ClippyExecution` |
| P4 | Clippy is installed for the workspace | Result error: `BudgetError::ClippyExecution` |

---

## Postconditions

| ID | Postcondition |
|----|---------------|
| Q1 | `load_baseline` returns `Ok(WarningCount)` with parsed value |
| Q2 | `run_clippy_and_count_warnings` returns count of lines containing "warning:" |
| Q3 | `check_budget` returns `Passed` when `current <= baseline` |
| Q4 | `check_budget` returns `Exceeded` when `current > baseline` |
| Q5 | `execute_budget_check` produces deterministic result for same inputs |
| Q6 | Exit code is 4 when budget exceeded, 0 when all gates pass |

---

## Invariants

| ID | Invariant |
|----|-----------|
| I1 | `WarningCount` is always >= 0 (enforced by u32) |
| I2 | Baseline comparison is deterministic (same inputs → same output) |
| I3 | Policy applies consistently regardless of branch |
| I4 | Budget check is independent of P0 and CI checks (not OR-ed) |

---

## Type Encoding

| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| WarningCount >= 0 | Compile-time (strongest) | `u32` in `WarningCount(u32)` |
| Baseline file exists | Runtime check | `File::open()` returns `Err` |
| Baseline content is integer | Runtime check | `str::parse::<u32>()` returns `Err` |
| Clippy available | Runtime check | `Command::status()` returns `Err` |

---

## Violation Examples (REQUIRED)

### P1 Violations

```
VIOLATES P1: load_baseline(Path::new("nonexistent-file"))
  -> returns Err(BudgetError::BaselineRead { message: "..." })
```

### P2 Violations

```
VIOLATES P2: load_baseline with file containing "not-a-number"
  -> returns Err(BudgetError::InvalidBaseline { content: "not-a-number" })

VIOLATES P2: load_baseline with empty file
  -> returns Err(BudgetError::EmptyBaseline)

VIOLATES P2: load_baseline with file containing "-5"
  -> returns Err(BudgetError::InvalidBaseline { content: "-5" })

VIOLATES P2: load_baseline with file containing "3.14"
  -> returns Err(BudgetError::InvalidBaseline { content: "3.14" })
```

### Q3/Q4 Violations (Logic Verification)

```
VIOLATES Q4: check_budget(WarningCount::new(10), WarningCount::new(5))
  -> returns BudgetCheckResult::Exceeded { current: 10, baseline: 5, delta: 5 }
  (NOT Passed - verifies postcondition Q4)
```

---

## Ownership Contracts

| Function | Ownership | Mutation Contract |
|----------|-----------|-------------------|
| `load_baseline(path: &Path)` | Shared borrow | No mutation, reads file |
| `run_clippy_and_count_warnings()` | No params | Spawns subprocess, no mutation |
| `check_budget(current, baseline)` | Copy (u32) | Pure function, no mutation |
| `execute_budget_check(path: &Path)` | Shared borrow | Calls other functions, no mutation |

---

## Edge Cases

| Edge Case | Expected Behavior |
|-----------|-------------------|
| Baseline = 0, Current = 0 | `Passed` |
| Baseline = 0, Current = 1 | `Exceeded { delta: 1 }` |
| Baseline = 5, Current = 5 | `Passed` (equality allowed) |
| Baseline = 5, Current = 4 | `Passed` |
| Empty clippy output | `WarningCount(0)` |
| Clippy output with no warnings | `WarningCount(0)` |
| Clippy output with mixed content | Count only "warning:" lines |
| Baseline file with trailing newline | Trim and parse |
| Baseline file with leading/trailing spaces | Trim and parse |

---

## Non-goals

- [ ] Delta-based budget (e.g., "allow +2 warnings") - use absolute count
- [ ] Per-crate warning budgets - workspace-wide only
- [ ] Warning severity classification - all warnings counted equally
- [ ] Auto-updating baseline - manual update required
- [ ] Historical tracking - only current vs baseline comparison
