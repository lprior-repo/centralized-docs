#!/usr/bin/env bash
# TCR Wrapper for Claude Code - With Quality Gates
# Three layers: Correctness (tests) + Quality (lints) + Consistency (fmt)
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

cd "$(git rev-parse --show-toplevel)"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║          TCR MODE: Quality-Gated Claude Session            ║${NC}"
echo -e "${CYAN}╠════════════════════════════════════════════════════════════╣${NC}"
echo -e "${CYAN}║  Layer 1: Tests must pass                                  ║${NC}"
echo -e "${CYAN}║  Layer 2: Linters must pass                                ║${NC}"
echo -e "${CYAN}║  Layer 3: Code must be formatted                           ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║  Fail ANY layer = ALL changes reverted                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Save current state for potential revert
BEFORE_SHA=$(git rev-parse HEAD 2>/dev/null || echo "none")

# Run Claude with all passed arguments
echo -e "${GREEN}Starting Claude...${NC}"
echo ""
claude "$@"
CLAUDE_EXIT=$?
echo ""

# Check if there are any changes
if git diff --quiet && git diff --cached --quiet && [ -z "$(git ls-files --others --exclude-standard)" ]; then
    echo -e "${YELLOW}No changes detected. Nothing to verify.${NC}"
    exit $CLAUDE_EXIT
fi

echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}   TCR: Claude exited. Running quality gates...${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""

FAILED=false

# ═══════════════════════════════════════════════════════════════
# LAYER 3: CONSISTENCY (runs first, auto-fixes)
# ═══════════════════════════════════════════════════════════════
echo -e "${YELLOW}[Layer 3/3] Consistency: Formatting code...${NC}"
go fmt ./... > /dev/null 2>&1 || true
goimports -w . > /dev/null 2>&1 || true
echo -e "${GREEN}  ✓ Code formatted${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════
# LAYER 2: QUALITY (linters and static analysis)
# ═══════════════════════════════════════════════════════════════
echo -e "${YELLOW}[Layer 2/3] Quality: Running static analysis...${NC}"

# Go vet (always available)
if ! go vet ./... 2>&1; then
    echo -e "${RED}  ✗ go vet failed${NC}"
    FAILED=true
else
    echo -e "${GREEN}  ✓ go vet passed${NC}"
fi

# golangci-lint (if available - comprehensive linting)
if command -v golangci-lint &> /dev/null; then
    if ! golangci-lint run --timeout=2m 2>&1; then
        echo -e "${RED}  ✗ golangci-lint failed${NC}"
        FAILED=true
    else
        echo -e "${GREEN}  ✓ golangci-lint passed${NC}"
    fi
else
    echo -e "${YELLOW}  ⊘ golangci-lint not installed (recommended: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest)${NC}"
fi

# staticcheck (if available)
if command -v staticcheck &> /dev/null; then
    if ! staticcheck ./... 2>&1; then
        echo -e "${RED}  ✗ staticcheck failed${NC}"
        FAILED=true
    else
        echo -e "${GREEN}  ✓ staticcheck passed${NC}"
    fi
fi

echo ""

# ═══════════════════════════════════════════════════════════════
# LAYER 1: CORRECTNESS (tests - the ultimate arbiter)
# ═══════════════════════════════════════════════════════════════
echo -e "${YELLOW}[Layer 1/3] Correctness: Running tests...${NC}"

if ! go test ./... -short -v 2>&1 | tee /tmp/tcr-test-output.txt; then
    echo -e "${RED}  ✗ Tests failed${NC}"
    FAILED=true
else
    echo -e "${GREEN}  ✓ Tests passed${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════
# VERDICT
# ═══════════════════════════════════════════════════════════════
if [ "$FAILED" = true ]; then
    echo -e "${RED}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║            TCR: QUALITY GATES FAILED                       ║${NC}"
    echo -e "${RED}║            REVERTING ALL CHANGES                           ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    # Nuclear revert
    git checkout -- . 2>/dev/null || true
    git clean -fd 2>/dev/null || true

    echo -e "${RED}All changes have been reverted.${NC}"
    echo -e "${YELLOW}Review the errors above and try again.${NC}"
    exit 1
else
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║            TCR: ALL QUALITY GATES PASSED                   ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    # Auto-commit the passing changes
    git add -A

    # Generate commit message from changes
    CHANGED_FILES=$(git diff --cached --name-only | head -5 | tr '\n' ', ' | sed 's/,$//')

    git commit -m "TCR: Quality-verified changes

Files: ${CHANGED_FILES}

✓ Tests passed
✓ Linters passed
✓ Code formatted

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>"

    echo -e "${GREEN}Changes committed: $(git log -1 --oneline)${NC}"
fi

exit 0
