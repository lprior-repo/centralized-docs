---
doc_id: production-readiness-report
chunk_id: production-readiness-report#15
chunk_level: summary
chunk_type: prose
heading: Deployment Readiness
token_count: 86
summary: - `max_total_size_bytes: 500MB` (cumulative scrape limit). - `max_markdown_size_bytes: 5MB` (post-co
---



- `max_total_size_bytes: 500MB` (cumulative scrape limit)
- `max_markdown_size_bytes: 5MB` (post-conversion limit)
- `max_pages: 10,000` (page flood prevention)
- `max_links_per_page: 1,000` (memory protection)

### Monitoring
- Graceful error handling (no panics)
- Comprehensive error messages with context
- Progress logging available

---

