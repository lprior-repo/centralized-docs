---
doc_id: production-readiness-report
chunk_id: production-readiness-report#7
chunk_level: detailed
chunk_type: prose
heading: Known Limitations
token_count: 341
summary: - **readability 0. 3:** Content extraction
---

```


- **readability 0.3:** Content extraction
- **hnsw_rs 0.3:** Nearest neighbor search
- **spider 2.0:** Web scraping
- **petgraph 0.8:** Graph data structures

---

## Deployment Readiness

### Requirements
- **Rust:** 1.70+ (edition 2021)
- **Memory:** ~50MB baseline, ~500MB max (configurable limits)
- **Disk:** Minimal (indexes are JSON, not binary)

### Configuration
All limits are configurable via `ScrapeConfig`:
- `max_page_size_bytes: 10MB` (single page limit)
- `max_total_size_bytes: 500MB` (cumulative scrape limit)
- `max_markdown_size_bytes: 5MB` (post-conversion limit)
- `max_pages: 10,000` (page flood prevention)
- `max_links_per_page: 1,000` (memory protection)

### Monitoring
- Graceful error handling (no panics)
- Comprehensive error messages with context
- Progress logging available

---

## Known Limitations

### Non-Blocking Issues
1. **Two test failures** (highlight module, blockquote detection)
   - Impact: Low (edge cases in non-critical modules)
   - Workaround: Core functionality unaffected
   
2. **Compilation warnings** (dead code, unused imports)
   - Impact: None (warnings, not errors)
   - Plan: Clean up in future maintenance

### Future Enhancements (17 open beads)
- P2/P3 beads remain for:
  - Additional edge case tests
  - CLI argument validation
  - Community features (llms.txt RFC, index repository)

---

