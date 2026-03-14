# Contract: doc-2y1p

**bead_id:** doc-2y1p
**bead_title:** CLI: search exit code should be 0 for no results
**phase:** p1

## Problem Statement
The search command returns exit code 1 when no results are found, but this is semantically incorrect - no results is a valid result state, not a user error. Exit code 1 should only be for actual errors like invalid index, missing arguments, etc.

## Acceptance Criteria

### Exit Code Semantics
- **Exit code 0**: Success (includes "no results found" - this is a valid result)
- **Exit code 1**: Actual errors only (invalid index, missing arguments, I/O failures, etc.)

### Expected Behavior
```
$ ctd search 'nonexistent' --index-dir /tmp/test_index
No results found for 'nonexistent'
Exit: 0
```

### Reproduction Case
```
$ ctd search 'nonexistent' --index-dir /tmp/test_index
No results found for 'nonexistent'
Error: No results found for 'nonexistent'
Exit: 1  ❌ WRONG - should be 0
```

## Contract Validation
1. Run search with non-existent query → verify exit code is 0
2. Run search with invalid arguments → verify exit code is 1
3. Run search with valid query → verify exit code is 0
