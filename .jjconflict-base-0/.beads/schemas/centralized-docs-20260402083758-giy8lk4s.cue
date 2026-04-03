
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402083758-giy8lk4s
// Title: calc: expose deterministic config hashing and add `compute_file_diff`
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402083758-giy8lk4s.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402083758-giy8lk4s"
  title: "calc: expose deterministic config hashing and add `compute_file_diff`"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "Discovered files include source paths relative to the source directory.",
      "Known states are keyed by those same source paths.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "A shared config-hash helper is callable from outside `analyze.rs`.",
      "`compute_file_diff` returns deterministic unchanged, changed, new, and deleted collections.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "The function performs no state writes.",
      "A path can appear in exactly one diff bucket.",
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
      "A unit test covers a mixed set of unchanged, changed, new, and deleted files.",
      "A unit test verifies the config-hash helper returns the same digest for the same config contents regardless of caller.",
    ]

    // Required error path tests
    required_error_tests: [
      "A missing file on disk returns an error from the hashing phase.",
      "A file read failure during parallel hashing aborts diff computation instead of misclassifying the file.",
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
//     timestamp: "2026-04-02T08:37:58Z"
//   }
// }