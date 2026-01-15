---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#27
chunk_level: summary
chunk_type: prose
heading: Recommendations
token_count: 106
summary:  **Migrate `scrape. rs` to new API:**
---

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

