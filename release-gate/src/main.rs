#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Release Go-No-Go Gate
//!
//! This binary enforces production release gates by checking:
//! 1. No open P0 beads exist
//! 2. Moon CI passes
//! 3. Clippy warning budget not exceeded
//!
//! Exit codes:
//! - 0: All gates passed, release is clear
//! - 1: P0 beads are open (release blocked)
//! - 2: Moon CI failed (release blocked)
//! - 3: Precondition failed (br or moon not available)
//! - 4: Warning budget exceeded

mod domain;
mod gate;
mod warning_budget;

use std::process::Command;

use crate::domain::{GateResult, P0CheckResult};
use crate::gate::{BeadError, ReleaseGate};
use crate::warning_budget::BudgetCheckResult;

const EXIT_GATE_PASSED: i32 = 0;
const EXIT_P0_BLOCKED: i32 = 1;
const EXIT_CI_FAILED: i32 = 2;
const EXIT_PRECONDITION_FAILED: i32 = 3;
const EXIT_BUDGET_EXCEEDED: i32 = 4;

fn main() {
    let result = run_gate();
    let exit_code = match result {
        Ok(GateResult {
            p0_check: P0CheckResult::Passed,
            ci_passed: true,
            budget_check,
        }) => {
            if budget_check.passed() {
                println!("✅ Release gate PASSED - all checks passed");
                EXIT_GATE_PASSED
            } else {
                eprintln!("❌ Release gate FAILED - Warning budget exceeded release");
                EXIT_BUDGET_EXCEEDED
            }
        }
        Ok(GateResult {
            p0_check: _,
            budget_check: BudgetCheckResult::Exceeded { .. },
            ..
        }) => {
            eprintln!("❌ Release gate FAILED - Warning budget exceeded release");
            EXIT_BUDGET_EXCEEDED
        }
        Ok(GateResult { p0_check, .. }) => {
            eprintln!("❌ Release gate FAILED - P0 beads blocked release");
            if let P0CheckResult::Failed(beads) = p0_check {
                eprintln!();
                eprintln!("Open P0 beads:");
                for b in &beads {
                    eprintln!("  - {}: {}", b.id, b.title);
                }
                eprintln!();
                eprintln!("Close all P0 beads before releasing.");
            }
            EXIT_P0_BLOCKED
        }
        Err(BeadError::CiFailed { .. }) => {
            eprintln!("❌ Release gate FAILED - CI checks failed");
            eprintln!();
            eprintln!("Fix CI failures before releasing.");
            EXIT_CI_FAILED
        }
        Err(BeadError::BudgetFailed { message }) => {
            eprintln!("❌ Release gate BUDGET CHECK FAILED: {message}");
            EXIT_PRECONDITION_FAILED
        }
        Err(BeadError::Precondition { tool, message }) => {
            eprintln!("❌ Release gate PRECONDITION FAILED");
            eprintln!();
            eprintln!("Tool '{tool}' not available: {message}");
            eprintln!();
            eprintln!("Ensure required tools are installed before running release gate.");
            EXIT_PRECONDITION_FAILED
        }
        Err(BeadError::Io { message }) => {
            eprintln!("❌ Release gate ERROR: {message}");
            EXIT_PRECONDITION_FAILED
        }
        Err(BeadError::Parse { message }) => {
            eprintln!("❌ Release gate PARSE ERROR: {message}");
            EXIT_PRECONDITION_FAILED
        }
    };
    std::process::exit(exit_code);
}

fn run_gate() -> Result<GateResult, BeadError> {
    check_preconditions()?;

    let gate = ReleaseGate::new();
    gate.execute()
}

fn check_preconditions() -> Result<(), BeadError> {
    let br_check = Command::new("br").arg("--version").output();
    let moon_check = Command::new("moon").arg("--version").output();

    let br_available = br_check.is_ok_and(|o| o.status.success());
    let moon_available = moon_check.is_ok_and(|o| o.status.success());

    if !br_available {
        return Err(BeadError::Precondition {
            tool: "br".to_string(),
            message: "br command not found in PATH".to_string(),
        });
    }

    if !moon_available {
        return Err(BeadError::Precondition {
            tool: "moon".to_string(),
            message: "moon command not found in PATH".to_string(),
        });
    }

    Ok(())
}
