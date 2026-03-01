# Contract: doc-vd3p - cli: Multiple parameter validation errors return exit code 0

## Metadata
- bead_id: doc-vd3p
- bead_title: cli: Multiple parameter validation errors return exit code 0
- phase: p0
- updated_at: 2026-03-01T06:04:00Z

## Problem Statement
When multiple parameter validation errors occur, the CLI returns exit code 0 instead of 1.

## Requirements
1. Find where validation errors are handled
2. Ensure exit code 1 is returned for validation errors
3. Test multiple invalid parameters

## Anti-Hallucination
- Read files before modifying
