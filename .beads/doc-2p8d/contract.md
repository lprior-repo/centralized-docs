# Contract: Warning for Unreadable Source Files

**bead_id:** doc-2p8d  
**bead_title:** cli: unreadable source files are silently ignored  
**phase:** p1  
**updated_at:** 2026-02-28T00:00:00Z

## Problem Statement

When source files or directories cannot be read due to permission denied errors, the `index` command:
1. Prints "Error: Skipping path due to I/O error" (confusing - says "Error" but continues)
2. Shows "Found 0 files" without clarifying that permission issues were the cause
3. Doesn't provide a summary of how many paths were skipped due to permissions

Users may not realize files were skipped and assume the source directory is empty.

## Expected Behavior

1. Use "Warning:" prefix instead of "Error:" for permission-related skips (since the program continues)
2. Track count of paths skipped due to I/O errors
3. Print a summary warning at the end of discovery if any paths were skipped

## Implementation Plan

### File: `ctd/src/discover.rs`

1. Add a counter `skipped_io_error` to track paths skipped due to I/O errors
2. Change the message from "Error: Skipping path due to I/O error" to "Warning: Skipping path due to I/O error"  
3. Increment the counter when skipping due to I/O error
4. After the WalkDir loop, print a summary if any paths were skipped

## Acceptance Criteria

1. ✅ Running `ctd index` on a directory with permission-denied subdirectories shows "Warning:" not "Error:"
2. ✅ Running `ctd index` on a directory with permission-denied subdirectories prints a summary at the end indicating how many paths couldn't be accessed
3. ✅ The fix does not break existing functionality (existing tests pass)
