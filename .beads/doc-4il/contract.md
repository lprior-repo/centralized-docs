# Contract Specification: ReDoS Pattern Detection (doc-4il)

## Context
- **Feature**: Fix incomplete ReDoS pattern detection in filter regex validation
- **Domain terms**:
  - ReDoS = Regular Expression Denial of Service (catastrophic backtracking)
  - Nested quantifier = a quantifier (+, *, {n,m}) applied to a group that already has a quantifier
- **Parent Bead**: doc-6i5 (rejected by test-reviewer)
- **Assumptions**:
  - User-provided regex filters should be validated for ReDoS patterns before compilation
  - The validation should be fast (not itself cause catastrophic backtracking)
  - Both `validate_filter_regex` (main.rs) and `compile_safe_regex` (scrape/validation.rs) must be tested
- **Open questions**:
  - Is there an existing CLI binary name? (Assuming `doc_transformer`)

## Preconditions
- [P1] Pattern is a valid string provided by caller
- [P2] Pattern length check happens BEFORE ReDoS check (to prevent ReDoS on the detector itself)
- [P3] Pattern must be valid regex syntax before ReDoS detection runs

## Postconditions
- [Q1] Function returns Ok(()) for safe regex patterns
- [Q2] Function returns Err with "ReDoS" message for dangerous nested quantifier patterns
- [Q3] Function returns Err with "too long" message for patterns > 500 chars
- [Q4] Detection completes in < 100ms for any input
- [Q5] `compile_safe_regex` returns compiled `Regex` for valid patterns

## Invariants
- [I1] No ReDoS pattern ever compiles successfully
- [I2] Validation regex itself cannot cause ReDoS (use bounded patterns)
- [I3] Both exported functions (`validate_filter_regex` and `compile_safe_regex`) have test coverage

## Error Taxonomy
- `Error::InvalidPattern` - when pattern has invalid regex syntax
- `Error::PatternTooLong` - when pattern exceeds 500 chars
- `Error::ReDoSRisk` - when pattern contains nested quantifiers
- `Error::CompilationFailed` - when regex fails to compile (used by `compile_safe_regex`)

## Contract Signatures

### main.rs::validate_filter_regex
```rust
fn validate_filter_regex(pattern: &str) -> Result<(), String>
```

### scrape/validation.rs::compile_safe_regex
```rust
pub(crate) fn compile_safe_regex(pattern: &str) -> Result<Regex>
```

## Type Encoding
| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| pattern.len() > 500 | Runtime check | Result<T, Error::PatternTooLong> |
| nested quantifiers | Runtime regex match | Result<T, Error::ReDoSRisk> |
| invalid syntax | Runtime regex compile | Result<T, Error::InvalidPattern> |

## Violation Examples (REQUIRED)

### Precondition Violations
- VIOLATES P2: `validate_filter_regex("a".repeat(1000))` -- should reject as "too long" BEFORE running ReDoS detector
- VIOLATES P3: `validate_filter_regex("[")` -- should reject for invalid syntax BEFORE ReDoS check

### Postcondition Violations
- VIOLATES Q2: `validate_filter_regex("(a+)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(\\w+)+")` -- should return Err with "ReDoS"  
- VIOLATES Q2: `validate_filter_regex("([a-z]+)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(a|a)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(.*)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(.+)*")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(a*)*")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(a{1,3})+")` -- should return Err with "ReDoS" (bounded quantifier nested)
- VIOLATES Q2: `validate_filter_regex("(a|b|c)+")` -- should return Err with "ReDoS" (multiple alternations)
- VIOLATES Q2: `validate_filter_regex("((a+)+)+")` -- should return Err with "ReDoS" (doubly nested)
- VIOLATES Q2: `validate_filter_regex("(\\d+\\.\\d+)+")` -- should return Err with "ReDoS" (char class with quantifier)

### Postcondition Q5 Violations (compile_safe_regex)
- VIOLATES Q5: `compile_safe_regex("(a+)+")` -- should return Err (ReDoS pattern should NOT compile)
- VIOLATES Q5: `compile_safe_regex("[")` -- should return Err (invalid syntax)

### Happy Path
- VIOLATES Q1: `validate_filter_regex("^/docs/.*\\.md$")` -- should return Ok(())
- VIOLATES Q5: `compile_safe_regex("^/docs/.*$")` -- should return Ok(Regex)

## Ownership Contracts
- Both functions take `&str` (borrowed), no ownership transfer
- No `&mut` parameters used

## Non-goals
- Detecting all possible ReDoS patterns (impossible - Rice's theorem)
- Performance benchmarking (covered by Q4)

## DSL Layer Specification
The test DSL separates WHAT (intent) from HOW (implementation):

```rust
// DSL Intent Layer (WHAT)
validate_regex("(a+)+").expect_redos_error();
validate_regex("^/docs/.*").expect_ok();
compile_regex("^/api/.*").expect_ok();
compile_regex("[invalid").expect_compile_error();
run_cli("--filter", "(a+)+").expect_failure_containing("ReDoS");

// DSL Implementation Layer (HOW - internal)
fn validate_regex(pattern: &str) -> ValidationResult { ... }
fn compile_regex(pattern: &str) -> CompileResult { ... }
fn run_cli(args: &[&str]) -> CliResult { ... }
```
