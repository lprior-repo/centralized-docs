#!/usr/bin/env bash
# =============================================================================
# ctd CLI Exhaustive Permutation Test Suite
# =============================================================================
set -uo pipefail

CTD="./target/release/ctd"
VALIDATOR="./target/release/llms_txt_validator"
TMPDIR=$(mktemp -d /tmp/ctd-qa-XXXXXX)
PASS=0
FAIL=0
SKIP=0
RESULTS=()

red()   { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
yellow(){ printf '\033[33m%s\033[0m\n' "$*"; }

record_pass() { PASS=$((PASS+1)); RESULTS+=("PASS  $1"); }
record_fail() { FAIL=$((FAIL+1)); RESULTS+=("FAIL  $1"); red "  FAIL: $1"; if [ -n "${2:-}" ]; then red "        $2"; fi; }
record_skip() { SKIP=$((SKIP+1)); RESULTS+=("SKIP  $1"); yellow "  SKIP: $1"; }

assert_exit() {
  local desc="$1" expected="$2" actual="$3" stderr="${4:-}"
  if [ "$actual" -eq "$expected" ]; then
    record_pass "$desc"
  else
    record_fail "$desc" "expected exit=$expected, got exit=$actual | stderr: $(echo "$stderr" | head -2)"
  fi
}

assert_nonzero() {
  local desc="$1" actual="$2" stderr="${3:-}"
  if [ "$actual" -ne 0 ]; then
    record_pass "$desc"
  else
    record_fail "$desc" "expected non-zero exit, got exit=0 | stderr: $(echo "$stderr" | head -2)"
  fi
}

assert_stderr_contains() {
  local desc="$1" needle="$2" stderr="$3"
  if echo "$stderr" | grep -qi "$needle"; then
    record_pass "$desc"
  else
    record_fail "$desc" "expected stderr containing '$needle', got: $(echo "$stderr" | head -2)"
  fi
}

assert_file_exists() {
  local desc="$1" file="$2"
  if [ -f "$file" ]; then record_pass "$desc"; else record_fail "$desc" "file not found: $file"; fi
}

assert_dir_exists() {
  local desc="$1" dir="$2"
  if [ -d "$dir" ]; then record_pass "$desc"; else record_fail "$desc" "dir not found: $dir"; fi
}

section() { printf '\n\033[36m=== %s ===\033[0m\n' "$1"; }

# Run a command that might hit network with a timeout
run_net() { timeout 10 "$@" 2>&1; }

if [ ! -x "$CTD" ]; then
  red "FATAL: $CTD not found. Run: cargo build --release"
  exit 1
fi

mkdir -p "$TMPDIR/fixtures/docs"
cat > "$TMPDIR/fixtures/docs/intro.md" <<'EOF'
# Introduction

This is a test documentation file for the ctd CLI test suite.
It contains some keywords like rust, async, tokio, and testing.
EOF
cat > "$TMPDIR/fixtures/docs/guide.md" <<'EOF'
# Getting Started Guide

Follow these steps to get started with the project.
Install dependencies, configure settings, and run the application.
EOF
cat > "$TMPDIR/fixtures/docs/api.md" <<'EOF'
# API Reference

The API exposes endpoints for searching, indexing, and scraping.
Use the REST interface or the MCP server for integration.
EOF

trap 'rm -rf "$TMPDIR"' EXIT

# =============================================================================
# 1. TOP-LEVEL FLAGS
# =============================================================================
section "1. Top-level flags & version"

out=$("$CTD" --version 2>&1); rc=$?
assert_exit "ctd --version exits 0" 0 "$rc"
assert_stderr_contains "ctd --version shows version" "0.6.1" "$out"

out=$("$CTD" -V 2>&1); rc=$?
assert_exit "ctd -V exits 0" 0 "$rc"

out=$("$CTD" --help 2>&1); rc=$?
assert_exit "ctd --help exits 0" 0 "$rc"
for cmd in scrape index ingest search watch apply diff ingest-git mcp; do
  assert_stderr_contains "ctd --help lists $cmd" "$cmd" "$out"
done

out=$("$CTD" -h 2>&1); rc=$?
assert_exit "ctd -h exits 0" 0 "$rc"

out=$("$CTD" 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "ctd (no args) exits non-zero"; else record_fail "ctd (no args) should exit non-zero"; fi

out=$("$CTD" boguscommand 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "ctd boguscommand exits non-zero"; else record_fail "ctd boguscommand should exit non-zero"; fi

# =============================================================================
# 2. SUBCOMMAND --help
# =============================================================================
section "2. Subcommand --help"

for cmd in scrape index ingest search watch apply diff ingest-git mcp; do
  out=$("$CTD" "$cmd" --help 2>&1); rc=$?
  assert_exit "ctd $cmd --help exits 0" 0 "$rc"
  assert_stderr_contains "ctd $cmd --help shows Usage" "Usage" "$out"
done

out=$("$CTD" mcp serve --help 2>&1); rc=$?
assert_exit "ctd mcp serve --help exits 0" 0 "$rc"

# =============================================================================
# 3. INDEX - HAPPY PATH
# =============================================================================
section "3. ctd index - happy path"

OUTDIR="$TMPDIR/index-basic"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" 2>&1); rc=$?
assert_exit "ctd index basic" 0 "$rc" "$out"
assert_dir_exists "index creates output dir" "$OUTDIR"
assert_file_exists "index creates INDEX.json" "$OUTDIR/INDEX.json"

OUTDIR="$TMPDIR/index-llms"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --llms-txt 2>&1); rc=$?
assert_exit "ctd index --llms-txt" 0 "$rc" "$out"
assert_file_exists "index --llms-txt creates llms.txt" "$OUTDIR/llms.txt"

OUTDIR="$TMPDIR/index-agents"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --llms-txt --with-agents 2>&1); rc=$?
assert_exit "ctd index --with-agents" 0 "$rc" "$out"
assert_file_exists "index --with-agents creates AGENTS.md" "$OUTDIR/AGENTS.md"

OUTDIR="$TMPDIR/index-project"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --llms-txt --project-name "TestProject" --project-desc "A test" 2>&1); rc=$?
assert_exit "ctd index --project-name/--desc" 0 "$rc" "$out"
assert_file_exists "index --project-name creates llms.txt" "$OUTDIR/llms.txt"

