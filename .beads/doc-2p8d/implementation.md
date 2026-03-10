# Implementation: Warning for Unreadable Source Files

**bead_id:** doc-2p8d  
**bead_title:** cli: unreadable source files are silently ignored  
**phase:** p2  
**updated_at:** 2026-02-28T00:00:00Z

## Changes Made

### File: `doc_transformer/src/discover.rs`

1. **Added counter for I/O errors** (line 46):
   ```rust
   let mut skipped_io_error = 0usize;
   ```

2. **Changed error message to warning** (lines 71-74):
   ```rust
   Err(e) => {
       skipped_io_error = skipped_io_error.saturating_add(1);
       eprintln!("Warning: Skipping path due to I/O error: {e}");
       continue;
   }
   ```

3. **Added summary warning** (lines 186-192):
   ```rust
   // Emit warning summary for I/O errors (permission denied, etc.) if any were found
   if skipped_io_error > 0 {
       eprintln!(
           "Warning: Skipped {skipped_io_error} path(s) due to I/O errors (e.g., permission denied). \
           Some files may not have been processed."
       );
   }
   ```

## Verification

Test case from bug report:
```bash
mkdir /tmp/perm-test
echo "# Test" > /tmp/perm-test/test.md
chmod 000 /tmp/perm-test/test.md
./doc_transformer index /tmp/perm-test --output /tmp/out
```

**Before fix:**
- Showed "Error: Skipping path due to I/O error" (confusing - says error but continues)
- No summary of how many paths were skipped

**After fix:**
- Shows "Warning: Skipping path due to I/O error" (clearer - it's a warning, not a fatal error)
- Shows summary: "Warning: Skipped 1 path(s) due to I/O errors (e.g., permission denied). Some files may not have been processed."

## Test Results

- All 11 discover tests pass
- Code compiles without errors
- Format and clippy checks pass (existing warnings are pre-existing)
