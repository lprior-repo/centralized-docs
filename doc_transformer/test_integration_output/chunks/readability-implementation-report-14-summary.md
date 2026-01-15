---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#14
chunk_level: summary
chunk_type: table
heading: Edge Cases Handled
token_count: 146
summary: ### Invariants ✅. - All errors are typed (`ExtractionError` enum)
---


### Invariants ✅
- All errors are typed (`ExtractionError` enum)
- No `.unwrap()` in production code path

## Edge Cases Handled

| Edge Case | Behavior | Test |
|-----------|----------|------|
| Empty HTML (`<body></body>`) | Readability extracts minimal content OR returns error | `test_extract_article_empty_content` |
| Navigation-only pages | Falls back to custom pruning | `test_readability_fallback_on_nav_only` |
| Invalid URL | `ExtractionError::InvalidUrl` | `test_extract_article_invalid_url` |
| Malformed HTML | Readability handles via `html5ever` parser | Implicit |
