---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:45:40Z
---

# Implementation Map

## File Structure

```
release-gate/
├── src/
│   ├── main.rs           # MODIFY: Add exit code handling
│   ├── domain.rs        # MODIFY: Add budget_check to GateResult
│   ├── gate.rs          # MODIFY: Add budget check to execute()
│   └── warning_budget.rs # NEW: Budget logic module
├── Cargo.toml           # NO CHANGE: All deps in std
├── .clippy-baseline     # NEW: Baseline warning count (at repo root)
└── tests/
    └── warning_budget_tests.rs  # NEW: Integration tests

.moon/
└── tasks.yml            # CONSIDER: Add baseline file tracking

Repository Root/
└── .clippy-baseline     # NEW: Single integer baseline count
```

---

## Implementation Order

### Phase 1: Domain Types and Pure Functions

#### 1.1 Create `release-gate/src/warning_budget.rs`

**Purpose:** New module for warning budget logic

**Exact signatures:**

```rust
//! Warning budget enforcement module

use std::path::Path;
use std::process::Command;

/// A non-negative count of clippy warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WarningCount(u32);

impl WarningCount {
    /// Create a new WarningCount
    #[must_use]
    pub fn new(count: u32) -> Self {
        Self(count)
    }
    
    /// Get the raw count value
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
    
    /// Create from clippy output string (counts "warning:" occurrences)
    #[must_use]
    pub fn from_clippy_output(output: &str) -> Self {
        let count = output
            .lines()
            .filter(|line| line.contains("warning:"))
            .count() as u32;
        Self(count)
    }
    
    /// Parse from baseline file content
    pub fn from_baseline(content: &str) -> Result<Self, BudgetError> {
        let line = content.lines().next().ok_or(BudgetError::EmptyBaseline)?;
        let trimmed = line.trim();
        let count: u32 = trimmed
            .parse()
            .map_err(|_| BudgetError::InvalidBaseline {
                content: trimmed.to_string(),
            })?;
        Ok(Self(count))
    }
}

/// Result of comparing current warning count to baseline budget
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetCheckResult {
    /// Current warnings <= baseline - release is clear
    Passed { current: WarningCount, baseline: WarningCount },
    /// Current warnings > baseline - release blocked
    Exceeded { current: WarningCount, baseline: WarningCount, delta: u32 },
}

impl BudgetCheckResult {
    /// Returns true if budget check passed
    #[must_use]
    pub fn is_passed(&self) -> bool {
        matches!(self, Self::Passed { .. })
    }
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
pub fn load_baseline(path: &Path) -> Result<WarningCount, BudgetError> {
    let content = std::fs::read_to_string(path).map_err(|e| BudgetError::BaselineRead {
        message: format!("Failed to read {}: {e}", path.display()),
    })?;
    WarningCount::from_baseline(&content)
}

/// Run clippy and capture warning count from output
pub fn run_clippy_and_count_warnings() -> Result<WarningCount, BudgetError> {
    let output = Command::new("cargo")
        .args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
        ])
        .output()
        .map_err(|e| BudgetError::ClippyExecution {
            message: format!("Failed to spawn clippy: {e}"),
        })?;
    
    // Combine stdout and stderr (clippy warnings go to stderr)
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    
    Ok(WarningCount::from_clippy_output(&combined))
}

/// Compare current warnings to baseline budget
#[must_use]
pub fn check_budget(current: WarningCount, baseline: WarningCount) -> BudgetCheckResult {
    if current.value() <= baseline.value() {
        BudgetCheckResult::Passed { current, baseline }
    } else {
        BudgetCheckResult::Exceeded {
            current,
            baseline,
            delta: current.value() - baseline.value(),
        }
    }
}

/// Execute full budget check workflow
pub fn execute_budget_check(baseline_path: &Path) -> Result<BudgetCheckResult, BudgetError> {
    let baseline = load_baseline(baseline_path)?;
    let current = run_clippy_and_count_warnings()?;
    Ok(check_budget(current, baseline))
}
```

**Tests to write first (TDD):** See `martin-fowler-tests.md`

**Location:** `release-gate/src/warning_budget.rs`

---

### Phase 2: Integration with Release Gate

#### 2.1 Modify `release-gate/src/domain.rs`

**Add import:**
```rust
use crate::warning_budget::BudgetCheckResult;
```

**Modify struct:**
```rust
/// Result of the entire gate check
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateResult {
    /// Result of the P0 beads check
    pub p0_check: P0CheckResult,
    /// Whether CI passed
    pub ci_passed: bool,
    /// Result of the warning budget check
    pub budget_check: BudgetCheckResult,
}
```

#### 2.2 Modify `release-gate/src/gate.rs`

**Add imports:**
```rust
use std::path::PathBuf;

use crate::warning_budget::{execute_budget_check, BudgetCheckResult, WarningCount};
```

**Add to `BeadError` enum:**
```rust
#[error("Budget check error: {source}")]
BudgetCheck { source: crate::warning_budget::BudgetError },
```

**Modify `ReleaseGate::execute()`:**
```rust
pub fn execute(&self) -> Result<GateResult, BeadError> {
    // Check 1: P0 beads must be closed
    let p0_result = self.check_p0_beads()?;

    // Fail fast if P0 beads are open
    if let P0CheckResult::Failed(_) = p0_result {
        return Ok(GateResult {
            p0_check: p0_result,
            ci_passed: false,
            budget_check: BudgetCheckResult::Passed {
                current: WarningCount::new(0),
                baseline: WarningCount::new(0),
            },
        });
    }

    // Check 2: Moon CI must pass
    let ci_passed = run_moon_ci()?;

    // Check 3: Warning budget must not be exceeded
    let baseline_path = PathBuf::from(".clippy-baseline");
    let budget_check = execute_budget_check(&baseline_path)
        .map_err(|e| BeadError::BudgetCheck { source: e })?;

    Ok(GateResult {
        p0_check: p0_result,
        ci_passed,
        budget_check,
    })
}
```

