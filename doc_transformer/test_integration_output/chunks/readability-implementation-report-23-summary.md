---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#23
chunk_level: summary
chunk_type: prose
heading: Migration Path
token_count: 52
summary:         // Use extracted.     Err(ExtractionError::NoContent) => {
---

```rust
// Before

        // Use extracted.content
    }
    Err(ExtractionError::NoContent) => {
        // Handle empty pages
    }
    Err(e) => {
        eprintln!("Extraction failed: {}", e);
    }
}
```

