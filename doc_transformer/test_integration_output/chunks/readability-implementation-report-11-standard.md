---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#11
chunk_level: standard
chunk_type: prose
heading: Migration Path
token_count: 225
summary: ## Performance Characteristics. **Readability Extraction:**
---


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

## Migration Path

For new code:
```rust
// Before
let result = prune_html(html, &config);

// After (with better error handling)
match extract_article(html, url) {
    Ok(extracted) => {
        println!("Title: {:?}", extracted.title);
        println!("Confidence: {}", extracted.confidence);
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

