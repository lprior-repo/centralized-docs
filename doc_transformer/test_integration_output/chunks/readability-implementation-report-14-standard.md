---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#14
chunk_level: standard
chunk_type: prose
heading: Verification Steps
token_count: 161
summary:    panic = \"deny\".    // Replace line 283
---

   ```toml
   panic = "deny"
   ```

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

## Verification Steps

Run tests:
```bash
cd doc_transformer
cargo test --lib filter
# Result: ok. 39 passed; 0 failed
```

Build project:
```bash
cargo build
# Result: Finished `dev` profile [unoptimized + debuginfo]
```

