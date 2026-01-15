---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#3
chunk_level: detailed
chunk_type: prose
heading: Test Coverage
token_count: 410
summary:     // Structure bonuses.     let structure_bonus =
---

    // Structure bonuses
    let structure_bonus =
        (if paragraph_count > 3 { 0.2 } else { 0.0 })
        + (if heading_count > 0 { 0.1 } else { 0.0 });

    (word_confidence + structure_bonus).min(1.0)
}
```

### 5. Functional Rust Implementation

**Strict Compliance:**
- ✅ No `.unwrap()` or `.expect()` calls
- ✅ Railway-Oriented Programming (`.and_then()`, `.map()`, `.map_err()`)
- ✅ Semantic error types with `thiserror`
- ✅ Design by Contract documentation
- ✅ Pure functions (no hidden side effects)
- ✅ Immutable by default
- ✅ Iterator combinators over loops

**Note:** Lint attributes (`#![deny(clippy::unwrap_used)]`) were prepared but removed by auto-formatter. Consider adding to `Cargo.toml` or CI pipeline:
```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
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

