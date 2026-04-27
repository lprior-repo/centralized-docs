# Centralized Docs (`ctd`)

## About This Project

**AI-Augmented Software — Human-Crafted**

This is AI-written code, but it's not "AI slop." I wrote this with heavy adversarial testing, functional Rust principles, and a commitment to craft. Every function has been questioned, every edge case probed.

**Why Rust?** Because I'm writing AI-assisted code, and the compiler is my quality control. The Rust compiler catches entire categories of bugs that would slip through in other languages—use-after-free, data races, type mismatches. It's the only language where I trust AI-generated code to not surprise me catastrophically.

**I wouldn't recommend running this in production.** This is a personal tool I built for my own workflow. It works for me, and I've tested it heavily—but I'm one person with one use case. Your mileage may vary significantly. Review the code, understand what it does, and adapt it to your needs.

If you're looking for a production-grade documentation pipeline, consider this a prototype or reference implementation. Fork it, modify it, make it yours.

## What This Is

`ctd` is a pure Rust CLI tool that transforms raw documentation and static sites into AI-optimized, searchable knowledge structures.

**The Problem I Solved:**

Getting AI agents to understand software libraries sucks:

- Blindly copy/pasting massive documentation chunks into prompts.
- Manually curating custom "skills" or context files for every library.
- Watching AI agents hallucinate from unstructured documentation downloads.

`ctd` creates an `llms.txt` entry point and a knowledge graph (DAG) so AI agents can intelligently traverse documentation rather than blindly reading everything.

## Quick Start

### Installation

**Release binary (Linux x86_64, macOS Apple Silicon):**
```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

**From source:**
```bash
cargo install --path centralized-docs
```

### Index a Documentation Site

```bash
ctd ingest https://kubernetes.io/docs/home/ --output ./k8s-docs
```

### Index Local Markdown Files

```bash
ctd index ./docs --output ./output --project-name "My Project" --with-agents
```

### Search the Index

```bash
ctd search "authentication" --index-dir ./output --json --limit 5
```

### Start the MCP Server for AI Agents

```bash
ctd mcp serve ./output
```

Connect your AI client to the running server. See [MCP Server docs](https://lprior-repo.github.io/centralized-docs/mcp-server.html) for full configuration.

### Monitor Documentation Changes

```bash
ctd watch https://docs.example.com --output ./changes
ctd diff ./old-scrape ./new-scrape --json
ctd apply https://docs.example.com --scrape-dir ./new-scrape --yes
```

## Core Features

- **Scrape Static Sites** — Crawl a documentation URL, extract content, convert to clean markdown.
- **Contextual Chunking** — Each chunk carries surrounding context (~50-100 tokens), solving "lost in the middle" problems.
- **Knowledge Graphs (DAG)** — Maps Parent, Sequential, and Related edges between documents.
- **llms.txt Generation** — `robots.txt` equivalent for AI agents.
- **Semantic Indexing** — BM25 full-text search optimized for AI queries.
- **MCP Server** — Native Model Context Protocol integration for Claude, Cursor, VS Code.
- **Idempotent Operations** — Running `ctd index` multiple times produces identical output. Unchanged files are skipped.
- **Incremental Indexing** — State database tracks changes, only re-processes what changed.
- **Watch/Apply/Diff** — Terraform-style workflow for monitoring documentation changes.

## Output Structure

```
./output/
├── llms.txt           # AI entry point (read this first)
├── INDEX.json         # Machine-readable index + knowledge DAG
├── NAVIGATION.md      # Human-readable navigation
├── AGENTS.md          # AI agent instructions (with --with-agents)
├── docs/              # Transformed documents with YAML frontmatter
├── chunks/            # Semantic chunks with context prefix
└── state.redb         # State database for incremental re-indexing
```

## Links

- **Documentation:** https://lprior-repo.github.io/centralized-docs/
- **GitHub:** https://github.com/lprior-repo/centralized-docs
- **Releases:** https://github.com/lprior-repo/centralized-docs/releases

## License

MIT
