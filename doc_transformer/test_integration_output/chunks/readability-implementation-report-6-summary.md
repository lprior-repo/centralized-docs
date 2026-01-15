---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#6
chunk_level: summary
chunk_type: table
heading: Implementation Completed
token_count: 131
summary:                 Err(ExtractionError::LowConfidence { .                 Ok(extracted)
---

```rust
                Err(ExtractionError::LowConfidence { ... })
            } else {
                Ok(extracted)
            }
        })
        .map(|extracted| FilterResult { ... })
        .unwrap_or_else(|_| fallback_prune_html(html, config))
}
```

### 4. Confidence Calculation

Smart heuristics for content quality:
```rust
fn calculate_confidence(content: &str) -> f32 {
    // Word count (max at 500)
    let word_confidence = (word_count / 500.0).min(1.0);

    // Structure bonuses
    let structure_bonus =
