# Implementation: Move Validation Before Artifact Writing

## Problem
The index command writes artifacts (INDEX.json, COMPASS.md, llms.txt) BEFORE validation runs. If validation fails, artifacts remain on disk but the command exits with failure.

## Solution
Move the validation step to run BEFORE artifact writing (steps 6 and 7). This ensures:
1. If validation fails, no artifacts are written
2. Clean atomic failure

## Files Changed

### 1. `doc_transformer/src/main.rs`

**Summary of changes:**
- Moved validation from STEP 8 to STEP 6 (between CHUNK and INDEX)
- Added early bail on validation failure before any artifacts written
- Renumbered steps: INDEX is now STEP 7, LLMS is now STEP 8
- Removed duplicate validation code at end of function

**Key code change (lines ~1587-1605):**
```rust
// STEP 6: VALIDATE (before artifact writing - ensures atomic failure)
println!("[STEP 6] VALIDATE");
let validation_result = validate::validate_all(output)?;
println!(
    "  {}/{} files passed ({} errors, {} warnings)\n",
    validation_result.files_passed,
    validation_result.files_checked,
    validation_result.total_errors,
    validation_result.total_warnings
);

// Bail early if validation fails - no artifacts written yet
if validation_result.total_errors > 0 {
    anyhow::bail!(
        "Validation failed: {} errors found across {} files",
        validation_result.total_errors,
        validation_result.files_checked
    );
}

// STEP 7: INDEX + GRAPH (formerly STEP 6)
println!("[STEP 7] INDEX + GRAPH");
index::build_and_write_index(...);  // Writes INDEX.json
index::build_and_write_compass(...); // Writes COMPASS.md
```

### 2. Test File: `doc_transformer/tests/validation_atomicity_tests.rs`

Created new test file with two tests:

1. **test_validation_failure_prevents_artifacts**: Verifies that when validation fails, no artifacts (INDEX.json, COMPASS.md, llms.txt) are written to disk.

2. **test_validation_success_allows_artifacts**: Verifies that when validation passes, artifacts are correctly written.

## Step-by-Step Implementation

### Step 1: Modify main.rs

1. Moved validation from after STEP 7 to between STEP 5 (CHUNK) and STEP 6 (INDEX)
2. Added early bail if validation fails - this prevents artifact writing
3. Renumbered steps accordingly (validation = STEP 6, INDEX = STEP 7, LLMS = STEP 8)
4. Kept validation summary in final output for user visibility
5. Removed duplicate validation code that was at the end

### Step 2: Add Test

Created `doc_transformer/tests/validation_atomicity_tests.rs` with:
- Test that validates failure prevents artifacts (by manually adding invalid file)
- Test that validation success allows artifacts

## Verification

Run the new tests:
```bash
cd doc_transformer
cargo test --test validation_atomicity_tests
```

Expected output:
```
running 2 tests
test test_validation_failure_prevents_artifacts ... ok
test test_validation_success_allows_artifacts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Run full test suite:
```bash
cargo test --test cli_integration_tests
```

All 66 tests pass.

## Quality Gates

- `cargo fmt --check` - PASS
- `cargo clippy -- -D warnings` - PASS
- `cargo test --test validation_atomicity_tests` - PASS (2 tests)
- `cargo test --test cli_integration_tests` - PASS (66 tests)

## Key Design Decisions

1. **Validation before artifacts**: By moving validation before artifact writing, we ensure atomic behavior - either all artifacts are valid and written, or nothing is written.

2. **Early bail**: Using `anyhow::bail!` for early exit on validation failure - consistent with existing error handling in the codebase.

3. **Test coverage**: Added explicit tests for the atomic behavior, verifying both positive (validation passes) and negative (validation fails) cases.

4. **Backward compatibility**: The final output still shows validation results in the summary, maintaining user visibility into validation status.
