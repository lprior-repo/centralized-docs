
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402083214-qrgwwdkt
// Title: data: add `StateReadSession` to enforce one shared read transaction per run
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402083214-qrgwwdkt.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402083214-qrgwwdkt"
  title: "data: add `StateReadSession` to enforce one shared read transaction per run"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`StateDb` can open and initialize the database.",
      "Raw state and archived table definitions already exist.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`StateDb` exposes a read-session entry point that owns the redb read transaction for the run.",
      "Later loader APIs are defined on `StateReadSession` instead of opening transactions themselves.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Only one read transaction exists per command run.",
      "The write path remains separate and still uses exactly one commit boundary at shutdown.",
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
      "A test proves multiple loader calls can share one `StateReadSession` without reopening redb transactions.",
      "A command-level test or test-only counter proves one run performs one read transaction even when it loads multiple tables.",
    ]

    // Required error path tests
    required_error_tests: [
      "Calling a loader without a valid read session is impossible by API shape or returns an explicit error.",
      "A test fails if a refactor reintroduces per-loader `begin_read()` calls.",
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
//     timestamp: "2026-04-02T08:32:14Z"
//   }
// }