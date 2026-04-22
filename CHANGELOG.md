# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.1] - 2026-04-21

### Added

- Windows x86_64 install support (MSYS2/Git Bash/WSL)

### Fixed

- Workspace compilation with all features enabled
- MCP server and scrape release-mode checks

## [0.7.0] - 2026-04-20

### Added

- **redb state database** — persistent index state across runs. Tracks file changes, URL states, and scrape history in an ACID database. Re-indexing only processes changed files.
- **Idempotent re-indexing** — running `ctd index` or `ctd ingest` multiple times produces the same output. Unchanged files are skipped using state snapshots.
- **`ctd compact`** — reclaim disk space from the state database after heavy indexing.
- **Terraform-style `watch`/`apply`/`diff`** — plan changes before committing them. `watch` shows what would change, `apply` executes the plan, `diff` compares two scrape directories.
- **MCP server (`ctd-mcp`)** — three tools (`search_docs`, `read_chunk`, `get_related_concepts`) for Claude Desktop, Claude Code, VS Code, and Cursor. Uses stdio transport, no server to manage.
- **BM25 search with HNSW tuning** — configurable via `--hnsw-m` and `--hnsw-ef-construction` for speed/accuracy tradeoffs.
- **`llms.txt` parser and validator** — parse `llms.txt` and `llms-full.txt` files. Validate structure with the `llms_txt_validator` binary.
- **Contextual chunking** — each chunk includes prefix context from the previous chunk (~50-100 tokens), giving LLMs continuity.
- **Knowledge graph** — DAG with Parent, Sequential, and Related edges between documents and chunks.
- **`--with-agents`** — generates an `AGENTS.md` file so AI agents can navigate your index without guessing.
- **`--category-config`** — override auto-detected document categories with a config file.
- **`--max-document-bytes`** — skip oversized documents during indexing.
- **Connect timeout** — `--connect-timeout-secs` flag on `watch` for unreliable networks.
- **Structured logging** — all output uses `tracing` spans instead of raw prints.
- **Multi-platform releases** — GitHub Actions CI builds for Linux x86_64, macOS aarch64, and Windows x86_64 with SHA256 verification.

[0.7.1]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.1
[0.7.0]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.0
