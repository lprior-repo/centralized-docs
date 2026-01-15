---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#8
chunk_level: detailed
chunk_type: table
heading: Integration Points
token_count: 349
summary: | Edge Case | Behavior | Test |. |-----------|----------|------|
---

| Edge Case | Behavior | Test |
|-----------|----------|------|
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

