# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.1] - 2026-04-21

### Fixed

- Restored all-features workspace feature gates for correct compilation
- Hardened MCP server and scrape release checks

### Changed

- Added Windows x86_64 platform support to install script (MSYS2/Git Bash/WSL)
- Added `.qa-*` and `.memsearch/` to `.gitignore`
- Removed stale pre-built `website/book/` from repository (now built by CI)
- Added CHANGELOG.md

## [0.7.0] - 2026-04-20

### Added

- MCP server (`ctd-mcp`) with three tools: `search_docs`, `read_chunk`, `get_related_concepts`
- `rmcp`-based stdio transport for MCP protocol
- Terraform-style `watch`/`apply`/`diff` workflow for index lifecycle management
- `compact` command for state database compaction
- BM25 search with configurable HNSW parameters (`--hnsw-m`, `--hnsw-ef-construction`)
- `llms.txt` / `llms-full.txt` parser and validator
- Contextual chunking with prefix context windows
- Knowledge graph (DAG) with Parent, Sequential, and Related edges
- `--with-agents` flag to generate `AGENTS.md` for AI agent integration
- `--max-document-bytes` flag to skip oversized documents
- Category auto-detection with configurable overrides (`--category-config`)
- Website documentation (mdBook) deployed to GitHub Pages
- Install scripts for Unix (`install.sh`) and Homebrew (`install-brew.sh`)
- GitHub Actions CI for multi-platform releases (Linux, macOS, Windows)
- Exhaustive CLI permutation test suite (608 tests)

### Changed

- Full clippy cleanup with functional Rust lint gates
- CLI ergonomics improvements across all commands
- Architectural drift correction across codebase

[0.7.1]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.1
[0.7.0]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.0
