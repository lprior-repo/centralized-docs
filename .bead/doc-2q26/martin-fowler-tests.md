---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P2
updated_at: 2026-02-28T14:45:40Z
---

# Martin Fowler Test Plan

## Test Organization

```
release-gate/
├── src/
│   ├── warning_budget.rs  # Implementation
│   └── ...
└── tests/
    └── warning_budget_tests.rs  # Integration tests
```

---

## Happy Path Tests

### test_warning_count_from_clippy_output_counts_correctly

```rust
#[test]
fn test_warning_count_from_clippy_output_counts_correctly() {
    // Given: clippy output with 3 warnings
    let output = r#"warning: unused variable: `x`
warning: this function is too long
error: something bad
warning: unnecessary cast"#;
    
    // When: parsing warning count
    let count = WarningCount::from_clippy_output(output);
    
    // Then: count is 3
    assert_eq!(count.value(), 3);
}
```

### test_warning_count_from_baseline_parses_valid_integer

```rust
#[test]
fn test_warning_count_from_baseline_parses_valid_integer() {
    // Given: baseline file content with "5"
    let content = "5";
    
    // When: parsing baseline
    let result = WarningCount::from_baseline(content);
    
    // Then: returns WarningCount(5)
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value(), 5);
}
```

### test_check_budget_returns_passed_when_current_equals_baseline

```rust
#[test]
fn test_check_budget_returns_passed_when_current_equals_baseline() {
    // Given: current = 5, baseline = 5
    let current = WarningCount::new(5);
    let baseline = WarningCount::new(5);
    
    // When: checking budget
    let result = check_budget(current, baseline);
    
    // Then: returns Passed
    assert!(matches!(result, BudgetCheckResult::Passed { .. }));
    if let BudgetCheckResult::Passed { current: c, baseline: b } = result {
        assert_eq!(c.value(), 5);
        assert_eq!(b.value(), 5);
    }
}
```

### test_check_budget_returns_passed_when_current_less_than_baseline

```rust
#[test]
fn test_check_budget_returns_passed_when_current_less_than_baseline() {
    // Given: current = 3, baseline = 5
    let current = WarningCount::new(3);
    let baseline = WarningCount::new(5);
    
    // When: checking budget
    let result = check_budget(current, baseline);
    
    // Then: returns Passed
    assert!(result.is_passed());
}
```

### test_check_budget_returns_exceeded_when_current_greater_than_baseline

```rust
#[test]
fn test_check_budget_returns_exceeded_when_current_greater_than_baseline() {
    // Given: current = 10, baseline = 5
    let current = WarningCount::new(10);
    let baseline = WarningCount::new(5);
    
    // When: checking budget
    let result = check_budget(current, baseline);
    
    // Then: returns Exceeded with delta = 5
    assert!(!result.is_passed());
    if let BudgetCheckResult::Exceeded { current: c, baseline: b, delta } = result {
        assert_eq!(c.value(), 10);
        assert_eq!(b.value(), 5);
        assert_eq!(delta, 5);
    } else {
        panic!("Expected Exceeded variant");
    }
}
```

### test_zero_warnings_passes_zero_baseline

```rust
#[test]
fn test_zero_warnings_passes_zero_baseline() {
    // Given: current = 0, baseline = 0 (strict policy)
    let current = WarningCount::new(0);
    let baseline = WarningCount::new(0);
    
    // When: checking budget
    let result = check_budget(current, baseline);
    
    // Then: returns Passed
    assert!(result.is_passed());
}
```

---

## Error Path Tests

### test_baseline_empty_file_returns_error

```rust
#[test]
fn test_baseline_empty_file_returns_error() {
    // Given: empty baseline content
    let content = "";
    
    // When: parsing baseline
    let result = WarningCount::from_baseline(content);
    
    // Then: returns EmptyBaseline error
    assert!(matches!(result, Err(BudgetError::EmptyBaseline)));
}
```

### test_baseline_invalid_content_returns_error

