
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402080950-pqotevy6
// Title: action: wire startup state open and file diff into `run_index`
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402080950-pqotevy6.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402080950-pqotevy6"
  title: "action: wire startup state open and file diff into `run_index`"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`StateDb::open`, `StateReadSession`, and `compute_file_diff` are available.",
      "`run_index` already performs discovery and holds the output directory lock.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`run_index` computes a file diff before the analyze phase.",
      "The command emits deterministic diff telemetry counts for unchanged, changed, new, and deleted files.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Discovery behavior remains unchanged.",
      "No state writes occur during the startup read step.",
    ]
  }

  // Test verification
  tests_passing: {
    all_tests_pass: bool & true

    happy_path_tests: [...string] & list.MinItems(2)
    error_path_tests: [...string] & list.MinItems(2)

    // Note: Actual test names provided by implementer, must include all required tests

    // Required happy path tests
    required_happy_tests: [
      "A command-level test verifies `run_index` opens the state DB and prints the new diff step when prior state exists.",
      "A test verifies the command reuses one read session for startup state access.",
    ]

    // Required error path tests
    required_error_tests: [
      "A failing state open returns an error instead of proceeding into analysis.",
      "A file-diff failure bubbles out of `run_index` before any later pipeline steps execute.",
    ]
  }

  // Code completion
  code_complete: {
    implementation_exists: string  // Path to implementation file
    tests_exist: string  // Path to test file
    ci_passing: bool & true
    no_unwrap_calls: bool & true  // Rust/functional constraint
    no_panics: bool & true  // Rust constraint
  }

  // Completion criteria
  completion: {
    all_sections_complete: bool & true
    documentation_updated: bool
    beads_closed: bool
    timestamp: string  // ISO8601 completion timestamp
  }
}

// Example implementation proof - create this file to validate completion:
//
// implementation.cue:
// package validation
//
// implementation: #BeadImplementation & {
//   contracts_verified: {
//     preconditions_checked: true
//     postconditions_verified: true
//     invariants_maintained: true
//     precondition_checks: [/* documented checks */]
//     postcondition_checks: [/* documented verifications */]
//     invariant_checks: [/* documented invariants */]
//   }
//   tests_passing: {
//     all_tests_pass: true
//     happy_path_tests: ["test_version_flag_works", "test_version_format", "test_exit_code_zero"]
//     error_path_tests: ["test_invalid_flag_errors", "test_no_flags_normal_behavior"]
//   }
//   code_complete: {
//     implementation_exists: "src/main.rs"
//     tests_exist: "tests/cli_test.rs"
//     ci_passing: true
//     no_unwrap_calls: true
//     no_panics: true
//   }
//   completion: {
//     all_sections_complete: true
//     documentation_updated: true
//     beads_closed: false
//     timestamp: "2026-04-02T08:09:50Z"
//   }
// }