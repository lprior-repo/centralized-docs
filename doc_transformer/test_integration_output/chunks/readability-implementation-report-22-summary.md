---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#22
chunk_level: summary
chunk_type: prose
heading: Migration Path
token_count: 130
summary: **BM25 Scoring:**. - Uses Tantivy (ephemeral in-memory index)
---




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