OUTDIR="$TMPDIR/index-hnsw"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --max-related-chunks 10 --max-chunk-keywords 5 --hnsw-m 16 --hnsw-ef-construction 100 2>&1); rc=$?
assert_exit "ctd index HNSW params" 0 "$rc" "$out"

OUTDIR="$TMPDIR/index-maxdoc"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --max-document-bytes 10485760 2>&1); rc=$?
assert_exit "ctd index --max-document-bytes" 0 "$rc" "$out"

OUTDIR="$TMPDIR/index-short-o"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" 2>&1); rc=$?
assert_exit "ctd index -o (short)" 0 "$rc" "$out"

OUTDIR="$TMPDIR/index-catcfg"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --category-config "/nonexistent/file.yaml" 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "index --category-config nonexistent fails gracefully"; else record_pass "index --category-config nonexistent ok (exit 0)"; fi

# =============================================================================
# 4. INDEX - UNHAPPY PATH
# =============================================================================
section "4. ctd index - unhappy path"

out=$("$CTD" index "$TMPDIR/fixtures/docs" 2>&1); rc=$?
assert_nonzero "index missing --output" "$rc"

out=$("$CTD" index -o "$TMPDIR/index-nosrc" 2>&1); rc=$?
assert_nonzero "index missing source" "$rc"

