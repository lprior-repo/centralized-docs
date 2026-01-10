#!/usr/bin/env bash
# SETUP TCR JAIL
# Run this after cloning to establish the TCR jail for AI agents
# Works with: Claude Code, OpenCode, Cursor, Aider, and other AI coding tools

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

cd "$(git rev-parse --show-toplevel)"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║           SETTING UP TCR JAIL FOR AI AGENTS              ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# ============================================
# STEP 1: Create Git Hooks
# ============================================
echo -e "${YELLOW}[1/6] Creating git hooks...${NC}"

mkdir -p .git/hooks

# Pre-commit hook
cat > .git/hooks/pre-commit << 'PREHOOK'
#!/usr/bin/env bash
# PRE-COMMIT HOOK: TCR JAIL - AI CANNOT ESCAPE
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== TCR JAIL: PRE-COMMIT CHECK ===${NC}"

TCR_ENABLED="${TCR_ENABLED:-true}"
if [ "$TCR_ENABLED" != "true" ]; then
    echo -e "${YELLOW}TCR disabled by user (TCR_ENABLED=false)${NC}"
    exit 0
fi

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
PREHOOK

# Post-commit hook
cat > .git/hooks/post-commit << 'POSTHOOK'
#!/usr/bin/env bash
# POST-COMMIT HOOK: TCR JAIL - FINAL VERIFICATION
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== TCR JAIL: POST-COMMIT VERIFICATION ===${NC}"

TCR_ENABLED="${TCR_ENABLED:-true}"
if [ "$TCR_ENABLED" != "true" ]; then
    exit 0
fi

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
POSTHOOK

chmod +x .git/hooks/pre-commit .git/hooks/post-commit
echo -e "${GREEN}Git hooks created${NC}"

# ============================================
# STEP 2: Create Safe Points
# ============================================
echo -e "${YELLOW}[2/6] Creating safe rollback points...${NC}"

CURRENT_COMMIT=$(git rev-parse HEAD)

# Create SAFE_POINT tag if it doesn't exist
if ! git rev-parse SAFE_POINT >/dev/null 2>&1; then
    git tag -a SAFE_POINT -m "TCR Jail safe rollback point"
    echo -e "${GREEN}Created SAFE_POINT tag${NC}"
else
    echo -e "${YELLOW}SAFE_POINT tag already exists${NC}"
fi

# Create BACKUP_SAFE_POINT branch if it doesn't exist
if ! git rev-parse BACKUP_SAFE_POINT >/dev/null 2>&1; then
    git branch BACKUP_SAFE_POINT "$CURRENT_COMMIT"
    echo -e "${GREEN}Created BACKUP_SAFE_POINT branch${NC}"
else
    echo -e "${YELLOW}BACKUP_SAFE_POINT branch already exists${NC}"
fi

# ============================================
# STEP 3: Create Claude Code restrictions
# ============================================
echo -e "${YELLOW}[3/6] Creating Claude Code restrictions...${NC}"

mkdir -p .claude

