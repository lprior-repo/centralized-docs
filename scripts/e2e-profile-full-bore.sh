#!/usr/bin/env bash
# ===========================================================================
# CTD FULL BORE E2E PROFILING HARNESS
# ===========================================================================
#
# Complete profiling suite: CPU, Memory, I/O, Async, Network, Disk
#
# Usage:
#   ./scripts/e2e-profile-full-bore.sh [--install] [--quick|--full|--exhaustive]
#
# Modes:
#   --install    Install dependencies and exit
#   --quick      Light profiling (~5 min)
#   --full       Full profiling with all tools (~15 min)
#   --exhaustive Extended runs + memory spikes (~30 min)
#
# Prerequisites (auto-installed with --install):
#   - heaptrack (pacman)
#   - perf (linux-tools)
#   - tokio-console (cargo)
#   - Build: cargo build --profile profiling --bin ctd
#
# ===========================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RESULTS_DIR="$PROJECT_ROOT/profile-results"
PROFILE_BIN="$PROJECT_ROOT/target/profiling/ctd"
TIMESTAMP=$(date +Y%m%d_%H%M%S)
PROFILE_PORT=8081
CPU_CORES=$(nproc)
CONCURRENCY=$((CPU_CORES * 2))

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BLUE='\033[0;34m'
NC='\033[0m'

# Tool paths
SAMPLY="/cache/cargo-shared/bin/samply"
HYPERFINE="/cache/cargo-shared/bin/hyperfine"
HEAPTRACK="heaptrack"
PERF="perf"
CARGO="$PROJECT_ROOT/target/profiling/cargo"

# ===========================================================================
# PARSE ARGUMENTS
# ===========================================================================

MODE="${1:---full}"
DO_INSTALL=false

if [[ "$MODE" == "--install" ]]; then
    DO_INSTALL=true
    MODE="--full"
fi

# ===========================================================================
# INSTALL DEPENDENCIES
# ===========================================================================

install_deps() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} INSTALLING PROFILING DEPENDENCIES${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[PACMAN] Installing system packages...${NC}"
    sudo pacman -S --noconfirm heaptrack

    echo -e "${GREEN}[DONE] Dependencies installed${NC}"
}

# ===========================================================================
# PREFLIGHT CHECKS
# ===========================================================================

check_tools() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PREFLIGHT CHECKS${NC}"
    echo -e "${CYAN}================================================================${NC}"

    local missing=0

    echo -e "${YELLOW}[CHECK] heaptrack...${NC}"
    if command -v heaptrack &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} heaptrack $(heaptrack --version 2>&1 | head -1)"
    else
        echo -e "  ${RED}✗${NC} heaptrack NOT FOUND"
        missing=1
    fi

    echo -e "${YELLOW}[CHECK] samply...${NC}"
    if [[ -x "$SAMPLY" ]]; then
        echo -e "  ${GREEN}✓${NC} samply"
    else
        echo -e "  ${RED}✗${NC} samply NOT FOUND at $SAMPLY"
        missing=1
    fi

    echo -e "${YELLOW}[CHECK] hyperfine...${NC}"
    if [[ -x "$HYPERFINE" ]]; then
        echo -e "  ${GREEN}✓${NC} hyperfine"
    else
        echo -e "  ${RED}✗${NC} hyperfine NOT FOUND at $HYPERFINE"
        missing=1
    fi

    echo -e "${YELLOW}[CHECK] perf...${NC}"
    if command -v perf &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} perf $(perf --version 2>&1 | head -1)"
    else
        echo -e "  ${RED}✗${NC} perf NOT FOUND (linux-tools package)"
        missing=1
    fi

    echo -e "${YELLOW}[CHECK] tokio-console...${NC}"
    if command -v tokio-console &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} tokio-console"
    else
        echo -e "  ${YELLOW}⚠${NC} tokio-console NOT FOUND (optional)"
    fi

    if [[ $missing -eq 1 ]]; then
        echo -e "${RED}[ERROR] Missing required tools. Run with --install or install manually.${NC}"
        exit 1
    fi
}

# ===========================================================================
# BUILD PROFILING BINARY
# ===========================================================================

