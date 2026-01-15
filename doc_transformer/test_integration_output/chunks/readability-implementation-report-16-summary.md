---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#16
chunk_level: summary
chunk_type: table
heading: Removed Custom Heuristics
token_count: 96
summary: |-----------|----------|------|. | Low confidence | Configurable threshold with fallback | `prune_ht
---




|-----------|----------|------|
| Low confidence | Configurable threshold with fallback | `prune_html` logic |

## Removed Custom Heuristics

**Deleted:**
- ~~`text_density_score()` function~~ (replaced by Readability + `calculate_text_density`)

**Retained (for fallback):**
- `extract_main_content()` - Used when Readability fails
- `filter_markdown()` - Post-processing cleanup

