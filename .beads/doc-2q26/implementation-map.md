---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:00:00Z
---

# Implementation Map

## File Structure

```
release-gate/
├── src/
│   ├── main.rs           # MODIFY: Add exit code handling
│   ├── domain.rs        # MODIFY: Add WarningCount, BudgetCheckResult
│   ├── gate.rs          # MODIFY: Add budget check to execute()
│   └── warning_budget.rs # NEW: Budget logic module
├── Cargo.toml           # MODIFY: Add dependencies
├── .clippy-baseline     # NEW: Baseline warning count
└── tests/
    └── warning_budget_tests.rs  # NEW: Integration tests

.moon/
├── tasks.yml            # CONSIDER: Add baseline file tracking
└── ...

.root
├── .clippy-baseline     # NEW: Baseline warning count
└── ...
```

---

## Implementation Order

### Phase 1: Domain Types and Core Logic

#### 1.1 Create `release-gate/src/warning_budget.rs`

**Purpose:** New module for warning budget logic

**Functions to implement:**

```rust
/// A non-negative count of clippy warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WarningCount(u32);

impl WarningCount {
    pub fn new(count: u32) -> Self;
    pub fn value(self) -> u32;
    pub fn from_clippy_output(output: &str) -> Self;
    pub fn from_baseline(content: &str) -> Result<Self, BudgetError>;
}

/// Result of comparing current warning count to baseline budget
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetCheckResult {
    Passed { current: WarningCount, baseline: WarningCount },
    Exceeded { current: WarningCount, baseline: WarningCount, delta: u32 },
}

impl BudgetCheckResult {
    pub fn is_passed(&self) -> bool;
}

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

/// Load the warning budget baseline from file
pub fn load_baseline(path: &Path) -> Result<WarningCount, BudgetError>;

/// Run clippy and capture warning count from output
pub fn run_clippy_and_count_warnings() -> Result<WarningCount, BudgetError>;

/// Compare current warnings to baseline budget
pub fn check_budget(current: WarningCount, baseline: WarningCount) -> BudgetCheckResult;

/// Execute full budget check workflow
pub fn execute_budget_check(baseline_path: &Path) -> Result<BudgetCheckResult, BudgetError>;
```

**Location:** `release-gate/src/warning_budget.rs`

---

### Phase 2: Integration with Release Gate

#### 2.1 Modify `release-gate/src/domain.rs`

**Add new types:**

```rust
/// Result of the entire gate check (MODIFIED to include budget)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateResult {
    pub p0_check: P0CheckResult,
    pub ci_passed: bool,
    pub budget_check: BudgetCheckResult,  // NEW FIELD
}
```

#### 2.2 Modify `release-gate/src/gate.rs`

**Add to `ReleaseGate::execute()`:**

```rust
/// Execute the release gate checks (MODIFIED)
pub fn execute(&self) -> Result<GateResult, BeadError> {
    // Check 1: P0 beads must be closed
    let p0_result = self.check_p0_beads()?;

    // Fail fast if P0 beads are open
    if let P0CheckResult::Failed(_) = p0_result {
        return Ok(GateResult {
            p0_check: p0_result,
            ci_passed: false,
            budget_check: BudgetCheckResult::Passed { current: WarningCount(0), baseline: WarningCount(0) },
        });
    }

    // Check 2: Moon CI must pass
    let ci_passed = run_moon_ci()?;

    // Check 3: Warning budget must not be exceeded
    let baseline_path = PathBuf::from(".clippy-baseline");
    let budget_check = execute_budget_check(&baseline_path)
        .unwrap_or(BudgetCheckResult::Passed { current: WarningCount(0), baseline: WarningCount(0) });

    Ok(GateResult {
        p0_check: p0_result,
        ci_passed,
        budget_check,
    })
}
```

#### 2.3 Modify `release-gate/src/main.rs`

**Add exit code constant:**

```rust
const EXIT_WARNING_BUDGET_EXCEEDED: i32 = 4;
```

**Modify exit logic:**

