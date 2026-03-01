# Implementation: analyze-contract malformed markdown handling

## Problem Analysis

In `analyze.rs` lines 110-118, the current implementation uses `filter_map` with `.ok()` which silently drops files that fail to parse:

```rust
let analyses: Vec<_> = files
    .iter()
    .filter_map(|file| {
        let file_path = source_dir.join(&file.source_path);
        analyze_single_file(&file.source_path, &file_path, config.as_ref())
            .map_err(|e| eprintln!("Error: analysis failed: {}: {}", file.source_path, e))
            .ok()  // <-- SILENTLY DROPS FAILED FILES
    })
    .collect();
```

This violates the contract:
- When some files fail to parse, the command still exits with success (0)
- No indication in output that files were dropped
- `processed_count + failed_count != discovered_count` (the invariant is violated)

## Solution

Created a new return type `AnalyzeResult` that tracks both successful analyses AND failed files:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub analyses: Vec<Analysis>,
    pub failed_files: Vec<FailedFile>,
    pub total_discovered: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedFile {
    pub source_path: String,
    pub error: String,
}
```

The function signature changes from:
```rust
pub fn analyze_files(...) -> Result<Vec<Analysis>>
```
to:
```rust
pub fn analyze_files(...) -> Result<AnalyzeResult>
```

Added `Deref` for backward compatibility so `AnalyzeResult` can be used like a `Vec<Analysis>`.

## Changes Made

1. **analyze.rs**:
   - Added `FailedFile` struct
   - Added `AnalyzeResult` struct with `len()`, `is_empty()`, and `Deref` impl
   - Modified `analyze_files` to use `partition` to separate successes from failures
   - When ALL files fail, returns an error with comprehensive error message
   - When SOME files fail, returns `AnalyzeResult` with failed_files list

2. **main.rs**:
   - Updated call site to handle `AnalyzeResult`
   - Added warning output for failed files (stderr)
   - Added "X files failed analysis" to summary output

3. **Test files** (multiple):
   - Updated to extract `.analyses` from the result

## Verification

- ✅ When some files fail: prints warning with failed file names and errors
- ✅ When all files fail: returns error with all failure reasons
- ✅ Invariant holds: `processed + failed = discovered`
- ✅ Exit code 0 for partial success (some files analyzed)
- ✅ Exit code non-zero when all files fail

## Acceptance Criteria

- [x] `AnalyzeResult` struct tracks `analyses`, `failed_files`, and `total_discovered`
- [x] `analyze_files` returns `Result<AnalyzeResult>`
- [x] Failed files are included in `failed_files` with error message
- [x] Main output prints "X failed" when there are failures
- [x] Exit code remains 0 if SOME files succeed (partial success), but output clearly indicates failures
- [x] Tests verify the invariant: processed_count + failed_count == discovered_count
