
package validation

import "list"

// Validation schema for bead: centralized-docs-20260228110508-yblnqrim
// Title: scrape-contract: fail command on partial scrape errors
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260228110508-yblnqrim.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260228110508-yblnqrim"
  title: "scrape-contract: fail command on partial scrape errors"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "Scrape run has at least one page-level error in result manifest.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "CLI exit code communicates partial failure state distinctly from full success.",
      "Terminal summary includes success_count and error_count with actionable guidance.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Manifest success_count/error_count remain accurate and unchanged by exit code policy.",
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
      "Scrape with zero errors exits 0.",
      "Scrape with all pages successful preserves current success output contract.",
    ]

    // Required error path tests
    required_error_tests: [
      "Scrape with mixed success/errors exits non-zero.",
      "Scrape with zero successes and errors exits non-zero with existing failure message.",
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
//     timestamp: "2026-02-28T11:05:08Z"
//   }
// }