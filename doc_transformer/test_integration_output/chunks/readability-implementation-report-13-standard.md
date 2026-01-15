---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#13
chunk_level: standard
chunk_type: prose
heading: Recommendations
token_count: 283
summary:  **Dummy URL in Legacy API:**.    - `prune_html()` uses `https://example
---

    }
}
```



2. **Dummy URL in Legacy API:**
   - `prune_html()` uses `https://example.com` as base URL
   - Not an issue for scraping (URL used for relative link resolution)
   - New code should use `extract_article()` with real URL

3. **Confidence Threshold:**
   - Default 0.3 is conservative
   - May need tuning based on target documentation sites
   - Consider making it configurable per-scrape

## Recommendations

### Immediate:
1. ✅ **DONE:** All tests pass
2. ✅ **DONE:** BEAD closed (`bd close centralized-docs-lhk`)

### Follow-up (Optional):
1. **Add lint attributes to `Cargo.toml`:**
   ```toml
   [lints.clippy]
   unwrap_used = "deny"
   expect_used = "deny"
   panic = "deny"
   ```

2. **Migrate `scrape.rs` to new API:**
   ```rust
   // Replace line 283
   let extracted = extract_article(&raw_html, &page_url)?;
   ```

3. **Add integration tests:**
   - Test against real documentation sites
   - Verify confidence scores match expectations
   - Benchmark Readability vs fallback performance

4. **Expose configuration:**
   - Add `min_confidence` to `ScrapeConfig`
   - Allow per-site confidence tuning

