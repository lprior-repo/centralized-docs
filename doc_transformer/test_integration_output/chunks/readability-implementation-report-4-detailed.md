---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#4
chunk_level: detailed
chunk_type: prose
heading: Design by Contract (DbC) Verification
token_count: 368
summary: ## Test Coverage. ### New Tests Added
---


```toml
```

## Test Coverage

### New Tests Added

1. **`test_extract_article_with_valid_html()`**
   - Validates successful extraction
   - Verifies confidence/density bounds

2. **`test_extract_article_empty_content()`**
   - Edge case: Empty HTML
   - Verifies graceful error handling

3. **`test_extract_article_invalid_url()`**
   - URL validation
   - Error type matching

4. **`test_calculate_confidence()`**
   - Confidence scoring logic
   - Long vs short content

### Test Results

```
test result: ok. 39 passed; 0 failed; 0 ignored
```

**All existing tests maintained compatibility:**
- `test_prune_html()` - Legacy API still works
- `test_prune_html_with_article_tag()` - Article extraction
- `test_readability_fallback_on_nav_only()` - Fallback logic
- BM25 tests (17 total) - Tantivy integration preserved

## Design by Contract (DbC) Verification

### Preconditions ✅
- `html` is valid UTF-8 (guaranteed by Rust `&str`)
- `url` is valid URL string (validated in function)
- `config` is valid `FilterConfig` (type-safe struct)

### Postconditions ✅
- `ExtractedContent.confidence` ∈ [0.0, 1.0]
- `ExtractedContent.density_score` ∈ [0.0, 1.0]
- `content` is never empty on success
- Errors are enumerated (no string errors)

### Invariants ✅
- Function never panics (Railway pattern catches all errors)
- Graceful degradation (fallback to custom pruning)
- All errors are typed (`ExtractionError` enum)
- No `.unwrap()` in production code path

