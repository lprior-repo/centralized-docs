---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#24
chunk_level: summary
chunk_type: prose
heading: Known Limitations
token_count: 140
summary:         // Handle empty pages.     Err(e) => {
---

// Before

    }
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
