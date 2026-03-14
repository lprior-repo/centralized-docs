# Contract: doc-3nx3

## bead_id: doc-3nx3
## bead_title: CLI: unreadable source files return exit code 0
## phase: p0
## updated_at: 2026-03-01T00:00:00Z

---

## Problem Statement

When source files have no read permissions (chmod 000), ctd index silently skips them and returns exit code 0.

**Expected:** Non-zero exit OR warning about unreadable files
**Actual:** Exit code 0, file silently skipped

**Severity:** P0 - Data integrity issue

---

## Preconditions

- Source directory contains files with no read permissions (chmod 000)
- User runs `ctd index <dir> --output <output>`

---

## Postconditions

### State Changes
- When unreadable files are encountered, the system MUST either:
  1. Return exit code 1 (user error), OR
  2. Output clear warning about unreadable files
- At minimum, exit code 0 should NOT be returned when files are silently skipped

### Return Guarantees
- Exit code 1 when unreadable source files exist
- Warning message printed to stderr or stdout indicating unreadable files

---

## Acceptance Tests

### Happy Path
- Valid readable source files: exit code 0, files indexed normally

### Error Path
- Source files with chmod 000: exit code 1 with clear error message
- Source files with no read permission: warning or error indicating permission denied

---

## Implementation Notes

- File discovery happens in `ctd/src/discover.rs` or similar
- Need to check file permissions before processing
- Handle PermissionDenied errors appropriately

---

## Verification

Test: `chmod 000 test.md && ctd index . --output /tmp/out && echo "Exit: $?"`
Expected: Non-zero exit code OR warning about unreadable files
