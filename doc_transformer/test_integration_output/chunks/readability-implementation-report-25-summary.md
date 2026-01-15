---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#25
chunk_level: summary
chunk_type: prose
heading: Known Limitations
token_count: 72
summary:    - `prune_html()` uses `https://example. com` as base URL
---



   - `prune_html()` uses `https://example.com` as base URL
   - New code should use `extract_article()` with real URL

3. **Confidence Threshold:**
   - Default 0.3 is conservative
   - May need tuning based on target documentation sites
   - Consider making it configurable per-scrape

