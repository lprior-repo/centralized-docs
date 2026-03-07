---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#1-summary
chunk_level: summary
chunk_type: prose
heading: `doc_transformer`
token_count: 134
summary: # CLI Reference. The `doc_transformer` CLI provides tools to scrape, index, and search documentation
---

# CLI Reference

The `doc_transformer` CLI provides tools to scrape, index, and search documentation.

## `doc_transformer`

**Usage:**
```bash
doc_transformer <COMMAND> [OPTIONS]
```

### Commands

#### `search`
Search indexed documentation using BM25.
- `QUERY`: Query string to search for.
- `--index-dir`, `-d <DIR>`: Directory containing `INDEX.json`.
- `--limit`, `-n <N>`: Maximum number of results to return (default: 10).
- `--no-color`: Disable colored output.
- `--json`: Output structured JSON for machine parsing.

#### `scrape`
Scrape a documentation website to local markdown files.
- `URL`: URL of the documentation site to scrape.