#### 2.3 Modify `release-gate/src/main.rs`

**Add import:**
```rust
use crate::warning_budget::BudgetCheckResult;
```

**Add constant:**
```rust
const EXIT_WARNING_BUDGET_EXCEEDED: i32 = 4;
```

**Modify main() match arm:**
```rust
fn main() {
    let result = run_gate();
    let exit_code = match result {
        Ok(GateResult {
            p0_check: P0CheckResult::Passed,
            ci_passed: true,
            budget_check: BudgetCheckResult::Passed { .. },
        }) => {
            println!("✅ Release gate PASSED - all checks passed");
            EXIT_GATE_PASSED
        }
        Ok(GateResult {
            budget_check: BudgetCheckResult::Exceeded { current, baseline, delta },
            ..
        }) => {
            eprintln!("❌ Release gate FAILED - Warning budget exceeded");
            eprintln!();
            eprintln!("Current warnings: {}", current.value());
            eprintln!("Baseline budget:  {}", baseline.value());
            eprintln!("Delta:            +{}", delta);
            eprintln!();
            eprintln!("Fix warnings or update .clippy-baseline before releasing.");
            EXIT_WARNING_BUDGET_EXCEEDED
        }
        Ok(GateResult { p0_check, .. }) => {
            // ... existing P0 handling ...
        }
        Err(BeadError::CiFailed { .. }) => {
            // ... existing CI handling ...
        }
        // ... other error handlers ...
    };
    std::process::exit(exit_code);
}
```

---

### Phase 3: Baseline File Creation

#### 3.1 Create `.clippy-baseline`

**Location:** Repository root (same level as `Cargo.toml`)

**Determine current count:**
```bash
cargo clippy --workspace --all-targets --all-features 2>&1 | grep -c "warning:"
```

**Initial content (example):**
```
0
```

**Commit this file to version control.**

---

### Phase 4: Module Registration

#### 4.1 Add module declaration

In `release-gate/src/main.rs` or create `release-gate/src/lib.rs`:

```rust
mod warning_budget;
```

---

### Phase 5: Moon Configuration (Optional)

#### 5.1 Modify `.moon/tasks.yml`

Add baseline file to inputs for cache invalidation:

```yaml
tasks:
  release-gate:
    command: "sh -c 'cd . && cargo run --release -p release-gate'"
    description: "Run release go-no-go gate"
    inputs: 
      - ".beads/issues.jsonl"
      - "release-gate/**"
      - ".clippy-baseline"  # NEW: Track baseline changes
    options:
      cache: false
```

---

## Function Implementation Checklist

| Function | File | Type | Dependencies |
|----------|------|------|--------------|
| `WarningCount::new()` | warning_budget.rs | NEW | None |
| `WarningCount::value()` | warning_budget.rs | NEW | None |
| `WarningCount::from_clippy_output()` | warning_budget.rs | NEW | None |
| `WarningCount::from_baseline()` | warning_budget.rs | NEW | None |
| `BudgetCheckResult::is_passed()` | warning_budget.rs | NEW | None |
| `load_baseline()` | warning_budget.rs | NEW | std::fs |
| `run_clippy_and_count_warnings()` | warning_budget.rs | NEW | std::process::Command |
| `check_budget()` | warning_budget.rs | NEW | None |
| `execute_budget_check()` | warning_budget.rs | NEW | All above |
| `GateResult.budget_check` | domain.rs | MODIFY | None |
| `ReleaseGate::execute()` | gate.rs | MODIFY | execute_budget_check |
| `main()` exit handling | main.rs | MODIFY | None |

---

## Dependencies

### `release-gate/Cargo.toml`

**No new dependencies required.** All functionality uses:
- `std::fs` - file reading
- `std::process::Command` - clippy execution
- `thiserror` - already present

---

## Integration Points

| Integration Point | What to Connect |
|-------------------|-----------------|
| Module system | `mod warning_budget;` in main.rs or lib.rs |
| Domain types | Import `BudgetCheckResult` in domain.rs |
| Gate logic | Import and call `execute_budget_check()` |
| Exit codes | Add `EXIT_WARNING_BUDGET_EXCEEDED = 4` |
| Baseline file | Read from `.clippy-baseline` at repo root |

---

## Where to Look for Patterns

| Pattern | File | Lines |
|---------|------|-------|
| Domain types (sum types) | `domain.rs` | 17-52 |
| Error enum with thiserror | `gate.rs` | 11-25 |
| Command execution | `gate.rs` | 108-141 |
| File reading | `gate.rs` | 78-98 |
| Exit code constants | `main.rs` | 27-30 |
| Match on GateResult | `main.rs` | 34-78 |

---

## Verification Commands

```bash
# Phase 1: Test domain types
moon run :test -p release-gate

# Phase 2: Test integration
moon run :build -p release-gate

# Phase 3: Full CI
moon run :ci

# Phase 4: Manual verification
./target/release/release-gate
echo $?  # Should be 0 if all checks pass

# Verify exit code 4 on budget exceeded
# (Temporarily modify baseline to -1, run, verify exit code)
```

---

## Test File

**Location:** `release-gate/tests/warning_budget_tests.rs`

See `martin-fowler-tests.md` for complete test specifications.
