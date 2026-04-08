# Centralized Docs (`ctd`)

A pure Rust CLI tool that transforms raw markdown documentation into AI-optimized, searchable structures.

## Introduction & Features

Centralized Docs (`ctd`) is a pure Rust CLI tool that transforms raw documentation and static sites into AI-optimized, searchable knowledge structures.

**The Origin Problem**

This project stemmed from the frustration of trying to get AI agents to understand modern software libraries. The standard workflows suck:

- Blindly copy/pasting massive chunks of documentation into prompts.
- Wasting time manually curating custom "skills" or context files for every new library.
- Watching AI agents hallucinate because they downloaded a massive, unstructured documentation site and lost the context in the noise.

You simply point your AI at the `llms.txt`, and it can intelligently traverse the DAG and search the index rather than blindly reading everything.

## Core Features

- **Scrape Static Sites** — Point the CLI at a documentation URL, and it will crawl the site, extract the content, and convert it to clean markdown.
- **Contextual Chunking** — Each chunk carries surrounding context, solving the typical "lost in the middle" problem when AI reads isolated pieces of text.
- **Knowledge Graphs (DAG)** — Detects and maps relationships between documents, making it easy for an agent to traverse related concepts systematically.
- **llms.txt Generation** — Automatically builds `llms.txt` files, acting as a `robots.txt` equivalent for AI agents.
- **Semantic Indexing** — Fast, full-text search optimized for AI using BM25, allowing agents to query exactly what they need.
- **MCP Server** — Built-in Model Context Protocol server for native AI agent integration (Claude Desktop, Claude Code, etc.).
- **Incremental Indexing** — State database tracks file changes, only re-processing what changed on re-index.

## Quick Start

### 1. Installation

**Release binary (Linux x86_64, macOS Apple Silicon, Windows x86_64):**
```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

**From source:**
```bash
cargo install --path centralized-docs
```

### 2. Scrape + Index a Documentation Site

```bash
ctd ingest https://kubernetes.io/docs/home/ --output ./k8s-docs
```

### 3. Index Local Markdown Files

```bash
ctd index ./docs --output ./output --project-name "My Project" --with-agents
```

### 4. Search the Index

```bash
ctd search "authentication" --index-dir ./output --json --limit 5
```

### 5. Start the MCP Server for AI Agents

```bash
ctd mcp serve ./output
```

Connect your AI client to the running server. See [MCP Server docs](website/src/mcp-server.md) for full configuration.

### 6. Monitor Documentation Changes

```bash
ctd watch https://docs.example.com --output ./changes
ctd diff ./old-scrape ./new-scrape --json
ctd apply https://docs.example.com --scrape-dir ./new-scrape --yes
```

## Output Structure

```
./output/
├── llms.txt           # AI entry point (read this first)
├── INDEX.json         # Machine-readable index + knowledge DAG
├── NAVIGATION.md      # Human-readable navigation
├── AGENTS.md          # Instructions for AI coding agents (with --with-agents)
├── docs/              # Transformed documents with YAML frontmatter
├── chunks/            # Semantic chunks with context prefix
└── state.redb         # State database for incremental re-indexing
```

## Documentation Site

The published docs site lives at `https://lprior-repo.github.io/centralized-docs/`.