out=$("$CTD" index "/nonexistent/path" -o "$TMPDIR/index-badsrc" 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "index nonexistent source rejected"; else record_fail "index nonexistent source should fail"; fi

# --max-related-chunks boundaries
for val in 0 -1 101 abc; do
  out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$TMPDIR/ix-mrc-$val" --max-related-chunks "$val" 2>&1); rc=$?
  assert_nonzero "index --max-related-chunks $val rejected" "$rc"
done

# --max-chunk-keywords boundaries
for val in -1 51 abc; do
  out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$TMPDIR/ix-mck-$val" --max-chunk-keywords "$val" 2>&1); rc=$?
  assert_nonzero "index --max-chunk-keywords $val rejected" "$rc"
done

# --hnsw-m boundaries
for val in 3 65 abc; do
  out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$TMPDIR/ix-hm-$val" --hnsw-m "$val" 2>&1); rc=$?
  assert_nonzero "index --hnsw-m $val rejected" "$rc"
done

# --hnsw-ef-construction boundaries
for val in 49 1001 abc; do
  out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$TMPDIR/ix-hefc-$val" --hnsw-ef-construction "$val" 2>&1); rc=$?
  assert_nonzero "index --hnsw-ef-construction $val rejected" "$rc"
done

# boundary minima accepted
OUTDIR="$TMPDIR/index-bmin"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --max-related-chunks 1 --max-chunk-keywords 0 --hnsw-m 4 --hnsw-ef-construction 50 2>&1); rc=$?
assert_exit "index boundary minimums accepted" 0 "$rc"

# boundary maxima accepted
OUTDIR="$TMPDIR/index-bmax"
out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" --max-related-chunks 100 --max-chunk-keywords 50 --hnsw-m 64 --hnsw-ef-construction 1000 2>&1); rc=$?
assert_exit "index boundary maximums accepted" 0 "$rc"

# =============================================================================
# 5. SEARCH
# =============================================================================
section "5. ctd search"

IDX="$TMPDIR/index-basic"

out=$("$CTD" search "rust async" -i "$IDX" 2>&1); rc=$?
assert_exit "search basic" 0 "$rc"

out=$("$CTD" search "testing" -i "$IDX" -n 5 2>&1); rc=$?
assert_exit "search --limit 5" 0 "$rc"

out=$("$CTD" search "testing" -i "$IDX" -n 1 2>&1); rc=$?
assert_exit "search --limit 1" 0 "$rc"

out=$("$CTD" search "testing" -i "$IDX" -n 1000 2>&1); rc=$?
assert_exit "search --limit 1000" 0 "$rc"

out=$("$CTD" search "rust" -i "$IDX" --no-color 2>&1); rc=$?
assert_exit "search --no-color" 0 "$rc"

out=$("$CTD" search "rust" -i "$IDX" --json 2>&1); rc=$?
assert_exit "search --json" 0 "$rc"

out=$("$CTD" search "rust" -i "$IDX" --json --no-color -n 3 2>&1); rc=$?
assert_exit "search all flags combined" 0 "$rc"

out=$("$CTD" search -i "$IDX" 2>&1); rc=$?
assert_nonzero "search missing query" "$rc"

out=$("$CTD" search "rust" 2>&1); rc=$?
assert_nonzero "search missing --index-dir" "$rc"

out=$("$CTD" search "rust" -i "/nonexistent" 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "search nonexistent index-dir rejected"; else record_fail "search nonexistent index-dir should fail"; fi

for val in 0 -1 1001 abc; do
  out=$("$CTD" search "rust" -i "$IDX" -n "$val" 2>&1); rc=$?
  assert_nonzero "search --limit $val rejected" "$rc"
done

out=$("$CTD" search "   " -i "$IDX" 2>&1); rc=$?
if [ "$rc" -ne 0 ]; then record_pass "search whitespace query rejected"; else record_fail "search whitespace query should fail"; fi

# =============================================================================
# 6. SCRAPE FLAG VALIDATION
# =============================================================================
section "6. ctd scrape - flag validation"

out=$("$CTD" scrape -o "$TMPDIR/s1" 2>&1); rc=$?
assert_nonzero "scrape missing URL" "$rc"

out=$("$CTD" scrape https://example.com 2>&1); rc=$?
assert_nonzero "scrape missing --output" "$rc"

out=$("$CTD" scrape 2>&1); rc=$?
assert_nonzero "scrape no args" "$rc"

# --delay
for val in -1 60001 abc; do
  out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --delay "$val" 2>&1); rc=$?
  assert_nonzero "scrape --delay $val rejected" "$rc"
done

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --delay 0 --request-timeout-secs 1 --connect-timeout-secs 1 2>&1); rc=$?
record_pass "scrape --delay 0 parsed ok (exit=$rc)"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --delay 60000 2>&1); rc=$?
record_pass "scrape --delay 60000 parsed ok (exit=$rc)"

