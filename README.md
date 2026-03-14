# Centralized Docs (`ctd`)

A pure Rust CLI tool that transforms raw markdown documentation into AI-optimized, searchable structures.

It provides tools to scrape, index, and search documentation using BM25, semantic chunking, and contextual retrieval patterns.

## Features

- **Semantic Indexing:** Full-text search optimized for AI using BM25.
- **Contextual Chunking:** Chunks documentation while preserving surrounding context for LLM retrieval.
- **`llms.txt` Parsing:** Extracts and structures data according to the `llms.txt` standard.

## Quick Start

### 1. Installation

**Release binary (Linux x86_64 and macOS Apple Silicon):**
```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

This installs `ctd` to `~/.local/bin` by default and verifies the downloaded archive against `SHA256SUMS.txt`.

**From source:**
```bash
cargo install --path centralized-docs
```

### 2. Basic Usage

```bash
# Index a directory of markdown files
ctd index ./docs --output ./output

# Search the generated index
ctd search "your query" --index-dir ./output

# Validate generated metadata
llms_txt_validator ./output/llms.txt
```

## Production Release

- Canonical release version: `0.6.1`
- Canonical release tag: `v0.6.1`
- Primary CLI: `ctd`
- Helper CLI: `llms_txt_validator`
- Release assets: Linux x86_64, macOS Apple Silicon, Windows x86_64

## Documentation Site

The published docs site lives at `https://lprior-repo.github.io/centralized-docs/`.

## Documentation

Full documentation is available in the `website/` directory, built with [mdBook](https://rust-lang.github.io/mdBook/).

```bash
# To view the docs locally:
cd website
mdbook serve --open
```

## Architecture

This project is built using Hexagonal Architecture (Ports and Adapters) to isolate domain logic from infrastructure. See the [Architecture Docs](./docs/ARCHITECTURE.md).
