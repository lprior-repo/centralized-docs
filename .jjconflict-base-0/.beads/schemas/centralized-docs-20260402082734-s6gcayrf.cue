
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402082734-s6gcayrf
// Title: action: reuse archived analyses for unchanged files in `run_index`
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402082734-s6gcayrf.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402082734-s6gcayrf"
  title: "action: reuse archived analyses for unchanged files in `run_index`"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`run_index` already has a `FileDiff` and an open `StateReadSession`.",
      "`StateReadSession::load_analyses` can retrieve archived analyses by hash.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "Only changed and new files are passed to `analyze::analyze_files`.",
      "The final `analyses` vector still contains all documents in deterministic path order.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "The downstream pipeline continues receiving `Vec<Analysis>` or the runtime type it already expects.",
      "Behavior for changed and new files remains the same as before.",
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
      "A focused pipeline test verifies one unchanged file is reused from state while one changed file is re-analyzed.",
      "A test verifies the merged analysis collection stays sorted by source path after combining reused and recomputed values.",
    ]

    // Required error path tests
    required_error_tests: [
      "Missing archived analysis bytes for an unchanged file cause a surfaced error.",
      "Invalid archived analysis bytes for an unchanged file return an explicit load or deserialize error.",
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
//     timestamp: "2026-04-02T08:27:34Z"
//   }
// }