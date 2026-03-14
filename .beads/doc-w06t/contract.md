# Contract: doc-w06t - scrape-command: fail command on partial scrape errors

## Overview

This contract specifies behavior changes to the `ctd scrape` command to properly handle and report partial scrape failures. When pages fail to scrape but some succeed, the CLI must exit with a non-zero status code and provide actionable diagnostics.

## Problem Statement

Currently, the `ctd scrape` command exits with code 0 even when some pages fail to scrape, as long as at least one page succeeds. This violates the principle that CI/scripts interpret zero as complete success.

## Ubiquitous Requirements (Must Hold Always)

- **UR-1**: THE SYSTEM SHALL return non-zero exit code when scrape encountered any unrecovered page errors.

## Event-Driven Requirements

- **ER-1**: WHEN scrape completes with `success_count > 0` AND `error_count > 0`, THE SYSTEM SHALL signal partial failure via exit status and summary diagnostics.

## Unwanted Behaviors (Negative Requirements)

- **NR-1**: IF scrape has recorded errors, THE SYSTEM SHALL NOT exit with code 0, because CI and scripts interpret zero as complete success.

## Preconditions

1. `auth_required`: false
2. `required_inputs`: URL and output directory
3. `system_state`: Scrape run has at least one page-level error in result manifest (i.e., `error_count > 0`)

## Postconditions

1. **State Changes**:
   - CLI exit code communicates partial failure state distinctly from full success
   - Terminal summary includes `success_count` and `error_count` with actionable guidance

2. **Return Guarantees**: None

## Invariants

- **INV-1**: Manifest `success_count`/`error_count` remain accurate and unchanged by exit code policy

## Exit Code Design

| Scenario | Exit Code | Rationale |
|----------|-----------|-----------|
| Full success (`error_count == 0`) | 0 | All pages scraped successfully |
| Partial failure (`success_count > 0 && error_count > 0`) | 2 | Some pages failed, partial data |
| Total failure (`success_count == 0 && error_count > 0`) | 2 | All pages failed |

Note: Exit code 1 is reserved for user errors (bad arguments, missing files) per existing code. Exit code 2 is used for pipeline errors.

## Summary Output Format

When partial failure occurs, output should include:
```
SCRAPE COMPLETE (PARTIAL FAILURE)
==================================
Success: {success_count} pages
Errors:  {error_count} pages failed
Hint: Check .scrape/manifest.json for error details
```

## Research Notes

### Files Examined
- `ctd/src/main.rs` - Entry point, run_scrape function (lines 1018-1106)
- `ctd/src/scrape/validation.rs` - ScrapeResult structure (lines 198-206)
- `ctd/src/scrape/mod.rs` - scrape_site function

### Existing Patterns
- Uses `anyhow::Result<()>` for fallible operations
- Uses `process::exit(code)` for CLI exit
- Existing exit codes: 0 (success), 1 (user error), 2 (pipeline error)
- ScrapeResult contains: pages, total_urls, success_count, error_count, errors, base_url

### Implementation Approach
1. After scrape completes, check `result.error_count > 0`
2. If errors exist, print partial failure summary
3. Call `process::exit(2)` to indicate partial/total failure

## Acceptance Tests

### Happy Path
- **test_full_success**: Given scrape with 0 errors, when command runs, then exit code is 0

### Error Paths  
- **test_partial_failure**: Given scrape with 5 success and 3 errors, when command runs, then exit code is 2 and summary shows counts
- **test_total_failure**: Given scrape with 0 success and 5 errors, when command runs, then exit code is 2

## Implementation Tasks

1. Modify `run_scrape()` in `main.rs` to check for errors after scrape
2. Print partial failure summary with success/error counts
3. Exit with code 2 when `error_count > 0`
