//! Clippy warning budget module
//!
//! Tracks clippy warning count as a quality metric and prevents warning regressions
//! in release validation.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::path::Path;
use std::process::Command;

use thiserror::Error;

/// Newtype for clippy warning count
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct WarningCount(u32);

impl WarningCount {
    /// Create a new `WarningCount` from a raw u32
    #[must_use]
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// Get the inner value
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Create from clippy output string (counts "warning:" occurrences)
    ///
    /// This is a pure function for the functional core.
    #[must_use]
    pub fn from_clippy_output(output: &str) -> Self {
        let count = output
            .lines()
            .filter(|line| strip_ansi_escape_codes(line).contains("warning:"))
            .count();
        Self(u32::try_from(count).unwrap_or(0))
    }

    /// Parse from baseline file content (first line as integer)
    ///
    /// This is a pure function for the functional core.
    ///
    /// # Errors
    /// Returns an error if the content is empty or not a valid non-negative integer
    pub fn from_baseline(content: &str) -> Result<Self, BudgetError> {
        let line = content.lines().next().ok_or(BudgetError::EmptyBaseline)?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return Err(BudgetError::EmptyBaseline);
        }

        trimmed
            .parse::<u32>()
            .map(Self)
            .map_err(|_| BudgetError::InvalidBaseline {
                content: trimmed.to_string(),
            })
    }
}

#[allow(clippy::while_let_on_iterator)]
fn strip_ansi_escape_codes(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                let _ = chars.next();
                while let Some(next) = chars.next() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
            }
        } else {
            output.push(ch);
        }
    }

    output
}

/// Result of comparing current warnings against baseline
/// Result of comparing current warnings against baseline
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetCheckResult {
    /// Warning count is within budget
    Passed {
        /// Current warning count
        current: WarningCount,
        /// Baseline warning count
        baseline: WarningCount,
    },
    /// Warning count exceeds baseline
    Exceeded {
        /// Current warning count
        current: WarningCount,
        /// Baseline warning count
        baseline: WarningCount,
        /// Delta (current - baseline)
        delta: u32,
    },
}

impl BudgetCheckResult {
    /// Returns true if the check passed (within budget)
    #[must_use]
    pub const fn passed(&self) -> bool {
        matches!(self, Self::Passed { .. })
    }
}

/// Errors that can occur during budget check
#[derive(Debug, Error)]
pub enum BudgetError {
    /// Baseline file is empty
    #[error("baseline file is empty")]
    EmptyBaseline,

    /// Invalid baseline content (not a non-negative integer)
    #[error("invalid baseline content: '{content}' - expected non-negative integer")]
    InvalidBaseline { content: String },

    /// Failed to read baseline file
    #[error("failed to read baseline file: {message}")]
    BaselineRead { message: String },

    /// Failed to run clippy
    #[error("failed to run clippy: {message}")]
    ClippyExecution { message: String },

    /// Clippy process failed with non-zero exit code
    #[error("clippy process failed with exit code {code}")]
    ClippyFailed { code: i32 },
}

/// Read the baseline warning count from the baseline file
///
/// # Errors
/// Returns an error if the file cannot be read or contains invalid content
pub fn read_baseline(path: &Path) -> Result<WarningCount, BudgetError> {
    let content = std::fs::read_to_string(path).map_err(|e| BudgetError::BaselineRead {
        message: format!("Failed to read {}: {e}", path.display()),
    })?;
    WarningCount::from_baseline(&content)
}

/// Run clippy on the workspace and count warnings
///
/// # Errors
/// Returns an error if clippy cannot be executed
pub fn run_clippy_count() -> Result<WarningCount, BudgetError> {
    let output = Command::new("cargo")
        .args(["clippy", "--workspace", "--", "-D", "warnings"])
        .output()
        .map_err(|e| BudgetError::ClippyExecution {
            message: format!("Failed to execute clippy: {e}"),
        })?;

    // Clippy returns exit code 101 for warnings (when using -D warnings)
    // Exit code 0 means no warnings
    // Any other exit code is an error
    let exit_code = output.status.code().unwrap_or(-1);

    // Exit code 0 = no warnings (success)
    // Exit code 101 = warnings found (but still compilation succeeded)
    // Other codes = actual errors
    if exit_code == 0 {
        Ok(WarningCount::new(0))
    } else if exit_code == 101 {
        // Count warnings from output using pure function
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{stdout}{stderr}");

        Ok(WarningCount::from_clippy_output(&combined))
    } else {
        Err(BudgetError::ClippyFailed { code: exit_code })
    }
}

