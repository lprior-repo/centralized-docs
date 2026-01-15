---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#10
chunk_level: summary
chunk_type: prose
heading: Test Coverage
token_count: 133
summary:  **`test_extract_article_invalid_url()`**.    - URL validation
---

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
