
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402082447-obssk9ml
// Title: data: add fixed-size `FileStateRaw` and `UrlStateRaw` pod types
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402082447-obssk9ml.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402082447-obssk9ml"
  title: "data: add fixed-size `FileStateRaw` and `UrlStateRaw` pod types"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "bytemuck is available to derive Pod-compatible traits.",
      "A new state module can be referenced from production code.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`FileStateRaw` and `UrlStateRaw` exist with explicit reserved bytes and stable layout attributes.",
      "The types derive the traits needed for safe byte casting.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "All fields remain fixed-size primitives or byte arrays.",
      "Reserved bytes are explicit instead of relying on compiler-inserted padding.",
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
      "A unit test asserts the size of each raw struct matches the intended storage footprint.",
      "A unit test verifies `bytemuck::bytes_of` and `pod_read_unaligned` round-trip both raw structs.",
    ]

    // Required error path tests
    required_error_tests: [
      "Compilation fails if a non-Pod field is accidentally added to either raw struct.",
      "A layout assertion test fails if a field change breaks the documented footprint.",
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
//     timestamp: "2026-04-02T08:24:47Z"
//   }
// }