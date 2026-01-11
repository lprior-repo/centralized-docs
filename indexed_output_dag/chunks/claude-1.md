---
doc_id: claude
chunk_id: claude#1
chunk_type: prose
heading: ⚠️ MANDATORY: TCR JAIL - READ FIRST
token_count: 317
summary: # Claude Code Context. This document provides context for Claude (and other AI tools) working on thi
---

# Claude Code Context

This document provides context for Claude (and other AI tools) working on this codebase.

## ⚠️ MANDATORY: TCR JAIL - READ FIRST

**YOU ARE IN A TCR (Test-Commit-Revert) JAIL. THERE IS NO ESCAPE.**

### Rules - NON-NEGOTIABLE

1. **NEVER use `git commit` directly** - Use `./scripts/tcr.sh "message"` ONLY
2. **NEVER use `git add` followed by `git commit`** - TCR handles this
3. **NEVER skip tests** - Tests MUST pass or changes are auto-reverted
4. **NEVER use `--no-verify`** - Git hooks enforce TCR, cannot be bypassed

### How TCR Works

```bash
# The ONLY way to commit:
./scripts/tcr.sh "Your commit message"

# What happens:
# 1. Code is formatted (go fmt)
# 2. Code is vetted (go vet)
# 3. Tests are run (go test ./...)
# 4. IF tests pass → Commit is created
# 5. IF tests fail → ALL CHANGES ARE REVERTED (no exceptions)
```

### Safe Rollback

If anything goes wrong, the user can always run:
```bash
./scripts/revert-to-safe.sh
# Returns to SAFE_POINT tag, nukes everything
```

### Consequences of Failure

- Tests fail → Your code is GONE
- Vet fails → Your code is GONE
- Format fails → Your code is GONE
- No partial commits allowed
- No "I'll fix it later"

**Write tests FIRST. Make them pass. Then commit.**

---

