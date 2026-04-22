
package validation

import "list"

// Validation schema for bead: centralized-docs-20260422091333-phrt01ce
// Title: cli: reject apply url and scrape mismatches
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260422091333-phrt01ce.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260422091333-phrt01ce"
  title: "cli: reject apply url and scrape mismatches"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "The scrape directory contains a readable manifest produced by ctd scrape or watch flows.",
      "The target URL provided to apply is intended to match the scrape content being committed.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "Mismatched URL and scrape inputs are rejected with an actionable error before state mutation.",
      "Matching URL and scrape inputs still apply successfully.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Apply does not store snapshot state for a target URL that the scrape manifest does not represent.",
      "Rejected mismatches leave the cache contents unchanged.",
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
      "Applying the checked-in Kubernetes scrape under `https://kubernetes.io/docs/home/` succeeds with `--yes`.",
      "Repeating apply with the same matching URL remains idempotent and does not invent extra changes.",
    ]

    // Required error path tests
    required_error_tests: [
      "Applying the checked-in Kubernetes scrape under `https://example.com` is rejected before any snapshot commit happens.",
      "A mismatched apply attempt leaves the cache in the same effective state as before the command.",
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
//     timestamp: "2026-04-22T09:13:33Z"
//   }
// }