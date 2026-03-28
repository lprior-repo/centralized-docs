# kubernetes.io - Agent Instructions

> Documentation scraped from https://kubernetes.io/docs/home/

## Project Overview

This documentation index contains 99 documents organized by category.

### Document Categories

- **ops**: 2 documents
- **tutorial**: 46 documents
- **ref**: 49 documents
- **concept**: 2 documents

## Navigation Guide

When working with this documentation:

1. **Start with llms.txt** - Read this first to understand the structure
2. **Use INDEX.json** - For programmatic lookup of documents and chunks
3. **Follow the DAG** - Use knowledge graph edges to find related content
4. **Chunk navigation** - Each chunk has `previous_chunk_id` and `next_chunk_id`

## File Structure

```
./
├── llms.txt           # AI entry point (read first)
├── llms-full.txt      # Full content for large context models
├── AGENTS.md          # This file - coding instructions
├── INDEX.json         # Machine-readable index + knowledge graph
├── COMPASS.md         # Human-readable navigation
├── docs/              # Transformed documents with frontmatter
└── chunks/            # Semantic chunks with context prefix
```

## Chunk Format

Each chunk file contains:
- YAML frontmatter with `chunk_id`, `doc_id`, `token_count`, navigation pointers
- Context prefix from previous chunk (~50-100 tokens)
- Main content (~170 tokens average)

## INDEX.json Structure

```json
{
  "documents": [...],    // Document metadata
  "chunks": [...],       // Chunk metadata with navigation
  "keywords": {...},     // Term → doc_id lookup
  "graph": {             // Knowledge DAG
    "nodes": [...],      // Documents and chunks
    "edges": [...]       // Relationships (Parent, Sequential, Related)
  }
}
```

## Best Practices

- **Don't guess**: Use INDEX.json to find exact document/chunk IDs
- **Read context**: When reading a chunk, consider reading previous/next chunks
- **Follow relationships**: Use graph edges to find related content
- **Check frontmatter**: Every document has `category`, `tags`, and `summary`

<!-- BEGIN BEADS INTEGRATION v:1 profile:full hash:d4f96305 -->
## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Dolt-powered version control with native sync
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**

```bash
bd ready --json
```

**Create new issues:**

```bash
bd create "Issue title" --description="Detailed context" -t bug|feature|task -p 0-4 --json
bd create "Issue title" --description="What this issue is about" -p 1 --deps discovered-from:bd-123 --json
```

**Claim and update:**

```bash
bd update <id> --claim --json
bd update bd-42 --priority 1 --json
```

**Complete work:**

```bash
bd close bd-42 --reason "Completed" --json
```

### Issue Types

- `bug` - Something broken
- `feature` - New functionality
- `task` - Work item (tests, docs, refactoring)
- `epic` - Large feature with subtasks
- `chore` - Maintenance (dependencies, tooling)

### Priorities

- `0` - Critical (security, data loss, broken builds)
- `1` - High (major features, important bugs)
- `2` - Medium (default, nice-to-have)
- `3` - Low (polish, optimization)
- `4` - Backlog (future ideas)

### Workflow for AI Agents

1. **Check ready work**: `bd ready` shows unblocked issues
2. **Claim your task atomically**: `bd update <id> --claim`
3. **Work on it**: Implement, test, document
4. **Discover new work?** Create linked issue:
   - `bd create "Found bug" --description="Details about what was found" -p 1 --deps discovered-from:<parent-id>`
5. **Complete**: `bd close <id> --reason "Done"`

### Auto-Sync

bd automatically syncs via Dolt:

- Each write auto-commits to Dolt history
- Use `bd dolt push`/`bd dolt pull` for remote sync
- No manual export/import needed!

### Important Rules

- ✅ Use bd for ALL task tracking
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and docs/QUICKSTART.md.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

<!-- END BEADS INTEGRATION -->
