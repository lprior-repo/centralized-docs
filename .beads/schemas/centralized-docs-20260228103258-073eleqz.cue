
package validation

import "list"

// Validation schema for bead: centralized-docs-20260228103258-073eleqz
// Title: search-contract: --json mode emits plain-text errors on failure
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260228103258-073eleqz.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260228103258-073eleqz"
  title: "search-contract: --json mode emits plain-text errors on failure"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "json_output flag is true.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "stdout contains valid JSON object with error fields on failure.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "All --json code paths are parseable RFC8259 JSON.",
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
      "Successful search outputs valid JSON with result schema.",
      "No-results case outputs valid JSON with status no_results.",
    ]

    // Required error path tests
    required_error_tests: [
      "Missing INDEX.json outputs JSON error object.",
      "Malformed INDEX.json outputs JSON error object.",
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
//     timestamp: "2026-02-28T10:32:58Z"
//   }
// }