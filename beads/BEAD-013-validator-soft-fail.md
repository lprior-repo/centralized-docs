# BEAD-013: INDEX Validator Soft-Fails on Data Corruption

**Epic**: Quality Assurance
**Severity**: Critical
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/bin/llms_txt_validator.rs`
- **The Smell**: The validator returns exit code 0 (success) even when it reports errors. The validator says "Validation passed" despite finding duplicate chunk IDs. This is misleading and allows corrupted data to pass CI checks.

**Evidence**:
```bash
$ ./target/release/llms_txt_validator --index ./corrupted_index.json
# 📊 Found 372 errors, 0 warnings, 0 info
# ❌ [ERROR] chunks: Duplicate chunk ID: ops/general/community#0
# (372 errors listed)
# ============================================================
# ❌ Validation failed
# $ echo $?
# 0  # Exit code is STILL ZERO!

$ git push  # Pushes corrupted data to production
```

**User Impact**:
- Corrupted INDEX.json files pass CI/CD checks
- Invalid data reaches production environments
- AI agents silently receive duplicate/corrupted indices
- No automated gate prevents shipping bad data
- False confidence in validation results

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Validation finds any error | `llms_txt_validator` | Exit with non-zero code (1 or higher) |
| Validation finds only warnings | `llms_txt_validator` | Exit with zero but print warnings |
| Validation finds no errors | `llms_txt_validator` | Exit zero, print success message |
| Duplicate chunk IDs detected | `llms_txt_validator` | Fail hard, exit code 1, report count |

### 2. DbC (Design by Contract)

**Preconditions**:
- INDEX.json file path is provided
- File exists and is readable
- JSON is parseable

**Postconditions**:
- If any errors found → exit code >= 1
- If only warnings → exit code 0
- If no errors → exit code 0
- Error count reported accurately
- Exit code reflects severity of issues

**Invariants**:
- Exit code 0 means "safe to ship"
- Exit code != 0 means "do not ship"
- Validation output is accurate to exit code

### 3. Schema & Edge Cases

**Exit Code Specification**:
| Error Count | Exit Code | Meaning |
|-------------|-----------|---------|
| 0 errors | 0 | Success, safe to ship |
| 1-10 errors | 1 | Data corruption detected |
| 11-100 errors | 2 | Severe corruption |
| >100 errors | 3 | Critical corruption |
| Parse errors (invalid JSON) | 4 | File is unreadable |

**Expected Behavior**:
```bash
# No errors
$ ./target/release/llms_txt_validator --index good.json
# 📊 Found 0 errors, 0 warnings, 0 info
# ============================================================
# ✅ Validation passed
# $ echo $?  # 0

# With errors
$ ./target/release/llms_txt_validator --index bad.json
# 📊 Found 372 errors, 0 warnings, 0 info
# ============================================================
# ❌ Validation failed: 372 errors found
# $ echo $?  # 3 (not 0!)

# CI fails
$ if ./target/release/llms_txt_validator --index bad.json; then git push; fi
# (exits early, no push)
```

**Edge Cases**:
| Scenario | Expected Exit Code | Expected Output |
|----------|-------------------|-----------------|
| 372 duplicate chunks | 3 | "❌ Validation failed: 372 errors found" |
| Missing required section | 1 | "⚠️  Validation passed with warnings" (if only warnings) |
| Invalid JSON | 4 | "Parse error: invalid JSON syntax" |
| File not found | 5 | "Error: file not found" |
| Zero errors, 2 warnings | 0 | "✅ Validation passed with warnings" |

---

## FIX LOCATIONS

1. **`doc_transformer/src/bin/llms_txt_validator.rs`** - Main validator logic
   - Add error tracking counter
   - Set exit code based on error count at end of main
   - Replace soft failures with hard failures on errors

2. **`src/main.rs`** - Validation integration
   - Check exit code from validator in CI
   - Fail build if validator returns non-zero

---

## TEST CASES

```rust
#[test]
fn test_validator_returns_nonzero_on_duplicates() {
    let output = Command::new("./target/release/llms_txt_validator")
        .args(["--index", "./test_data/duplicates.json"])
        .output()
        .unwrap();

    assert!(!output.status.success()); // Exit code must be non-zero
    assert_eq!(output.status.code(), Some(3)); // Critical corruption

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation failed"));
    assert!(stderr.contains("372 errors"));
}

#[test]
fn test_validator_zero_on_success() {
    let output = Command::new("./target/release/llms_txt_validator")
        .args(["--index", "./test_data/good.json"])
        .output()
        .unwrap();

    assert!(output.status.success()); // Exit code must be zero
    assert_eq!(output.status.code(), Some(0));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Validation passed"));
}

#[test]
fn test_validator_ci_gate() {
    // Simulate CI check
    let status = Command::new("./target/release/llms_txt_validator")
        .args(["--index", "./test_data/duplicates.json"])
        .status()
        .unwrap();

    // CI should fail
    assert!(!status.success());
}

#[test]
fn test_validator_exit_codes() {
    let test_cases = vec![
        ("good.json", 0),
        ("minor_errors.json", 1),
        ("major_errors.json", 2),
        ("critical_errors.json", 3),
    ];

    for (file, expected_code) in test_cases {
        let output = Command::new("./target/release/llms_txt_validator")
            .args(["--index", &format!("./test_data/{}", file)])
            .output()
            .unwrap();

        assert_eq!(
            output.status.code(),
            Some(expected_code),
            "Exit code mismatch for {}",
            file
        );
    }
}
```

---

## VERIFICATION

After fix:
```bash
# Test with corrupted data
$ ./target/release/llms_txt_validator --index corrupted.json
# 📊 Found 372 errors, 0 warnings, 0 info
# ============================================================
# ❌ Validation failed: 372 errors found - DO NOT SHIP
# $ echo $?
# 3

# CI fails properly
$ ./target/release/doc_transformer index ./docs ./output
$ ./target/release/llms_txt_validator --index ./output/INDEX.json
# ❌ Validation failed: 372 errors found - DO NOT SHIP
# $ git push  # Would fail in real CI with pre-commit hook

# Test with good data
$ ./target/release/llms_txt_validator --index good.json
# 📊 Found 0 errors, 0 warnings, 0 info
# ============================================================
# ✅ Validation passed - safe to ship
# $ echo $?
# 0
```
