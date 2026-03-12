# Centralized Docs (`doc_transformer`)

A pure Rust CLI tool that transforms raw markdown documentation into AI-optimized, searchable structures. 

It provides tools to parse, index, and search documentation using BM25, semantic chunking, and contextual retrieval patterns.

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

This installs `doc_transformer` to `~/.local/bin` by default.

**Homebrew (builds from source):**
```bash
brew install --formula https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/Formula/doc-transformer.rb
```

**From source:**
```bash
cargo install --path doc_transformer
```

### 2. Basic Usage

```bash
# Index a directory of markdown files
doc_transformer index ./docs --output ./output

# Search the generated index
doc_transformer search "your query" --index-dir ./output
```

## Documentation

Full documentation is available in the `website/` directory, built with [mdBook](https://rust-lang.github.io/mdBook/).

```bash
# To view the docs locally:
cd website
mdbook serve --open
```

## Architecture

This project is built using Hexagonal Architecture (Ports and Adapters) to isolate domain logic from infrastructure. See the [Architecture Docs](./docs/ARCHITECTURE.md).
