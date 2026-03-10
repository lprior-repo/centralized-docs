# Test Defects: doc-6i5 (ReDoS Pattern Detection)

## Status: REJECTED

---

## Critical Defects (Must Fix)

### 1. Missing Integration/E2E Tests
**Doctrine Violated:** Testing Trophy (Real Execution)

The test plan contains ONLY unit tests for `validate_filter_regex`. The Testing Trophy demands "tremendous amounts of integration and E2E tests that validate the system actually works."

**Defect:** Zero integration or end-to-end tests described.

**Impact:** Cannot verify:
- The CLI actually uses the validation correctly
- The system doesn't hang when given dangerous ReDoS input in production
- The error messages propagate correctly to users

**Required Fix:** Add integration tests that:
- Run the actual CLI with dangerous regex filters
- Verify the process terminates within 100ms
- Verify error output contains "ReDoS" for dangerous patterns

---

### 2. Missing Function Coverage: compile_safe_regex
**Doctrine Violated:** Kent Beck (TDD) - Exhaustive Coverage

The contract-spec.md (lines 36-44) explicitly specifies TWO functions:
```
### main.rs::validate_filter_regex
fn validate_filter_regex(pattern: &str) -> Result<(), String>

### scrape/validation.rs::compile_safe_regex
pub(crate) fn compile_safe_regex(pattern: &str) -> Result<Regex>
```

The martin-fowler-tests.md only tests `validate_filter_regex`. The `compile_safe_regex` function has ZERO test coverage.

**Required Fix:** Add tests for `compile_safe_regex`:
- Happy path: valid patterns compile successfully
- Error path: ReDoS patterns return Err
- Edge case: invalid regex syntax returns Err

---

### 3. Test Names Not BDD-Style
**Doctrine Violated:** Dan North (BDD) - Executable Specifications

Current test names (martin-fowler-tests.md lines 3-29):
```
test_returns_ok_for_valid_simple_pattern
test_rejects_canonical_redos_a_plus_a_plus
test_accepts_empty_string
```

BDD requires behavior-describing names that read like specifications:
```
given_user_provides_valid_simple_pattern_when_validating_then_returns_ok
given_user_provides_nested_quantifier_when_validating_then_returns_redos_error
```

**Required Fix:** Rename all tests to BDD Given-When-Then format.

---

## High Priority Defects

### 4. No DSL Layer
**Doctrine Violated:** Dave Farley (ATDD) - Separation of WHAT from HOW

ATDD requires a DSL that allows non-technical stakeholders to read and write tests. The current plan is prose documentation, not a reusable specification language.

**Required Fix:** Create a simple DSL structure:
```rust
// Example DSL concept
validate_regex("(a+)+")    .expect_error_containing("ReDoS");
validate_regex("^/docs/.*") .expect_ok();
```

---

### 5. Incomplete ReDoS Pattern Coverage
**Doctrine Violated:** Combinatorial Permutations

Only 8 ReDoS patterns are tested. Many dangerous nested quantifier variants exist:
- `(a{1,3})+` - bounded quantifier nested
- `(a|b|c)+` - multiple alternations nested
- `((a+)+)+` - doubly nested
- `(\d+\.\d+)+` - character class with quantifier nested

**Required Fix:** Add more ReDoS pattern variants to test.

---

## Medium Priority Defects

### 6. Missing Advanced Testing Paradigms
**Doctrine Violated:** Advanced Testing (Property-based, Fuzz, Mutation)

No consideration for:
- **Property-based testing**: Generate arbitrary valid regexes and verify they pass
- **Fuzz testing**: Feed random strings to the ReDoS detector to find edge cases
- **Mutation testing**: Verify the detector catches intentionally modified ReDoS patterns

**Required Fix:** Add property-based tests for valid regex invariants.

---

## Summary

| Defect | Severity | Doctrine |
|--------|----------|----------|
| No Integration/E2E tests | CRITICAL | Testing Trophy |
| Missing compile_safe_regex tests | CRITICAL | TDD Exhaustiveness |
| Non-BDD test names | HIGH | Dan North BDD |
| No DSL layer | HIGH | Dave Farley ATDD |
| Incomplete ReDoS patterns | MEDIUM | Combinatorial |
| No advanced testing | MEDIUM | Advanced Paradigms |

**Verdict:** This test plan fails to meet the Testing Trophy, BDD, and ATDD doctrines. The system cannot be proven to work in reality without integration tests, and the contract is not fully tested.
