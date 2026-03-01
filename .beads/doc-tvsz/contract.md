# Contract: cli-contract - Normalize Invalid-Input Exit Codes Across Validators

## Overview
- **Bead ID**: doc-tvsz
- **Type**: bug (P0)
- **Goal**: Ensure consistent exit codes for invalid user input across all CLI commands

## Ubiquitous Requirements
- THE SYSTEM SHALL return a consistent exit code class for invalid user input across all commands.

## Event-Driven Requirements
- **Trigger**: WHEN argument validation fails at any layer
- **Shall**: THE SYSTEM SHALL emit deterministic error classification and exit code semantics.

## Unwanted Behaviors
- **Condition**: IF two commands reject equivalent invalid input types
- **Shall NOT**: THE SYSTEM SHALL NOT return different exit codes solely due to validation layer placement
- **Because**: automation cannot reliably map user errors without stable contract

## Preconditions
- Auth required: false
- Required inputs: none
- System state: User provides invalid numeric/range/format inputs to CLI options.

## Postconditions
- Exit codes for invalid-input class are consistent across parser-level and runtime-level checks.
- Error messages remain specific while preserving stable machine-readable class.

## Invariants
- Operational failures remain distinguishable from invalid-user-input failures.

## Research Questions
- Should contract reserve distinct code for usage/help vs invalid value? (Not yet answered)

## Acceptance Tests

### Happy Paths
- test_happy_path: Valid inputs → Exit code is 0, Output is correct

### Error Paths  
- test_error_path: Invalid inputs → Exit code is non-zero, Error message is clear

## Verification Checkpoints
1. **Research Gate**: All research questions answered, research notes documented
2. **Test Gate**: All tests written and failing
3. **Implementation Gate**: All tests pass
4. **Integration Gate**: E2E tests pass

## Anti-Hallucination Rules
- READ files before modifying them
- Use functional patterns: map, and_then, ?
- Return Result<T, Error> from all fallible functions
- ZERO UNWRAP law: NEVER use .unwrap or .expect
- Test first: Tests MUST exist before implementation
