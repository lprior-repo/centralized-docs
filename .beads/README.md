# Beads - AI-Native Issue Tracking

Welcome to Beads! This repository uses **beads** for issue tracking - a lightweight, Rust-based implementation designed to live directly in your codebase alongside your code.

## What is beads?

beads is a non-invasive issue tracker that lives in your repo, making it perfect for AI coding agents and developers who want their issues close to their code. No web UI, no background daemon, no auto-commits - everything is explicit and predictable.

**Learn more:** [github.com/steveyegge/beads](https://github.com/steveyegge/beads)

## Quick Start

### Essential Commands

**Note:** `br` is non-invasive and never executes git commands. After `br sync --flush-only`, you must manually run `git add .beads/ && git commit`.

```bash
# Create new issues
br create "Add user authentication"

# View all issues
br list

# View issue details
br show <issue-id>

# Update issue status
br update <issue-id> --status in_progress
br close <issue-id>

# Sync to JSONL (no git)
br sync --flush-only
git add .beads/
git commit -m "sync beads"
```

### Working with Issues

Issues in Beads are:
- **Git-native**: Stored in `.beads/issues.jsonl` and synced like code
- **AI-friendly**: CLI-first design works perfectly with AI coding agents
- **Non-invasive**: Never auto-commits or modifies git state
- **Explicit**: All git operations are manual and predictable

## Why Beads?

✨ **AI-Native Design**
- Built specifically for AI-assisted development workflows
- CLI-first interface works seamlessly with AI coding agents
- No context switching to web UIs
- Non-invasive: never auto-commits

🚀 **Developer Focused**
- Issues live in your repo, right next to your code
- Works offline, syncs when you push
- Fast, lightweight, Rust implementation
- Explicit git operations for predictability

🔧 **Git Integration**
- Manual sync with explicit git commands
- SQLite + JSONL hybrid storage
- Intelligent JSONL merge resolution

## Get Started with Beads

Try beads in your own projects:

```bash
# Install beads
curl -fsSL "https://raw.githubusercontent.com/steveyegge/beads/main/install.sh" | bash

# Initialize in your repo
br init

# Create your first issue
br create "Try out beads"
```

## Learn More

- **Documentation**: [github.com/steveyegge/beads](https://github.com/steveyegge/beads)
- **Quick Start Guide**: Run `br --help`
- **Issue Viewer**: Use `bv` (beads_viewer) for TUI and graph analysis

---

*beads: Lightweight, non-invasive issue tracking for AI agents* ⚡
