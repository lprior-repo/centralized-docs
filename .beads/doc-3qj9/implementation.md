# Implementation: doc-3qj9

## Bug Summary
- **ID**: doc-3qj9
- **Title**: index: Empty or non-directory source returns exit code 0
- **Priority**: P0 (Bug)
- **Status**: Implemented

## Problem Statement
When indexing an empty directory, non-directory file (like /etc/passwd), or directory with no markdown files, the command was returning exit code 0 (success) with 0 documents indexed. Additionally, "Validation: 0/0 passed" was shown as a success indicator which was misleading.

## Root Cause
In `main.rs`, after discovering files, there was a check for empty files that only printed a warning but did NOT fail the operation:

```rust
// BEFORE (buggy):
// Warn if no files were found (but don't fail - empty is valid)
if files.is_empty() {
    eprintln!(
        "Warning: No markdown files found in source directory. Cannot index empty source.\n\
         Hint: ..."
    );
}
// Code continued with 0 files, eventually returning exit code 0
```

## Solution

### 1. Changed Warning to Error (main.rs)
The warning was converted to a proper error that exits with code 1:

```rust
// AFTER (fixed):
// Exit with error if no markdown files found (user error - exit code 1)
if files.is_empty() {
    anyhow::bail!(
        "No markdown files found in source directory. Cannot index empty source.\n\
         Hint: Ensure the source directory contains files with .md, .mdx, .markdown, .txt, or .rst extensions."
    );
}
```

### 2. Updated CLI Tests
Updated `cli_integration_tests.rs` to expect exit code 1 for:
- Empty directory
- Non-directory file (like /etc/passwd)
- Directory with no markdown files

## Contract Requirements Met

| Requirement | Status |
|------------|--------|
| Exit code 1 for empty source | ✅ Implemented |
| Clear error message about no markdown files | ✅ Implemented |
| No "0/0 passed" misleading output | ✅ Fixed (validation never runs) |

## Testing

### Manual Testing
```bash
# Empty directory - returns exit code 1
$ mkdir /tmp/empty && doc_transformer index /tmp/empty --output /tmp/out
Error: No markdown files found...
Exit code: 1

# Non-directory file - returns exit code 1  
$ doc_transformer index /etc/passwd --output /tmp/out
Error: No markdown files found...
Exit code: 1

# Directory with no markdown files - returns exit code 1
$ mkdir /tmp/no_md && echo "{}" > /tmp/no_md/file.json
$ doc_transformer index /tmp/no_md --output /tmp/out
Error: No markdown files found...
Exit code: 1

# Valid directory with markdown - returns exit code 0
$ echo "# Test" > /tmp/test.md
$ doc_transformer index /tmp --output /tmp/out
Exit code: 0
```

### Unit Tests
- Updated `test_index_empty_directory` in `cli_integration_tests.rs`
- Added `test_index_non_directory_file` 
- Added `test_index_directory_with_no_markdown_files`

## Files Changed
1. `doc_transformer/src/main.rs` - Changed warning to error, exit code 1 for empty sources
2. `doc_transformer/tests/cli_integration_tests.rs` - Updated tests to expect exit code 1

## Verification
All three contract requirements are met:
1. ✅ Exit code 1 when source has no processable markdown files
2. ✅ Clear error message includes "No markdown files found"
3. ✅ "0/0 passed" no longer shown (validation skipped due to early error)
