---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#6
chunk_level: standard
chunk_type: table
heading: Edge Cases Handled
token_count: 277
summary: - `ExtractedContent. confidence` ∈ [0
---





- `ExtractedContent.confidence` ∈ [0.0, 1.0]
- `ExtractedContent.density_score` ∈ [0.0, 1.0]
- `content` is never empty on success
- Errors are enumerated (no string errors)

### Invariants ✅
- Function never panics (Railway pattern catches all errors)
- Graceful degradation (fallback to custom pruning)
- All errors are typed (`ExtractionError` enum)
- No `.unwrap()` in production code path

## Edge Cases Handled

| Edge Case | Behavior | Test |
|-----------|----------|------|
| Empty HTML (`<body></body>`) | Readability extracts minimal content OR returns error | `test_extract_article_empty_content` |
| Navigation-only pages | Falls back to custom pruning | `test_readability_fallback_on_nav_only` |
| Invalid URL | `ExtractionError::InvalidUrl` | `test_extract_article_invalid_url` |
| Malformed HTML | Readability handles via `html5ever` parser | Implicit |
| Paywalls/cookie banners | Readability removes or graceful failure | Implicit |
| Multiple articles | Readability chooses main content | Implicit |
| Low confidence | Configurable threshold with fallback | `prune_html` logic |

