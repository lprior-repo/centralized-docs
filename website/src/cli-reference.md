# CLI Reference

The `ctd` CLI provides tools to scrape, index, and search documentation.

## Version

- Production release: `v0.7.0`
- Primary binary: `ctd`
- MCP server: `ctd mcp serve` (built into `ctd`, also available as `ctd-mcp`)
- Helper binary: `llms_txt_validator`

## `ctd`

**Usage:**
```bash
ctd <COMMAND>
```

### Commands

#### `search`
Search indexed documentation using BM25.
- `QUERY`: Query string to search for.
- `--index-dir`, `-i <DIR>`: Directory containing `INDEX.json` (required).
- `--limit`, `-n <N>`: Maximum number of results to return (default: 10).
- `--no-color`: Disable colored output.
- `--json`: Output structured JSON for machine parsing.

> **Exit codes:** Text mode exits 1 when no results are found (like `grep`). JSON mode exits 0 with an empty `results` array, since the response itself is valid.

#### `scrape`
Scrape a documentation website to local markdown files.
- `URL`: URL of the documentation site to scrape.
- `--output`, `-o <DIR>`: Output directory for scraped content.
- `--no-sitemap`: Disable sitemap.xml discovery (use crawling instead).
- `--filter`, `-f <REGEX>`: Regex pattern to filter URLs by path.
- `--delay`, `-d <DELAY>`: Delay between requests in milliseconds (0-60000, default: 0).
- `--request-timeout-secs <SECS>`: Request timeout in seconds (1-600, default: 30).
- `--connect-timeout-secs <SECS>`: TCP connect timeout in seconds (1-60, default: 10).
- `--max-retries <N>`: Max spider retries (0 disables spider retry, default: 3).
- `--redirect-policy <POLICY>`: Redirect policy (loose, strict, none, default: loose).
- `--max-page-bytes <BYTES>`: Max bytes per page (spider-level, before transform).
- `--max-total-bytes <BYTES>`: Max total bytes across crawl (spider-level).
- `--concurrency <N>`: Concurrency (1-128, default: 4) capped for politeness.
- `--query`, `-q <QUERY>`: Filter pages by BM25 relevance to query.
- `--threshold <SCORE>`: Minimum BM25 score to keep a page (0.0-10.0, default: 0.1).

#### `ingest-git`
Clone and index Git-hosted documentation.
- `REPO_URL`: Git repository URL to clone.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--branch <BRANCH>`: Git branch to checkout (default: main).
- `--depth <N>`: Clone depth (0 = full, 1 = shallow/faster, default: 1).
- `--project-name <NAME>`: Project name for llms.txt header.
- `--filter`, `-f <REGEX>`: Regex pattern to filter file paths (for example `^docs/en/`).

#### `index`
Index local markdown files into an AI-optimized structure.
- `SOURCE`: Source directory containing markdown files.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--llms-txt`: Generate llms.txt entry point files (default: true).
- `--with-agents`: Generate AGENTS.md file for AI coding agents.
- `--project-name <NAME>`: Project name for llms.txt header (default: Documentation).
- `--project-desc <DESC>`: Project description for llms.txt (default: `AI-optimized documentation index`).
- `--category-config <FILE>`: Path to category rules config file.
- `--max-related-chunks <N>`: Maximum number of related chunks per document (1-100, default: 20).
- `--max-chunk-keywords <N>`: Maximum number of chunk keywords to include in similarity (0-50, default: 12).
- `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16).
- `--hnsw-ef-construction <EF>`: HNSW graph construction effort (50-1000, default: 200).
- `--max-document-bytes <BYTES>`: Maximum document size in bytes (default: 10MB, warn at 5MB).

#### `ingest`
Scrape and index in one step.
- `URL`: URL of the documentation site.
- `--output`, `-o <DIR>`: Output directory for final indexed content.
- Available scrape flags: `--filter`, `--delay`, `--request-timeout-secs`, `--connect-timeout-secs`, `--max-retries`, `--redirect-policy`, `--max-page-bytes`, `--max-total-bytes`, `--concurrency`, `--query`, `--threshold`.
- Available ingest-only/index bridge flag: `--project-name`.
- `ingest` does **not** expose the full `index` flag surface.

#### `mcp serve`
Start the MCP server for AI agent integration using the Model Context Protocol.
- `INDEX_DIR`: Directory containing `INDEX.json` (required)

The server uses stdio transport, allowing AI clients like Claude Desktop or Claude Code to connect directly. See [MCP Server](mcp-server.md) for full documentation.

#### `watch`
Scrape a site and produce a change plan (Terraform-style plan).
- `URL`: URL of the documentation site to watch.
- `--output`, `-o <DIR>`: Output directory for change reports.
- `--cache <PATH>`: Path to the redb cache file for snapshots (default: `.cache/ctd_cache.redb`).
- `--no-sitemap`: Disable sitemap.xml discovery (use crawling instead).
- `--json`: Output structured JSON to stdout.
- Available scrape flags: `--filter`, `--delay`, `--request-timeout-secs`, `--connect-timeout-secs`, `--max-retries`, `--redirect-policy`, `--concurrency`.

#### `apply`
Commit a change plan snapshot (Terraform-style apply).
- `URL`: URL of the documentation site to apply snapshot for.
- `--cache <PATH>`: Path to the redb cache file for snapshots (default: `.cache/ctd_cache.redb`).
- `--scrape-dir <DIR>`: The scraped content directory (with manifest.json).
- `--yes`: Skip confirmation prompt.

#### `diff`
Compare two scrape directories and show diff.
- `DIR_A`: First scrape directory (must contain `manifest.json`).
- `DIR_B`: Second scrape directory (must contain `manifest.json`).
- `--output`, `-o <DIR>`: Output directory for diff reports.
- `--json`: Output structured JSON to stdout.

> **Note:** `diff` operates on scrape output directories (created by `ctd scrape`), not on index output directories. Scrape directories contain `manifest.json`.

#### `compact`
Compact the state database to reclaim disk space.
- `STATE_DB_PATH`: Path to the state database file (for example `.cache/ctd_cache.redb`).

## `llms_txt_validator`

Validate generated `llms.txt` and `INDEX.json` outputs.

```bash
llms_txt_validator ./output/llms.txt
llms_txt_validator --index ./output/INDEX.json
llms_txt_validator -V
```