/// Alternative: Run clippy and count all warning lines (including duplicates)
///
/// This is more accurate for baseline tracking since it counts total warnings
/// including duplicates shown in output
///
/// # Errors
/// Returns an error if clippy cannot be executed
#[allow(dead_code)]
pub fn run_clippy_count_total() -> Result<WarningCount, BudgetError> {
    let output = Command::new("cargo")
        .args(["clippy", "--workspace"])
        .output()
        .map_err(|e| BudgetError::ClippyExecution {
            message: format!("Failed to execute clippy: {e}"),
        })?;

    let _exit_code = output.status.code().unwrap_or(-1);

    // If exit code is 0, there might still be warnings shown (but they were allowed)
    // If non-zero, there could be warnings or errors
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // Count all lines containing "warning:" to get total warnings shown
    // This includes duplicates shown in summaries
    let warning_count = u32::try_from(
        combined
            .lines()
            .filter(|line| line.contains("warning:"))
            .count(),
    )
    .unwrap_or(0);

    // If there are warnings but exit code is 0, they're allowed warnings
    // We still track them for baseline purposes
    Ok(WarningCount::new(warning_count))
}

/// Compare current warning count against baseline
#[must_use]
pub fn check_budget(current: WarningCount, baseline: WarningCount) -> BudgetCheckResult {
    if current.0 <= baseline.0 {
        BudgetCheckResult::Passed { current, baseline }
    } else {
        BudgetCheckResult::Exceeded {
            current,
            baseline,
            delta: current.0 - baseline.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;

    // ========================================================================
    // WarningCount tests
    // ========================================================================

    #[test]
    fn test_warning_count_new() {
        let count = WarningCount::new(42);
        assert_eq!(count.get(), 42);
    }

    #[test]
    fn test_warning_count_default() {
        let count = WarningCount::default();
        assert_eq!(count.get(), 0);
    }

    #[test]
    fn test_warning_count_ordering() {
        let low = WarningCount::new(5);
        let high = WarningCount::new(10);
        assert!(low < high);
        assert!(high > low);
    }

    // ========================================================================
    // from_clippy_output tests (pure function)
    // ========================================================================

    #[test]
    fn test_from_clippy_output_counts_correctly() {
        // Given: clippy output with 3 warnings
        let output = "warning: unused variable: `x`
warning: this function is too long
error: something bad
warning: unnecessary cast";

        // When: parsing warning count
        let count = WarningCount::from_clippy_output(output);

        // Then: count is 3
        assert_eq!(count.get(), 3);
    }

    #[test]
    fn test_from_clippy_output_handles_empty() {
        // Given: empty clippy output
        let output = "";

        // When: parsing warning count
        let count = WarningCount::from_clippy_output(output);

        // Then: count is 0
        assert_eq!(count.get(), 0);
    }

    #[test]
    fn test_from_clippy_output_handles_no_warnings() {
        // Given: clippy output with no warnings (only info)
        let output = "Checking crate1 v0.1.0
Checking crate2 v0.1.0
Finished dev [unoptimized + debuginfo]";

        // When: parsing warning count
        let count = WarningCount::from_clippy_output(output);

        // Then: count is 0
        assert_eq!(count.get(), 0);
    }

    #[test]
    fn test_from_clippy_output_counts_all_warning_lines() {
        // Given: output with "warning:" in different contexts
        let output = "some warning: text here
warning: actual warning
WARNING: uppercase also counted
warning:another warning";

        // When: parsing warning count (case-sensitive for contains)
        let count = WarningCount::from_clippy_output(output);

        // Then: counts all lines containing "warning:" (case-sensitive)
        // "some warning:" - counted (contains "warning:")
        // "warning: actual" - counted
        // "WARNING:" - not counted (case-sensitive, no lowercase "warning:")
        // "warning:another" - counted
        assert_eq!(count.get(), 3);
    }

    #[test]
    fn test_from_clippy_output_handles_ansi_color_codes() {
        let output = "\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m: unused variable\u{1b}[0m\n\
                      \u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m: dead code\u{1b}[0m\n\
                      error: compile failed";

        let count = WarningCount::from_clippy_output(output);

        assert_eq!(count.get(), 2);
    }

    // ========================================================================
    // from_baseline tests (pure function)
    // ========================================================================

    #[test]
    fn test_from_baseline_parses_valid_integer() {
        // Given: baseline file content with "5"
        let content = "5";

        // When: parsing baseline
        let result = WarningCount::from_baseline(content);

        // Then: returns WarningCount(5)
        assert!(result.is_ok());
        if let Ok(count) = result {
            assert_eq!(count.get(), 5);
        }
    }

    #[test]
    fn test_from_baseline_trims_whitespace() {
        // Given: baseline content with whitespace
        let test_cases: Vec<(&str, u32)> =
            vec![("  5  ", 5), ("5\n", 5), ("\t5\t", 5), ("  10  \n", 10)];

        for (content, expected) in test_cases {
            // When: parsing baseline
            let result = WarningCount::from_baseline(content);

            // Then: parses successfully
            assert!(result.is_ok(), "Failed for content: '{content}'");
            if let Ok(count) = result {
                assert_eq!(count.get(), expected);
            }
        }
    }

    #[test]
    fn test_from_baseline_empty_returns_error() {
        // Given: empty baseline content
        let content = "";

        // When: parsing baseline
        let result = WarningCount::from_baseline(content);

        // Then: returns EmptyBaseline error
        assert!(matches!(result, Err(BudgetError::EmptyBaseline)));
    }

    #[test]
    fn test_from_baseline_invalid_content_returns_error() {
        // Given: baseline content that is not a valid integer
        let test_cases: Vec<&str> = vec!["not-a-number", "-5", "3.14", "abc123"];

        for content in test_cases {
            // When: parsing baseline
            let result = WarningCount::from_baseline(content);

            // Then: returns InvalidBaseline error
            assert!(
                matches!(result, Err(BudgetError::InvalidBaseline { .. })),
                "Expected error for content: '{content}'"
            );
        }
    }

    #[test]
    fn test_from_baseline_large_value() {
        // Given: baseline with large value
        let content = "4294967295"; // u32::MAX

        // When: parsing baseline
        let result = WarningCount::from_baseline(content);

        // Then: parses successfully
        assert!(result.is_ok());
        if let Ok(count) = result {
            assert_eq!(count.get(), u32::MAX);
        }
    }

    #[test]
    fn test_from_baseline_whitespace_only_returns_error() {
        // Given: baseline content with only whitespace on first line
        let content = "   \n5";

        // When: parsing baseline
        let result = WarningCount::from_baseline(content);

        // Then: returns EmptyBaseline error (trimmed line is empty)
        assert!(matches!(result, Err(BudgetError::EmptyBaseline)));
    }

    // ========================================================================
    // read_baseline tests (I/O shell function)
    // ========================================================================

    #[test]
    fn test_read_baseline_file_not_found_returns_error() {
        // Given: path to non-existent file
        let path = PathBuf::from("/nonexistent/path/.clippy-baseline-12345");

        // When: loading baseline
        let result = read_baseline(&path);

        // Then: returns BaselineRead error
        assert!(matches!(result, Err(BudgetError::BaselineRead { .. })));
    }

    #[test]
    fn test_read_baseline_valid_file() {
        // Given: temp file with valid content
        let temp_dir = tempfile::tempdir().ok();
        if let Some(dir) = temp_dir {
            let path = dir.path().join(".clippy-baseline");
            let mut file = std::fs::File::create(&path).ok();
            if let Some(ref mut f) = file {
                let _ = f.write_all(b"42");
            }

            // When: loading baseline
            let result = read_baseline(&path);

            // Then: returns WarningCount(42)
            assert!(result.is_ok());
            if let Ok(count) = result {
                assert_eq!(count.get(), 42);
            }
        }
    }

    #[test]
    fn test_read_baseline_empty_file_returns_error() {
        // Given: temp file with empty content
        let temp_dir = tempfile::tempdir().ok();
        if let Some(dir) = temp_dir {
            let path = dir.path().join(".clippy-baseline");
            let _ = std::fs::File::create(&path);

            // When: loading baseline
            let result = read_baseline(&path);

            // Then: returns EmptyBaseline error
            assert!(matches!(result, Err(BudgetError::EmptyBaseline)));
        }
    }

    // ========================================================================
    // check_budget tests
    // ========================================================================

    #[test]
    fn test_check_budget_passed() {
        let baseline = WarningCount::new(10);
        let current = WarningCount::new(8);

        let result = check_budget(current, baseline);

        assert!(result.passed());
        match result {
            BudgetCheckResult::Passed {
                current: c,
                baseline: b,
            } => {
                assert_eq!(c.get(), 8);
                assert_eq!(b.get(), 10);
            }
            BudgetCheckResult::Exceeded { .. } => panic!("Expected Passed variant"),
        }
    }

    #[test]
    fn test_check_budget_exceeded() {
        let baseline = WarningCount::new(10);
        let current = WarningCount::new(15);

        let result = check_budget(current, baseline);

        assert!(!result.passed());
        match result {
            BudgetCheckResult::Exceeded {
                current: c,
                baseline: b,
                delta,
            } => {
                assert_eq!(c.get(), 15);
                assert_eq!(b.get(), 10);
                assert_eq!(delta, 5);
            }
            BudgetCheckResult::Passed { .. } => panic!("Expected Exceeded variant"),
        }
    }

    #[test]
    fn test_check_budget_equal() {
        let baseline = WarningCount::new(10);
        let current = WarningCount::new(10);

        let result = check_budget(current, baseline);

        // Equal counts should pass (within budget)
        assert!(result.passed());
    }

    #[test]
    fn test_check_budget_zero_warnings_passes_zero_baseline() {
        // Given: current = 0, baseline = 0 (strict policy)
        let current = WarningCount::new(0);
        let baseline = WarningCount::new(0);

        // When: checking budget
        let result = check_budget(current, baseline);

        // Then: returns Passed
        assert!(result.passed());
    }

    #[test]
    fn test_check_budget_passed_when_current_less_than_baseline() {
        // Given: current = 3, baseline = 5
        let current = WarningCount::new(3);
        let baseline = WarningCount::new(5);

        // When: checking budget
        let result = check_budget(current, baseline);

        // Then: returns Passed
        assert!(result.passed());
    }

    #[test]
    fn test_check_budget_exceeded_with_correct_delta() {
        // Given: current = 10, baseline = 5
        let current = WarningCount::new(10);
        let baseline = WarningCount::new(5);

        // When: checking budget
        let result = check_budget(current, baseline);

        // Then: returns Exceeded with delta = 5
        match result {
            BudgetCheckResult::Exceeded { delta, .. } => {
                assert_eq!(delta, 5);
            }
            BudgetCheckResult::Passed { .. } => {
                panic!("Expected Exceeded, not Passed");
            }
        }
    }

    // ========================================================================
    // Contract verification tests
    // ========================================================================

    #[test]
    fn test_invariant_warning_count_non_negative() {
        // Verifies I1: WarningCount is always >= 0 (compile-time via u32)
        // This test verifies runtime behavior
        let count = WarningCount::new(0);
        assert_eq!(count.get(), 0);

        let count = WarningCount::new(100);
        assert_eq!(count.get(), 100);
    }

    #[test]
    fn test_invariant_deterministic_comparison() {
        // Verifies I2: baseline comparison is deterministic
        let current = WarningCount::new(7);
        let baseline = WarningCount::new(5);

        // Run multiple times, should always return same result
        for _ in 0..10 {
            let result = check_budget(current, baseline);
            assert!(matches!(
                result,
                BudgetCheckResult::Exceeded { delta: 2, .. }
            ));
        }
    }

    #[test]
    fn test_postcondition_passed_when_current_le_baseline() {
        // Verifies Q3: check_budget returns Passed when current <= baseline
        let cases: Vec<(WarningCount, WarningCount)> = vec![
            (WarningCount::new(0), WarningCount::new(0)),
            (WarningCount::new(3), WarningCount::new(5)),
            (WarningCount::new(5), WarningCount::new(5)),
        ];

        for (current, baseline) in cases {
            let result = check_budget(current, baseline);
            assert!(
                result.passed(),
                "Expected Passed for {current:?} <= {baseline:?}"
            );
        }
    }

    #[test]
    fn test_postcondition_exceeded_when_current_gt_baseline() {
        // Verifies Q4: check_budget returns Exceeded when current > baseline
        let current = WarningCount::new(10);
        let baseline = WarningCount::new(5);

        let result = check_budget(current, baseline);

        assert!(matches!(
            result,
            BudgetCheckResult::Exceeded { delta: 5, .. }
        ));
    }

    #[test]
    fn test_budget_check_result_passed_variant() {
        let result = BudgetCheckResult::Passed {
            current: WarningCount::new(5),
            baseline: WarningCount::new(10),
        };

        assert!(result.passed());
    }

    #[test]
    fn test_budget_check_result_exceeded_variant() {
        let result = BudgetCheckResult::Exceeded {
            current: WarningCount::new(15),
            baseline: WarningCount::new(10),
            delta: 5,
        };

        assert!(!result.passed());
    }
}
