# Traceability Matrix: cli: Error Message Format Inconsistency (doc-3o2b)

## Overview

This document maps the contract specifications to test cases and source files for the bug fix addressing incorrect "Warning:" prefix usage for error conditions.

## Bug Summary

**Issue**: Error messages should consistently start with "Error:" prefix but some locations incorrectly use "Warning:" prefix for actual error conditions.

**Impact**: 
- Users cannot programmatically distinguish errors from warnings via stderr prefix
- Shell scripts may incorrectly treat errors as non-fatal
- Violates consistency principle for CLI error handling

## Contract to Test Mapping

### Precondition: P1 - Error conditions use Error: prefix

| Contract Clause | Violation ID | Source File | Line(s) | Current Code | Test Case |
|-----------------|--------------|--------------|---------|--------------|-----------|
| P1: Error conditions use Error: prefix | P1-1 | `discover.rs` | 70 | `Warning: Skipping path due to I/O error: {e}` | `test_discover_io_error_uses_error_prefix_not_warning` |
| P1: Error conditions use Error: prefix | P1-2 | `discover.rs` | 127 | `Warning: Skipping empty file {path}` | `test_discover_empty_file_uses_error_prefix_not_warning` |
| P1: Error conditions use Error: prefix | P1-3 | `index.rs` | 430 | `Warning: Failed to build Tantivy index: {e}` | `test_index_tantivy_failure_uses_error_prefix_not_warning` |
| P1: Error conditions use Error: prefix | P1-4 | `index.rs` | 831 | `Warning: HNSW index build failed ({e})` | `test_index_hnsw_failure_uses_error_prefix_not_warning` |
| P1: Error conditions use Error: prefix | P1-5 | `filter.rs` | 650 | `Warning: Skipping path due to I/O error: {e}` | `test_filter_io_error_uses_error_prefix_not_warning` |

### Postcondition: Q1 - Error messages MUST start with "Error:"

| Contract Clause | Test Case | Verification Method |
|-----------------|-----------|---------------------|
| Q1: All error messages start with "Error:" | `test_all_error_messages_start_with_error_prefix` | Grep stderr for `^Error: ` pattern |
| Q1: All error messages start with "Error:" | `test_discover_io_error_uses_error_prefix_not_warning` | Assert stderr contains "Error:" not "Warning:" |
| Q1: All error messages start with "Error:" | `test_discover_empty_file_uses_error_prefix_not_warning` | Assert stderr contains "Error:" not "Warning:" |
| Q1: All error messages start with "Error:" | `test_index_tantivy_failure_uses_error_prefix_not_warning` | Assert stderr contains "Error:" not "Warning:" |
| Q1: All error messages start with "Error:" | `test_index_hnsw_failure_uses_error_prefix_not_warning` | Assert stderr contains "Error:" not "Warning:" |
| Q1: All error messages start with "Error:" | `test_filter_io_error_uses_error_prefix_not_warning` | Assert stderr contains "Error:" not "Warning:" |

### Postcondition: Q3 - No error uses Warning: prefix

| Contract Clause | Test Case | Verification Method |
|-----------------|-----------|---------------------|
| Q3: Error conditions must not use Warning: | `test_all_warning_messages_start_with_warning_prefix` | Verify legitimate warnings still work |
| Q3: Error conditions must not use Warning: | All 5 P1 violation tests | Assert "Warning:" NOT present for error conditions |

## File to Fix Mapping

| File | Line(s) | Current (Incorrect) | Fixed (Correct) | Test Coverage |
|------|---------|---------------------|-----------------|---------------|
| `doc_transformer/src/discover.rs` | 70 | `eprintln!("Warning: Skipping path due to I/O error: {e}");` | `eprintln!("Error: Skipping path due to I/O error: {e}");` | P1-1 |
| `doc_transformer/src/discover.rs` | 127 | `eprintln!("Warning: Skipping empty file {}", path.display());` | `eprintln!("Error: Skipping empty file {}", path.display());` | P1-2 |
| `doc_transformer/src/index.rs` | 430 | `eprintln!("Warning: Failed to build Tantivy index: {e}");` | `eprintln!("Error: Failed to build Tantivy index: {e}");` | P1-3 |
| `doc_transformer/src/index.rs` | 831 | `eprintln!("Warning: HNSW index build failed ({e}), skipping related chunk edges");` | `eprintln!("Error: HNSW index build failed ({e}), skipping related chunk edges");` | P1-4 |
| `doc_transformer/src/filter.rs` | 650 | `eprintln!("Warning: Skipping path due to I/O error: {e}");` | `eprintln!("Error: Skipping path due to I/O error: {e}");` | P1-5 |

## Test Execution Order

### Phase 1: Pre-Fix Tests (Should Fail)
1. `test_discover_io_error_uses_error_prefix_not_warning` - Verify current behavior is wrong
2. `test_discover_empty_file_uses_error_prefix_not_warning` - Verify current behavior is wrong
3. `test_index_tantivy_failure_uses_error_prefix_not_warning` - Verify current behavior is wrong
4. `test_index_hnsw_failure_uses_error_prefix_not_warning` - Verify current behavior is wrong
5. `test_filter_io_error_uses_error_prefix_not_warning` - Verify current behavior is wrong

### Phase 2: Fix Application
Apply the 5 code changes listed in "File to Fix Mapping" section.

### Phase 3: Post-Fix Tests (Should Pass)
1. All 5 violation tests from Phase 1
2. `test_all_error_messages_start_with_error_prefix` - General contract verification
3. `test_all_warning_messages_start_with_warning_prefix` - Ensure legitimate warnings still work

## Verification Commands

### Manual Verification
```bash
# Test discover.rs:70 equivalent (permission denied scenario)
mkdir -p /tmp/test_perm && chmod 000 /tmp/test_perm
doc_transformer index /tmp/test_perm --output /tmp/out 2>&1 | grep "^Error:" || echo "FAIL: Missing Error: prefix"
chmod 755 /tmp/test_perm && rm -rf /tmp/test_perm /tmp/out

# Test index.rs:430 equivalent (Tantivy build failure)
# Would require mocking disk full or other index failure conditions

# Test filter.rs:650 equivalent
# Same as discover.rs:70 - permission denied scenario
```

### Automated Test Pattern
```rust
#[test]
fn test_discover_io_error_uses_error_prefix_not_warning() {
    // Setup: Create scenario that triggers I/O error
    // Execute: Run index command
    // Assert: stderr starts with "Error:" NOT "Warning:"
    let output = Command::new("doc_transformer")
        .args(["index", "/path", "--output", "/tmp/out"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Skipping path due to I/O error:"),
            "Expected 'Error:' prefix, got: {}", stderr);
    assert!(!stderr.contains("Warning: Skipping path due to I/O error:"),
            "Should NOT contain 'Warning:' prefix for error condition");
}
```

## Exit Criteria

- [ ] All 5 code changes applied (Warning: -> Error:)
- [ ] All 5 violation tests pass
- [ ] Contract verification test `test_all_error_messages_start_with_error_prefix` passes
- [ ] Legitimate warnings still work (test_all_warning_messages_start_with_warning_prefix)
- [ ] No regressions in existing error message tests

## Related Documents

- `contract-spec.md` - Full contract specification including Q1, Q3 invariants
- `martin-fowler-tests.md` - Complete test plan with Given-When-Then scenarios
