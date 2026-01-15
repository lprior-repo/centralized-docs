---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#7
chunk_level: detailed
chunk_type: table
heading: Dependencies Verified
token_count: 251
summary: | Edge Case | Behavior | Test |. |-----------|----------|------|
---

| Edge Case | Behavior | Test |
|-----------|----------|------|
| Navigation-only pages | Falls back to custom pruning | `test_readability_fallback_on_nav_only` |
| Invalid URL | `ExtractionError::InvalidUrl` | `test_extract_article_invalid_url` |
| Malformed HTML | Readability handles via `html5ever` parser | Implicit |
| Paywalls/cookie banners | Readability removes or graceful failure | Implicit |
| Multiple articles | Readability chooses main content | Implicit |
| Low confidence | Configurable threshold with fallback | `prune_html` logic |

## Removed Custom Heuristics

**Deleted:**
- ~~`text_density_score()` function~~ (replaced by Readability + `calculate_text_density`)

**Retained (for fallback):**
- `extract_main_content()` - Used when Readability fails
- `filter_markdown()` - Post-processing cleanup

## Dependencies Verified

Already in `Cargo.toml`:
```toml
readability = "0.3"     # Mozilla algorithm
thiserror = "1.0"       # Error types
tantivy = "0.25"        # BM25 search
```