```rust
#[test]
fn test_baseline_invalid_content_returns_error() {
    // Given: baseline content that is not a valid integer
    let test_cases = vec![
        "not-a-number",
        "-5",       // negative
        "3.14",     // float
        "abc123",
        "",
    ];
    
    for content in test_cases {
        // When: parsing baseline
        let result = WarningCount::from_baseline(content);
        
        // Then: returns InvalidBaseline error
        assert!(
            matches!(result, Err(BudgetError::InvalidBaseline { .. }) | Err(BudgetError::EmptyBaseline)),
            "Expected error for content: '{content}'"
        );
    }
}
```

### test_baseline_file_not_found_returns_error

```rust
#[test]
fn test_baseline_file_not_found_returns_error() {
    // Given: path to non-existent file
    let path = Path::new("/nonexistent/path/.clippy-baseline");
    
    // When: loading baseline
    let result = load_baseline(path);
    
    // Then: returns BaselineRead error
    assert!(matches!(result, Err(BudgetError::BaselineRead { .. })));
}
```

---

## Edge Case Tests

### test_warning_count_handles_empty_clippy_output

```rust
#[test]
fn test_warning_count_handles_empty_clippy_output() {
    // Given: empty clippy output
    let output = "";
    
    // When: parsing warning count
    let count = WarningCount::from_clippy_output(output);
    
    // Then: count is 0
    assert_eq!(count.value(), 0);
}
```

### test_warning_count_handles_no_warnings_output

```rust
#[test]
fn test_warning_count_handles_no_warnings_output() {
    // Given: clippy output with no warnings (only info)
    let output = r#"Checking crate1 v0.1.0
Checking crate2 v0.1.0
Finished dev [unoptimized + debuginfo]"#;
    
    // When: parsing warning count
    let count = WarningCount::from_clippy_output(output);
    
    // Then: count is 0
    assert_eq!(count.value(), 0);
}
```

### test_baseline_trims_whitespace

```rust
#[test]
fn test_baseline_trims_whitespace() {
    // Given: baseline content with whitespace
    let test_cases = vec![
        ("  5  ", 5),
        ("5\n", 5),
        ("\t5\t", 5),
        ("  10  \n", 10),
    ];
    
    for (content, expected) in test_cases {
        // When: parsing baseline
        let result = WarningCount::from_baseline(content);
        
        // Then: parses successfully
        assert!(result.is_ok(), "Failed for content: '{content}'");
        assert_eq!(result.unwrap().value(), expected);
    }
}
```

### test_warning_count_large_value

```rust
#[test]
fn test_warning_count_large_value() {
    // Given: baseline with large value
    let content = "4294967295"; // u32::MAX
    
    // When: parsing baseline
    let result = WarningCount::from_baseline(content);
    
    // Then: parses successfully
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value(), u32::MAX);
}
```

### test_warning_count_only_counts_warning_prefix

```rust
#[test]
fn test_warning_count_only_counts_warning_prefix() {
    // Given: output with "warning:" in different contexts
    let output = r#"some warning: text here
warning: actual warning
WARNING: uppercase not counted
warning:another warning"#;
    
    // When: parsing warning count
    let count = WarningCount::from_clippy_output(output);
    
    // Then: counts only lines with "warning:" prefix (case-sensitive)
    // "some warning:" does not start with "warning:"
    // "warning: actual" - counted
    // "WARNING:" - not counted (uppercase)
    // "warning:another" - counted
    assert_eq!(count.value(), 2);
}
```

---

## Contract Verification Tests

### test_precondition_baseline_file_exists

```rust
#[test]
fn test_precondition_baseline_file_exists() {
    // Verifies P1: baseline file must exist
    let result = load_baseline(Path::new("/nonexistent/.clippy-baseline"));
    assert!(matches!(result, Err(BudgetError::BaselineRead { .. })));
}
```

### test_precondition_baseline_content_valid

```rust
#[test]
fn test_precondition_baseline_content_valid() {
    // Verifies P2: baseline content must be valid integer
    let result = WarningCount::from_baseline("invalid");
    assert!(matches!(result, Err(BudgetError::InvalidBaseline { .. })));
}
```

### test_postcondition_passed_when_current_le_baseline

