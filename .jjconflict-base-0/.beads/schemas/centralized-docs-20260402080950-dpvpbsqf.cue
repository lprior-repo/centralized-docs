
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402080950-dpvpbsqf
// Title: data: add archive-safe persisted output records and rkyv derives
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402080950-dpvpbsqf.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402080950-dpvpbsqf"
  title: "data: add archive-safe persisted output records and rkyv derives"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "The archive boundary and ownership model are already defined.",
      "Runtime domain types for analysis, transform, chunk, scrape, and snapshot outputs already exist.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "Persisted record types exist for every redb output table named in the architecture spec.",
      "Conversion helpers exist between runtime domain values and persisted record values where shapes differ.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Persisted record types stay append-only and serialization-focused.",
      "No task later in the plan needs to revisit `Arc<str>` or `DateTime<Utc>` compatibility.",
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
      "Focused tests archive and access one sample value for each persisted record family.",
      "Round-trip tests prove persisted records convert back into the runtime domain types required by command flows.",
    ]

    // Required error path tests
    required_error_tests: [
      "Invalid archived bytes for any persisted record type return an rkyv access error during tests.",
      "Compilation fails if a nested persisted field lacks the required rkyv support.",
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