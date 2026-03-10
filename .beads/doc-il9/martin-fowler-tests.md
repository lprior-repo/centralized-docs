# Martin Fowler Test Plan

## Happy Path Tests
- test_extract_title_returns_h1_text_when_present
- test_extract_title_returns_url_derived_fallback_when_no_h1
- test_extract_title_returns_untitled_when_no_h1_and_invalid_url
- test_extract_title_handles_whitespace_in_h1_correctly

## Edge Case Tests
- test_extract_title_handles_empty_markdown
- test_extract_title_handles_markdown_without_headers
- test_extract_title_handles_multiple_h1_headers_returns_first
- test_extract_title_handles_h1_with_special_characters

## Contract Verification Tests
- test_lazy_regex_is_compiled_once
- test_lazy_regex_reuse_on_subsequent_calls

## Given-When-Then Scenarios

### Scenario 1: H1 header present in markdown
Given: markdown containing "# My Title" as first line
When: extract_title is called
Then: returns "My Title"

### Scenario 2: No H1 header in markdown
Given: markdown with no H1 header (e.g., "## Section\nContent")
When: extract_title is called with valid URL "https://example.com/docs/my-doc"
Then: returns "my doc" (URL-derived fallback)

### Scenario 3: Invalid markdown and URL
Given: empty markdown and invalid URL "not-a-url"
When: extract_title is called
Then: returns "Untitled"

### Scenario 4: H1 with leading/trailing whitespace
Given: markdown with "#   Spaced Title   "
When: extract_title is called
Then: returns "Spaced Title" (whitespace trimmed)
