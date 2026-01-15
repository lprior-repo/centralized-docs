---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#18
chunk_level: summary
chunk_type: prose
heading: Integration Points
token_count: 143
summary: readability = \"0. 3\"     # Mozilla algorithm
---

readability = "0.3"     # Mozilla algorithm
thiserror = "1.0"       # Error types
tantivy = "0.25"        # BM25 search
```

## Integration Points

### Used By:
- `/home/lewis/src/centralized-docs/doc_transformer/src/scrape.rs`
  - `prune_html()` called at line 283
  - `filter_markdown()` called for post-processing

### Public API Surface:
```rust
// New API (recommended)
pub fn extract_article(html: &str, url: &str) -> Result<ExtractedContent, ExtractionError>

// Legacy API (backwards compatible)
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult
