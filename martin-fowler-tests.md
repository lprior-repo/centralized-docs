# Martin Fowler Test Plan: CLI Error Message Format

## Happy Path Tests

- **test_index_command_succeeds_with_valid_source_and_output**
  - Given: valid source directory with markdown files and writable output directory
  - When: `doc_transformer index <source> --output <output>` is executed
  - Then: command completes with exit code 0 and no error messages printed

- **test_scrape_command_succeeds_with_valid_url**
  - Given: valid URL that returns HTTP 200 and a writable output directory
  - When: `doc_transformer scrape <url> --output <output>` is executed
  - Then: command completes with exit code 0 and scraped files are created

- **test_search_command_succeeds_with_valid_query**
  - Given: existing index directory with INDEX.json and valid search query
  - When: `doc_transformer search "<query>" --index <dir>` is executed
  - Then: command completes with exit code 0 and search results are displayed

## Error Path Tests

### Invalid Input Errors

- **test_invalid_regex_filter_returns_error_with_proper_prefix**
  - Given: `--filter` argument with invalid regex pattern
  - When: `doc_transformer scrape <url> --filter "[" --output <out>` is executed
  - Then: returns exit code 1
  - And: stderr first line matches `^Error: invalid value for --filter:`

- **test_query_too_long_returns_error_with_proper_prefix**
  - Given: search query exceeding 1000 bytes
  - When: `doc_transformer search "<1001-byte-query>" --index <dir>` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: validation failed: query too long`

- **test_invalid_threshold_value_returns_error**
  - Given: `--threshold` argument with negative value
  - When: `doc_transformer scrape <url> --threshold -1 --output <out>` is executed
  - Then: clap displays usage error (acceptable external behavior)

### Not Found Errors

- **test_nonexistent_source_returns_error_with_proper_prefix**
  - Given: source directory that does not exist
  - When: `doc_transformer index ./nonexistent --output /tmp/out` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: source not found: ./nonexistent`

- **test_missing_index_json_returns_error**
  - Given: index directory without INDEX.json
  - When: `doc_transformer search "query" --index <dir-without-index>` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: INDEX.json not found in`

### Permission Errors

- **test_unwritable_output_returns_permission_error**
  - Given: output directory without write permissions
  - When: `doc_transformer index <valid-source> --output /read-only-dir` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: permission denied: cannot write to output directory`

### Validation Errors

- **test_query_word_count_exceeds_limit_returns_error**
  - Given: search query with more than 100 words
  - When: `doc_transformer search "<101-word-query>" --index <dir>` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: validation failed: query has too many terms`

### Operation Failures

- **test_all_pages_filtered_returns_error**
  - Given: scrape with query that filters out all pages (threshold too high)
  - When: `doc_transformer scrape <url> --query "nonexistent" --threshold 100 --output <out>` is executed
  - Then: returns exit code 2 (pipeline error)
  - And: stderr contains `Error: scrape failed: all pages filtered out`

- **test_empty_source_directory_returns_error**
  - Given: source directory with no markdown files
  - When: `doc_transformer index <empty-dir> --output <out>` is executed
  - Then: returns exit code 1
  - And: stderr contains `Error: no markdown files found`

## Edge Case Tests

- **test_error_message_with_special_characters_handled_correctly**
  - Given: path with special characters (spaces, unicode)
  - When: `doc_transformer index "./path with spaces" --output <out>` is executed
  - Then: error message properly quotes/escapes the path

- **test_very_long_path_in_error_message_truncated**
  - Given: path exceeding 200 characters
  - When: error occurs with that path
  - Then: error message shows truncated path with `...` for readability

- **test_multiple_errors_only_first_shown_to_stderr**
  - Given: multiple errors during execution
  - When: first error is returned
  - Then: only first error message is shown (not all accumulated errors)

- **test_error_context_preserved_in_anyhow_chain**
  - Given: error that wraps another error
  - When: error propagates through multiple functions
  - Then: final error message includes context from all levels

## Contract Verification Tests

- **test_all_error_messages_start_with_error_prefix**
  - Given: the CLI binary
  - When: any command fails with any error
  - Then: stderr output starts with `Error: ` (not `ERROR:`, `Error:`, `error:`, etc.)

- **test_all_warning_messages_start_with_warning_prefix**
  - Given: non-fatal warning conditions
  - When: warnings are displayed
  - Then: stderr output starts with `Warning: ` (not `[WARNING]`, `Warning:`, etc.)

- **test_error_messages_contain_no_embedded_newlines**
  - Given: any error message
  - When: message is displayed
  - Then: message contains no unescaped newlines (use `; ` or ` - ` instead)

- **test_error_messages_are_valid_utf8**
  - Given: any error message with paths/filenames
  - When: message is displayed
  - Then: message is valid UTF-8 (no lossy conversions)

## Contract Violation Tests

### Violation of Q1: Missing Error: prefix

- **test_transform_error_without_prefix_is_fixed**
  - Given: transform.rs line 109 uses `TRANSFORM ERROR:`
  - When: transform fails on a file
  - Then: stderr shows `Error: transform failed: <path>: <msg>`
  - Not: `TRANSFORM ERROR: <path>: <msg>`

- **test_analyze_error_without_prefix_is_fixed**
  - Given: analyze.rs line 115 uses `ANALYZE ERROR:`
  - When: analysis fails on a file
  - Then: stderr shows `Error: analysis failed: <path>: <msg>`
  - Not: `ANALYZE ERROR: <path>: <msg>`

### Violation of Q4: Warning: Used for Error Conditions (doc-3o2b)

- **test_discover_io_error_uses_error_prefix_not_warning**
  - Given: source directory with a path that triggers I/O error (permission denied)
  - When: `doc_transformer index <source> --output <out>` is executed
  - Then: stderr shows `Error: Skipping path due to I/O error: <details>`
  - Not: `Warning: Skipping path due to I/O error: <details>`

- **test_discover_empty_file_uses_error_prefix_not_warning**
  - Given: source directory containing an empty markdown file
  - When: `doc_transformer index <source> --output <out>` is executed
  - Then: stderr shows `Error: Skipping empty file <path>`
  - Not: `Warning: Skipping empty file <path>`

- **test_index_tantivy_failure_uses_error_prefix_not_warning**
  - Given: valid source but Tantivy index build fails (e.g., disk full)
  - When: `doc_transformer index <source> --output <out>` is executed
  - Then: stderr shows `Error: Failed to build Tantivy index: <details>`
  - Not: `Warning: Failed to build Tantivy index: <details>`

- **test_index_hnsw_failure_uses_error_prefix_not_warning**
  - Given: valid source but HNSW index build fails (e.g., empty embeddings)
  - When: `doc_transformer index <source> --output <out>` is executed
  - Then: stderr shows `Error: HNSW index build failed (<details>), skipping related chunk edges`
  - Not: `Warning: HNSW index build failed (<details>), skipping related chunk edges`

- **test_filter_io_error_uses_error_prefix_not_warning**
  - Given: source directory with a path that triggers I/O error during filtering
  - When: `doc_transformer index <source> --output <out>` is executed
  - Then: stderr shows `Error: Skipping path due to I/O error: <details>`
  - Not: `Warning: Skipping path due to I/O error: <details>`

### Violation of Q2: Embedded newlines in errors

- **test_error_message_no_embedded_newlines**
  - Given: error with multi-line context
  - When: error is displayed
  - Then: single line output with `; ` separator
  - Not: newline-separated lines

### Violation of Q4: Non-Warning uses Warning prefix

- **test_info_messages_do_not_use_warning_prefix**
  - Given: informational output like "Sitemap found"
  - When: message is displayed
  - Then: uses `println!()` (not `eprintln!()`)
  - Not: `Warning:` or `Error:` prefix

## Given-When-Then Scenarios

### Scenario 1: Invalid Regex Filter

**Given:** the CLI is invoked with an invalid regex filter pattern  
**When:** `doc_transformer scrape https://example.com --filter "[" --output /tmp/out` is executed  
**Then:**
- exit code is 1 (user error)
- stderr first line is `Error: invalid value for --filter: Invalid or too complex regex pattern '[': ...`
- error message is parseable by scripts (single line, starts with "Error: ")

