#!/usr/bin/env bash
# INESCAPABLE REVERT SCRIPT
# Returns repository to SAFE_POINT tag, no matter what state it's in

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}=== REVERTING TO SAFE_POINT ===${NC}"
echo ""

cd "$(git rev-parse --show-toplevel)"

# Step 1: Abort any in-progress operations
echo -e "${YELLOW}[1/6] Aborting any in-progress git operations...${NC}"
git merge --abort 2>/dev/null || true
git rebase --abort 2>/dev/null || true
git cherry-pick --abort 2>/dev/null || true
git am --abort 2>/dev/null || true
git reset --merge 2>/dev/null || true

# Step 2: Clean untracked files and ignored files
echo -e "${YELLOW}[2/6] Removing ALL untracked and ignored files...${NC}"
git clean -fdx

# Step 3: Reset staging area
echo -e "${YELLOW}[3/6] Resetting staging area...${NC}"
git reset HEAD 2>/dev/null || true

# Step 4: Discard all working directory changes
echo -e "${YELLOW}[4/6] Discarding ALL working directory changes...${NC}"
git checkout -- . 2>/dev/null || true

# Step 5: Hard reset to SAFE_POINT tag
echo -e "${YELLOW}[5/6] Hard reset to SAFE_POINT tag...${NC}"
git reset --hard SAFE_POINT

# Step 6: Verify the reset
echo -e "${YELLOW}[6/6] Verifying reset...${NC}"
CURRENT=$(git rev-parse HEAD)
SAFE=$(git rev-parse SAFE_POINT)

if [ "$CURRENT" = "$SAFE" ]; then
    echo ""
    echo -e "${GREEN}=== SUCCESS ===${NC}"
    echo -e "${GREEN}Repository is now at SAFE_POINT${NC}"
    echo -e "${GREEN}Commit: $(git log -1 --oneline)${NC}"
    echo ""
    git status --short
else
    echo ""
    echo -e "${RED}=== FAILED ===${NC}"
    echo -e "${RED}Something went wrong. Manual intervention required.${NC}"
    echo "Current: $CURRENT"
    echo "Expected: $SAFE"
    exit 1
fi
