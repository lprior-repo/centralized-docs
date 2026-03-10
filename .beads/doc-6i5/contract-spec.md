# Contract Specification: ReDoS Pattern Detection

## Context
- **Feature**: Fix incomplete ReDoS pattern detection in filter regex validation
- **Domain terms**:
  - ReDoS = Regular Expression Denial of Service (catastrophic backtracking)
  - Nested quantifier = a quantifier (+, *, {n,m}) applied to a group that already has a quantifier
- **Assumptions**:
  - User-provided regex filters should be validated for ReDoS patterns before compilation
  - The validation should be fast (not itself cause catastrophic backtracking)
- **Open questions**:
  - Is the issue in main.rs only, validation.rs only, or both?
  - Are there other code paths that use regex validation?

## Preconditions
- [P1] Pattern is a valid string provided by caller
- [P2] Pattern length check happens BEFORE ReDoS check (to prevent ReDoS on the detector itself)

## Postconditions
- [Q1] Function returns Ok(()) for safe regex patterns
- [Q2] Function returns Err with "ReDoS" message for dangerous nested quantifier patterns
- [Q3] Function returns Err with "too long" message for patterns > 500 chars
- [Q4] Detection completes in < 100ms for any input

## Invariants
- [I1] No ReDoS pattern ever compiles successfully
- [I2] Validation regex itself cannot cause ReDoS (use bounded patterns)

## Error Taxonomy
- `Error::InvalidPattern` - when pattern has invalid regex syntax
- `Error::PatternTooLong` - when pattern exceeds 500 chars
- `Error::ReDoSRisk` - when pattern contains nested quantifiers

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

### Postcondition Violations
- VIOLATES Q2: `validate_filter_regex("(a+)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(\\w+)+")` -- should return Err with "ReDoS"  
- VIOLATES Q2: `validate_filter_regex("([a-z]+)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(a|a)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(.*)+")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(.+)*")` -- should return Err with "ReDoS"
- VIOLATES Q2: `validate_filter_regex("(a*)*")` -- should return Err with "ReDoS"

### Happy Path
- VIOLATES Q1: `validate_filter_regex("^/docs/.*\\.md$")` -- should return Ok(())

## Ownership Contracts
- Both functions take `&str` (borrowed), no ownership transfer
- No `&mut` parameters used

## Non-goals
- Detecting all possible ReDoS patterns (impossible - Rice's theorem)
- Performance benchmarking (covered by Q4)
