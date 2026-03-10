# Contract: doc-fi1y - Validator returns exit 0 despite errors

## bead_id: doc-fi1y
## bead_title: index: Validator returns exit 0 despite errors
## phase: p0
## updated_at: 2026-02-28T15:44:00Z

---

## Problem Statement

The llms_txt_validator returns exit code 0 (success) even when it reports errors. The validator says "Validation passed" despite finding duplicate chunk IDs. This allows corrupted data to pass CI checks.

## Evidence

```bash
$ ./target/release/llms_txt_validator --index ./corrupted_index.json
# Found 372 errors, 0 warnings
# Exit code is STILL ZERO!

$ git push  # Pushes corrupted data to production
```

## Expected Behavior

| Error Count | Exit Code | Meaning |
|-------------|-----------|---------|
| 0 errors | 0 | Success, safe to ship |
| 1-10 errors | 1 | Data corruption detected |
| 11-100 errors | 2 | Severe corruption |
| >100 errors | 3 | Critical corruption |
| Parse errors (invalid JSON) | 4 | File is unreadable |

## Requirements

1. Validation finds any error → exit non-zero (1-3 based on severity)
2. Exit code should reflect severity (1-4)
3. CI should fail on validation errors
4. Error count reported accurately

## Acceptance Criteria

- [ ] Validator returns exit code >= 1 when errors found
- [ ] Exit codes follow the severity table above
- [ ] Output message says "Validation failed" not "Validation passed"
- [ ] CI gates fail when validator returns non-zero

## Test Scenarios

1. **Corrupted data** → exit 3, message "Validation failed: N errors"
2. **Good data** → exit 0, message "Validation passed"
3. **Invalid JSON** → exit 4, message "Parse error"
4. **File not found** → exit 5, message "Error: file not found"