```rust
match result {
    Ok(gate_result) => {
        if let P0CheckResult::Failed(_) = gate_result.p0_check {
            eprintln!("Release blocked: P0 beads are open");
            std::process::exit(EXIT_P0_BLOCKED);
        }
        
        if !gate_result.ci_passed {
            eprintln!("Release blocked: CI failed");
            std::process::exit(EXIT_CI_FAILED);
        }
        
        if let BudgetCheckResult::Exceeded { current, baseline, delta } = gate_result.budget_check {
            eprintln!(
                "Release blocked: Warning budget exceeded (current: {}, baseline: {}, delta: {})",
                current.value(),
                baseline.value(),
                delta
            );
            std::process::exit(EXIT_WARNING_BUDGET_EXCEEDED);
        }
        
        println!("✅ Release gate passed - all checks OK");
        std::process::exit(EXIT_GATE_PASSED);
    }
    Err(e) => {
        eprintln!("Release gate error: {}", e);
        std::process::exit(EXIT_ERROR);
    }
}
```

---

### Phase 3: Baseline File Creation

#### 3.1 Create `.clippy-baseline`

**Location:** Repository root: `.clippy-baseline`

**Initial content:** Determine current warning count:

```bash
cargo clippy --workspace --all-targets --all-features 2>&1 | grep -c "warning:"
```

**Example content:**
```
5
```

---

### Phase 4: Tests

#### 4.1 Create `release-gate/tests/warning_budget_tests.rs`

**Based on:** `martin-fowler-tests.md`

**Test file location:** `release-gate/tests/warning_budget_tests.rs`

---

### Phase 5: Moon Configuration (Optional)

#### 5.1 Modify `.moon/tasks.yml`

**Consider adding:** Baseline file to task inputs for cache invalidation:

```yaml
tasks:
  release-gate:
    inputs:
      - ".beads/**"
      - ".clippy-baseline"  # NEW: Track baseline changes
```

---

## Function Implementation Checklist

| Function | File | Line Target | Dependencies |
|----------|------|-------------|--------------|
| `WarningCount::new()` | warning_budget.rs | ~15 | None |
| `WarningCount::value()` | warning_budget.rs | ~20 | None |
| `WarningCount::from_clippy_output()` | warning_budget.rs | ~25 | None |
| `WarningCount::from_baseline()` | warning_budget.rs | ~35 | None |
| `BudgetCheckResult::is_passed()` | warning_budget.rs | ~70 | None |
| `load_baseline()` | warning_budget.rs | ~80 | std::fs |
| `run_clippy_and_count_warnings()` | warning_budget.rs | ~95 | std::process::Command |
| `check_budget()` | warning_budget.rs | ~120 | None |
| `execute_budget_check()` | warning_budget.rs | ~130 | load_baseline, run_clippy_and_count_warnings, check_budget |
| Modify `GateResult` | domain.rs | ~55 | None |
| Modify `ReleaseGate::execute()` | gate.rs | ~40 | execute_budget_check |
| Modify `main.rs` | main.rs | ~40 | BudgetCheckResult |

---

## Dependencies to Add

### `release-gate/Cargo.toml`

```toml
[dependencies]
# Existing
itertools = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tap = "1.0"
thiserror = "2.0"

# New - none required (using built-in string parsing)
# The implementation uses only std::fs and std::process::Command
```

---

## Integration Points

| Integration Point | What to Connect |
|-------------------|-----------------|
| Release Gate | Import `warning_budget` module in `gate.rs` |
| Exit Codes | Add `EXIT_WARNING_BUDGET_EXCEEDED = 4` in `main.rs` |
| Baseline File | Read from `.clippy-baseline` in repository root |
| CI Pipeline | Moon task runs release-gate binary |

---

## Where to Look for Patterns

| Pattern | Location | Reference |
|---------|----------|-----------|
| Domain types | `release-gate/src/domain.rs` | `P0CheckResult`, `BeadStatus` |
| Error handling | `release-gate/src/gate.rs` | `BeadError` enum |
| Command execution | `release-gate/src/gate.rs` | `run_moon_ci()` |
| Exit code logic | `release-gate/src/main.rs` | Existing exit constants |
| Test organization | `release-gate/src/gate.rs` | N/A (no existing tests) |
