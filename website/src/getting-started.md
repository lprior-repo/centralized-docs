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
llms_txt_validator --version
```

## Index Local Documentation

```bash
ctd index ./docs --output ./output --project-name "My Docs"
```

Expected outputs:

- `llms.txt`
- `INDEX.json`
- `COMPASS.md`
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
