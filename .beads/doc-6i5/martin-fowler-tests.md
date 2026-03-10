# Martin Fowler Test Plan: ReDoS Pattern Detection

## Happy Path Tests
- test_returns_ok_for_valid_simple_pattern
- test_returns_ok_for_valid_anchor_patterns
- test_returns_ok_for_character_classes
- test_returns_ok_for_alternation_safe

## Error Path Tests (ReDoS Detection)
- test_rejects_canonical_redos_a_plus_a_plus
- test_rejects_nested_star_star
- test_rejects_nested_plus_plus
- test_rejects_w_plus_nested
- test_rejects_bracket_expression_nested
- test_rejects_alternation_nested
- test_rejects_dot_star_nested
- test_rejects_dot_plus_nested

## Edge Case Tests
- test_accepts_empty_string
- test_accepts_single_char
- test_rejects_exactly_501_chars
- test_rejects_very_long_pattern_1000_chars

## Contract Verification Tests
- test_precondition_length_check_before_redos_check
- test_postcondition_returns_error_with_redos_message
- test_postcondition_returns_error_with_too_long_message

## Contract Violation Tests

### From contract-spec.md - Postcondition Q2 violations:
- `test_violates_q2_a_plus_a_plus`
  Given: pattern "(a+)+"
  When: validate_filter_regex("(a+)+")
  Then: returns Err containing "ReDoS" -- NOT Ok, NOT panic

- `test_violates_q2_w_plus_nested`
  Given: pattern "(\\w+)+"
  When: validate_filter_regex("(\\w+)+")
  Then: returns Err containing "ReDoS"

- `test_violates_q2_bracket_expression_nested`
  Given: pattern "([a-z]+)+"
  When: validate_filter_regex("([a-z]+)+")
  Then: returns Err containing "ReDoS"

- `test_violates_q2_alternation_nested`
  Given: pattern "(a|a)+"
  When: validate_filter_regex("(a|a)+")
  Then: returns Err containing "ReDoS"

- `test_violates_q2_dot_star_nested`
  Given: pattern "(.*)+"
  When: validate_filter_regex("(.*)+")
  Then: returns Err containing "ReDoS"

- `test_violates_q2_dot_plus_star`
  Given: pattern "(.+)*"
  When: validate_filter_regex("(.+)*")
  Then: returns Err containing "ReDoS"

- `test_violates_q2_a_star_star`
  Given: pattern "(a*)*"
  When: validate_filter_regex("(a*)*")
  Then: returns Err containing "ReDoS"

## Given-When-Then Scenarios

### Scenario 1: User provides dangerous nested quantifier
Given: User runs `doc_transformer scrape https://example.com/ --filter "(a+)+" --no-sitemap`
When: System validates the filter regex
Then: 
- Returns error with message containing "ReDoS"
- Does NOT hang or crash
- Does NOT compile the dangerous regex

### Scenario 2: User provides valid regex filter
Given: User runs `doc_transformer scrape https://example.com/ --filter "^/docs/.*" --no-sitemap`
When: System validates the filter regex
Then:
- Returns Ok (validation passes)
- Filter is used for URL matching

### Scenario 3: User provides too-long regex
Given: User provides pattern with 600+ characters
When: System validates the filter regex
Then:
- Returns error with message containing "too long" or "500"
- Pattern is rejected before ReDoS check

## Test Execution Notes
- All ReDoS pattern tests MUST complete in < 100ms (detection must be fast)
- Tests should verify error message content for user feedback
- Both main.rs and validation.rs functions should be tested
