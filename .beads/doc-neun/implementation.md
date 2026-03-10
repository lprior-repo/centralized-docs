# Implementation: doc-neun - Standardize Exit Codes

## Summary

This fix standardizes exit codes between `llms_txt_validator` and `doc_transformer` to ensure consistent CLI behavior. Both binaries now use:
- Exit 0: Success
- Exit 1: User error (bad arguments, missing files, invalid input)
- Exit 2: Pipeline error (internal failures)

## Changes Made

### File: `doc_transformer/src/bin/llms_txt_validator.rs`

#### 1. File Not Found (Line 568-571)
**Before:**
```rust
if !path.exists() {
    eprintln!("Error: file not found: {}", path.display());
    std::process::exit(5);
}
```

**After:**
```rust
if !path.exists() {
    eprintln!("Error: file not found: {}", path.display());
    std::process::exit(1);  // User error: file not found is user input issue
}
```

#### 2. JSON Parse Error (Lines 583-591)
**Before:**
```rust
let parse_result: Result<IndexJson, _> = serde_json::from_str(&content);
match parse_result {
    Ok(_) => validate_index_json(&path)?,
    Err(e) => {
        // Invalid JSON is a parse error - exit code 4
        eprintln!("Error: Parse error (invalid JSON): {}", e);
        std::process::exit(4);
    }
}
```

**After:**
```rust
let parse_result: Result<IndexJson, _> = serde_json::from_str(&content);
match parse_result {
    Ok(_) => validate_index_json(&path)?,
    Err(e) => {
        // Invalid JSON is a user input error - exit code 1
        eprintln!("Error: Parse error (invalid JSON): {}", e);
        std::process::exit(1);
    }
}
```

#### 3. Validation Errors (Lines 598-610)
**Before:**
```rust
if error_count > 0 {
    let exit_code = match error_count {
        1..=10 => 1,
        11..=100 => 2,
        _ => 3,
    };
    std::process::exit(exit_code);
}
```

**After:**
```rust
if error_count > 0 {
    // All validation errors are user input issues (invalid content)
    // Consistent with doc_transformer: exit 1 for user errors
    std::process::exit(1);
}
```

## Rationale

| Error Type | Old Code | New Code | Reason |
|------------|----------|----------|--------|
| File not found | Exit 5 | Exit 1 | User error - file path is user input |
| JSON parse error | Exit 4 | Exit 1 | User error - invalid input format |
| Validation errors (any count) | Exit 1/2/3 | Exit 1 | User error - content doesn't meet spec |

The standardized exit codes allow scripts to reliably detect:
- Success (0) vs failure (non-zero)
- User input errors (1) vs pipeline errors (2)

## Test Verification

After building, verify the fix:
```bash
# Test file not found -> exit 1
./target/release/llms_txt_validator /nonexistent/file.txt
echo "Exit code: $?"  # Should be 1

# Test valid file -> exit 0
./target/release/llms_txt_validator valid_llms.txt
echo "Exit code: $?"  # Should be 0

# Test invalid JSON -> exit 1
echo "not valid json" > /tmp/bad.json
./target/release/llms_txt_validator --index /tmp/bad.json
echo "Exit code: $?"  # Should be 1
```

## Consistency Check

Both binaries now use the same exit code scheme:

| Exit Code | Meaning | llms_txt_validator | doc_transformer |
|-----------|---------|-------------------|-----------------|
| 0 | Success | ✅ | ✅ |
| 1 | User error | ✅ (fixed) | ✅ |
| 2 | Pipeline error | N/A (no such errors) | ✅ |
