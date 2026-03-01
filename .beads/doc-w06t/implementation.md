# Implementation: doc-w06t - scrape-command: fail command on partial scrape errors

## Files Modified

1. `doc_transformer/src/main.rs` - Modified `run_scrape()` function

## Implementation Details

### Changes to `run_scrape()` function

**Location**: `doc_transformer/src/main.rs`, lines 1018-1106

**Changes Made**:

1. After scrape completes (line 1072), added check for errors:
   - Check `result.error_count > 0` after scrape completes
   - If errors exist, print partial failure summary with success/error counts
   - Exit with code 2 using `process::exit(2)` to indicate partial/total failure

2. Modified success summary output (lines 1097-1103) to always include error count:
   - Added error_count to the summary output
   - This ensures users always see both success and error counts

**Code Changes**:

```rust
// After line 1072, after scrape completes:
// Check for partial/total failure - exit with code 2 if any errors
if result.error_count > 0 {
    println!();
    println!("{}", "=".repeat(70));
    println!("SCRAPE COMPLETE (PARTIAL FAILURE)");
    println!("{}", "=".repeat(70));
    println!("Success: {} pages", result.success_count);
    println!("Errors:  {} pages failed", result.error_count);
    println!();
    println!("Hint: Check .scrape/manifest.json for error details");
    println!("{}\n", "=".repeat(70));
    process::exit(2);
}

// Also modify the success summary to always show error_count (even if 0)
println!("\n{}", "=".repeat(70));
println!("SCRAPE COMPLETE");
println!("{}", "=".repeat(70));
println!("Output:  {}", output.display());
println!("Pages:   {} scraped", result.success_count);
if result.error_count > 0 {
    println!("Errors:  {} pages failed", result.error_count);
}
println!("Files:   .scrape/*.md + manifest.json");
println!("{}\n", "=".repeat(70));
```

### Exit Code Behavior

| Scenario | Exit Code | Behavior |
|----------|-----------|----------|
| Full success (`error_count == 0`) | 0 | Normal success summary |
| Partial failure (`success_count > 0 && error_count > 0`) | 2 | Partial failure summary + exit |
| Total failure (`success_count == 0 && error_count > 0`) | 2 | Partial failure summary + exit |

This aligns with the existing exit code convention in main.rs (lines 43-47):
- 0 = success
- 1 = user error (bad arguments)
- 2 = pipeline error

Partial/total scrape failure is a pipeline-level issue, so exit code 2 is appropriate.

## Clause Mapping

| Contract Clause | Implementation |
|-----------------|-----------------|
| UR-1: Non-zero exit on errors | Added `process::exit(2)` when `error_count > 0` |
| ER-1: Signal partial failure via exit status | Exit code 2 indicates partial/total failure |
| NR-1: Do not exit 0 with errors | Explicit check prevents zero exit on errors |
| Postcondition: Summary includes counts | Both success_count and error_count printed |

## Testing Notes

To verify this implementation:
1. Run scrape against a site that returns some errors
2. Verify exit code is 2 (not 0)
3. Verify summary shows both success and error counts

Example test scenarios:
- Site with mixed success/failure pages → exit code 2
- Site with all failures → exit code 2  
- Site with all success → exit code 0
