---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#3-summary
chunk_level: summary
chunk_type: prose
heading: `doc_transformer`
token_count: 134
summary: - `--concurrency <N>`: Concurrency (1-2, default: 1). - `--query`, `-q <QUERY>`: Filter pages by BM25 relevance to query
---

- `--concurrency <N>`: Concurrency (1-2, default: 1).
- `--query`, `-q <QUERY>`: Filter pages by BM25 relevance to query.
- `--threshold <SCORE>`: Minimum BM25 score to keep a page (0.0-10.0, default: 0.1).

#### `ingest-git`
Clone and index Git-hosted documentation.
- `REPO_URL`: Git repository URL to clone.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--branch <BRANCH>`: Git branch to checkout (default: main).
- `--depth <N>`: Clone depth (0 = full, 1 = shallow/faster, default: 1).
- `--project-name <NAME>`: Project name for llms.txt header.
