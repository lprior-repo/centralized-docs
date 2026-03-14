bead_id: doc-1b5q
bead_title: ctd: Fix category-config file content leak
phase: p0
updated_at: 2026-03-01T13:55:00Z

# Contract

## Problem Statement
When --category-config points to a file with YAML parse errors (e.g., /etc/passwd), the error message echoes the sensitive file contents instead of showing a generic "invalid config" message.

## Preconditions
- None (CLI tool invocation)

## Postconditions
1. When --category-config points to a file with YAML parse errors, the error message shall NOT include file contents
2. Error message shall indicate "invalid config" or similar generic message
3. Exit code shall be non-zero for user input errors

## Acceptance Criteria
- Test: `ctd index qa-fixtures/basic --output /tmp/test --category-config /etc/passwd`
- Expected: Shows 'invalid config' message, NOT file contents
- Exit code: non-zero (user error)

## Invariants
- Sensitive file contents never appear in error messages