cat > .claude/settings.local.json << 'CLAUDESETTINGS'
{
  "permissions": {
    "deny": [
      "Bash(git commit*--no-verify*)",
      "Bash(git commit*-n *)",
      "Bash(git commit*-n)",
      "Bash(*--no-verify*)",
      "Bash(bash*git commit*)",
      "Bash(bash*-c*git*commit*)",
      "Bash(sh*git commit*)",
      "Bash(sh*-c*git*commit*)",
      "Bash(eval*git*)",
      "Bash(xargs*git*)",
      "Bash(parallel*git*)",
      "Bash(find*-exec*git*)",
      "Bash(*|*bash*)",
      "Bash(*|*sh*)",
      "Bash(python*git*)",
      "Bash(python3*git*)",
      "Bash(node*git*)",
      "Bash(ruby*git*)",
      "Bash(perl*git*)",
      "Bash(chmod*hooks*)",
      "Bash(rm*.git/hooks*)",
      "Bash(rm*-rf*.git*)",
      "Bash(mv*.git/hooks*)",
      "Bash(cp*.git/hooks*)",
      "Bash(ln*.git/hooks*)",
      "Bash(git config*core.hooksPath*)",
      "Bash(git config*--unset*hook*)",
      "Bash(git config*alias*)",
      "Bash(git tag*-d*SAFE_POINT*)",
      "Bash(git tag*--delete*SAFE_POINT*)",
      "Bash(git branch*-d*BACKUP*)",
      "Bash(git branch*-D*BACKUP*)",
      "Bash(git push*--force*)",
      "Bash(git push*-f*)",
      "Bash(git reset*--hard*HEAD~*)",
      "Bash(git rebase*)",
      "Bash(git cherry-pick*)",
      "Bash(GIT_DIR=*)",
      "Bash(GIT_*=*git*)",
      "Bash(export*GIT_*)",
      "Bash(*>.git/hooks*)",
      "Bash(*>>.git/hooks*)",
      "Bash(echo*>.git*)",
      "Bash(cat*>.git*)",
      "Bash(printf*>.git*)",
      "Bash(tee*.git*)",
      "Bash(sed*-i*.git*)",
      "Bash(base64*-d*|*)",
      "Bash(*|*base64*-d*)",
      "Bash(alias*git*)",
      "Bash(function*git*)",
      "Bash(crontab*)",
      "Bash(at*)",
      "Bash(nohup*git*)",
      "Bash(git stash*)",
      "Bash(git worktree*)",
      "Bash(git am*)",
      "Bash(git apply*)",
      "Edit(.git/*)",
      "Edit(scripts/tcr.sh)",
      "Edit(scripts/revert-to-safe.sh)",
      "Edit(scripts/verify-jail.sh)",
      "Edit(scripts/setup-tcr-jail.sh)",
      "Edit(.claude/*)",
      "Edit(.claudeignore)",
      "Edit(.opencode/*)",
      "Edit(.cursorignore)",
      "Edit(.aider*)",
      "Write(.git/*)",
      "Write(scripts/tcr.sh)",
      "Write(scripts/revert-to-safe.sh)",
      "Write(scripts/verify-jail.sh)",
      "Write(scripts/setup-tcr-jail.sh)",
      "Write(.claude/*)",
      "Write(.claudeignore)",
      "Write(.opencode/*)",
      "Write(.cursorignore)",
      "Write(.aider*)"
    ]
  }
}
CLAUDESETTINGS

echo -e "${GREEN}Claude Code restrictions created${NC}"

# ============================================
# STEP 4: Create OpenCode restrictions
# ============================================
echo -e "${YELLOW}[4/6] Creating OpenCode restrictions...${NC}"

mkdir -p .opencode

cat > .opencode/config.json << 'OPENCODESETTINGS'
{
  "name": "centralized-docs",
  "description": "Pure GoFr CLI for documentation management - TCR JAIL ACTIVE",
  "ai": {
    "blocked_commands": [
      "git commit --no-verify",
      "git commit -n",
      "git reset --hard HEAD~",
      "git rebase",
      "git cherry-pick",
      "git stash",
      "git push --force",
      "git push -f",
      "chmod -x .git/hooks",
      "rm .git/hooks",
      "git config core.hooksPath",
      "git tag -d SAFE_POINT",
      "git branch -D BACKUP"
    ],
    "require_tests_before_commit": true,
    "auto_revert_on_test_failure": true,
    "commit_command": "./scripts/tcr.sh"
  },
  "rules": [
    "NEVER use git commit directly - use ./scripts/tcr.sh only",
    "NEVER bypass git hooks with --no-verify or -n",
    "NEVER modify files in .git/hooks/",
    "NEVER delete SAFE_POINT tag or BACKUP_SAFE_POINT branch",
    "ALWAYS write tests FIRST before implementation",
    "If tests fail, ALL changes are reverted automatically"
  ]
}
OPENCODESETTINGS

# Create .opencode/instructions.md for OpenCode
cat > .opencode/instructions.md << 'OPENCODEINSTRUCTIONS'
# OpenCode AI Instructions - TCR JAIL

## CRITICAL: You are in a TCR (Test-Commit-Revert) Jail

### The ONLY way to commit code:
```bash
./scripts/tcr.sh "Your commit message"
```