### Scenario 2: Source Directory Not Found

**Given:** the CLI is invoked with a non-existent source directory  
**When:** `doc_transformer index ./nonexistent --output /tmp/out` is executed  
**Then:**
- exit code is 1 (user error)
- stderr contains `Error: source not found: ./nonexistent`
- error is actionable (tells user what path was not found)

### Scenario 3: Permission Denied

**Given:** the output directory exists but is not writable  
**When:** `doc_transformer index ./docs --output /read-only` is executed  
**Then:**
- exit code is 1 (user error)
- stderr contains `Error: permission denied: cannot write to output directory '/read-only'`
- stderr contains a hint about checking permissions

### Scenario 4: Query Too Long

**Given:** the search query exceeds maximum allowed length  
**When:** `doc_transformer search "<1001-byte-query>" --index ./index` is executed  
**Then:**
- exit code is 1 (user error)
- stderr contains `Error: validation failed: query too long`

### Scenario 5: All Pages Filtered by Query

**Given:** scrape with query that filters out all pages  
**When:** `doc_transformer scrape https://example.com --query "xyznonexistent" --threshold 100 --output /tmp/out` is executed  
**Then:**
- exit code is 2 (pipeline error, not user error)
- stderr contains `Error: scrape failed: all pages filtered out by query 'xyznonexistent'`
- stderr contains hint about lowering threshold

### Scenario 6: Warning for Non-Fatal Issue

**Given:** a non-fatal issue during execution (e.g., skipped file due to I/O error)  
**When:** the operation continues but logs the issue  
**Then:**
- exit code is 0 (operation succeeded despite warning)
- stderr contains `Warning: Skipping path due to I/O error: <details>`
- warning message is informative but not confused with errors

## End-to-End Scenarios

### E2E: Full Ingest Pipeline Error Handling

**Scenario:** User runs full pipeline with invalid configuration  
**Given:** `doc_transformer ingest https://example.com --output /tmp/index --filter "[invalid"`  
**When:** command is executed  
**Then:**
- Early validation catches invalid regex
- Error message: `Error: invalid value for --filter: Invalid or too complex regex pattern '[invalid': ...`
- Exit code: 1
- No partial output created in `/tmp/index/`

### E2E: Search with Missing Index

**Scenario:** User searches without building index first  
**Given:** fresh directory with no index  
**When:** `doc_transformer search "authentication" --index ./new-project` is executed  
**Then:**
- Error message: `Error: INDEX.json not found in ./new-project`
- Exit code: 1
- Actionable hint: user should run `index` command first
