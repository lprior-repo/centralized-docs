# Martin Fowler Test Plan: ReDoS Pattern Detection (doc-4il)

## DSL Layer (Separates WHAT from HOW)

```rust
// ============================================================
// DSL - WHAT (Test Intent / Behavior Specification)
// ============================================================

// These functions express test intent without revealing HOW tests run

/// Validates a regex pattern for ReDoS risks - expects validation result
fn validate_regex(pattern: &str) -> ValidationOutcome {
    // Implementation hidden - calls validate_filter_regex
}

/// Compiles a regex pattern safely - expects compilation result
fn compile_regex(pattern: &str) -> CompileOutcome {
    // Implementation hidden - calls compile_safe_regex
}

/// Runs the CLI with given arguments - integration test
fn run_cli(args: &[&str]) -> CliOutcome {
    // Implementation hidden - spawns process
}

/// Property: generates arbitrary safe regex patterns for fuzz testing
fn generate_safe_regex() -> String {
    // Implementation hidden - property-based generation
}
```

## Happy Path Tests (BDD Given-When-Then)

### validate_filter_regex Happy Path
- `given_user_provides_valid_simple_pattern_when_validating_then_returns_ok`
  - Given: pattern "^/docs/.*\\.md$"
  - When: validate_filter_regex("^/docs/.*\\.md$")
  - Then: returns Ok(())

- `given_user_provides_valid_anchor_patterns_when_validating_then_returns_ok`
  - Given: pattern "^https?://.*"
  - When: validate_filter_regex("^https?://.*")
  - Then: returns Ok(())

- `given_user_provides_valid_character_class_when_validating_then_returns_ok`
  - Given: pattern "[a-zA-Z0-9]+"
  - When: validate_filter_regex("[a-zA-Z0-9]+")
  - Then: returns Ok(())

- `given_user_provides_safe_alternation_when_validating_then_returns_ok`
  - Given: pattern "(foo|bar|baz)"
  - When: validate_filter_regex("(foo|bar|baz)")
  - Then: returns Ok(())

- `given_user_provides_empty_string_when_validating_then_returns_ok`
  - Given: pattern ""
  - When: validate_filter_regex("")
  - Then: returns Ok(())

### compile_safe_regex Happy Path
- `given_user_provides_valid_pattern_when_compiling_then_returns_regex`
  - Given: pattern "^/api/v1/.*"
  - When: compile_safe_regex("^/api/v1/.*")
  - Then: returns Ok(Regex) that matches "/api/v1/users"

- `given_user_provides_complex_valid_pattern_when_compiling_then_returns_regex`
  - Given: pattern "^\\d{4}-\\d{2}-\\d{2}$"
  - When: compile_safe_regex("^\\d{4}-\\d{2}-\\d{2}$")
  - Then: returns Ok(Regex) that matches "2024-01-15"

## Error Path Tests (ReDoS Detection)

