# Agent Instructions

## Core Engineering Rules & Development Lifecycle

*These are absolute guarantees. The system MUST ALWAYS do these and MUST NEVER violate them.*

```jsonl
{"rule": "CODEBASE_LOCATION", "mandatory": true, "description": "Code lives in the current working directory. Always operate relative to this root."}
{"rule": "WORKSPACE_ISOLATION", "mandatory": true, "description": "ALWAYS use Jujutsu (jj) for workspace isolation. Create a new jj workspace/bookmark for every task."}
{"rule": "RUST_CONTRACTS", "mandatory": true, "description": "You MUST ALWAYS invoke the `rust-contract` skill to specify the Rust contract before coding."}
{"rule": "FUNCTIONAL_RUST", "mandatory": true, "description": "You MUST ALWAYS invoke the `functional-rust` skill to implement. Data->Calc->Actions, zero panics/unwrap/mut, clippy-flawless."}
{"rule": "COMBATIVE_TESTING", "mandatory": true, "description": "Always write combative tests. Tests MUST compile."}
{"rule": "TOOLING", "mandatory": true, "description": "Always use `moon`, NEVER `cargo`. 100% moon CICD run."}
{"rule": "QA_REVIEW", "mandatory": true, "description": "You MUST ALWAYS invoke the `qa-enforcer` skill for review, adversarial testing, and strict validation."}
{"rule": "QUALITY_GATES", "mandatory": true, "description": "ALL tests, linting, and code MUST compile. Fix ALL issues, even if they are not from your changes."}
{"rule": "NO_MIGRATIONS", "mandatory": true, "description": "Migrations don't exist."}

{"phase": "1_RESEARCH", "transition_condition": "Understand constraints and codebase context."}
{"phase": "2_PLAN", "transition_condition": "Draft a deterministic plan."}
{"phase": "3_CONTRACT_REVIEW", "transition_condition": "Invoke `rust-contract` skill. Output contract-spec.md and martin-fowler-tests.md."}
{"phase": "4_IMPLEMENTATION", "transition_condition": "Use `jj` to isolate. Invoke `functional-rust` skill. Implement via TDD."}
{"phase": "5_VERIFICATION", "transition_condition": "Verify implementation against the contract spec."}
{"phase": "6_QA_ENFORCEMENT", "transition_condition": "Invoke `qa-enforcer` skill. Fix all found issues."}
{"phase": "7_MOON_CICD", "transition_condition": "Execute 100% moon CICD run. Must pass completely."}
{"phase": "8_MERGE", "transition_condition": "Merge it IF AND ONLY IF all of the above phases are true and verified."}
```

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work atomically
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Non-Interactive Shell Commands

**ALWAYS use non-interactive flags** with file operations to avoid hanging on confirmation prompts.

Shell commands like `cp`, `mv`, and `rm` may be aliased to include `-i` (interactive) mode on some systems, causing the agent to hang indefinitely waiting for y/n input.

**Use these forms instead:**
```bash
# Force overwrite without prompting
cp -f source dest           # NOT: cp source dest
mv -f source dest           # NOT: mv source dest
rm -f file                  # NOT: rm file

# For recursive operations
rm -rf directory            # NOT: rm -r directory
cp -rf source dest          # NOT: cp -r source dest
```

**Other commands that may prompt:**
- `scp` - use `-o BatchMode=yes` for non-interactive
- `ssh` - use `-o BatchMode=yes` to fail instead of prompting
- `apt-get` - use `-y` flag
- `brew` - use `HOMEBREW_NO_AUTO_UPDATE=1` env var

<!-- BEGIN BEADS INTEGRATION -->
## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Auto-syncs to JSONL for version control
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

bd automatically syncs with git:

- Exports to `.beads/issues.jsonl` after changes (5s debounce)
- Imports from JSONL when newer (e.g., after `git pull`)
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
   bd sync
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
