# Implementation: doc-2y1p

**bead_id:** doc-2y1p
**bead_title:** CLI: search exit code should be 0 for no results
**phase:** p2

## Changes Made

### 1. Core Logic Fix (`ctd/src/main.rs`)

Changed the exit code for "no results found" from 1 to 0 in the `map_error_to_exit_code` function:

**Before:**
```rust
// "no results found" is a search error condition - scripts need to know search failed
// Both JSON and text modes should return exit code 1 for no results
if error_string_lower.contains("no results found") {
    // No results is an error condition -> exit 1
    return 1;
}
```

**After:**
```rust
// "no results found" is NOT an error - it's a valid result state
// Exit code 0 means success (even with empty results)
// Exit code 1 is for actual errors (invalid index, missing args, etc.)
if error_string_lower.contains("no results found") {
    // No results is a valid result -> exit 0 (success)
    return 0;
}
```

### 2. Test Fix (`ctd/tests/cli_integration_tests.rs`)

Updated `test_search_no_results` to expect exit code 0 (success) instead of exit code 1:

**Before:**
```rust
// No-results is an error condition - scripts need to know search failed
// Both JSON and text modes return exit code 1 for no results (bead doc-3f31)
assert!(
    !search_result.status.success(),
    "Search with no matches should return exit 1 (error condition)"
);
```

**After:**
```rust
// No-results is a valid result - exit code 0 means success (even with empty results)
// Exit code 1 is for actual errors (doc-2y1p: search exit code should be 0 for no results)
assert!(
    search_result.status.success(),
    "Search with no matches should return exit 0 (valid result, not error)"
);
```

## Validation

- Exit code 0 for no results (valid result)
- Exit code 1 for actual errors (invalid index, missing arguments, etc.)
