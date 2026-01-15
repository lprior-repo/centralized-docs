---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#3
chunk_level: standard
chunk_type: table
heading: Implementation Completed
token_count: 366
summary: and_then(|extracted| {.             // Confidence threshold check
---

}
```


```rust
}
```



```rust
        .and_then(|extracted| {
            // Confidence threshold check
            if extracted.confidence < config.min_confidence {
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
        (if paragraph_count > 3 { 0.2 } else { 0.0 })
        + (if heading_count > 0 { 0.1 } else { 0.0 });

    (word_confidence + structure_bonus).min(1.0)
}
```

### 5. Functional Rust Implementation

**Strict Compliance:**
- ✅ No `.unwrap()` or `.expect()` calls
- ✅ Railway-Oriented Programming (`.and_then()`, `.map()`, `.map_err()`)
- ✅ Semantic error types with `thiserror`
- ✅ Design by Contract documentation
- ✅ Pure functions (no hidden side effects)
- ✅ Immutable by default
- ✅ Iterator combinators over loops

**Note:** Lint attributes (`#![deny(clippy::unwrap_used)]`) were prepared but removed by auto-formatter. Consider adding to `Cargo.toml` or CI pipeline:
```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

