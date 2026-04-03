
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402083758-otj0apj8
// Title: data: add zero-copy state dependencies to centralized-docs crate
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402083758-otj0apj8.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402083758-otj0apj8"
  title: "data: add zero-copy state dependencies to centralized-docs crate"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`centralized-docs/Cargo.toml` defines the main crate dependencies.",
      "The workspace builds with Cargo-managed dependencies.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "`centralized-docs/Cargo.toml` contains `bytemuck` and `rkyv` entries with the requested features.",
      "No existing dependency entries required by current code are removed in this task.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "Dependency changes remain limited to the centralized-docs crate manifest.",
      "The task is safe to merge before any code consumes the new crates.",
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
      "`cargo check -p centralized-docs` resolves the new dependencies successfully.",
      "`cargo metadata` shows `bytemuck` and `rkyv` in the centralized-docs dependency graph.",
    ]

    // Required error path tests
    required_error_tests: [
      "Cargo reports an invalid feature name if the manifest entry is malformed.",
      "Cargo fails dependency resolution if an invalid crate version or feature combination is declared.",
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