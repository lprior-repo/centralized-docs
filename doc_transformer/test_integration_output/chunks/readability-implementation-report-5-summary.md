---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#5
chunk_level: summary
chunk_type: table
heading: Implementation Completed
token_count: 128
summary:  Improved `prune_html()` Function. Updated legacy API to use new extraction system with Railway-Orie
---

### 3. Improved `prune_html()` Function

Updated legacy API to use new extraction system with Railway-Oriented Programming:

```rust
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult {
    extract_article(html, "https://example.com")
        .and_then(|extracted| {
            // Confidence threshold check
            if extracted.confidence < config.min_confidence {
                Err(ExtractionError::LowConfidence { ... })
            } else {
                Ok(extracted)
            }
