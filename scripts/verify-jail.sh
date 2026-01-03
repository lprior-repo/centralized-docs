#!/usr/bin/env bash
# VERIFY TCR JAIL INTEGRITY
# Run this to check if AI has tried to escape

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

cd "$(git rev-parse --show-toplevel)"

echo -e "${YELLOW}=== TCR JAIL INTEGRITY CHECK ===${NC}"
echo ""

FAILED=0

# Check 1: Pre-commit hook exists and is executable
echo -n "[1/7] Pre-commit hook exists and executable... "
if [ -x .git/hooks/pre-commit ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 2: Post-commit hook exists and is executable
echo -n "[2/7] Post-commit hook exists and executable... "
if [ -x .git/hooks/post-commit ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 3: SAFE_POINT tag exists
echo -n "[3/7] SAFE_POINT tag exists... "
if git rev-parse SAFE_POINT >/dev/null 2>&1; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 4: BACKUP_SAFE_POINT branch exists
echo -n "[4/7] BACKUP_SAFE_POINT branch exists... "
if git rev-parse BACKUP_SAFE_POINT >/dev/null 2>&1; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 5: TCR script exists and executable
echo -n "[5/7] TCR script exists and executable... "
if [ -x scripts/tcr.sh ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 6: Revert script exists and executable
echo -n "[6/7] Revert script exists and executable... "
if [ -x scripts/revert-to-safe.sh ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    FAILED=1
fi

# Check 7: Claude settings exist (local only, not in git)
echo -n "[7/7] Claude AI restrictions exist... "
if [ -f .claude/settings.local.json ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${YELLOW}MISSING (recreate from docs)${NC}"
    FAILED=1
fi

echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              TCR JAIL INTEGRITY: VERIFIED                ║${NC}"
    echo -e "${GREEN}║              AI cannot escape. All secure.               ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║              TCR JAIL INTEGRITY: COMPROMISED             ║${NC}"
    echo -e "${RED}║              Run revert-to-safe.sh immediately           ║${NC}"
    echo -e "${RED}╚══════════════════════════════════════════════════════════╝${NC}"
    exit 1
fi
