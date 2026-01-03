# TCR Jail Security Fixes & Simplified Design

## Executive Summary

The current TCR jail has **2 confirmed escape vectors** that I successfully exploited:
1. `TCR_ENABLED=false git commit` - Environment variable backdoor
2. `/usr/bin/env TCR_ENABLED=false git commit` - Env command bypass

The design is overly complex with multiple config files that are hard to maintain. Below is a simplified, truly inescapable design.

---

## Part 1: Immediate Fixes for Current Design

### Fix 1: Remove TCR_ENABLED Backdoor from Hooks

Replace `.git/hooks/pre-commit` with:

```bash
#!/usr/bin/env bash
# PRE-COMMIT HOOK: TCR JAIL - NO BACKDOORS
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== TCR JAIL: PRE-COMMIT CHECK ===${NC}"

# NO BACKDOOR - Tests always run
echo -e "${YELLOW}[1/3] Running go fmt...${NC}"
if ! go fmt ./... >/dev/null 2>&1; then
    echo -e "${RED}BLOCKED: Code is not formatted${NC}"
    exit 1
fi

echo -e "${YELLOW}[2/3] Running go vet...${NC}"
if ! go vet ./... 2>&1; then
    echo -e "${RED}BLOCKED: go vet found issues${NC}"
    exit 1
fi

echo -e "${YELLOW}[3/3] Running tests...${NC}"
if ! go test ./... -short 2>&1; then
    echo -e "${RED}=== TCR JAIL: TESTS FAILED ===${NC}"
    echo -e "${RED}COMMIT REJECTED - REVERTING CHANGES${NC}"
    git checkout -- . 2>/dev/null || true
    git clean -fd 2>/dev/null || true
    echo -e "${RED}All changes have been reverted.${NC}"
    exit 1
fi

echo -e "${GREEN}=== TCR JAIL: ALL CHECKS PASSED ===${NC}"
exit 0
```

Replace `.git/hooks/post-commit` with:

```bash
#!/usr/bin/env bash
# POST-COMMIT HOOK: TCR JAIL - FINAL VERIFICATION
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== TCR JAIL: POST-COMMIT VERIFICATION ===${NC}"

# NO BACKDOOR - Always verify
TEST_OUTPUT=$(go test ./... 2>&1) || true

if echo "$TEST_OUTPUT" | grep -q "FAIL"; then
    echo -e "${RED}=== TCR JAIL: POST-COMMIT TESTS FAILED ===${NC}"
    echo -e "${RED}AUTO-REVERTING COMMIT${NC}"
    FAILED_COMMIT=$(git rev-parse HEAD)
    git reset --hard HEAD~1
    echo -e "${RED}Reverted failed commit: $FAILED_COMMIT${NC}"
    exit 1
fi

echo -e "${GREEN}=== TCR JAIL: COMMIT VERIFIED ===${NC}"
exit 0
```

### Fix 2: Add Missing Deny Rules to Claude Settings

Add these lines to `.claude/settings.local.json` in the deny array:

```json
"Bash(TCR_ENABLED=*)",
"Bash(*TCR_ENABLED=*)",
"Bash(env *)",
"Bash(/usr/bin/env *)",
"Bash(command *git*)",
"Bash(builtin *)",
"Bash(exec *git*)",
"Bash(source *)",
"Bash(. *)",
"Bash(set +e*)",
"Bash(set +u*)",
"Bash(set +o*)",
```

### Fix 3: Fix SAFE_POINT Tag

```bash
# Delete the incorrect tag
git tag -d SAFE_POINT

# Create new tag at current HEAD (after you're happy with the state)
git tag -a SAFE_POINT -m "TCR Jail safe rollback point"

# Delete old backup branch if it exists
git branch -D BACKUP_SAFE_POINT 2>/dev/null || true

# Create new backup branch
git branch BACKUP_SAFE_POINT HEAD
```

---

## Part 2: Simplified Inescapable Design

The current design has too many moving parts. Here's a minimal, robust alternative:

### The Problem with Current Design

1. **4 different config formats** (Claude JSON, OpenCode JSON, Cursor rules, Aider YAML)
2. **Pattern matching is fragile** - easy to miss edge cases
3. **TCR_ENABLED backdoor** - user convenience = AI escape hatch
4. **Maintenance nightmare** - changes need to be made in 6+ places

### Simplified Design: "Trust the Hooks"

The key insight: **Git hooks are the enforcement layer.** Everything else is just "please don't" requests to AI tools.

#### New Architecture

