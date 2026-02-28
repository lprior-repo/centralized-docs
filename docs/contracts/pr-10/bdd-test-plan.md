# BDD Test Plan - PR #10

## Naming Convention
All tests use `given_<context>_when_<action>_then_<outcome>`.

## Scenarios by Issue

### `doc-nnly` (null bytes)
1. `given_query_contains_null_byte_when_validate_query_then_reject_with_null_bytes_not_allowed`
   - Given: query `"test\0query"`
   - When: `validate_query`
   - Then: returns `Err(NullBytesNotAllowed)`

2. `given_query_contains_multiple_null_bytes_when_validate_query_then_reject_with_null_bytes_not_allowed`
   - Given: query `"a\0b\0c"`
   - When: `validate_query`
   - Then: returns `Err(NullBytesNotAllowed)`

3. `given_query_without_null_bytes_when_validate_query_then_preserve_trimmed_query`
   - Given: query with leading/trailing whitespace but valid content
   - When: `validate_query`
   - Then: returns trimmed query and no content mutation beyond trimming

### `doc-1vtz` (partial failure exit semantics)
4. `given_advanced_search_parser_failure_and_fallback_success_when_run_search_then_return_partial_failure_error`
   - Given: Tantivy index present, advanced query fails, fallback runs
   - When: `run_search`
   - Then: returns error describing partial failure

5. `given_advanced_search_success_when_run_search_then_return_success`
   - Given: valid advanced query
   - When: `run_search`
   - Then: returns `Ok(())`

6. `given_index_json_missing_when_run_search_then_return_index_missing_error`
   - Given: missing `INDEX.json`
   - When: `run_search`
   - Then: returns `Err(IndexMissing)` equivalent

### `doc-13x9` + review `discussion_r2866841648` (URL validation)
7. `given_url_contains_space_when_validate_url_then_reject_with_actionable_message`
   - Given: `https://example.com/foo bar`
   - When: `validate_url`
   - Then: error states spaces must be encoded

8. `given_url_contains_unencoded_reserved_char_in_path_when_validate_url_then_reject`
   - Given: `https://example.com/foo[bar]`
   - When: `validate_url`
   - Then: returns unencoded-character error

9. `given_ipv6_host_literal_when_validate_url_then_accept`
   - Given: `https://[::1]:3000/docs`
   - When: `validate_url`
   - Then: returns parsed URL (regression guard)

10. `given_percent_encoded_reserved_chars_when_validate_url_then_accept`
    - Given: `https://example.com/foo%5Bbar%5D`
    - When: `validate_url`
    - Then: returns parsed URL

### `doc-2mzo` and `doc-sgzo` (discovery behavior)
11. `given_directory_contains_markdown_variant_extensions_when_discover_files_then_include_all_supported_variants`
    - Given: files with `.md`, `.markdown`, `.mdown`, `.mkd`
    - When: `discover_files`
    - Then: all variants appear in manifest

12. `given_directory_contains_txt_files_when_discover_files_then_include_file_and_emit_warning`
    - Given: directory with `.txt`
    - When: `discover_files`
    - Then: file is discovered and warning is emitted

13. `given_single_markdown_variant_file_when_discover_single_file_then_manifest_contains_one_entry`
    - Given: single `.markdown` file path
    - When: `discover_single_file`
    - Then: one discovered entry

14. `given_test_helper_discovery_when_discover_test_files_then_extension_support_matches_production`
    - Given: same fixture through prod and test helper
    - When: discover in both paths
    - Then: file sets match for supported extensions

### `doc-3e4v` (chunk non-hang)
15. `given_repeated_token_estimation_calls_when_estimate_tokens_then_complete_without_hanging`
    - Given: high-volume loop of diverse strings
    - When: `estimate_tokens` repeatedly
    - Then: function returns within threshold and no deadlock

16. `given_tokenizer_unavailable_when_estimate_tokens_then_fallback_estimate_is_positive`
    - Given: simulated tokenizer init failure
    - When: `estimate_tokens`
    - Then: returns `(text.len()/4).max(1)` style positive estimate

## Contract Verification Tests
17. `given_each_error_variant_when_triggered_then_error_message_identifies_exact_violation`
18. `given_partial_search_failure_when_cli_exits_then_exit_code_is_non_zero`
19. `given_valid_inputs_when_functions_return_ok_then_invariants_hold`

## Integration Scenarios (Martin Fowler style)

### Scenario A: Advanced Search Falls Back
- Given: Tantivy index exists and query includes unsupported syntax
- When: user runs search
- Then: user sees advanced failure reason
- Then: fallback search executes
- Then: process exits non-zero to signal partial failure

### Scenario B: URL Validation for IPv6 and Reserved Characters
- Given: URL inputs from users include IPv6 hosts and malformed reserved chars
- When: scrape URL validation executes
- Then: IPv6 host literals are accepted
- Then: malformed path characters are rejected with actionable guidance

### Scenario C: Discovery Extension Coverage
- Given: source tree with `.md`, `.markdown`, `.mdown`, `.mkd`, `.txt`
- When: discovery runs
- Then: markdown variants are included
- Then: `.txt` files are included with explicit warning
