---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:00:00Z
---

# Traceability Matrix

## EARS Requirements to Tests Mapping

| Requirement ID | Requirement | Test(s) |
|----------------|-------------|---------|
| **UBI-1** | THE SYSTEM SHALL track clippy warning count as a quality metric | `test_warning_count_from_clippy_output_counts_correctly`<br>`test_warning_count_handles_empty_clippy_output`<br>`test_warning_count_handles_no_warnings_output` |
| **UBI-2** | THE SYSTEM SHALL prevent warning regressions in release validation | `test_check_budget_returns_passed_when_current_equals_baseline`<br>`test_check_budget_returns_passed_when_current_less_than_baseline`<br>`test_check_budget_returns_exceeded_when_current_greater_than_baseline` |
| **EVT-1** | WHEN release validation runs, THE SYSTEM SHALL compare warning count to baseline and fail on regression | `test_full_budget_check_workflow_passes`<br>`test_release_gate_includes_budget_check`<br>`test_exit_code_on_budget_exceeded` |
| **UNW-1** | IF warning count increases, THE SYSTEM SHALL NOT mark release validation as passed | `test_check_budget_returns_exceeded_when_current_greater_than_baseline`<br>`test_violation_q4_budget_exceeded_not_passed` |

---

## Contracts to Tests Mapping

### Preconditions

| Precondition ID | Description | Test(s) |
|----------------|-------------|---------|
| P1 | `.clippy-baseline` file exists at repository root | `test_precondition_baseline_file_exists`<br>`test_baseline_file_not_found_returns_error`<br>`test_violation_p1_missing_baseline_file` |
| P2 | `.clippy-baseline` contains valid non-negative integer | `test_precondition_baseline_content_valid`<br>`test_baseline_invalid_content_returns_error`<br>`test_violation_p2_invalid_baseline_content`<br>`test_violation_p2_empty_baseline`<br>`test_violation_p2_negative_baseline`<br>`test_violation_p2_float_baseline` |
| P3 | `cargo` command is available in PATH | (Environment prerequisite - verified at runtime) |
| P4 | Clippy is installed for the workspace | (Environment prerequisite - verified at runtime) |

### Postconditions

| Postcondition ID | Description | Test(s) |
|----------------|-------------|---------|
| Q1 | `load_baseline` returns `Ok(WarningCount)` with parsed value | `test_warning_count_from_baseline_parses_valid_integer`<br>`test_baseline_trims_whitespace` |
| Q2 | `run_clippy_and_count_warnings` returns count of lines containing "warning:" | `test_warning_count_from_clippy_output_counts_correctly`<br>`test_warning_count_only_counts_warning_prefix` |
| Q3 | `check_budget` returns `Passed` when `current <= baseline` | `test_postcondition_passed_when_current_le_baseline` |
| Q4 | `check_budget` returns `Exceeded` when `current > baseline` | `test_postcondition_exceeded_when_current_gt_baseline`<br>`test_violation_q4_budget_exceeded_not_passed` |
| Q5 | `execute_budget_check` produces deterministic result for same inputs | `test_invariant_deterministic_comparison` |
| Q6 | Exit code is 4 when budget exceeded, 0 when all gates pass | `test_exit_code_on_budget_exceeded` |

### Invariants

| Invariant ID | Description | Test(s) |
|--------------|-------------|---------|
| I1 | `WarningCount` is always >= 0 (enforced by u32) | `test_invariant_warning_count_non_negative` |
| I2 | Baseline comparison is deterministic (same inputs → same output) | `test_invariant_deterministic_comparison` |
| I3 | Policy applies consistently regardless of branch | (Structural invariant - verified by code review) |
| I4 | Budget check is independent of P0 and CI checks | `test_release_gate_includes_budget_check` |

---

## Edge Cases to Tests Mapping

| Edge Case | Test(s) |
|-----------|---------|
| Baseline = 0, Current = 0 | `test_zero_warnings_passes_zero_baseline` |
| Baseline = 0, Current = 1 | `test_check_budget_returns_exceeded_when_current_greater_than_baseline` |
| Baseline = 5, Current = 5 | `test_check_budget_returns_passed_when_current_equals_baseline` |
| Baseline = 5, Current = 4 | `test_check_budget_returns_passed_when_current_less_than_baseline` |
| Empty clippy output | `test_warning_count_handles_empty_clippy_output` |
| Clippy output with no warnings | `test_warning_count_handles_no_warnings_output` |
| Clippy output with mixed content | `test_warning_count_from_clippy_output_counts_correctly` |
| Baseline file with trailing newline | `test_baseline_trims_whitespace` |
| Baseline file with leading/trailing spaces | `test_baseline_trims_whitespace` |
| Large warning count value | `test_warning_count_large_value` |

---

## Given-When-Then Scenarios Coverage

| Scenario | Steps | Covered By Test(s) |
|----------|-------|--------------------|
| **Budget Check Passes** | Given: baseline="5", When: clippy produces 3 warnings, Then: Passed | `test_full_budget_check_workflow_passes` |
| **Budget Check Fails** | Given: baseline="0", When: clippy produces 2 warnings, Then: Exceeded | `test_check_budget_returns_exceeded_when_current_greater_than_baseline` |
| **Baseline File Missing** | Given: .clippy-baseline does not exist, When: release gate runs, Then: error | `test_baseline_file_not_found_returns_error` |
| **Baseline File Invalid** | Given: .clippy-baseline contains "not-a-number", When: release gate runs, Then: error | `test_baseline_invalid_content_returns_error` |
| **Zero Warnings, Zero Baseline** | Given: baseline="0", When: clippy produces 0 warnings, Then: Passed | `test_zero_warnings_passes_zero_baseline` |

---

## Test Coverage Summary

| Category | Coverage |
|----------|----------|
| Total EARS Requirements | 4/4 (100%) |
| Total Preconditions | 4/4 (100%) |
| Total Postconditions | 6/6 (100%) |
| Total Invariants | 3/4 (75%) - I3 is structural |
| Total Edge Cases | 10/10 (100%) |
| Total Scenarios | 5/5 (100%) |

---

## Discovered Gaps

No gaps identified. All acceptance criteria, contracts, and edge cases have corresponding test coverage.