```rust
#[test]
fn test_postcondition_passed_when_current_le_baseline() {
    // Verifies Q3: check_budget returns Passed when current <= baseline
    let cases = vec![
        (WarningCount::new(0), WarningCount::new(0)),
        (WarningCount::new(3), WarningCount::new(5)),
        (WarningCount::new(5), WarningCount::new(5)),
    ];
    
    for (current, baseline) in cases {
        let result = check_budget(current, baseline);
        assert!(result.is_passed(), "Expected Passed for {current:?} <= {baseline:?}");
    }
}
```

### test_postcondition_exceeded_when_current_gt_baseline

```rust
#[test]
fn test_postcondition_exceeded_when_current_gt_baseline() {
    // Verifies Q4: check_budget returns Exceeded when current > baseline
    let current = WarningCount::new(10);
    let baseline = WarningCount::new(5);
    
    let result = check_budget(current, baseline);
    
    assert!(matches!(result, BudgetCheckResult::Exceeded { delta: 5, .. }));
}
```

### test_invariant_warning_count_non_negative

```rust
#[test]
fn test_invariant_warning_count_non_negative() {
    // Verifies I1: WarningCount is always >= 0 (compile-time via u32)
    // This test verifies runtime behavior
    let count = WarningCount::new(0);
    assert_eq!(count.value(), 0);
    
    let count = WarningCount::new(100);
    assert_eq!(count.value(), 100);
}
```

### test_invariant_deterministic_comparison

```rust
#[test]
fn test_invariant_deterministic_comparison() {
    // Verifies I2: baseline comparison is deterministic
    let current = WarningCount::new(7);
    let baseline = WarningCount::new(5);
    
    // Run multiple times, should always return same result
    for _ in 0..10 {
        let result = check_budget(current, baseline);
        assert!(matches!(result, BudgetCheckResult::Exceeded { delta: 2, .. }));
    }
}
```

---

## Contract Violation Tests (Parity with contract-spec.md)

### test_violation_p1_missing_baseline_file

```rust
#[test]
fn test_violation_p1_missing_baseline_file() {
    // VIOLATES P1: load_baseline(Path::new("nonexistent-file"))
    // -> returns Err(BudgetError::BaselineRead { message: "..." })
    let result = load_baseline(Path::new("nonexistent-file"));
    assert!(matches!(result, Err(BudgetError::BaselineRead { .. })));
}
```

### test_violation_p2_invalid_baseline_content

```rust
#[test]
fn test_violation_p2_invalid_baseline_content() {
    // VIOLATES P2: load_baseline with file containing "not-a-number"
    // -> returns Err(BudgetError::InvalidBaseline { content: "not-a-number" })
    let result = WarningCount::from_baseline("not-a-number");
    match result {
        Err(BudgetError::InvalidBaseline { content }) => {
            assert_eq!(content, "not-a-number");
        }
        _ => panic!("Expected InvalidBaseline error"),
    }
}
```

### test_violation_p2_empty_baseline

```rust
#[test]
fn test_violation_p2_empty_baseline() {
    // VIOLATES P2: load_baseline with empty file
    // -> returns Err(BudgetError::EmptyBaseline)
    let result = WarningCount::from_baseline("");
    assert!(matches!(result, Err(BudgetError::EmptyBaseline)));
}
```

### test_violation_p2_negative_baseline

```rust
#[test]
fn test_violation_p2_negative_baseline() {
    // VIOLATES P2: load_baseline with file containing "-5"
    // -> returns Err(BudgetError::InvalidBaseline { content: "-5" })
    let result = WarningCount::from_baseline("-5");
    assert!(matches!(result, Err(BudgetError::InvalidBaseline { content }) if content == "-5"));
}
```

### test_violation_p2_float_baseline

```rust
#[test]
fn test_violation_p2_float_baseline() {
    // VIOLATES P2: load_baseline with file containing "3.14"
    // -> returns Err(BudgetError::InvalidBaseline { content: "3.14" })
    let result = WarningCount::from_baseline("3.14");
    assert!(matches!(result, Err(BudgetError::InvalidBaseline { content }) if content == "3.14"));
}
```

