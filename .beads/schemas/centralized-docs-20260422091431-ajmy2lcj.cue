
package validation

import "list"

// Validation schema for bead: centralized-docs-20260422091431-ajmy2lcj
// Title: cli: honor json mode on watch and diff failures
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260422091431-ajmy2lcj.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260422091431-ajmy2lcj"
  title: "cli: honor json mode on watch and diff failures"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "The caller supplied `--json` to a command that supports JSON output.",
      "The command encounters a runtime or input failure before normal completion.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "Failure output is valid JSON for watch and diff when `--json` is present.",
      "Exit codes for those failures remain stable and machine consumers can parse the error body.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "JSON mode always means stdout or stderr stays machine-readable for supported commands.",
      "Human-only formatting is not emitted on watch or diff failure paths when JSON mode is requested.",
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
      "A successful `ctd diff <scrape> <scrape> --json` run still emits valid JSON output.",
      "A successful `ctd watch --json` run continues to emit valid JSON without regression.",
    ]

    // Required error path tests
    required_error_tests: [
      "`ctd diff /nope-a /nope-b --json` emits valid JSON describing the manifest lookup failure instead of plain text.",
      "`ctd watch http://127.0.0.1:9 --output <dir> --json --cache <db>` emits valid JSON describing the network failure instead of plain text.",
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
//     timestamp: "2026-04-22T09:14:31Z"
//   }
// }