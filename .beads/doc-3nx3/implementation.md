# Implementation: doc-3nx3

## bead_id: doc-3nx3
## bead_title: CLI: unreadable source files return exit code 0
## phase: p1
## updated_at: 2026-03-01T00:00:00Z

---

## Summary

Fixed the P0 bug where unreadable source files (chmod 000 or non-readable directories) were silently skipped and the index command returned exit code 0. Now the command returns exit code 1 when ANY file or directory cannot be read due to permission denied.

---

## Files Modified

### doc_transformer/src/discover.rs

**Lines 235-244** (permission denied check):
```rust
// FAIL if permission denied files were encountered - even if some readable files exist
// This ensures we don't silently skip unreadable files - user must fix permissions
if !permission_denied_files.is_empty() {
    let file_list = permission_denied_files.join(", ");
    anyhow::bail!(
        "Error: Cannot read {} file(s) due to permission denied: {}. \
         Please check file permissions with 'chmod +r' or remove unreadable files.",
        permission_denied_files.len(),
        file_list
    );
}
```

**New tests:**
- `test_unreadable_directory_returns_nonzero_exit` - verifies failure when a subdirectory has no read permissions
- `test_discover_files_with_readable_files` - verifies happy path still works

---

## Root Cause

The original code tracked `permission_denied_files` in a vector but never used it to return an error. The code would:
1. Detect permission denied errors during directory walking
2. Print error messages to stderr
3. Continue processing other files
4. Return Ok with whatever files could be read

This allowed the index command to succeed (exit 0) even when some files were skipped due to permissions.

---

## Fix Applied

Added a check at the end of `discover_files()` that bails with an error whenever `permission_denied_files` is not empty. This is stricter than originally planned - it fails even if SOME readable files exist, ensuring no silent data loss.

---

## Test Coverage

1. Unit test: `test_unreadable_directory_returns_nonzero_exit` - verifies failure on permission denied
2. Unit test: `test_discover_files_with_readable_files` - verifies happy path still works

---

## QA Verification Results

| Scenario | Exit Code | Result |
|----------|-----------|--------|
| Unreadable directory (no readable files) | 1 | ✓ PASS |
| Mixed (some readable, some not) | 1 | ✓ PASS |
| All readable (happy path) | 0 | ✓ PASS |

---

## Verification

```bash
# Run unit tests
cargo test --lib test_unreadable_directory_returns_nonzero_exit
cargo test --lib test_discover_files_with_readable_files

# Manual test: Create unreadable directory and run index
mkdir /tmp/test-perm
echo "# Test" > /tmp/test-perm/readable.md
mkdir /tmp/test-perm/restricted
echo "# Hidden" > /tmp/test-perm/restricted/hidden.md
chmod 000 /tmp/test-perm/restricted
doc_transformer index /tmp/test-perm --output /tmp/out
echo "Exit code: $?"
# Expected: exit code 1, error message about permission denied
```
