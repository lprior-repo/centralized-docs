#!/usr/bin/env bash
# ===========================================================================
# CTD PROFILING HARNESS — Self-Reinforcing Performance Loop
# ===========================================================================
#
# This script runs the full profiling suite for ctd and produces a
# consolidated report. Run it after every change to catch regressions.
#
# Usage:
#   ./scripts/profile.sh [--quick] [--full] [--baseline]
#
# Modes:
#   --quick    Only hyperfine + bloat (fast, ~2 min)
#   --full     Everything: hyperfine + bloat + criterion benches (slow, ~15 min)
#   --baseline Save results as baseline for future comparison
#
# Prerequisites:
#   - Build with: cargo build --profile profiling --bin ctd
#   - Install: cargo install hyperfine cargo-bloat
#   - Test corpus at /tmp/ctd-test-50 (auto-generated if missing)
#
# ===========================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RESULTS_DIR="$PROJECT_ROOT/profile-results"
PROFILE_BIN="$PROJECT_ROOT/target/profiling/ctd"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

MODE="${1:---quick}"
BASELINE=false
if [[ "$MODE" == "--baseline" ]]; then
    BASELINE=true
    MODE="--full"
fi

mkdir -p "$RESULTS_DIR"

echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN} CTD Performance Profiling Harness${NC}"
echo -e "${CYAN} Mode: $MODE | Timestamp: $TIMESTAMP${NC}"
echo -e "${CYAN}================================================================${NC}"

