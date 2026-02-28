# Contract: doc-27y3 - Validator says 'passed' despite missing sections

## bead_id: doc-27y3
## bead_title: index: Validator says 'passed' despite missing sections
## phase: p0
## updated_at: 2026-02-28T15:59:00Z

---

## Problem Statement

Validator reports 'Validation passed' even when required sections are missing. Warnings about missing 'Core Concepts' and 'API Reference' sections appear, but exit code is 0 and message says 'passed'.

## Evidence

```bash
$ ./target/release/llms_txt_validator /tmp/test_index/llms.txt
# Found 0 errors, 2 warnings
# Missing required section: Core Concepts
# Missing required section: API Reference
# Validation passed (with warnings)
# Exit code: 0
```

## Expected Behavior

| Scenario | Exit Code | Message |
|----------|-----------|---------|
| Required sections missing | 1 | "Validation failed: missing required sections" |
| Optional sections missing | 0 | "Validation passed with warnings" |
| No issues | 0 | "Validation passed" |

## Requirements

1. Required section missing → exit non-zero (1)
2. Only optional sections missing → exit zero with warnings
3. Error message clearly indicates which required sections are missing

## Acceptance Criteria

- [ ] Validator returns exit code 1 when required sections missing
- [ ] Exit code 0 when only optional sections missing
- [ ] Output message indicates failed due to missing required sections

## Labels

regression, validator, warnings
