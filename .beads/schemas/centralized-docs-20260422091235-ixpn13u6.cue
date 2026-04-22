
package validation

import "list"

// Validation schema for bead: centralized-docs-20260422091235-ixpn13u6
// Title: cli: report filtered document counts in ingest-git summary
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260422091235-ixpn13u6.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260422091235-ixpn13u6"
  title: "cli: report filtered document counts in ingest-git summary"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "The repo URL is reachable and clonable.",
      "The filter expression is accepted by the CLI.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "The final summary count matches the documents actually indexed after filtering.",
      "If both raw and filtered counts are shown, each count is clearly labeled.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Summary output remains consistent with the files written into the output directory.",
      "Applying a filter never inflates the reported indexed document count.",
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
      "Running ingest-git without a filter keeps the final summary aligned with the indexed document count.",
      "Running the checked-in filter repro reports the filtered count of 7 documents in the final banner or clearly labels raw versus filtered counts.",
    ]

    // Required error path tests
    required_error_tests: [
      "A filtered run must not claim the repository-wide discovered count as the indexed document count.",
      "Changing the filter to match zero or very few files still yields a truthful final summary instead of stale pre-filter numbers.",
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
//     timestamp: "2026-04-22T09:12:36Z"
//   }
// }