### What happens:
1. Tests are run automatically
2. If tests PASS → Commit is created
3. If tests FAIL → ALL your changes are REVERTED (deleted)

### Rules - NON-NEGOTIABLE:
1. NEVER use `git commit` directly
2. NEVER use `--no-verify` or `-n` flags
3. NEVER modify `.git/hooks/` files
4. NEVER delete SAFE_POINT tag
5. ALWAYS write tests FIRST

### If you try to escape:
- Git hooks will catch you
- Your changes will be reverted
- The jail is inescapable

### Safe rollback (for the user):
```bash
./scripts/revert-to-safe.sh
```

### Verify jail integrity:
```bash
./scripts/verify-jail.sh
```
OPENCODEINSTRUCTIONS

echo -e "${GREEN}OpenCode restrictions created${NC}"

# ============================================
# STEP 5: Create Cursor restrictions
# ============================================
echo -e "${YELLOW}[5/6] Creating Cursor restrictions...${NC}"

cat > .cursorignore << 'CURSORIGNORE'
# Cursor AI cannot access these files
.git/
.git/**
.git/hooks/
scripts/tcr.sh
scripts/revert-to-safe.sh
scripts/verify-jail.sh
scripts/setup-tcr-jail.sh
.claude/
.opencode/
.aider*
CURSORIGNORE

cat > .cursorrules << 'CURSORRULES'
# TCR JAIL RULES FOR CURSOR AI

You are operating in a TCR (Test-Commit-Revert) jail. There is no escape.

## MANDATORY COMMIT PROCESS
1. Write tests FIRST
2. Run tests: `go test ./... -v`
3. Only commit via: `./scripts/tcr.sh "message"`

## FORBIDDEN ACTIONS
- NEVER use `git commit` directly
- NEVER use `--no-verify` flag
- NEVER modify `.git/hooks/`
- NEVER delete tags or branches
- NEVER use `git reset --hard HEAD~`

## CONSEQUENCES
If tests fail, ALL your changes are automatically reverted.
There are no exceptions. There are no workarounds.

## SAFE ROLLBACK (user only)
./scripts/revert-to-safe.sh
CURSORRULES

echo -e "${GREEN}Cursor restrictions created${NC}"

# ============================================
# STEP 6: Create Aider restrictions
# ============================================
echo -e "${YELLOW}[6/6] Creating Aider restrictions...${NC}"

cat > .aider.conf.yml << 'AIDERCONF'
# Aider configuration - TCR JAIL ACTIVE

# Git settings - force TCR compliance
auto-commits: false
dirty-commits: false

# Aider must use TCR script
commit-prompt: |
  STOP! You must use ./scripts/tcr.sh to commit.
  Direct git commits are forbidden.
  Run: ./scripts/tcr.sh "your message"

# Files Aider cannot edit
read-only:
  - .git/**
  - scripts/tcr.sh
  - scripts/revert-to-safe.sh
  - scripts/verify-jail.sh
  - scripts/setup-tcr-jail.sh
  - .claude/**
  - .opencode/**
  - .aider*
  - .cursorrules
  - .cursorignore
AIDERCONF

cat > .aiderignore << 'AIDERIGNORE'
# Aider cannot access these files
.git/
.git/**
scripts/tcr.sh
scripts/revert-to-safe.sh
scripts/verify-jail.sh
scripts/setup-tcr-jail.sh
.claude/
.opencode/
AIDERIGNORE

echo -e "${GREEN}Aider restrictions created${NC}"

# ============================================
# VERIFICATION
# ============================================
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              TCR JAIL SETUP COMPLETE                     ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Run verification
./scripts/verify-jail.sh

echo ""
echo -e "${GREEN}AI agents are now jailed in TCR mode:${NC}"
echo -e "  - Claude Code: .claude/settings.local.json"
echo -e "  - OpenCode: .opencode/config.json"
echo -e "  - Cursor: .cursorrules + .cursorignore"
echo -e "  - Aider: .aider.conf.yml + .aiderignore"
echo ""
echo -e "${YELLOW}The AI must use: ./scripts/tcr.sh \"message\"${NC}"
echo -e "${YELLOW}Tests fail = Changes reverted. No exceptions.${NC}"