build_binary() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} BUILDING PROFILING BINARY${NC}"
    echo -e "${CYAN}================================================================${NC}"

    if [[ ! -f "$PROFILE_BIN" ]] || [[ "$PROFILE_BIN" -ot "$PROJECT_ROOT/Cargo.toml" ]]; then
        echo -e "${YELLOW}[BUILD] cargo build --profile profiling --bin ctd${NC}"
        cargo build --profile profiling --bin ctd
    else
        echo -e "${GREEN}[BUILD] Binary already up-to-date${NC}"
    fi

    echo -e "${GREEN}[BUILD] Binary: $(ls -lh "$PROFILE_BIN" | awk '{print $5, $9}')${NC}"
}

# ===========================================================================
# BENCHMARK SERVER MANAGEMENT
# ===========================================================================

start_benchmark_server() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} STARTING BENCHMARK SERVER${NC}"
    echo -e "${CYAN}================================================================${NC}"

    local server_dir="$PROJECT_ROOT/benchmark_server"
    local server_pid_file="/tmp/ctd-benchmark-server.pid"
    local max_wait=30

    if ss -tlnp 2>/dev/null | grep -q ":$PROFILE_PORT "; then
        echo -e "${YELLOW}[SERVER] Server already running on port $PROFILE_PORT${NC}"
        return 0
    fi

    echo -e "${YELLOW}[SERVER] Starting benchmark_server on port $PROFILE_PORT...${NC}"

    cd "$server_dir"
    cargo run --release &> /tmp/ctd-benchmark-server.log &
    local server_pid=$!
    echo $server_pid > "$server_pid_file"
    cd - > /dev/null

    echo -e "${YELLOW}[SERVER] Waiting for server to start (pid: $server_pid)...${NC}"
    local waited=0
    while ! ss -tlnp 2>/dev/null | grep -q ":$PROFILE_PORT "; do
        sleep 1
        waited=$((waited + 1))
        if [[ $waited -ge $max_wait ]]; then
            echo -e "${RED}[SERVER] Timeout waiting for server startup${NC}"
            cat /tmp/ctd-benchmark-server.log
            exit 1
        fi
    done

    echo -e "${GREEN}[SERVER] Benchmark server running on http://localhost:$PROFILE_PORT${NC}"
    echo -e "${GREEN}[SERVER]   - 10,000 pages${NC}"
    echo -e "${GREEN}[SERVER]   - ~625 KB per page${NC}"
    echo -e "${GREEN}[SERVER]   - 50ms simulated latency${NC}"
}

stop_benchmark_server() {
    local pid_file="/tmp/ctd-benchmark-server.pid"
    if [[ -f "$pid_file" ]]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            echo -e "${YELLOW}[SERVER] Stopping benchmark server (pid: $pid)${NC}"
            kill "$pid" 2>/dev/null || true
            sleep 1
            kill -9 "$pid" 2>/dev/null || true
        fi
        rm -f "$pid_file"
    fi
}

# ===========================================================================
# GENERATE TEST CORPORA
# ===========================================================================

