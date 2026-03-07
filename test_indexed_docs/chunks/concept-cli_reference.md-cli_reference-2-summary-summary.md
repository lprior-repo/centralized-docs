---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#2-summary
chunk_level: summary
chunk_type: prose
heading: `doc_transformer`
token_count: 133
summary: - `--output`, `-o <DIR>`: Output directory for scraped content. - `--no-sitemap`: Disable sitemap
---

- `--output`, `-o <DIR>`: Output directory for scraped content.
- `--no-sitemap`: Disable sitemap.xml discovery (use crawling instead).
- `--filter`, `-f <REGEX>`: Regex pattern to filter URLs by path.
- `--delay`, `-d <MS>`: Delay between requests in milliseconds (0-60000, default: 250).
- `--request-timeout-secs <SECS>`: Request timeout in seconds (1-600, default: 30).
- `--max-retries <N>`: Max spider retries (0 disables spider retry, default: 3).
- `--redirect-policy <POLICY>`: Redirect policy (loose, strict, none, default: loose).
- `--max-page-bytes <BYTES>`: Max bytes per page.
- `--max-total-bytes <BYTES>`: Max total bytes across crawl.
