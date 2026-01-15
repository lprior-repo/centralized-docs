---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#10
chunk_level: standard
chunk_type: prose
heading: Performance Characteristics
token_count: 205
summary: ### Used By:. ### Public API Surface:
---


```toml
```


### Used By:

### Public API Surface:
```rust
// New API (recommended)
pub fn extract_article(html: &str, url: &str) -> Result<ExtractedContent, ExtractionError>

// Legacy API (backwards compatible)
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult

// Supporting types
pub struct ExtractedContent { ... }
pub enum ExtractionError { ... }
pub struct FilterConfig { ... }
```

## Performance Characteristics

**Readability Extraction:**
- Fast (single-pass HTML parsing)
- Uses `html5ever` (Servo/Firefox engine)
- Memory-efficient (streaming parser)

**Fallback Pruning:**
- Only triggers on Readability failure (~5-10% of pages)
- Uses `scraper` crate (CSS selector-based)

**BM25 Scoring:**
- Uses Tantivy (ephemeral in-memory index)
- ~50 LOC (was 440 LOC with custom implementation)

