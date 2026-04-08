# Getting Started

This guide covers the production release flow for `ctd` `v0.7.0`.

## Install a Release Binary

```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

The installer:

- detects your platform (Linux x86_64, macOS ARM, Windows x86_64)
- downloads the matching release archive
- verifies the archive with `SHA256SUMS.txt`
- installs `ctd`, `ctd-mcp`, and `llms_txt_validator` into `~/.local/bin`

## Install From Source

```bash
cargo install --path centralized-docs
```

## Verify the Installation

```bash
ctd --version
ctd --help
llms_txt_validator -V
```

## MCP Server Installation

The MCP server is included with `ctd`. After installing, verify MCP functionality:

```bash
ctd mcp serve --help
```

### Quick Start

1. **Index your documentation:**
   ```bash
   ctd index ./docs --output ./output --project-name "My Docs"
   ```

2. **Start the MCP server:**
   ```bash
   ctd mcp serve ./output
   ```

3. **Connect** your AI client (Claude Desktop, Claude Code, etc.) to the running server.

See the [MCP Server](mcp-server.md) documentation for full configuration details.

## Index Local Documentation

```bash
ctd index ./docs --output ./output --project-name "My Docs"
```

Expected outputs:

- `llms.txt` — AI entry point (read this first)
- `INDEX.json` — Machine-readable index with chunks and knowledge DAG
- `NAVIGATION.md` — Human-readable navigation guide
- `docs/` — Transformed documents with YAML frontmatter
- `chunks/` — Semantic chunks with context prefix
- `state.redb` — State database for incremental re-indexing

Optional (with `--with-agents`):

- `AGENTS.md` — Instructions file for AI coding agents

## Search the Resulting Index

```bash
ctd search "authentication" --index-dir ./output --limit 5 --json
```

## Validate the Generated Metadata

```bash
llms_txt_validator ./output/llms.txt
llms_txt_validator --index ./output/INDEX.json
```

## Documentation Site

To preview it locally:

```bash
cd website
mdbook serve --open
```
