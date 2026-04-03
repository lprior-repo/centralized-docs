
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402083758-o1tz0efl
// Title: action: wire scrape command to one shared read session and one shutdown commit
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402083758-o1tz0efl.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402083758-o1tz0efl"
  title: "action: wire scrape command to one shared read session and one shutdown commit"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "Scrape startup load, classification, reuse, and batch-building helpers already exist.",
      "`StateDb::commit_changes` can commit URL-state and scrape-output updates.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`ctd scrape` uses one shared read session for all reads and one commit call for all writes in the run.",
      "A successful scrape run persists URL-state and scrape-output updates exactly once at shutdown.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "The command performs no per-page writes.",
      "A failed scrape run before shutdown leaves previously committed state intact.",
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
      "An integration-style test proves an unchanged scrape page is reused from state and a changed page is reprocessed and committed in one run.",
      "A test-only transaction counter or equivalent assertion proves one scrape run uses one read transaction and one write transaction.",
    ]

    // Required error path tests
    required_error_tests: [
      "A forced failure before commit preserves the prior scrape state database contents.",
      "A shutdown commit error is returned from the scrape command instead of being swallowed.",
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