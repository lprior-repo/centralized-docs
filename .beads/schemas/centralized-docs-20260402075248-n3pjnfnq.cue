
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402075248-n3pjnfnq
// Title: data: implement `StateChanges` and atomic `commit_changes` write path
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402075248-n3pjnfnq.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402075248-n3pjnfnq"
  title: "data: implement `StateChanges` and atomic `commit_changes` write path"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`StateDb` is open and tables are initialized.",
      "Raw state structs and archived output tables already exist.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`StateDb::commit_changes` writes all batch mutations using one `begin_write`/`commit` cycle.",
      "Raw states are stored as bytes and variable-size outputs are stored as rkyv byte vectors.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "No per-row transaction boundaries are introduced.",
      "Deletion lists remove only the targeted keys from their corresponding tables.",
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
      "A temporary database test commits file updates, URL updates, output blobs, and deletions in one batch successfully.",
      "A test verifies committed raw states and archived outputs are readable after one successful batch write.",
    ]

    // Required error path tests
    required_error_tests: [
      "An invalid write payload returns an error and preserves previously committed state.",
      "A mid-batch table write failure leaves the database without partially applied mutations.",
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
//     timestamp: "2026-04-02T07:52:48Z"
//   }
// }