# ===========================================================================
# 0. Ensure test corpus exists
# ===========================================================================
if [[ ! -d /tmp/ctd-test-50 ]] || [[ $(ls /tmp/ctd-test-50/*.md 2>/dev/null | wc -l) -lt 50 ]]; then
    echo -e "${YELLOW}[SETUP] Generating 50-file test corpus...${NC}"
    mkdir -p /tmp/ctd-test-50
    python3 -c "
import os, random
topics = ['api','architecture','tutorial','reference','guide','internals',
          'deployment','security','performance','testing','configuration',
          'monitoring','troubleshooting','migration','plugins','sdk',
          'authentication','caching','database','networking']
for i in range(50):
    topic = topics[i % len(topics)]
    filename = f'/tmp/ctd-test-50/{topic}_{i:03d}.md'
    sections = []
    for j in range(random.randint(5, 20)):
        hl = random.choice([2, 3, 4])
        heading = '#' * hl + f' Section {j} - {topic.title()} Details'
        words = random.randint(50, 300)
        content = ' '.join([f'{topic}' if random.random() < 0.05 else f'word{k}' for k in range(words)])
        sections.append(f'{heading}\n\n{content}\n')
    doc = f'# {topic.title()} Document {i}\n\nThis is document {i} about {topic}.\n\n' + '\n'.join(sections)
    with open(filename, 'w') as f:
        f.write(doc)
print('Generated 50 test files')
"
fi

# ===========================================================================
# 1. Ensure profiling binary exists
# ===========================================================================
if [[ ! -f "$PROFILE_BIN" ]]; then
    echo -e "${YELLOW}[BUILD] Building profiling binary...${NC}"
    cargo build --profile profiling --bin ctd
fi

# ===========================================================================
# 2. Binary Size Analysis (cargo-bloat)
# ===========================================================================
echo -e "${CYAN}[BLOAT] Binary size & crate breakdown...${NC}"
/cache/cargo-shared/bin/cargo-bloat bloat --profile profiling --bin ctd --crates -n 30 \
    > "$RESULTS_DIR/bloat_report_$TIMESTAMP.txt" 2>&1

echo -e "${GREEN}[BLOAT] Done → profile-results/bloat_report_$TIMESTAMP.txt${NC}"

# ===========================================================================
# 3. Wall-Clock Timing (hyperfine)
# ===========================================================================
echo -e "${CYAN}[HYPERFINE] Wall-clock timing (50 docs)...${NC}"
rm -rf /tmp/ctd-profile-index-output 2>/dev/null

/cache/cargo-shared/bin/hyperfine \
    --warmup 1 \
    --runs 3 \
    --export-markdown "$RESULTS_DIR/hyperfine_50docs_$TIMESTAMP.md" \
    --export-json "$RESULTS_DIR/hyperfine_50docs_$TIMESTAMP.json" \
    "$PROFILE_BIN index /tmp/ctd-test-50 --output /tmp/ctd-profile-index-output" \
    2>&1 | tee "$RESULTS_DIR/hyperfine_50docs_$TIMESTAMP.txt"

echo -e "${GREEN}[HYPERFINE] Done → profile-results/hyperfine_50docs_$TIMESTAMP.*${NC}"

# ===========================================================================
# 4. Step-by-step pipeline timing (parse stdout)
# ===========================================================================
echo -e "${CYAN}[PIPELINE] Step-by-step timing...${NC}"
rm -rf /tmp/ctd-profile-index-output 2>/dev/null

# Run once with full output and time each step
PIPELINE_OUT="$RESULTS_DIR/pipeline_steps_$TIMESTAMP.txt"
time "$PROFILE_BIN" index /tmp/ctd-test-50 --output /tmp/ctd-profile-index-output \
    > "$PIPELINE_OUT" 2>&1

echo -e "${GREEN}[PIPELINE] Raw output → profile-results/pipeline_steps_$TIMESTAMP.txt${NC}"

# ===========================================================================
# 5. Criterion Microbenchmarks (only in --full mode)
# ===========================================================================
if [[ "$MODE" == "--full" ]]; then
    echo -e "${CYAN}[CRITERION] Running microbenchmarks...${NC}"
    
    cargo bench --package centralized-docs --bench filter_benchmarks -- --noplot 2>&1 \
        | tee "$RESULTS_DIR/bench_filter_$TIMESTAMP.txt" || true
    
    cargo bench --package centralized-docs --bench watch_benchmarks -- --noplot 2>&1 \
        | tee "$RESULTS_DIR/bench_watch_$TIMESTAMP.txt" || true
    
    echo -e "${GREEN}[CRITERION] Done${NC}"
fi

# ===========================================================================
# 6. Generate Comparison Report
# ===========================================================================
REPORT="$RESULTS_DIR/report_$TIMESTAMP.md"

echo -e "${CYAN}[REPORT] Generating consolidated report...${NC}"

# Parse the latest hyperfine JSON for timing
LATEST_HYPERFINE=$(ls -t "$RESULTS_DIR"/hyperfine_50docs_*.json 2>/dev/null | head -1)
if [[ -n "$LATEST_HYPERFINE" ]]; then
    MEAN_TIME=$(python3 -c "
import json, sys
with open('$LATEST_HYPERFINE') as f:
    data = json.load(f)
print(f\"{data['results'][0]['mean']:.3f}\")
" 2>/dev/null || echo "N/A")
    MEDIAN_TIME=$(python3 -c "
import json, sys
with open('$LATEST_HYPERFINE') as f:
    data = json.load(f)
print(f\"{data['results'][0]['median']:.3f}\")
" 2>/dev/null || echo "N/A")
    USER_TIME=$(python3 -c "
import json, sys
with open('$LATEST_HYPERFINE') as f:
    data = json.load(f)
print(f\"{data['results'][0]['user']:.3f}\")
" 2>/dev/null || echo "N/A")
    SYSTEM_TIME=$(python3 -c "
import json, sys
with open('$LATEST_HYPERFINE') as f:
    data = json.load(f)
print(f\"{data['results'][0]['system']:.3f}\")
" 2>/dev/null || echo "N/A")
else
    MEAN_TIME="N/A"
    MEDIAN_TIME="N/A"
    USER_TIME="N/A"
    SYSTEM_TIME="N/A"
fi

# Get binary size
BINARY_SIZE=$(ls -lh "$PROFILE_BIN" 2>/dev/null | awk '{print $5}' || echo "N/A")

# Get bloat top 10
BLOAT_TOP=$(head -20 "$RESULTS_DIR/bloat_report_$TIMESTAMP.txt" 2>/dev/null || echo "N/A")

# Get chunk count from pipeline output
CHUNK_COUNT=$(grep "Generated.*chunks" "$PIPELINE_OUT" 2>/dev/null | head -1 || echo "N/A")
DOC_COUNT=$(grep "Found.*files" "$PIPELINE_OUT" 2>/dev/null | head -1 || echo "N/A")

cat > "$REPORT" << REPORT_EOF
# CTD Performance Report — $TIMESTAMP

## Summary

| Metric | Value |
|--------|-------|
| **Mean wall-clock time** | ${MEAN_TIME}s |
| **Median wall-clock time** | ${MEDIAN_TIME}s |
| **User CPU time** | ${USER_TIME}s |
| **System CPU time** | ${SYSTEM_TIME}s |
| **Binary size** | ${BINARY_SIZE} |
| **Test corpus** | 50 docs / ~1MB |
| **Documents indexed** | ${DOC_COUNT} |
| **Chunks generated** | ${CHUNK_COUNT} |

## Top Crates by Binary Size (cargo-bloat)

\`\`\`
${BLOAT_TOP}
\`\`\`

## Pipeline Steps

$(grep -E '^\[STEP|^  (Found|Processed|Generated|Created|Hierarchical)' "$PIPELINE_OUT" 2>/dev/null | sed 's/^/  /')

## Microbenchmark Highlights (if --full)

$(if [[ "$MODE" == "--full" ]]; then
    grep -E '^filter_markdown|^hash/|^snapshot_from_scrape|^compute_plan/|^format_markdown|^json/' \
        "$RESULTS_DIR/bench_filter_$TIMESTAMP.txt" "$RESULTS_DIR/bench_watch_$TIMESTAMP.txt" 2>/dev/null \
        | grep 'time:' | sed 's/^/  /'
else
    echo "  (run with --full for criterion results)"
fi)

## Action Items

$(if python3 -c "exit(0 if float('${MEAN_TIME}') > 8.0 else 1)" 2>/dev/null; then
    echo "⚠️  **Index time > 8s for 50 docs — investigate CHUNK and INDEX+GRAPH steps**"
fi)

---
_Generated by scripts/profile.sh_
REPORT_EOF

echo -e "${GREEN}[REPORT] → profile-results/report_$TIMESTAMP.md${NC}"

# ===========================================================================
# 7. Baseline comparison (if previous baseline exists)
# ===========================================================================
BASELINE_FILE="$RESULTS_DIR/baseline.json"
if [[ "$BASELINE" == true ]]; then
    cp "$RESULTS_DIR/hyperfine_50docs_$TIMESTAMP.json" "$BASELINE_FILE"
    echo -e "${GREEN}[BASELINE] Saved new baseline → profile-results/baseline.json${NC}"
fi

if [[ -f "$BASELINE_FILE" ]] && [[ -f "$LATEST_HYPERFINE" ]] && [[ "$BASELINE" == false ]]; then
    echo -e "${CYAN}[COMPARE] Comparing against baseline...${NC}"
    python3 -c "
import json
with open('$BASELINE_FILE') as f:
    base = json.load(f)['results'][0]
with open('$LATEST_HYPERFINE') as f:
    curr = json.load(f)['results'][0]
    
base_mean = base['mean']
curr_mean = curr['mean']
pct_change = ((curr_mean - base_mean) / base_mean) * 100
status = '🔴 REGRESSION' if pct_change > 5 else ('🟢 IMPROVED' if pct_change < -5 else '🟡 STABLE')
print(f'  Baseline: {base_mean:.3f}s')
print(f'  Current:  {curr_mean:.3f}s')
print(f'  Change:   {pct_change:+.1f}% {status}')
" 2>/dev/null || echo "  (comparison failed)"
fi

echo ""
echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN} Profiling complete! Results in profile-results/${NC}"
echo -e "${CYAN} Report: profile-results/report_$TIMESTAMP.md${NC}"
echo -e "${CYAN}================================================================${NC}"

# List all generated files
ls -la "$RESULTS_DIR"/*"$TIMESTAMP"* 2>/dev/null
