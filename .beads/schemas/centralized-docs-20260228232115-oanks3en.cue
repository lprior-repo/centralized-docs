
package validation

import "list"

// Validation schema for bead: centralized-docs-20260228232115-oanks3en
// Title: cli: Invalid URL returns exit code 2 instead of 1
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260228232115-oanks3en.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260228232115-oanks3en"
  title: "cli: Invalid URL returns exit code 2 instead of 1"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "User provides URL argument to scrape or ingest command",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "On invalid URL format error, exit code is 1",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Exit code mapping is consistent across all commands",
    ]
  }

  // Test verification
  tests_passing: {
    all_tests_pass: bool & true

    happy_path_tests: [...string] & list.MinItems(2)
    error_path_tests: [...string] & list.MinItems(3)

    // Note: Actual test names provided by implementer, must include all required tests

    // Required happy path tests
    required_happy_tests: [
      "scrape with valid https URL succeeds with exit 0",
      "index with valid directory succeeds with exit 0",
    ]

    // Required error path tests
    required_error_tests: [
      "scrape with invalid URL returns exit 1",
      "ingest with invalid URL returns exit 1",
      "scrape with ftp scheme returns exit 1",
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
//     timestamp: "2026-02-28T23:21:15Z"
//   }
// }