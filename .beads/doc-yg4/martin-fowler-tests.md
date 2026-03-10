# Martin Fowler Test Plan

## Happy Path Tests
- test_returns_scraped_page_when_extraction_succeeds
  - Given: A valid HTML page with extractable content
  - When: transform_page is called
  - Then: Returns Ok(ScrapedPage) with non-empty markdown

## Error Path Tests
- test_returns_error_when_transform_content_returns_empty_string
  - Given: A page where transform_content returns ""
  - When: transform_page is called
  - Then: Returns Err(Error::EmptyExtractionResult)

- test_returns_error_when_transform_content_returns_whitespace_only
  - Given: A page where transform_content returns "   "
  - When: transform_page is called
  - Then: Returns Err(Error::EmptyExtractionResult)

## Edge Case Tests
- test_handles_page_with_minimal_content
  - Given: A page with minimal valid content
  - When: transform_page is called
  - Then: Returns Ok(ScrapedPage) with that minimal content

## Contract Verification Tests
- test_precondition_p1_caller_provides_valid_page
  - Verifies caller provides valid page reference

- test_precondition_p2_caller_provides_valid_config
  - Verifies caller provides valid TransformConfig

- test_postcondition_q1_markdown_not_empty
  - Given: Valid page and config
  - When: transform_page is called
  - Then: Returns Ok with non-empty markdown OR Err if empty

- test_postcondition_q2_error_on_empty_result
  - Given: Empty extraction result
  - When: transform_page is called
  - Then: Returns Err(Error::EmptyExtractionResult)

## Contract Violation Tests
- test_q1_violation_empty_string_returns_error
  - Given: transform_content returns ""
  - When: transform_page is called
  - Then: Returns Err(EmptyExtractionResult)

- test_q1_violation_whitespace_only_returns_error
  - Given: transform_content returns "   "
  - When: transform_page is called
  - Then: Returns Err(EmptyExtractionResult)

## Given-When-Then Scenarios

### Scenario 1: Successful content extraction
Given: A page with title "Test" and some paragraph content
When: transform_page is called
Then:
- Returns Ok(ScrapedPage)
- markdown field is not empty
- title field is extracted
- word_count > 0

### Scenario 2: Failed content extraction (empty result)
Given: A page where transform_content returns empty string
When: transform_page is called
Then:
- Returns Err(Error::EmptyExtractionResult)
- No partial ScrapedPage is constructed

### Scenario 3: Failed content extraction (whitespace only)
Given: A page where transform_content returns only whitespace
When: transform_page is called
Then:
- Returns Err(Error::EmptyExtractionResult)
- No partial ScrapedPage is constructed
