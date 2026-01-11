#!/usr/bin/env bash
# TCR (Test-Commit-Revert) WRAPPER
# The AI MUST use this for all commits - there is no escape

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

cd "$(git rev-parse --show-toplevel)"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              TCR JAIL - INESCAPABLE MODE                 ║${NC}"
echo -e "${CYAN}║  Tests pass → Commit allowed                             ║${NC}"
echo -e "${CYAN}║  Tests fail → ALL changes reverted                       ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Capture current state for revert
SAFE_COMMIT=$(git rev-parse HEAD)
echo -e "${YELLOW}Safe point: $SAFE_COMMIT${NC}"

# Check for changes
if git diff --quiet && git diff --cached --quiet; then
    echo -e "${YELLOW}No changes to commit.${NC}"
    exit 0
fi

echo ""
echo -e "${YELLOW}[1/5] Formatting code...${NC}"
go fmt ./...

echo -e "${YELLOW}[2/5] Running go vet...${NC}"
if ! go vet ./... 2>&1; then
    echo -e "${RED}=== VET FAILED - REVERTING ===${NC}"
    git checkout -- .
    git clean -fd
    echo -e "${RED}All changes reverted. Fix vet issues before trying again.${NC}"
    exit 1
fi

echo -e "${YELLOW}[3/5] Running tests...${NC}"
if ! go test ./... -v 2>&1; then
    echo -e "${RED}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║              TCR JAIL: TESTS FAILED                      ║${NC}"
    echo -e "${RED}║         ALL CHANGES ARE BEING REVERTED                   ║${NC}"
    echo -e "${RED}╚══════════════════════════════════════════════════════════╝${NC}"

    # NUCLEAR REVERT - no escape
    git checkout -- .
    git clean -fd
    git reset --hard "$SAFE_COMMIT"

    echo ""
    echo -e "${RED}Changes reverted to: $(git log -1 --oneline)${NC}"
    echo -e "${RED}The AI must write passing tests FIRST.${NC}"
    exit 1
fi

echo -e "${GREEN}[4/5] Tests passed! Staging changes...${NC}"
git add -A

echo -e "${GREEN}[5/5] Creating commit...${NC}"
COMMIT_MSG="${1:-TCR: Passing changes}"

git commit -m "$COMMIT_MSG

🔒 TCR verified - tests passed before commit

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>"

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              TCR JAIL: COMMIT SUCCESSFUL                 ║${NC}"
echo -e "${GREEN}║              Tests passed, changes saved                 ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}New commit: $(git log -1 --oneline)${NC}"
