# Centralized Docs (`ctd`)

> **AI-Generated & Human-Reviewed** — Built with AI assistance, adversarial testing, and functional Rust principles.

## Links

| Resource | URL |
|----------|-----|
| **Documentation** | https://lprior-repo.github.io/centralized-docs/ |
| **GitHub** | https://github.com/lprior-repo/centralized-docs |
| **Releases** | https://github.com/lprior-repo/centralized-docs/releases |
| **MCP Server** | https://lprior-repo.github.io/centralized-docs/mcp-server.html |

---

## What It Is

`ctd` transforms raw documentation into AI-optimized, searchable knowledge structures:

- **Scrape** any static documentation site
- **Chunk** content with surrounding context (~50-100 tokens)
- **Index** using BM25 full-text search
- **Graph** document relationships (Parent, Sequential, Related edges)
- **Generate** `llms.txt` entry point for AI agents
- **Serve** via MCP (Model Context Protocol) for native AI integration
- **Track** changes with idempotent, incremental operations

---

## AI Warning

**I wouldn't recommend running this in production.** This is a personal tool I built for my own workflow. It works for me—I've tested it heavily with adversarial testing and functional Rust principles. But I'm one person with one use case.

Why Rust? Because I'm writing AI-assisted code, and the compiler is my quality control. Rust catches use-after-free, data races, and type mismatches that would slip through in other languages.

Consider this a prototype or reference implementation. Fork it, modify it, make it yours.

---

## Quick Start

```bash
# Install
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash

# Scrape + Index
ctd ingest https://kubernetes.io/docs/home/ --output ./k8s-docs

# Search
ctd search "authentication" --index-dir ./k8s-docs --json --limit 5

# MCP Server
ctd mcp serve ./k8s-docs
```

---

## How to Install

### Release Binary (Linux x86_64, macOS Apple Silicon)

```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

### Build From Source

```bash
cargo install --path centralized-docs
```

### Verify

```bash
ctd --version
ctd --help
```

---

## Commands

| Command | Description |
|---------|-------------|
| `ctd ingest <URL>` | Scrape and index a documentation site in one step |
| `ctd scrape <URL>` | Scrape a site to local markdown files |
| `ctd index <PATH>` | Index local markdown files |
| `ctd search <QUERY>` | Search the index using BM25 |
| `ctd watch <URL>` | Preview changes (Terraform-style plan) |
| `ctd apply <URL>` | Commit changes (Terraform-style apply) |
| `ctd diff <DIR_A> <DIR_B>` | Compare two scrape directories |
| `ctd compact <STATE_DB>` | Reclaim disk space from state database |
| `ctd mcp serve <DIR>` | Start MCP server for AI agent integration |
| `ctd ingest-git <REPO_URL>` | Clone and index Git-hosted documentation |

---

## Origin Story

I got tired of AI agents hallucinating from documentation.

The standard workflow sucks:

- Blindly copy/pasting massive chunks of docs into prompts
- Manually curating "skills" or context files for every library
- Watching AI agents lose context in unstructured documentation

So I built `ctd`. Point an AI at the `llms.txt`, and it intelligently traverses the knowledge graph and searches the index rather than blindly reading everything.

---

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

---

## License

MIT
