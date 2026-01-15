---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#12
chunk_level: standard
chunk_type: prose
heading: Known Limitations
token_count: 257
summary: For new code:. // After (with better error handling)
---

For new code:
```rust
// Before

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

## Known Limitations

1. **Auto-formatter Conflict:**
   - Lint attributes (`#![deny(clippy::unwrap_used)]`) were removed by formatter
   - **Recommendation:** Add to project-level `Cargo.toml` or CI pipeline

2. **Dummy URL in Legacy API:**
   - `prune_html()` uses `https://example.com` as base URL
   - Not an issue for scraping (URL used for relative link resolution)
   - New code should use `extract_article()` with real URL

3. **Confidence Threshold:**
   - Default 0.3 is conservative
   - May need tuning based on target documentation sites
   - Consider making it configurable per-scrape

