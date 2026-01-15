---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#19
chunk_level: summary
chunk_type: prose
heading: Integration Points
token_count: 61
summary: // Legacy API (backwards compatible). pub fn prune_html(html: &str, config: &FilterConfig) -> Filter
---



```rust

// Legacy API (backwards compatible)
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult

// Supporting types
pub struct ExtractedContent { ... }
pub enum ExtractionError { ... }
pub struct FilterConfig { ... }
```