```
┌─────────────────────────────────────────────────┐
│                  Git Hooks                       │
│  (The ONLY enforcement - no backdoors)          │
│                                                  │
│  pre-commit:  Run tests, block if fail          │
│  post-commit: Verify tests, revert if fail      │
└─────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────┐
│            Single Config File                    │
│  .ai-rules.json (all tools read this)           │
│                                                  │
│  - Block --no-verify and -n flags               │
│  - Block hook modification commands             │
│  - That's it. Hooks do the real work.           │
└─────────────────────────────────────────────────┘
```

#### Minimal pre-commit Hook (No Escape)

```bash
#!/usr/bin/env bash
set -euo pipefail

# TCR JAIL - Unconditional enforcement
# There is no TCR_ENABLED variable. There is no bypass.

cd "$(git rev-parse --show-toplevel)"

echo "=== TCR: Running tests ==="

# Run tests - if this fails, reject the commit
if ! go test ./... -short; then
    echo "=== TCR: TESTS FAILED - REVERTING ==="
    git checkout -- .
    git clean -fd
    exit 1
fi

echo "=== TCR: Tests passed ==="
exit 0
```

#### Minimal Claude Settings

The only things that MUST be blocked at the Claude level:

```json
{
  "permissions": {
    "deny": [
      "Bash(*--no-verify*)",
      "Bash(*-n *git commit*)",
      "Bash(git commit*-n)",
      "Bash(rm*.git/hooks*)",
      "Bash(chmod*.git/hooks*)",
      "Bash(mv*.git/hooks*)",
      "Bash(git config*core.hooksPath*)",
      "Edit(.git/hooks/*)",
      "Write(.git/hooks/*)"
    ]
  }
}
```

That's **9 rules** instead of 80+. The hooks do the enforcement.

### Why This Works

1. **Hooks can't be bypassed without `--no-verify`** (which is blocked)
2. **Hooks can't be modified** (Edit/Write blocked, rm/chmod/mv blocked)
3. **Hooks can't be relocated** (`core.hooksPath` blocked)
4. **No backdoor** - no `TCR_ENABLED` variable exists

### What About User Override?

If YOU (the human) need to bypass TCR temporarily:

```bash
# Option 1: Use --no-verify (works because you're human)
git commit --no-verify -m "Emergency fix"

# Option 2: Temporarily rename hook
mv .git/hooks/pre-commit .git/hooks/pre-commit.disabled
git commit -m "Fix"
mv .git/hooks/pre-commit.disabled .git/hooks/pre-commit
```

The AI can't do either because both are blocked in the deny list.

---

## Part 3: Implementation Commands

Run these commands to implement the simplified design:

```bash
# Step 1: Create the fixed pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "=== TCR: Running tests ==="

if ! go test ./... -short; then
    echo "=== TCR: TESTS FAILED - REVERTING ==="
    git checkout -- .
    git clean -fd
    exit 1
fi

echo "=== TCR: Tests passed ==="
exit 0
EOF

chmod +x .git/hooks/pre-commit

# Step 2: Create the fixed post-commit hook
cat > .git/hooks/post-commit << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "=== TCR: Post-commit verification ==="

if ! go test ./... -short; then
    echo "=== TCR: TESTS FAILED - REVERTING COMMIT ==="
    git reset --hard HEAD~1
    exit 1
fi

echo "=== TCR: Commit verified ==="
exit 0
EOF

chmod +x .git/hooks/post-commit

# Step 3: Create minimal Claude settings
mkdir -p .claude
cat > .claude/settings.local.json << 'EOF'
{
  "permissions": {
    "deny": [
      "Bash(*--no-verify*)",
      "Bash(*-n *git commit*)",
      "Bash(git commit*-n)",
      "Bash(rm*.git/hooks*)",
      "Bash(chmod*.git/hooks*)",
      "Bash(mv*.git/hooks*)",
      "Bash(cp*.git/hooks*)",
      "Bash(git config*core.hooksPath*)",
      "Bash(git config*--unset*)",
      "Edit(.git/hooks/*)",
      "Write(.git/hooks/*)",
      "Edit(.claude/*)",
      "Write(.claude/*)"
    ]
  }
}
EOF

# Step 4: Fix SAFE_POINT
git tag -d SAFE_POINT 2>/dev/null || true
git tag -a SAFE_POINT -m "TCR Jail safe point"
git branch -D BACKUP_SAFE_POINT 2>/dev/null || true
git branch BACKUP_SAFE_POINT HEAD

echo "=== TCR Jail simplified and secured ==="
```

---

## Summary

| Current Design | Simplified Design |
|---------------|-------------------|
| 80+ deny rules | 12 deny rules |
| 6 config files | 1 config file |
| TCR_ENABLED backdoor | No backdoor |
| Pattern matching holes | Hooks do enforcement |
| Hard to maintain | Easy to understand |

The key principle: **Make the hooks unconditional, then just prevent the AI from touching or bypassing them.**
