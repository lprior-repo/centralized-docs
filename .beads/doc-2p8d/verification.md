# Verification: Warning for Unreadable Source Files

**bead_id:** doc-2p8d  
**bead_title:** cli: unreadable source files are silently ignored  
**phase:** p3  
**updated_at:** 2026-02-28T00:00:00Z

## Acceptance Criteria

1. ✅ Running `doc_transformer index` on a directory with permission-denied subdirectories shows "Warning:" not "Error:"
2. ✅ Running `doc_transformer index` on a directory with permission-denied subdirectories prints a summary at the end indicating how many paths couldn't be accessed
3. ✅ The fix does not break existing functionality (existing tests pass)

## Verification Steps Performed

### 1. Build verification
```bash
cargo build --release  # ✅ Compiles successfully
```

### 2. Test with inaccessible directory
```bash
mkdir -p /tmp/perm-test/subdir
echo "# Test" > /tmp/perm-test/subdir/test.md
chmod 000 /tmp/perm-test/subdir
./doc_transformer index /tmp/perm-test --output /tmp/out
```

**Output:**
```
[STEP 1] DISCOVER
Warning: Skipping path due to I/O error: IO error for operation on /tmp/perm-test/subdir: Permission denied (os error 13)
Warning: Skipped 1 path(s) due to I/O errors (e.g., permission denied). Some files may not have been processed.
  Found 0 files
```

✅ Shows "Warning:" instead of "Error:"  
✅ Shows summary of skipped paths

### 3. Unit tests
```bash
cargo test --lib discover  # ✅ All 11 tests pass
```

## Notes

- The warning message now uses "Warning:" prefix consistently
- A summary is printed at the end when any paths are skipped due to I/O errors
- The behavior for other error types (broken symlinks, empty files, oversized files) remains unchanged