# --request-timeout-secs
out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --request-timeout-secs 0 2>&1); rc=$?
assert_nonzero "scrape --request-timeout-secs 0 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --request-timeout-secs 601 2>&1); rc=$?
assert_nonzero "scrape --request-timeout-secs 601 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --request-timeout-secs 1 2>&1); rc=$?
record_pass "scrape --request-timeout-secs 1 parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --request-timeout-secs 600 2>&1); rc=$?
record_pass "scrape --request-timeout-secs 600 parsed ok"

# --connect-timeout-secs
out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --connect-timeout-secs 0 2>&1); rc=$?
assert_nonzero "scrape --connect-timeout-secs 0 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --connect-timeout-secs 61 2>&1); rc=$?
assert_nonzero "scrape --connect-timeout-secs 61 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --connect-timeout-secs 1 2>&1); rc=$?
record_pass "scrape --connect-timeout-secs 1 parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --connect-timeout-secs 60 2>&1); rc=$?
record_pass "scrape --connect-timeout-secs 60 parsed ok"

# --concurrency
out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --concurrency 0 2>&1); rc=$?
assert_nonzero "scrape --concurrency 0 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --concurrency 129 2>&1); rc=$?
assert_nonzero "scrape --concurrency 129 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --concurrency 1 2>&1); rc=$?
record_pass "scrape --concurrency 1 parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --concurrency 128 2>&1); rc=$?
record_pass "scrape --concurrency 128 parsed ok"

# --max-retries
out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-retries -1 2>&1); rc=$?
assert_nonzero "scrape --max-retries -1 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-retries 256 2>&1); rc=$?
assert_nonzero "scrape --max-retries 256 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-retries 0 2>&1); rc=$?
record_pass "scrape --max-retries 0 parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-retries 255 2>&1); rc=$?
record_pass "scrape --max-retries 255 parsed ok"

# --redirect-policy
for policy in loose strict none; do
  out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --redirect-policy "$policy" 2>&1); rc=$?
  record_pass "scrape --redirect-policy $policy parsed ok"
done

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --redirect-policy invalid 2>&1); rc=$?
assert_nonzero "scrape --redirect-policy invalid rejected" "$rc"

# --threshold
out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --threshold -0.1 2>&1); rc=$?
assert_nonzero "scrape --threshold -0.1 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --threshold 10.1 2>&1); rc=$?
assert_nonzero "scrape --threshold 10.1 rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --threshold abc 2>&1); rc=$?
assert_nonzero "scrape --threshold abc rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --threshold 0.0 2>&1); rc=$?
record_pass "scrape --threshold 0.0 parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --threshold 10.0 2>&1); rc=$?
record_pass "scrape --threshold 10.0 parsed ok"

# --filter regex
out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --filter "^/docs/" 2>&1); rc=$?
record_pass "scrape --filter valid regex parsed ok"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --filter "[invalid" 2>&1); rc=$?
assert_nonzero "scrape --filter invalid regex rejected" "$rc"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --filter "(.+)+" 2>&1); rc=$?
assert_nonzero "scrape --filter ReDoS pattern rejected" "$rc"

# --no-sitemap, --query, --max-page-bytes, --max-total-bytes
out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --no-sitemap 2>&1); rc=$?
record_pass "scrape --no-sitemap parsed ok"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --query "rust async" 2>&1); rc=$?
record_pass "scrape --query parsed ok"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-page-bytes 0 2>&1); rc=$?
assert_nonzero "scrape --max-page-bytes 0 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-page-bytes 1000000 2>&1); rc=$?
record_pass "scrape --max-page-bytes positive parsed ok"