generate_corpora() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} GENERATING TEST CORPORA${NC}"
    echo -e "${CYAN}================================================================${NC}"

    # Small corpus: 50 files
    if [[ ! -d /tmp/ctd-test-50 ]] || [[ $(ls /tmp/ctd-test-50/*.md 2>/dev/null | wc -l) -lt 50 ]]; then
        echo -e "${YELLOW}[CORPUS] Generating 50-file test corpus...${NC}"
        mkdir -p /tmp/ctd-test-50
        python3 -c "
import os, random
topics = ['api','architecture','tutorial','reference','guide','internals',
          'deployment','security','performance','testing','configuration',
          'monitoring','troubleshooting','migration','plugins','sdk']
for i in range(50):
    topic = topics[i % len(topics)]
    filename = f'/tmp/ctd-test-50/{topic}_{i:03d}.md'
    sections = []
    for j in range(random.randint(5, 15)):
        hl = random.choice([2, 3, 4])
        heading = '#' * hl + f' Section {j} - {topic.title()} Details'
        words = random.randint(50, 200)
        content = ' '.join([f'{topic}' if random.random() < 0.05 else f'word{k}' for k in range(words)])
        sections.append(f'{heading}\n\n{content}\n')
    doc = f'# {topic.title()} Document {i}\n\nThis is document {i} about {topic}.\n\n' + '\n'.join(sections)
    with open(filename, 'w') as f:
        f.write(doc)
print('Generated 50 test files')
"
        echo -e "${GREEN}[CORPUS] 50-file corpus ready${NC}"
    else
        echo -e "${GREEN}[CORPUS] 50-file corpus already exists${NC}"
    fi

    # Medium corpus: 500 files
    if [[ ! -d /tmp/ctd-test-500 ]] || [[ $(ls /tmp/ctd-test-500/*.md 2>/dev/null | wc -l) -lt 500 ]]; then
        echo -e "${YELLOW}[CORPUS] Generating 500-file test corpus...${NC}"
        mkdir -p /tmp/ctd-test-500
        python3 -c "
import os, random, shutil
# Copy 50-base and multiply
base_files = []
if os.path.exists('/tmp/ctd-test-50'):
    base_files = [f for f in os.listdir('/tmp/ctd-test-50') if f.endswith('.md')]
for i in range(500):
    src = f'/tmp/ctd-test-50/{base_files[i % len(base_files)]}' if base_files else None
    dst = f'/tmp/ctd-test-500/doc_{i:04d}.md'
    if src:
        with open(src) as sf:
            content = sf.read().replace(f' Document {(i-1) % 50}', f' Document {i}')
        with open(dst, 'w') as df:
            df.write(content)
    else:
        with open(dst, 'w') as df:
            df.write(f'# Document {i}\n\nContent for document {i}.\n')
print('Generated 500 test files')
" 2>/dev/null || echo -e "${YELLOW}[CORPUS] Skipping 500-file corpus${NC}"
        echo -e "${GREEN}[CORPUS] 500-file corpus ready${NC}"
    else
        echo -e "${GREEN}[CORPUS] 500-file corpus already exists${NC}"
    fi
}

# ===========================================================================
# PROFILER: SAMPLY (CPU Flamegraph)
# ===========================================================================

profile_cpu_flamegraph() {
    local output_file="$RESULTS_DIR/samply_cpu_$TIMESTAMP.json.gz"
    local duration="${1:-60}"

    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: CPU FLAMEGRAPH (samply, ${duration}s)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[SAMPLY] Recording CPU profile...${NC}"
    echo -e "${YELLOW}[SAMPLY] Duration: ${duration}s | Output: $output_file${NC}"

    "$SAMPLY" record \
        --duration "$duration" \
        --output "$output_file" \
        -- \
        "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-samply" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/samply_output_$TIMESTAMP.log" || true

    if [[ -f "$output_file" ]]; then
        echo -e "${GREEN}[SAMPLY] Done → $output_file${NC}"
        echo -e "${GREEN}[SAMPLY] View with: firefox $output_file${NC}"
    else
        echo -e "${RED}[SAMPLY] Failed to generate profile${NC}"
    fi
}

# ===========================================================================
# PROFILER: HEAPTRACK (Memory)
# ===========================================================================

profile_memory_heaptrack() {
    local output_file="/tmp/ctd-heaptrack-$$"

    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: HEAP MEMORY (heaptrack)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[HEAPTRACK] Recording memory profile...${NC}"
    echo -e "${YELLOW}[HEAPTRACK] Output prefix: $output_file${NC}"

    heaptrack --analyze "$PROFILE_BIN" \
        -- \
        "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-heaptrack" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/heaptrack_output_$TIMESTAMP.log" || true

    local heaptrack_output=$(ls /tmp/ctd-heaptrack-*.heaptrack 2>/dev/null | head -1 || echo "")

    if [[ -n "$heaptrack_output" ]]; then
        local final_output="$RESULTS_DIR/heaptrack_$(basename "$heaptrack_output")"
        mv "$heaptrack_output" "$final_output"
        echo -e "${GREEN}[HEAPTRACK] Done → $final_output${NC}"
        echo -e "${GREEN}[HEAPTRACK] View with: heaptrack_gui $final_output${NC}"
    else
        echo -e "${RED}[HEAPTRACK] Failed to generate profile${NC}"
    fi
}

# ===========================================================================
# PROFILER: HYPERFINE (Wall-Clock Timing)
# ===========================================================================

profile_wallclock_hyperfine() {
    local runs="${1:-3}"

    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: WALL-CLOCK TIMING (hyperfine, ${runs} runs)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[HYPERFINE] Running statistical benchmarks...${NC}"

    # Clean output directories
    rm -rf /tmp/ctd-hyperfine-{1,2,3} 2>/dev/null || true

    # Run hyperfine
    "$HYPERFINE" \
        --warmup 1 \
        --runs "$runs" \
        --export-json "$RESULTS_DIR/hyperfine_$TIMESTAMP.json" \
        --export-markdown "$RESULTS_DIR/hyperfine_$TIMESTAMP.md" \
        "$PROFILE_BIN scrape http://localhost:$PROFILE_PORT/ --output /tmp/ctd-hyperfine-1 --concurrency $CONCURRENCY" \
        "$PROFILE_BIN scrape http://localhost:$PROFILE_PORT/ --output /tmp/ctd-hyperfine-2 --concurrency $CONCURRENCY" \
        "$PROFILE_BIN scrape http://localhost:$PROFILE_PORT/ --output /tmp/ctd-hyperfine-3 --concurrency $CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/hyperfine_output_$TIMESTAMP.log"

    echo -e "${GREEN}[HYPERFINE] Done → profile-results/hyperfine_$TIMESTAMP.*${NC}"
}

# ===========================================================================
# PROFILER: PERF (CPU Cache, Branch, Lock)
# ===========================================================================

profile_perf_events() {
    local duration="${1:-60}"

    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: CPU EVENTS (perf, ${duration}s)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    if ! command -v perf &> /dev/null; then
        echo -e "${YELLOW}[PERF] perf not available, skipping${NC}"
        return 0
    fi

    echo -e "${YELLOW}[PERF] Recording CPU events...${NC}"

    # CPU cache analysis
    echo -e "${YELLOW}[PERF]   - Cache misses${NC}"
    perf stat -e cache-misses,cache-references,branch-misses,branches \
        -o "$RESULTS_DIR/perf_cache_$TIMESTAMP.txt" \
        -- "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-perf" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee /dev/stderr || true

    # Branch prediction analysis
    echo -e "${YELLOW}[PERF]   - Branch prediction${NC}"
    perf stat -e branch-mispredicts,bru-mispredictions \
        -o "$RESULTS_DIR/perf_branch_$TIMESTAMP.txt" \
        -- "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-perf2" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee /dev/stderr || true

    # Instruction count
    echo -e "${YELLOW}[PERF]   - Instruction count${NC}"
    perf stat -e instructions,cycles \
        -o "$RESULTS_DIR/perf_instructions_$TIMESTAMP.txt" \
        -- "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-perf3" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee /dev/stderr || true

    echo -e "${GREEN}[PERF] Done → profile-results/perf_*_$TIMESTAMP.txt${NC}"
}

# ===========================================================================
# PROFILER: I/O ANALYSIS
# ===========================================================================

profile_io() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: I/O (iotop)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[IO] Starting I/O monitoring in background...${NC}"

    (while true; do
        echo "=== $(date '+%H:%M:%S') ===" >> "$RESULTS_DIR/iotop_$TIMESTAMP.log"
        iotop -b -n 2 >> "$RESULTS_DIR/iotop_$TIMESTAMP.log" 2>/dev/null || true
        sleep 2
    done) &
    local io_pid=$!

    # Run scrape workload
    "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-io" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/io_workload_$TIMESTAMP.log" || true

    # Stop I/O monitoring
    kill $io_pid 2>/dev/null || true
    sleep 1

    echo -e "${GREEN}[IO] Done → profile-results/iotop_$TIMESTAMP.log${NC}"
}

# ===========================================================================
# PROFILER: NETWORK ANALYSIS
# ===========================================================================

profile_network() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: NETWORK (tcpdump summary)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[NETWORK] Capturing network stats during scrape...${NC}"

    # Use /proc/net/dev to get interface stats before/after
    local before_dev=$(cat /proc/net/dev | grep -E "eth0|eno|enp|wl" | head -1 || echo "")
    local before_time=$(date +%s)

    # Run scrape
    "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-net" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/network_workload_$TIMESTAMP.log" || true

    local after_time=$(date +%s)
    local after_dev=$(cat /proc/net/dev | grep -E "eth0|eno|enp|wl" | head -1 || echo "")

    echo -e "${YELLOW}[NETWORK] Interface stats:${NC}"
    echo -e "  Before: $before_dev"
    echo -e "  After:  $after_dev"
    echo -e "  Duration: $((after_time - before_time))s"

    echo -e "${GREEN}[NETWORK] Done → profile-results/network_workload_$TIMESTAMP.log${NC}"
}

# ===========================================================================
# PROFILER: TOKIO-CONSOLE (Async)
# ===========================================================================

profile_tokio_console() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: ASYNC (tokio-console)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    if ! command -v tokio-console &> /dev/null; then
        echo -e "${YELLOW}[TOKIO] tokio-console not installed, skipping async profiling${NC}"
        return 0
    fi

    echo -e "${YELLOW}[TOKIO] Starting tokio-console listener...${NC}"
    echo -e "${YELLOW}[TOKIO] Run scrape with TOKIO_CONSOLE_ENABLED=1${NC}"

    # Create a background tokio-console listener
    (tokio-console listen --port 6666 > "$RESULTS_DIR/tokio_console_$TIMESTAMP.log" 2>&1 || true) &
    local console_pid=$!

    sleep 1

    # Run with tokio console enabled (requires RUSTFLAGS="--cfg tokio_unstable")
    echo -e "${YELLOW}[TOKIO] Starting ctd with tokio unstable...${NC}"

    RUSTFLAGS="--cfg tokio_unstable" cargo build --profile profiling --bin ctd 2>&1 | tail -5

    TOKIO_CONSOLE_ENABLED=1 \
        "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-tokio" \
        --concurrency "$CONCURRENCY" \
        2>&1 | head -100 || true

    sleep 1
    kill $console_pid 2>/dev/null || true

    echo -e "${GREEN}[TOKIO] Done → profile-results/tokio_console_$TIMESTAMP.log${NC}"
}

# ===========================================================================
# PROFILER: STEP TIMING (Per-Step Latency)
# ===========================================================================

profile_step_timing() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: STEP TIMING${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[STEPS] Running with detailed step timing...${NC}"

    # Use /usr/bin/time for detailed timing
    /usr/bin/time -v "$PROFILE_BIN" scrape "http://localhost:$PROFILE_PORT/" \
        --output "/tmp/ctd-scrape-steps" \
        --concurrency "$CONCURRENCY" \
        2>&1 | tee "$RESULTS_DIR/step_timing_$TIMESTAMP.log"

    # Also parse stdout for step markers
    grep -E '\[STEP|^\[DIFF|^\[CONFIG|Pages scraped|Files:|Output:' \
        "$RESULTS_DIR/step_timing_$TIMESTAMP.log" \
        > "$RESULTS_DIR/step_summary_$TIMESTAMP.txt" || true

    echo -e "${GREEN}[STEPS] Done → profile-results/step_*_$TIMESTAMP.{log,txt}${NC}"
}

# ===========================================================================
# PROFILER: CARGO-BLOAT (Binary Size)
# ===========================================================================

profile_bloat() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: BINARY SIZE (cargo-bloat)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[BLOAT] Analyzing binary size by crate...${NC}"

    /cache/cargo-shared/bin/cargo-bloat bloat \
        --profile profiling \
        --bin ctd \
        --crates \
        -n 50 \
        > "$RESULTS_DIR/bloat_report_$TIMESTAMP.txt" 2>&1

    # Top 20 by size
    head -30 "$RESULTS_DIR/bloat_report_$TIMESTAMP.txt"

    echo -e "${GREEN}[BLOAT] Done → profile-results/bloat_report_$TIMESTAMP.txt${NC}"
}

# ===========================================================================
# INDEX PIPELINE PROFILING
# ===========================================================================

profile_index_pipeline() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: INDEX PIPELINE (50 docs)${NC}"
    echo -e "${CYAN}================================================================${NC}"

    echo -e "${YELLOW}[INDEX] Profiling local index pipeline...${NC}"

    /usr/bin/time -v "$PROFILE_BIN" index /tmp/ctd-test-50 \
        --output "/tmp/ctd-index-profile" \
        2>&1 | tee "$RESULTS_DIR/index_pipeline_$TIMESTAMP.log"

    # Parse step timing
    grep -E '\[STEP|Found|Processed|Generated|Created|Hierarchical|INDEX|GRAPH|LLMS' \
        "$RESULTS_DIR/index_pipeline_$TIMESTAMP.log" \
        > "$RESULTS_DIR/index_steps_$TIMESTAMP.txt" || true

    # Run hyperfine on index
    "$HYPERFINE" \
        --warmup 1 \
        --runs 3 \
        --export-json "$RESULTS_DIR/hyperfine_index_$TIMESTAMP.json" \
        "$PROFILE_BIN index /tmp/ctd-test-50 --output /tmp/ctd-index-{n}" \
        2>&1 | tee "$RESULTS_DIR/hyperfine_index_$TIMESTAMP.log"

    echo -e "${GREEN}[INDEX] Done → profile-results/index_*_$TIMESTAMP.*${NC}"
}

# ===========================================================================
# SEARCH LATENCY PROFILING
# ===========================================================================

profile_search_latency() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} PROFILING: SEARCH LATENCY${NC}"
    echo -e "${CYAN}================================================================${NC}"

    # First ensure we have an index
    if [[ ! -f "/tmp/ctd-index-profile/INDEX.json" ]]; then
        echo -e "${YELLOW}[SEARCH] Building index first...${NC}"
        "$PROFILE_BIN" index /tmp/ctd-test-50 --output /tmp/ctd-index-profile 2>&1 | tail -5
    fi

    echo -e "${YELLOW}[SEARCH] Profiling search latency...${NC}"

    # Search queries
    local queries=("api" "configuration" "tutorial" "reference" "security")

    for query in "${queries[@]}"; do
        echo -e "  Query: $query"
        /usr/bin/time -v "$PROFILE_BIN" search "$query" \
            --index-dir /tmp/ctd-index-profile \
            2>&1 | grep -E "real|user|sys|Results:" >> "$RESULTS_DIR/search_latency_$TIMESTAMP.log"
    done

    # Hyperfine for search
    "$HYPERFINE" \
        --warmup 3 \
        --runs 10 \
        --export-json "$RESULTS_DIR/hyperfine_search_$TIMESTAMP.json" \
        "$PROFILE_BIN search api --index-dir /tmp/ctd-index-profile" \
        "$PROFILE_BIN search configuration --index-dir /tmp/ctd-index-profile" \
        "$PROFILE_BIN search tutorial --index-dir /tmp/ctd-index-profile" \
        2>&1 | tee "$RESULTS_DIR/hyperfine_search_$TIMESTAMP.log"

    echo -e "${GREEN}[SEARCH] Done → profile-results/search_*_$TIMESTAMP.*${NC}"
}

# ===========================================================================
# GENERATE CONSOLIDATED REPORT
# ===========================================================================

generate_report() {
    local report_file="$RESULTS_DIR/FULL_BORE_REPORT_$TIMESTAMP.md"

    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} GENERATING CONSOLIDATED REPORT${NC}"
    echo -e "${CYAN}================================================================${NC}"

    # Parse hyperfine results
    local hyperfine_json=$(ls -t "$RESULTS_DIR"/hyperfine_*.json 2>/dev/null | head -1)
    local mean_time="N/A"
    local median_time="N/A"

    if [[ -n "$hyperfine_json" ]] && [[ -f "$hyperfine_json" ]]; then
        mean_time=$(python3 -c "import json; d=json.load(open('$hyperfine_json')); print(f\"{d['results'][0]['mean']:.3f}\")" 2>/dev/null || echo "N/A")
        median_time=$(python3 -c "import json; d=json.load(open('$hyperwise_json')); print(f\"{d['results'][0]['median']:.3f}\")" 2>/dev/null || echo "N/A")
    fi

    # Parse step timing
    local step_output=$(ls -t "$RESULTS_DIR"/step_timing_*.log 2>/dev/null | head -1 || echo "")

    # Binary size
    local binary_size=$(ls -lh "$PROFILE_BIN" 2>/dev/null | awk '{print $5}' || echo "N/A")

    cat > "$report_file" << REPORT_EOF
# CTD Full Bore E2E Profiling Report — $TIMESTAMP

## System Configuration

| Component | Value |
|-----------|-------|
| CPU Cores | $CPU_CORES |
| Concurrency | $CONCURRENCY |
| Benchmark Server | localhost:$PROFILE_PORT (10K pages, ~625KB/page, 50ms latency) |
| Profile Binary | $PROFILE_BIN |
| Binary Size | $binary_size |

## Executive Summary

| Metric | Value |
|--------|-------|
| **Scrape Wall-Clock (mean)** | ${mean_time}s |
| **Scrape Wall-Clock (median)** | ${median_time}s |
| **Target** | 10,000 pages @ 50ms/page |

## Profiling Results

### 1. CPU Flamegraph
\`\`\`
File: $RESULTS_DIR/samply_cpu_$TIMESTAMP.json.gz
View: firefox $RESULTS_DIR/samply_cpu_$TIMESTAMP.json.gz
\`\`\`

### 2. Heap Memory
\`\`\`
File: $RESULTS_DIR/heaptrack_*.heaptrack
View: heaptrack_gui $RESULTS_DIR/heaptrack_*.heaptrack
\`\`\`

### 3. Wall-Clock Timing (hyperfine)
\`\`\`
$(cat "$RESULTS_DIR/hyperfine_$TIMESTAMP.md" 2>/dev/null || echo "See hyperfine JSON")
\`\`\`

### 4. CPU Cache/Branch Performance
\`\`\`
Cache: $RESULTS_DIR/perf_cache_$TIMESTAMP.txt
Branch: $RESULTS_DIR/perf_branch_$TIMESTAMP.txt
Instructions: $RESULTS_DIR/perf_instructions_$TIMESTAMP.txt
\`\`\`

### 5. Binary Size (Top 20 Crates)
\`\`\`
$(head -25 "$RESULTS_DIR/bloat_report_$TIMESTAMP.txt" 2>/dev/null || echo "See bloat report")
\`\`\`

### 6. Step Timing
\`\`\`
$(cat "$RESULTS_DIR/step_summary_$TIMESTAMP.txt" 2>/dev/null || echo "See step_timing log")
\`\`\`

### 7. Index Pipeline (50 docs)
\`\`\`
$(cat "$RESULTS_DIR/index_steps_$TIMESTAMP.txt" 2>/dev/null || echo "See index_pipeline log")
\`\`\`

### 8. Search Latency
\`\`\`
$(cat "$RESULTS_DIR/search_latency_$TIMESTAMP.log" 2>/dev/null | head -20 || echo "See search logs")
\`\`\`

## Generated Files

| File | Description |
|------|-------------|
| FULL_BORE_REPORT_$TIMESTAMP.md | This report |
| samply_cpu_$TIMESTAMP.json.gz | CPU flamegraph (Firefox Profiler) |
| hyperfine_$TIMESTAMP.json | Wall-clock timing data |
| heaptrack_*.heaptrack | Heap memory profile |
| perf_cache_$TIMESTAMP.txt | CPU cache statistics |
| bloat_report_$TIMESTAMP.txt | Binary size by crate |
| step_timing_$TIMESTAMP.log | Per-step timing |
| index_pipeline_$TIMESTAMP.log | Index pipeline output |
| search_latency_$TIMESTAMP.log | Search latency measurements |

---

_Generated by e2e-profile-full-bore.sh at $(date)_
_RECOMMENDED: Run with --exhaustive for statistically significant results_
REPORT_EOF

    echo -e "${GREEN}[REPORT] Generated → $report_file${NC}"
}

# ===========================================================================
# BASELINE COMPARISON
# ===========================================================================

compare_baseline() {
    local baseline_file="$RESULTS_DIR/baseline/full_bore_baseline.json"

    if [[ -f "$baseline_file" ]]; then
        echo -e "${CYAN}[BASELINE] Comparing against saved baseline...${NC}"

        local current_hyperfine=$(ls -t "$RESULTS_DIR"/hyperfine_*.json 2>/dev/null | head -1)

        if [[ -n "$current_hyperfine" ]]; then
            python3 -c "
import json, sys

try:
    with open('$baseline_file') as f:
        base = json.load(f)
    with open('$current_hyperfine') as f:
        curr = json.load(f)

    base_mean = float(base.get('mean_time', base['results'][0]['mean']))
    curr_mean = float(curr['results'][0]['mean'])

    pct = ((curr_mean - base_mean) / base_mean) * 100
    status = '🔴 REGRESSION' if pct > 5 else ('🟢 IMPROVED' if pct < -5 else '🟡 STABLE')

    print(f'  Baseline: {base_mean:.3f}s')
    print(f'  Current:  {curr_mean:.3f}s')
    print(f'  Change:   {pct:+.1f}% {status}')
except Exception as e:
    print(f'  Comparison failed: {e}')
" 2>/dev/null || echo "  (comparison unavailable)"
        fi
    else
        echo -e "${YELLOW}[BASELINE] No baseline found. Save with --baseline flag.${NC}"
    fi
}

# ===========================================================================
# SAVE BASELINE
# ===========================================================================

save_baseline() {
    mkdir -p "$RESULTS_DIR/baseline"

    local current_hyperfine=$(ls -t "$RESULTS_DIR"/hyperfine_*.json 2>/dev/null | head -1)

    if [[ -n "$current_hyperfine" ]]; then
        cp "$current_hyperfine" "$RESULTS_DIR/baseline/full_bore_baseline.json"
        echo -e "${GREEN}[BASELINE] Saved → $RESULTS_DIR/baseline/full_bore_baseline.json${NC}"
    fi
}

# ===========================================================================
# MAIN EXECUTION
# ===========================================================================

main() {
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} CTD FULL BORE E2E PROFILING HARNESS${NC}"
    echo -e "${CYAN} Mode: $MODE | Timestamp: $TIMESTAMP${NC}"
    echo -e "${CYAN}================================================================${NC}"

    mkdir -p "$RESULTS_DIR"

    # Phase 0: Install if requested
    if [[ "$DO_INSTALL" == true ]]; then
        install_deps
        exit 0
    fi

    # Phase 1: Preflight checks
    check_tools

    # Phase 2: Build
    build_binary

    # Phase 3: Generate corpora
    generate_corpora

    # Phase 4: Start benchmark server
    start_benchmark_server

    # Cleanup on exit
    trap 'echo -e "${YELLOW}[CLEANUP] Stopping benchmark server...${NC}"; stop_benchmark_server' EXIT

    # Phase 5: Run all profilers
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} RUNNING ALL PROFILERS${NC}"
    echo -e "${CYAN} Mode: $MODE | Concurrency: $CONCURRENCY${NC}"
    echo -e "${CYAN}================================================================${NC}"

    case "$MODE" in
        --quick)
            echo -e "${YELLOW}[MODE] QUICK PROFILING (~5 min)${NC}"
            profile_wallclock_hyperfine 3
            profile_bloat
            profile_step_timing
            profile_index_pipeline
            ;;
        --full)
            echo -e "${YELLOW}[MODE] FULL PROFILING (~15 min)${NC}"
            profile_bloat
            profile_wallclock_hyperfine 3
            profile_cpu_flamegraph 60
            profile_memory_heaptrack
            profile_step_timing
            profile_index_pipeline
            profile_search_latency
            if command -v perf &> /dev/null; then
                profile_perf_events 30
            fi
            ;;
        --exhaustive)
            echo -e "${YELLOW}[MODE] EXHAUSTIVE PROFILING (~30 min)${NC}"
            profile_bloat
            profile_wallclock_hyperfine 5
            profile_cpu_flamegraph 120
            profile_memory_heaptrack
            profile_step_timing
            profile_index_pipeline
            profile_search_latency
            if command -v perf &> /dev/null; then
                profile_perf_events 60
            fi
            profile_io
            profile_network
            ;;
        *)
            echo -e "${RED}[ERROR] Unknown mode: $MODE${NC}"
            exit 1
            ;;
    esac

    # Phase 6: Generate report
    generate_report

    # Phase 7: Baseline comparison
    compare_baseline

    echo ""
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN} FULL BORE PROFILING COMPLETE${NC}"
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${GREEN}Results directory: $RESULTS_DIR${NC}"
    echo -e "${GREEN}Report: $RESULTS_DIR/FULL_BORE_REPORT_$TIMESTAMP.md${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "  1. View flamegraph:    firefox $RESULTS_DIR/samply_cpu_$TIMESTAMP.json.gz"
    echo -e "  2. View heap:          heaptrack_gui $RESULTS_DIR/heaptrack_*.heaptrack"
    echo -e "  3. View report:        cat $RESULTS_DIR/FULL_BORE_REPORT_$TIMESTAMP.md"
    echo -e "  4. Save baseline:      $0 --baseline"
    echo ""
}

# Run main
main "$@"