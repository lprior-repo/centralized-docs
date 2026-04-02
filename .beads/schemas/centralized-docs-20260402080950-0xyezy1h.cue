
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402080950-0xyezy1h
// Title: action: capture transform artifacts by source path and reuse archived transforms
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402080950-0xyezy1h.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402080950-0xyezy1h"
  title: "action: capture transform artifacts by source path and reuse archived transforms"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "Archived transform loaders exist on `StateReadSession`.",
      "The index pipeline already produces transformed markdown for changed and new files.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "A deterministic transform-artifact map exists at the command boundary keyed by source path.",
      "Unchanged files can reuse stored transform outputs through the shared read session.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Transform reuse is driven by stored transform hashes in `FileStateRaw`.",
      "No later task needs to guess how transformed markdown maps back to a source file.",
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
      "A focused test proves transform output for an unchanged file is loaded from state while a changed file gets a new transform artifact keyed by the same source path.",
      "A test verifies the transform-artifact map remains deterministic across runs with identical inputs.",
    ]

    // Required error path tests
    required_error_tests: [
      "Missing archived transform bytes for an unchanged file return an explicit error.",
      "A changed file that reaches commit preparation without a transform artifact fails before commit.",
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