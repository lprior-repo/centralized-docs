---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#8
chunk_level: standard
chunk_type: table
heading: Dependencies Verified
token_count: 134
summary: | Low confidence | Configurable threshold with fallback | `prune_html` logic |. ## Removed Custom He
---






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