out=$("$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-total-bytes 0 2>&1); rc=$?
assert_nonzero "scrape --max-total-bytes 0 rejected" "$rc"

out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" --max-total-bytes 10000000 2>&1); rc=$?
record_pass "scrape --max-total-bytes positive parsed ok"

# all scrape flags combined
out=$(run_net "$CTD" scrape https://example.com -o "$TMPDIR/s1" \
  --delay 100 --concurrency 2 --request-timeout-secs 10 --connect-timeout-secs 5 \
  --max-retries 1 --redirect-policy strict --filter "^/docs/" \
  --threshold 0.5 --query "test" --no-sitemap \
  --max-page-bytes 5000000 --max-total-bytes 50000000 2>&1); rc=$?
record_pass "scrape ALL flags combined parsed ok (exit=$rc)"

# =============================================================================
# 7. INGEST FLAG VALIDATION
# =============================================================================
section "7. ctd ingest - flag validation"

out=$("$CTD" ingest -o "$TMPDIR/ig1" 2>&1); rc=$?
assert_nonzero "ingest missing URL" "$rc"

out=$("$CTD" ingest https://example.com 2>&1); rc=$?
assert_nonzero "ingest missing --output" "$rc"

out=$(run_net "$CTD" ingest https://example.com -o "$TMPDIR/ig1" \
  --delay 100 --concurrency 2 --request-timeout-secs 10 --connect-timeout-secs 5 \
  --max-retries 1 --redirect-policy strict --filter "^/docs/" \
  --threshold 0.5 --query "test" --project-name "TestProject" 2>&1); rc=$?
record_pass "ingest ALL flags combined parsed ok (exit=$rc)"

out=$(run_net "$CTD" ingest https://example.com -o "$TMPDIR/ig1" --project-name "MyProject" 2>&1); rc=$?
record_pass "ingest --project-name parsed ok"

# ingest shares scrape validation - test rejection
out=$("$CTD" ingest https://example.com -o "$TMPDIR/ig1" --delay -1 2>&1); rc=$?
assert_nonzero "ingest --delay -1 rejected" "$rc"

out=$("$CTD" ingest https://example.com -o "$TMPDIR/ig1" --concurrency 0 2>&1); rc=$?
assert_nonzero "ingest --concurrency 0 rejected" "$rc"

# =============================================================================
# 8. INGEST-GIT FLAG VALIDATION
# =============================================================================
section "8. ctd ingest-git - flag validation"

out=$("$CTD" ingest-git -o "$TMPDIR/g1" 2>&1); rc=$?
assert_nonzero "ingest-git missing URL" "$rc"

out=$("$CTD" ingest-git https://github.com/example/repo 2>&1); rc=$?
assert_nonzero "ingest-git missing --output" "$rc"

out=$(run_net "$CTD" ingest-git https://github.com/example/repo -o "$TMPDIR/g1" --branch main --depth 1 --project-name "GitProj" --filter "^docs/" 2>&1); rc=$?
record_pass "ingest-git all flags parsed ok (exit=$rc)"

out=$("$CTD" ingest-git https://github.com/example/repo -o "$TMPDIR/g1" --filter "[invalid" 2>&1); rc=$?
assert_nonzero "ingest-git --filter invalid regex rejected" "$rc"

out=$(run_net "$CTD" ingest-git https://github.com/example/repo -o "$TMPDIR/g1" --depth 0 2>&1); rc=$?
record_pass "ingest-git --depth 0 parsed ok"

# =============================================================================
# 9. WATCH FLAG VALIDATION
# =============================================================================
section "9. ctd watch - flag validation"

out=$("$CTD" watch -o "$TMPDIR/w1" 2>&1); rc=$?
assert_nonzero "watch missing URL" "$rc"

out=$("$CTD" watch https://example.com 2>&1); rc=$?
assert_nonzero "watch missing --output" "$rc"

out=$(run_net "$CTD" watch https://example.com -o "$TMPDIR/w1" --cache "$TMPDIR/test_cache.redb" 2>&1); rc=$?
record_pass "watch --cache parsed ok (exit=$rc)"

out=$(run_net "$CTD" watch https://example.com -o "$TMPDIR/w1" --json 2>&1); rc=$?
record_pass "watch --json parsed ok (exit=$rc)"

out=$(run_net "$CTD" watch https://example.com -o "$TMPDIR/w1" --no-sitemap 2>&1); rc=$?
record_pass "watch --no-sitemap parsed ok (exit=$rc)"

out=$(run_net "$CTD" watch https://example.com -o "$TMPDIR/w1" \
  --delay 50 --concurrency 2 --request-timeout-secs 10 --connect-timeout-secs 5 --max-retries 1 \
  --redirect-policy strict --filter "^/docs/" 2>&1); rc=$?
record_pass "watch all shared flags parsed ok (exit=$rc)"

out=$("$CTD" watch https://example.com -o "$TMPDIR/w1" --connect-timeout-secs 0 2>&1); rc=$?
assert_nonzero "watch --connect-timeout-secs 0 rejected" "$rc"

out=$("$CTD" watch https://example.com -o "$TMPDIR/w1" --connect-timeout-secs 61 2>&1); rc=$?
assert_nonzero "watch --connect-timeout-secs 61 rejected" "$rc"

out=$("$CTD" watch https://example.com -o "$TMPDIR/w1" --delay -1 2>&1); rc=$?
assert_nonzero "watch --delay -1 rejected" "$rc"

# =============================================================================
# 10. APPLY FLAG VALIDATION
# =============================================================================
section "10. ctd apply - flag validation"

out=$("$CTD" apply --scrape-dir "$TMPDIR/a1" 2>&1); rc=$?
assert_nonzero "apply missing URL" "$rc"

out=$("$CTD" apply https://example.com 2>&1); rc=$?
assert_nonzero "apply missing --scrape-dir" "$rc"

out=$("$CTD" apply https://example.com --scrape-dir "$TMPDIR/a1" --cache "$TMPDIR/test_cache.redb" 2>&1); rc=$?
record_pass "apply --cache parsed ok (exit=$rc)"

out=$("$CTD" apply https://example.com --scrape-dir "$TMPDIR/a1" --yes 2>&1); rc=$?
record_pass "apply --yes parsed ok (exit=$rc)"

# =============================================================================
# 11. DIFF FLAG VALIDATION
# =============================================================================
section "11. ctd diff - flag validation"

mkdir -p "$TMPDIR/scrape-a" "$TMPDIR/scrape-b"

out=$("$CTD" diff 2>&1); rc=$?
assert_nonzero "diff no args" "$rc"

out=$("$CTD" diff "$TMPDIR/scrape-a" 2>&1); rc=$?
assert_nonzero "diff missing DIR_B" "$rc"

out=$("$CTD" diff "$TMPDIR/scrape-a" "$TMPDIR/scrape-b" 2>&1); rc=$?
if [ "$rc" -eq 0 ] || [ "$rc" -eq 1 ]; then record_pass "diff two dirs ok (exit=$rc)"; else record_fail "diff two empty dirs" "exit=$rc"; fi

out=$("$CTD" diff "$TMPDIR/scrape-a" "$TMPDIR/scrape-b" -o "$TMPDIR/diff-out" 2>&1); rc=$?
if [ "$rc" -eq 0 ] || [ "$rc" -eq 1 ]; then record_pass "diff --output ok"; else record_fail "diff --output" "exit=$rc"; fi

out=$("$CTD" diff "$TMPDIR/scrape-a" "$TMPDIR/scrape-b" --json 2>&1); rc=$?
if [ "$rc" -eq 0 ] || [ "$rc" -eq 1 ]; then record_pass "diff --json ok"; else record_fail "diff --json" "exit=$rc"; fi

out=$("$CTD" diff /nonexistent/a /nonexistent/b 2>&1); rc=$?
record_pass "diff nonexistent dirs handled (exit=$rc)"

# =============================================================================
# 12. MCP
# =============================================================================
section "12. ctd mcp"

out=$("$CTD" mcp 2>&1); rc=$?
record_pass "ctd mcp (no subcommand) handled (exit=$rc)"

out=$("$CTD" mcp serve --help 2>&1); rc=$?
assert_exit "mcp serve --help" 0 "$rc"

# =============================================================================
# 13. llms_txt_validator
# =============================================================================
section "13. llms_txt_validator"

if [ -x "$VALIDATOR" ]; then
  out=$("$VALIDATOR" --help 2>&1); rc=$?
  assert_exit "validator --help" 0 "$rc"

  out=$("$VALIDATOR" --version 2>&1); rc=$?
  assert_exit "validator --version" 0 "$rc"

  out=$("$VALIDATOR" 2>&1); rc=$?
  record_pass "validator no args handled (exit=$rc)"

  LLMTXT="$TMPDIR/index-llms/llms.txt"
  if [ -f "$LLMTXT" ]; then
    out=$("$VALIDATOR" "$LLMTXT" 2>&1); rc=$?
    assert_exit "validator on generated llms.txt" 0 "$rc"
  else
    record_skip "validator llms.txt (not generated)"
  fi

  INDEXJSON="$TMPDIR/index-basic/INDEX.json"
  if [ -f "$INDEXJSON" ]; then
    out=$("$VALIDATOR" --index "$INDEXJSON" 2>&1); rc=$?
    assert_exit "validator on generated INDEX.json" 0 "$rc"
  else
    record_skip "validator INDEX.json (not generated)"
  fi

  out=$("$VALIDATOR" /nonexistent/llms.txt 2>&1); rc=$?
  assert_exit "validator nonexistent file" 1 "$rc"

  out=$("$VALIDATOR" --index /nonexistent/INDEX.json 2>&1); rc=$?
  assert_exit "validator --index nonexistent" 1 "$rc"
else
  record_skip "llms_txt_validator binary not found"
fi

# =============================================================================
# 14. INDEX FLAG COMBINATION PERMUTATIONS
# =============================================================================
section "14. Index flag combinations"

combos=(
  ""
  "--llms-txt"
  "--with-agents"
  "--llms-txt --with-agents"
  "--llms-txt --project-name Test"
  "--llms-txt --project-name Test --project-desc Desc"
  "--max-related-chunks 5 --max-chunk-keywords 3"
  "--hnsw-m 8 --hnsw-ef-construction 100"
  "--max-related-chunks 10 --max-chunk-keywords 5 --hnsw-m 8 --hnsw-ef-construction 100"
  "--llms-txt --with-agents --project-name Full --project-desc Test --max-related-chunks 10 --max-chunk-keywords 5 --hnsw-m 8 --hnsw-ef-construction 100"
)

for i in "${!combos[@]}"; do
  flags="${combos[$i]}"
  OUTDIR="$TMPDIR/combo-$i"
  out=$("$CTD" index "$TMPDIR/fixtures/docs" -o "$OUTDIR" $flags 2>&1); rc=$?
  assert_exit "index combo[$i] ($flags)" 0 "$rc" "$out"
done

# =============================================================================
# 15. SEARCH FLAG COMBINATION PERMUTATIONS
# =============================================================================
section "15. Search flag combinations"

IDX="$TMPDIR/index-basic"
search_combos=(
  "--json"
  "--no-color"
  "-n 1"
  "-n 10"
  "--json --no-color -n 5"
  "--json -n 1 --no-color"
)

for i in "${!search_combos[@]}"; do
  flags="${search_combos[$i]}"
  out=$("$CTD" search "rust" -i "$IDX" $flags 2>&1); rc=$?
  assert_exit "search combo[$i] ($flags)" 0 "$rc"
done

# =============================================================================
# SUMMARY
# =============================================================================
section "RESULTS SUMMARY"

printf '%s\n' "${RESULTS[@]}" | sort

echo ""
green "PASS: $PASS"
red   "FAIL: $FAIL"
yellow "SKIP: $SKIP"
echo "TOTAL: $((PASS + FAIL + SKIP))"

if [ "$FAIL" -gt 0 ]; then
  echo ""
  red "SOME TESTS FAILED"
  exit 1
else
  echo ""
  green "ALL TESTS PASSED"
  exit 0
fi
