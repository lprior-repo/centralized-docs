
package validation

import "list"

// Validation schema for bead: centralized-docs-20260402075248-fzuq6frk
// Title: action: add scrape URL diffing and archived scrape output reuse
//
// This schema validates that implementation is complete.
// Use: cue vet centralized-docs-20260402075248-fzuq6frk.cue implementation.cue

#BeadImplementation: {
  bead_id: "centralized-docs-20260402075248-fzuq6frk"
  title: "action: add scrape URL diffing and archived scrape output reuse"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "`StateDb::load_url_states` and `commit_changes` are available.",
      "Scrape output types derive rkyv traits for archived persistence.",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "The scrape command loads URL state at startup, hashes fetched pages, and processes only changed/new pages.",
      "Updated URL-state rows and new scrape output blobs are committed in one shutdown batch.",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "The network fetch still occurs for all pages under the current spider-rs limitation.",
      "Unchanged pages skip downstream CPU work instead of being fully reprocessed.",
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
      "A command or integration test proves an unchanged scraped page is reused from state while a changed page is reprocessed.",
      "A test verifies updated URL-state rows and new scrape output blobs are committed together after scrape completion.",
    ]

    // Required error path tests
    required_error_tests: [
      "Corrupt archived scrape output for an unchanged page returns an explicit error or forces a defined fallback path.",
      "A URL-state load or commit failure aborts the scrape command instead of silently disabling reuse.",
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
//     timestamp: "2026-04-02T07:52:48Z"
//   }
// }