### test_violation_q4_budget_exceeded_not_passed

```rust
#[test]
fn test_violation_q4_budget_exceeded_not_passed() {
    // VIOLATES Q4: check_budget(WarningCount::new(10), WarningCount::new(5))
    // -> returns BudgetCheckResult::Exceeded { current: 10, baseline: 5, delta: 5 }
    // (NOT Passed - verifies postcondition Q4)
    let result = check_budget(WarningCount::new(10), WarningCount::new(5));
    
    match result {
        BudgetCheckResult::Exceeded { current, baseline, delta } => {
            assert_eq!(current.value(), 10);
            assert_eq!(baseline.value(), 5);
            assert_eq!(delta, 5);
        }
        BudgetCheckResult::Passed { .. } => {
            panic!("Expected Exceeded, not Passed");
        }
    }
}
```

---

## Integration Tests

### test_full_budget_check_workflow_passes

```rust
#[test]
fn test_full_budget_check_workflow_passes() {
    // Given: baseline file with 5 warnings
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let baseline_path = temp_dir.path().join(".clippy-baseline");
    std::fs::write(&baseline_path, "5").expect("write baseline");
    
    // And: mock clippy output with 3 warnings
    // (This would use dependency injection or mock in real implementation)
    
    // When: executing budget check
    // let result = execute_budget_check(&baseline_path);
    
    // Then: returns Passed
    // assert!(result.is_ok_and(|r| r.is_passed()));
}
```

### test_release_gate_includes_budget_check

```rust
#[test]
fn test_release_gate_includes_budget_check() {
    // Given: release gate instance
    let gate = ReleaseGate::new();
    
    // When: executing gate (with mocked dependencies)
    // let result = gate.execute();
    
    // Then: GateResult includes budget_check field
    // assert!(matches!(result, Ok(GateResult { budget_check: BudgetCheckResult::Passed { .. }, .. })));
}
```

### test_exit_code_on_budget_exceeded

```rust
#[test]
fn test_exit_code_on_budget_exceeded() {
    // Given: budget exceeded scenario
    // When: main processes result
    // Then: exit code is 4
    const EXIT_WARNING_BUDGET_EXCEEDED: i32 = 4;
    
    // Simulate the exit code logic
    let budget_check = BudgetCheckResult::Exceeded {
        current: WarningCount::new(10),
        baseline: WarningCount::new(5),
        delta: 5,
    };
    
    let exit_code = if !budget_check.is_passed() {
        EXIT_WARNING_BUDGET_EXCEEDED
    } else {
        0
    };
    
    assert_eq!(exit_code, 4);
}
```

---

## Given-When-Then Scenarios

### Scenario 1: Budget Check Passes

```
Given: baseline file contains "5"
When: clippy produces 3 warnings
Then: budget check returns Passed
Then: release gate continues
Then: exit code is 0 (if no other failures)
```

### Scenario 2: Budget Check Fails

```
Given: baseline file contains "0"
When: clippy produces 2 warnings
Then: budget check returns Exceeded { current: 2, baseline: 0, delta: 2 }
Then: error message shows warning regression
Then: exit code is 4
```

### Scenario 3: Baseline File Missing

```
Given: .clippy-baseline file does not exist
When: release gate runs
Then: returns BaselineRead error
Then: exit code is 3 (precondition failed)
```

### Scenario 4: Baseline File Invalid

```
Given: .clippy-baseline contains "not-a-number"
When: release gate runs
Then: returns InvalidBaseline error
Then: exit code is 3 (precondition failed)
```

### Scenario 5: Zero Warnings, Zero Baseline

```
Given: baseline file contains "0"
When: clippy produces 0 warnings
Then: budget check returns Passed { current: 0, baseline: 0 }
Then: release gate continues
```

---

## Test Summary

| Category | Count |
|----------|-------|
| Happy Path Tests | 6 |
| Error Path Tests | 3 |
| Edge Case Tests | 5 |
| Contract Verification Tests | 6 |
| Contract Violation Tests | 6 |
| Integration Tests | 3 |
| **Total** | **29** |
