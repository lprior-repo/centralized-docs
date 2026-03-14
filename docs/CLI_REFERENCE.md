# CLI Reference

The `ctd` CLI provides tools to scrape, index, and search documentation.

## `ctd`

**Usage:**
```bash
ctd <COMMAND> [OPTIONS]
```

### Commands

#### `search`
Search indexed documentation using BM25.
- `QUERY`: Query string to search for.
- `--index-dir`, `-i <DIR>`: Directory containing `INDEX.json`.
- `--limit`, `-n <N>`: Maximum number of results to return (default: 10).
- `--no-color`: Disable colored output.
- `--json`: Output structured JSON for machine parsing.

#### `scrape`
Scrape a documentation website to local markdown files.
- `URL`: URL of the documentation site to scrape.
- `--output`, `-o <DIR>`: Output directory for scraped content.
- `--no-sitemap`: Disable sitemap.xml discovery (use crawling instead).
- `--filter`, `-f <REGEX>`: Regex pattern to filter URLs by path.
- `--delay`, `-d <MS>`: Delay between requests in milliseconds (0-60000, default: 0).
- `--request-timeout-secs <SECS>`: Request timeout in seconds (1-600, default: 30).
- `--max-retries <N>`: Max spider retries (0 disables spider retry, default: 3).
- `--redirect-policy <POLICY>`: Redirect policy (loose, strict, none, default: loose).
- `--max-page-bytes <BYTES>`: Max bytes per page.
- `--max-total-bytes <BYTES>`: Max total bytes across crawl.
- `--concurrency <N>`: Concurrency (default: 4).
- `--query`, `-q <QUERY>`: Filter pages by BM25 relevance to query.
- `--threshold <SCORE>`: Minimum BM25 score to keep a page (0.0-10.0, default: 0.1).

#### `ingest-git`
Clone and index Git-hosted documentation.
- `REPO_URL`: Git repository URL to clone.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--branch <BRANCH>`: Git branch to checkout (default: main).
- `--depth <N>`: Clone depth (0 = full, 1 = shallow/faster, default: 1).
- `--project-name <NAME>`: Project name for llms.txt header.
- `--filter`, `-f <REGEX>`: Regex pattern to filter file paths.

#### `index`
Index local markdown files into an AI-optimized structure.
- `SOURCE`: Source directory containing markdown files.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--llms-txt`: Generate `llms.txt` entry point files (default: true).
- `--project-name <NAME>`: Project name for llms.txt header (default: Documentation).
- `--project-desc <DESC>`: Project description for llms.txt.
- `--category-config <FILE>`: Path to category rules config file.
- `--max-related-chunks <N>`: Maximum number of related chunks per document (1-100, default: 20).
- `--max-chunk-keywords <N>`: Maximum number of chunk keywords (0-50, default: 12).
- `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16).
- `--hnsw-ef-construction <EF>`: HNSW graph construction effort (50-1000, default: 200).
- `--max-document-bytes <BYTES>`: Maximum document size in bytes (default: 10MB).

#### `ingest`
Scrape and index in one step.
- `URL`: URL of the documentation site.
- `--output`, `-o <DIR>`: Output directory for final indexed content.
- All scrape and index options available.
