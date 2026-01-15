---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#9
chunk_level: detailed
chunk_type: prose
heading: Performance Characteristics
token_count: 309
summary: **Deleted:**. - `filter_markdown()` - Post-processing cleanup
---



**Deleted:**

- `filter_markdown()` - Post-processing cleanup

## Dependencies Verified

Already in `Cargo.toml`:
```toml
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

