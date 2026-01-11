# HNSW Parameters CLI Arguments - Test Plan

## Implementation Summary

Added three new CLI arguments to the `index` command:
- `--max-related-chunks <N>` (range: 1-100, default: 20)
- `--hnsw-m <M>` (range: 4-64, default: 16)
- `--hnsw-ef-construction <EF>` (range: 50-800, default: 200)

## Validation Rules

### max_related_chunks
- **Valid range:** 1-100
- **Error cases:**
  - < 1: "max_related_chunks must be at least 1"
  - > 100: "max_related_chunks must be at most 100"
  - Non-integer: "max_related_chunks must be a positive integer, got '...'"

### hnsw_m
- **Valid range:** 4-64
- **Error cases:**
  - < 4: "hnsw_m must be at least 4 for proper connectivity (too sparse otherwise)"
  - > 64: "hnsw_m must be at most 64 for reasonable performance"
  - Non-integer: "hnsw_m must be a positive integer, got '...'"

### hnsw_ef_construction
- **Valid range:** 50-800
- **Error cases:**
  - < 50: "hnsw_ef_construction must be at least 50 for acceptable build quality"
  - > 800: "hnsw_ef_construction must be at most 800 for reasonable build times"
  - Non-integer: "hnsw_ef_construction must be a positive integer, got '...'"

## Test Cases

### Valid Inputs
1. `--max-related-chunks 1` → OK
2. `--max-related-chunks 50` → OK
3. `--max-related-chunks 100` → OK
4. `--hnsw-m 4` → OK
5. `--hnsw-m 16` → OK
6. `--hnsw-m 64` → OK
7. `--hnsw-ef-construction 50` → OK
8. `--hnsw-ef-construction 200` → OK
9. `--hnsw-ef-construction 800` → OK
10. All three combined → OK

### Invalid Inputs (Should Error)
1. `--max-related-chunks 0` → Error
2. `--max-related-chunks 101` → Error
3. `--max-related-chunks -1` → Error
4. `--max-related-chunks abc` → Error
5. `--hnsw-m 3` → Error
6. `--hnsw-m 65` → Error
7. `--hnsw-m -1` → Error
8. `--hnsw-m abc` → Error
9. `--hnsw-ef-construction 49` → Error
10. `--hnsw-ef-construction 801` → Error
11. `--hnsw-ef-construction -1` → Error
12. `--hnsw-ef-construction abc` → Error

## Execution Instructions

```bash
# Navigate to project
cd /home/lewis/src/centralized-docs/doc_transformer

# Build the project
cargo build --release

# Test valid inputs
./target/release/doc_transformer --help
./target/release/doc_transformer index --help

# Manual test with valid values
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --max-related-chunks 20 --hnsw-m 16 --hnsw-ef-construction 200

# Test edge cases (expect to see errors from clap)
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --max-related-chunks 0
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --max-related-chunks 101
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --hnsw-m 3
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --hnsw-m 65
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --hnsw-ef-construction 49
./target/release/doc_transformer index /path/to/source --output /tmp/test_output --hnsw-ef-construction 801
```

## Expected Output

When parameters are valid:
```
[CONFIG] Graph Parameters:
  max_related_chunks: <value> (default: 20)
  hnsw_m: <value> (default: 16)
  hnsw_ef_construction: <value> (default: 200)
```

When parameters are invalid:
Clap will display an error message and exit with code 2.

## Implementation Files Modified

1. `/home/lewis/src/centralized-docs/doc_transformer/src/main.rs`
   - Added validation functions: `validate_max_related_chunks`, `validate_hnsw_m`, `validate_hnsw_ef_construction`
   - Added arguments to `Commands::Index` struct
   - Updated `run_index` function signature to accept new parameters
   - Added logging for parameters in run_index function
   - Updated all call sites to pass new parameters

## Verification Checklist

- [x] All validation functions implemented
- [x] All arguments added to clap struct
- [x] All function signatures updated
- [x] All call sites updated
- [x] Compilation successful (cargo build)
- [x] Help text displays new arguments
- [x] Valid inputs accepted and logged
- [x] Invalid inputs rejected by clap
- [x] Edge cases handled correctly
- [x] No panic on invalid input
