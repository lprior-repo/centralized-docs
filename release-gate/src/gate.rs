//! Release gate implementation

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

use crate::domain::{Bead, BeadStatus, GateResult, P0CheckResult};
use crate::warning_budget::{
    check_budget, read_baseline, run_clippy_count, BudgetCheckResult, WarningCount,
};

/// Error types for bead operations
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum BeadError {
    #[error("IO error: {message}")]
    Io { message: String },

    #[error("Parse error: {message}")]
    Parse { message: String },

    #[error("Precondition failed: tool '{tool}' - {message}")]
    Precondition { tool: String, message: String },

    #[error("CI failed: {message}")]
    CiFailed { message: String },

    #[error("Budget check failed: {message}")]
    BudgetFailed { message: String },
}

pub struct ReleaseGate {
    /// Path to the beads issues file
    issues_path: PathBuf,
}

impl ReleaseGate {
    /// Create a new release gate
    #[must_use]
    pub fn new() -> Self {
        let issues_path = PathBuf::from(".beads").join("issues.jsonl");
        Self { issues_path }
    }

    /// Execute the release gate checks
    pub fn execute(&self) -> Result<GateResult, BeadError> {
        // Check 1: P0 beads must be closed
        let p0_result = self.check_p0_beads()?;

        // Fail fast if P0 beads are open
        if let P0CheckResult::Failed(_) = p0_result {
            return Ok(GateResult {
                p0_check: p0_result,
                ci_passed: false,
                budget_check: BudgetCheckResult::Passed {
                    current: WarningCount::default(),
                    baseline: WarningCount::default(),
                },
            });
        }

        // Check 2: Moon CI must pass
        let ci_passed = run_moon_ci()?;

        // Check 3: Warning budget must not be exceeded
        let budget_check = run_budget_check()?;

        Ok(GateResult {
            p0_check: p0_result,
            ci_passed,
            budget_check,
        })
    }

    /// Check for open P0 beads
    fn check_p0_beads(&self) -> Result<P0CheckResult, BeadError> {
        let beads = self.read_beads()?;
        let p0_beads: Vec<Bead> = beads
            .into_iter()
            .filter(|b| b.priority == 0 && b.status.is_open())
            .collect();

        if p0_beads.is_empty() {
            Ok(P0CheckResult::Passed)
        } else {
            Ok(P0CheckResult::Failed(p0_beads))
        }
    }

    /// Read and parse all beads from the issues file
    fn read_beads(&self) -> Result<Vec<Bead>, BeadError> {
        let file = File::open(&self.issues_path).map_err(|e| BeadError::Io {
            message: format!("Failed to open beads issues file: {e}"),
        })?;

        let reader = BufReader::new(file);

        let beads: Vec<Bead> = reader
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                let line = line.ok()?;
                if line.trim().is_empty() {
                    return None;
                }
                parse_bead_line(idx + 1, &line).ok()
            })
            .collect();

        Ok(beads)
    }
}

impl Default for ReleaseGate {
    fn default() -> Self {
        Self::new()
    }
}

/// Run Moon CI and return whether it passed
fn run_moon_ci() -> Result<bool, BeadError> {
    println!("Running Moon CI checks...");
    println!();

    let output = Command::new("moon")
        .args(["run", ":ci"])
        .output()
        .map_err(|e| BeadError::Io {
            message: format!("Failed to run moon CI: {e}"),
        })?;

    let success = output.status.success();

    // Print CI output for visibility
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        print!("{stdout}");
    }
    if !stderr.is_empty() {
        eprint!("{stderr}");
    }

    if success {
        println!();
        println!("✅ Moon CI passed");
    } else {
        println!();
        eprintln!("❌ Moon CI failed");
    }

    Ok(success)
}

/// Run the warning budget check
fn run_budget_check() -> Result<BudgetCheckResult, BeadError> {
    println!("Running warning budget check...");
    println!();

    // Read baseline
    let baseline_path = PathBuf::from(".clippy-baseline");
    let baseline = read_baseline(&baseline_path).map_err(|e| BeadError::BudgetFailed {
        message: e.to_string(),
    })?;

    // Run clippy and count warnings
    let current = run_clippy_count().map_err(|e| BeadError::BudgetFailed {
        message: e.to_string(),
    })?;

    // Compare against baseline
    let result = check_budget(current, baseline);

    match &result {
        BudgetCheckResult::Passed { current, baseline } => {
            println!();
            println!(
                "✅ Warning budget check PASSED - {} warnings (baseline: {})",
                current.get(),
                baseline.get()
            );
        }
        BudgetCheckResult::Exceeded {
            current,
            baseline,
            delta,
        } => {
            println!();
            eprintln!(
                "❌ Warning budget EXCEEDED - {} warnings (baseline: {}, +{})",
                current.get(),
                baseline.get(),
                delta
            );
        }
    }

    Ok(result)
}

/// Parse a single line from the beads issues JSONL file
fn parse_bead_line(line_number: usize, line: &str) -> Result<Bead, BeadError> {
    let json: serde_json::Value = serde_json::from_str(line).map_err(|e| BeadError::Parse {
        message: format!("Line {line_number}: JSON parse error: {e}"),
    })?;

    let id = json
        .get("id")
        .and_then(serde_json::Value::as_str)
        .map(String::from)
        .ok_or_else(|| BeadError::Parse {
            message: format!("Line {line_number}: Missing 'id' field"),
        })?;

    let title = json
        .get("title")
        .and_then(serde_json::Value::as_str)
        .map_or("(no title)".to_string(), String::from);

    let status = json
        .get("status")
        .and_then(serde_json::Value::as_str)
        .map(BeadStatus::from_str)
        .unwrap_or_default();

    let priority = json
        .get("priority")
        .and_then(serde_json::Value::as_u64)
        .and_then(|p| u8::try_from(p).ok())
        .unwrap_or(4);

    Ok(Bead {
        id,
        title,
        status,
        priority,
    })
}
