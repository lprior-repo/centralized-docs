#!/usr/bin/env bash
set -euo pipefail

REPO="${DOC_TRANSFORMER_REPO:-lprior-repo/centralized-docs}"
FORMULA_URL="${DOC_TRANSFORMER_BREW_FORMULA_URL:-https://raw.githubusercontent.com/$REPO/main/Formula/doc-transformer.rb}"
FORMULA_NAME="doc-transformer"

fail() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

command -v brew >/dev/null 2>&1 || fail "Homebrew is not installed. Install Homebrew first: https://brew.sh"

if brew list --versions "$FORMULA_NAME" >/dev/null 2>&1; then
	brew upgrade "$FORMULA_NAME"
else
	brew install --formula "$FORMULA_URL"
fi
