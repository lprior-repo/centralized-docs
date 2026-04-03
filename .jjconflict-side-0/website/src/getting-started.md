# Getting Started

This guide covers the production release flow for `ctd` `v0.6.1`.

## Install a Release Binary

```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

The installer:

- detects your platform
- downloads the matching release archive
- verifies the archive with `SHA256SUMS.txt`
- installs `ctd` and `llms_txt_validator` into `~/.local/bin`

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

- `llms.txt`
- `llms-full.txt`
- `INDEX.json`
- `COMPASS.md`
- `AGENTS.md`
- `docs/`
- `chunks/`

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