### validate_filter_regex ReDoS Detection
- `given_user_provides_canonical_redos_a_plus_a_plus_when_validating_then_returns_redos_error`
  - Given: pattern "(a+)+"
  - When: validate_filter_regex("(a+)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_nested_star_star_when_validating_then_returns_redos_error`
  - Given: pattern "(.*)*"
  - When: validate_filter_regex("(.*)*")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_nested_plus_plus_when_validating_then_returns_redos_error`
  - Given: pattern "(a++)++"
  - When: validate_filter_regex("(a++)++")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_w_plus_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(\\w+)+"
  - When: validate_filter_regex("(\\w+)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_bracket_expression_nested_when_validating_then_returns_redos_error`
  - Given: pattern "([a-z]+)+"
  - When: validate_filter_regex("([a-z]+)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_alternation_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(a|a)+"
  - When: validate_filter_regex("(a|a)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_dot_star_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(.*)+"
  - When: validate_filter_regex("(.*)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_dot_plus_star_when_validating_then_returns_redos_error`
  - Given: pattern "(.+)*"
  - When: validate_filter_regex("(.+)*")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_a_star_star_when_validating_then_returns_redos_error`
  - Given: pattern "(a*)*"
  - When: validate_filter_regex("(a*)*")
  - Then: returns Err containing "ReDoS"

### Additional ReDoS Patterns (Extended Coverage)
- `given_user_provides_bounded_quantifier_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(a{1,3})+"
  - When: validate_filter_regex("(a{1,3})+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_multiple_alternations_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(a|b|c)+"
  - When: validate_filter_regex("(a|b|c)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_doubly_nested_when_validating_then_returns_redos_error`
  - Given: pattern "((a+)+)+"
  - When: validate_filter_regex("((a+)+)+")
  - Then: returns Err containing "ReDoS"

- `given_user_provides_char_class_with_quantifier_nested_when_validating_then_returns_redos_error`
  - Given: pattern "(\\d+\\.\\d+)+"
  - When: validate_filter_regex("(\\d+\\.\\d+)+")
  - Then: returns Err containing "ReDoS"

### compile_safe_regex Error Path
- `given_user_provides_redos_pattern_when_compiling_then_returns_error`
  - Given: pattern "(a+)+"
  - When: compile_safe_regex("(a+)+")
  - Then: returns Err (should NOT compile dangerous patterns)

- `given_user_provides_invalid_regex_syntax_when_compiling_then_returns_error`
  - Given: pattern "["
  - When: compile_safe_regex("[")
  - Then: returns Err with "InvalidPattern"

## Edge Case Tests

- `given_user_provides_single_character_when_validating_then_returns_ok`
  - Given: pattern "a"
  - When: validate_filter_regex("a")
  - Then: returns Ok(())

- `given_user_provides_exactly_501_characters_when_validating_then_returns_too_long_error`
  - Given: pattern with 501 'a' characters
  - When: validate_filter_regex(pattern)
  - Then: returns Err containing "too long"

- `given_user_provides_1000_characters_when_validating_then_returns_too_long_error`
  - Given: pattern with 1000 'a' characters
  - When: validate_filter_regex(pattern)
  - Then: returns Err containing "too long"

- `given_user_provides_invalid_syntax_bracket_when_validating_then_returns_invalid_error`
  - Given: pattern "["
  - When: validate_filter_regex("[")
  - Then: returns Err containing "invalid" or "syntax"

## Contract Verification Tests

- `test_precondition_length_check_happens_before_redos_check`
  - Given: pattern with 600 characters (exceeds limit)
  - When: validate_filter_regex(pattern)
  - Then: returns "too long" error (NOT ReDoS error - length check is precondition)

- `test_postcondition_returns_ok_for_safe_patterns`
  - Given: safe pattern "^/docs/.*$"
  - When: validate_filter_regex("^/docs/.*$")
  - Then: returns Ok(())

- `test_postcondition_returns_error_with_redos_message_for_dangerous_patterns`
  - Given: dangerous pattern "(a+)+"
  - When: validate_filter_regex("(a+)+")
  - Then: returns Err containing "ReDoS"

- `test_invariant_no_redos_pattern_compiles`
  - Given: multiple dangerous patterns
  - When: compile_safe_regex is called with each
  - Then: all return Err (no dangerous regex ever compiles)

- `test_invariant_validation_completes_under_100ms`
  - Given: dangerous pattern "(a+)+"
  - When: validate_filter_regex("(a+)+") is timed
  - Then: completes in < 100ms

## Integration/E2E Tests (Testing Trophy)

- `given_user_runs_cli_with_redos_filter_when_invoked_then_terminates_quickly_with_error`
  - Given: CLI installed at target/release/doc_transformer
  - When: process spawns with args ["scrape", "--filter", "(a+)+", "https://example.com/"]
  - Then: 
    - Process exits with non-zero code
    - stderr contains "ReDoS"
    - Execution completes in < 100ms

- `given_user_runs_cli_with_valid_filter_when_invoked_then_succeeds`
  - Given: CLI installed at target/release/doc_transformer
  - When: process spawns with args ["scrape", "--filter", "^/docs/.*", "https://example.com/"]
  - Then:
    - Process exits with zero code OR continues execution (filter accepted)

- `given_user_runs_cli_with_too_long_filter_when_invoked_then_shows_length_error`
  - Given: CLI installed at target/release/doc_transformer
  - When: process spawns with args ["scrape", "--filter", "a".repeat(600), "https://example.com/"]
  - Then:
    - Process exits with non-zero code
    - stderr contains "too long" or "500"

- `given_user_runs_cli_with_invalid_syntax_filter_when_invoked_then_shows_syntax_error`
  - Given: CLI installed at target/release/doc_transformer
  - When: process spawns with args ["scrape", "--filter", "[", "https://example.com/"]
  - Then:
    - Process exits with non-zero code
    - stderr contains "invalid" or "syntax"

## Property-Based Tests (Advanced Testing)

- `property_valid_simple_regexes_always_pass_validation`
  - Given: arbitrary valid regex generated (no nested quantifiers)
  - When: validate_filter_regex(generated)
  - Then: returns Ok(()) with 100% probability

- `property_compiled_regex_matches_its_literal_pattern`
  - Given: arbitrary valid regex pattern
  - When: compile_safe_regex(pattern)
  - Then: the returned Regex matches pattern itself

- `property_no_redos_pattern_ever_compiles`
  - Given: arbitrary nested quantifier pattern
  - When: compile_safe_regex(pattern)
  - Then: returns Err (property: dangerous patterns never compile)

## Contract Violation Tests (One per violation example in contract.md)

- `test_violates_p2_validate_filter_regex_1000_chars_returns_too_long`
  - Given: pattern "a".repeat(1000)
  - When: validate_filter_regex("a".repeat(1000))
  - Then: returns Err containing "too long" (NOT ReDoS - precondition violation)

- `test_violates_p3_validate_filter_regex_invalid_syntax_returns_error`
  - Given: pattern "["
  - When: validate_filter_regex("[")
  - Then: returns Err containing "invalid" or "syntax"

- `test_violates_q2_validate_filter_regex_a_plus_a_plus_returns_redos`
  - Given: pattern "(a+)+"
  - When: validate_filter_regex("(a+)+")
  - Then: returns Err containing "ReDoS"

- `test_violates_q5_compile_safe_regex_redos_pattern_returns_error`
  - Given: pattern "(a+)+"
  - When: compile_safe_regex("(a+)+")
  - Then: returns Err (should NOT compile dangerous pattern)

- `test_violates_q5_compile_safe_regex_invalid_syntax_returns_error`
  - Given: pattern "["
  - When: compile_safe_regex("[")
  - Then: returns Err with InvalidPattern

## Given-When-Then Scenarios (Full E2E)

### Scenario 1: User provides dangerous nested quantifier via CLI
Given: User runs `doc_transformer scrape https://example.com/ --filter "(a+)+" --no-sitemap`
When: System validates the filter regex before using it
Then: 
- CLI terminates within 100ms
- Returns error with message containing "ReDoS"
- Does NOT hang or crash
- Does NOT compile the dangerous regex

### Scenario 2: User provides valid regex filter via CLI
Given: User runs `doc_transformer scrape https://example.com/ --filter "^/docs/.*" --no-sitemap`
When: System validates the filter regex
Then:
- Validation passes (returns Ok)
- Filter is used for URL matching
- Scrape operation proceeds

### Scenario 3: User provides too-long regex via CLI
Given: User provides pattern with 600+ characters via --filter flag
When: System validates the filter regex
Then:
- Returns error containing "too long" or "500"
- Pattern is rejected BEFORE ReDoS check (precondition enforcement)
- CLI exits with non-zero code

### Scenario 4: User provides invalid regex syntax via CLI
Given: User provides pattern "[" via --filter flag
When: System validates the filter regex
Then:
- Returns error containing "invalid" or "syntax"
- Pattern is rejected BEFORE ReDoS check (precondition enforcement)
- CLI exits with non-zero code

### Scenario 5: Programmatic use of compile_safe_regex
Given: Code calls compile_safe_regex("^/api/.*$")
When: Function is invoked
Then:
- Returns Ok(Regex) that can be used for matching
- The compiled Regex works correctly

### Scenario 6: Programmatic use of compile_safe_regex with dangerous pattern
Given: Code calls compile_safe_regex("(a+)+")
When: Function is invoked
Then:
- Returns Err (ReDoS pattern detected)
- No Regex object is returned
- Calling code handles error gracefully
