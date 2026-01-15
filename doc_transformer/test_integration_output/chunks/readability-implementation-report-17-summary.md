---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#17
chunk_level: summary
chunk_type: prose
heading: Dependencies Verified
token_count: 76
summary: **Deleted:**. - `extract_main_content()` - Used when Readability fails
---

**Deleted:**

- `extract_main_content()` - Used when Readability fails
- `filter_markdown()` - Post-processing cleanup

## Dependencies Verified

Already in `Cargo.toml`:
```toml
readability = "0.3"     # Mozilla algorithm
thiserror = "1.0"       # Error types
tantivy = "0.25"        # BM25 search
```

