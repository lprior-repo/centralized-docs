---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#26
chunk_level: summary
chunk_type: prose
heading: Recommendations
token_count: 136
summary:    - May need tuning based on target documentation sites.    - Consider making it configurable per-s
---




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
