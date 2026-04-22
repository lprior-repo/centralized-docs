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

### Added — redb State Database

- `StateDb` newtype wrapper over `redb::Database` with builder pattern
- 9 redb table definitions: file state, URL state, multimap (source_path → chunk_ids), snapshots, documents, scrape outputs, transforms, analysis, chunks, metadata
- `rkyv` zero-copy deserialization for fixed-size `FileStateRaw` (200 bytes) and `UrlStateRaw` (120 bytes)
- `bytemuck`-derived POD types for direct byte casts from redb storage
- Builder durability tuning: file-backed, in-memory, and paranoid modes
- Bulk load operations for idempotent state initialization across multiple calls
- Snapshot APIs on `StateReadSession` and `StateDb` for watch/apply workflow
- `StateReadSession` wrapping redb read transactions with owned archived bytes
- Batch builder for URL-state and scrape-output commit batches
- Compact state DB via `ctd compact` CLI command with churn-recovery tests
- State commit at shutdown — single redb write transaction for all changes

### Added — Idempotent Operations

- Cache open idempotency verified across 100+ open/close cycles
- State table initialization idempotent on repeated calls
- Bulk load idempotency for both file-state and URL-state
- Analysis reuse for unchanged files during re-indexing
- Transform artifact capture with source-path reuse

### Added — Watch/Apply/Diff Workflow

- Terraform-style `watch`/`apply`/`diff` commands for index lifecycle
- Snapshot persistence migrated from DocCache to StateDb
- `--connect-timeout-secs` flag on watch command
- Change plan output in both Markdown and JSON formats
- Load archived scrape outputs for unchanged pages (skip re-scrape)

### Added — MCP Server

- `ctd-mcp` binary with `rmcp` v1.3 stdio transport
- Three tools: `search_docs`, `read_chunk`, `get_related_concepts`
- `CtdMcpError` enum with structured error handling
- Client config guides for Claude Desktop, Claude Code, VS Code, and Cursor

### Added — Search & Indexing

- BM25 search with configurable HNSW parameters (`--hnsw-m`, `--hnsw-ef-construction`)
- `llms.txt` / `llms-full.txt` parser and validator binary
- Contextual chunking with prefix context windows (~50-100 tokens)
- Knowledge graph (DAG) with Parent, Sequential, and Related edges
- `--with-agents` flag to generate `AGENTS.md` for AI agent integration
- `--max-document-bytes` flag to skip oversized documents
- Category auto-detection with configurable overrides (`--category-config`)
- `--max-related-chunks` and `--max-chunk-keywords` tuning flags

### Added — Scrape & Networking

- Shared state session with graceful shutdown commit
- Connect timeout enforcement via TCP pre-check
- Tracing instrumentation across all async functions and MCP tools

### Added — CLI

- `ctd` main binary with 10 subcommands (search, scrape, ingest-git, index, ingest, watch, apply, diff, compact, mcp serve)
- `ctd-mcp` dedicated MCP server binary
- `llms_txt_validator` validation helper binary
- Exhaustive CLI permutation test suite (608 tests, 100% pass rate)

### Added — Distribution & Docs

- Website documentation (mdBook) deployed to GitHub Pages
- Install scripts for Unix (`install.sh`) and Homebrew (`install-brew.sh`)
- GitHub Actions CI for multi-platform releases (Linux x86_64, macOS aarch64, Windows x86_64)
- SHA256 checksum verification in install script

### Changed

- Full clippy cleanup with functional Rust lint gates (zero-unwrap, no-panic, no-indexing)
- CLI ergonomics improvements across all commands
- Pod crate split with newtypes and debug assertions
- Replaced LRU backend with blessed LRU for bounded in-memory cache
- All `println!`/`dbg!` replaced with structured `tracing` spans
- Architectural drift correction across codebase (300-line file enforcement)

### Fixed

- Exit non-zero on corrupt state database with proper error code mapping
- Proptest regex excludes Unicode whitespace (NBSP, NEL) for portability
- 42 test inquisition findings resolved (10 LETHAL, 20 MAJOR, 12 MINOR)
- 6 QA bugs: exit codes, stale tests, clippy, unused imports

[0.7.1]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.1
[0.7.0]: https://github.com/lprior-repo/centralized-docs/releases/tag/v0.7.0
