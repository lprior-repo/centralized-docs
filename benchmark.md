# Benchmark & Profiling Guide — `ctd`

This document is the single source of truth for performance baselines,
profiling methodology, known bottlenecks, and the self-reinforcing
measurement loop for the `centralized-docs` workspace.

---

## Quick Reference

| What | Command |
|------|---------|
| Run profiling loop | `./scripts/profile.sh --full` |
| Save new baseline | `./scripts/profile.sh --baseline` |
| Quick check (2 min) | `./scripts/profile.sh` |
| Build profiling binary | `cargo build --profile profiling --bin ctd` |
| Run criterion benches | `cargo bench --package centralized-docs` |
| Binary bloat analysis | `cargo bloat bloat --profile profiling --bin ctd --crates -n 30` |

---

## Architecture Overview

`ctd` is a multi-step documentation indexing pipeline:

```
SOURCE ──► DISCOVER ──► ANALYZE ──► ASSIGN ──► TRANSFORM ──► CHUNK ──► VALIDATE ──► INDEX+GRAPH ──► LLMS.TXT
```

Each step is a pure transformation on immutable data. The pipeline is
defined in `centralized-docs/src/cmd/index.rs`.

---

## Baseline Numbers

### Before Optimization (2026-03-29)

Captured before `LazyLock` tokenizer cache + `rayon` parallelism.

| Metric | Before |
|--------|--------|
| Mean wall-clock | **9.51s ± 0.40s** |
| Median | 9.61s |
| Min / Max | 9.08s / 9.86s |
| User CPU | 9.10s |
| System CPU | 0.44s |
| Peak RSS | ~140MB |
| Binary size | 120MB (14.1MB .text) |

### After Optimization (2026-03-30)

After: `LazyLock<Arc<CoreBPE>>` tokenizer cache + `rayon::par_iter` parallel chunking.

| Metric | After | Change |
|--------|-------|--------|
| Mean wall-clock | **740ms ± 14ms** | **-92.2%** |
| Min / Max | 724ms / 760ms | — |
| User CPU | 4.07s | -55% (parallel work across cores) |
| System CPU | 0.43s | stable |

### Per-Step Comparison

```
STEP                             BEFORE     AFTER    CHANGE
STEP 1: DISCOVER                  0.000s    0.000s        —
STEP 2: ANALYZE                   0.001s    0.001s        —
STEP 3: ASSIGN IDs                0.000s    0.000s        —
STEP 4: TRANSFORM                 0.002s    0.000s        —
STEP 5: CHUNK                     8.544s    0.234s   -97.3%  ← LazyLock + rayon
STEP 6: VALIDATE                  0.001s    0.001s        —
STEP 7: INDEX + GRAPH             0.508s    0.506s    -0.4%
STEP 8: LLMS.TXT + AGENTS.MD      0.000s    0.000s        —
 TOTAL                             9.065s    0.753s   -91.7%
 ```

### Microbenchmark Baselines (Criterion)

| Benchmark | Time | Throughput |
|-----------|------|------------|
| `filter_markdown_1mb` | 7.39ms | 135 MB/s |
| `hash/1kb_page` | 22.1ns | 45 MB/s |
| `hash/1mb_page` | 22.1µs | 45 GB/s |
| `snapshot_from_scrape/1k` | 101µs | 9.9M elem/s |
| `snapshot_from_scrape/10k` | 1.15ms | 8.7M elem/s |
| `compute_plan/all_added/1k` | 161µs | 6.2M elem/s |
| `compute_plan/all_added/10k` | 1.71ms | 5.9M elem/s |
| `compute_plan/all_unchanged/10k` | 2.55ms | 3.9M elem/s |
| `format_markdown/10k` | 285µs | 35M elem/s |
| `json/serialize_plan/10k` | 941µs | 10.6M elem/s |
| `json/deserialize_plan/10k` | 1.62ms | 6.2M elem/s |
| `e2e/compute_plan_1000` | 464µs | — |

### Binary Bloat (cargo-bloat, top 15)

```
 File  .text     Size Crate
 1.6%  13.9%   2.0MiB [Unknown]
 1.3%  11.0%   1.6MiB std
 1.1%   8.9%   1.3MiB openssl_sys       ← spider pulls this in; reqwest uses rustls
 0.7%   5.7% 831.1KiB tantivy
 0.5%   4.1% 586.3KiB html5ever
 0.4%   3.7% 541.4KiB bitpacking
 0.4%   3.6% 518.2KiB ctd
 0.4%   3.3% 481.7KiB redb
 0.3%   2.8% 397.9KiB regex_automata
 0.3%   2.7% 386.6KiB libgit2_sys
 0.3%   2.7% 383.3KiB lol_html
 0.3%   2.4% 342.0KiB zstd_sys
 0.2%   1.8% 261.7KiB spider
 0.2%   1.7% 246.4KiB clap_builder
 0.2%   1.5% 223.4KiB tantivy_columnar
11.9% 100.0%  14.1MiB .text section size, the file size is 119.1MiB
```

---

## Known Bottlenecks (Priority Order)

### 1. ~~CHUNK — tiktoken BPE re-initialized 3× per document~~ ✅ FIXED

**Fix applied 2026-03-30:** `LazyLock<Arc<CoreBPE>>` in `contextual-chunker/src/chunk.rs`.
Single global instance, cloned via `Arc` (atomic refcount). `estimate_tokens()`
also reuses the cache. Result: **8.5s → 0.23s (-97.3%)**.

### 2. ~~CHUNK — Sequential document processing~~ ✅ FIXED

**Fix applied 2026-03-30:** `rayon::par_iter` over documents in `chunk_all()`.
Documents are independent — embarrassingly parallel. Functional aggregation
via `flat_map` + `sum` — zero `mut` in the public API.

### 3. Binary — openssl_sys 1.3MB dead weight (unfixed)

`reqwest` is configured with `rustls-tls` (no openssl). But `spider`
pulls in `openssl_sys` transitively through its default features.

**Fix:** Configure spider with `default-features = false` and only
`rustls` TLS features.

### 4. INDEX+GRAPH — HNSW scaling (now 67% of remaining time)

At 0.5s for 4K chunks, this is now the dominant step. Will degrade as
corpus grows due to HNSW's O(n log n) construction cost. Next optimization
target — consider `hnsw_ef_construction` tuning or batch insertion.

---

## Profiling Setup

### Cargo Configuration

**Workspace `Cargo.toml`:**
```toml
[profile.profiling]
inherits = "release"
opt-level = 3
lto = true
codegen-units = 1
debug = "line-tables-only"
strip = false

[profile.bench]
inherits = "release"
opt-level = 3
debug = "line-tables-only"
```

**`.cargo/config.toml`:**
```toml
[build]
rustflags = [
    "-C", "force-frame-pointers=yes",
    "-C", "symbol-mangling-version=v0",
]
```

### Installed Tools

| Tool | Version | Purpose |
|------|---------|---------|
| `cargo-flamegraph` | 0.6.11 | Flamegraph generation (needs perf/sudo) |
| `samply` | 0.13.1 | Modern profiler (needs perf/sudo) |
| `hyperfine` | installed | Wall-clock statistical timing |
| `cargo-bloat` | installed | Binary size breakdown by crate |
| `criterion` | 0.5 | Microbenchmarking (in dev-deps) |

**Note:** `perf_event_paranoid=2` on this machine prevents unprivileged
profiling. `samply` and `flamegraph` require `sudo` to lower it to 1.
The self-reinforcing loop uses `hyperfine` + `cargo-bloat` + `criterion`
which work without elevated privileges.

### Flamegraphs / Samply (when perf is available)

```bash
# Lower perf paranoia (requires sudo once)
echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid

# Flamegraph of full index pipeline
cargo flamegraph --profile profiling --bin ctd -- /tmp/ctd-test-50 --output /tmp/ctd-profile-out

# Samply (interactive, opens Firefox Profiler UI)
samply record target/profiling/ctd index /tmp/ctd-test-50 --output /tmp/ctd-profile-out
```

---

## Self-Reinforcing Profiling Loop

### The Script: `scripts/profile.sh`

```bash
./scripts/profile.sh              # Quick: hyperfine + bloat (~2 min)
./scripts/profile.sh --full       # Full: + criterion benches (~5 min)
./scripts/profile.sh --baseline   # Full + save as comparison baseline
```

### What It Does

1. **Generates test corpus** (50 files, ~1MB) if not present
2. **Builds profiling binary** if stale
3. **Runs `cargo-bloat`** — binary size by crate
4. **Runs `hyperfine`** — statistical wall-clock timing (3 runs, 1 warmup)
5. **Captures per-step timing** — watches stdout for `[STEP N]` markers
6. **Runs criterion benches** (in `--full` mode)
7. **Generates consolidated report** in `profile-results/report_*.md`
8. **Compares against baseline** — shows 🔴/🟢/🟡 regression indicator

### Baseline Comparison

After saving a baseline with `--baseline`, future runs automatically diff:

```
  Baseline: 9.514s
  Current:  6.123s
  Change:   -35.7% 🟢 IMPROVED
```

### Output Files

All results go to `profile-results/`:

```
profile-results/
├── baseline.json                          # Saved baseline for comparison
├── step_timing.json                       # Per-step breakdown
├── hyperfine_50docs_*.json/md/txt         # Timing data
├── bloat_report_*.txt                     # Binary bloat
├── pipeline_steps_*.txt                   # Full stdout capture
├── bench_filter_*.txt                     # Criterion filter results
├── bench_watch_*.txt                      # Criterion watch results
└── report_*.md                            # Consolidated report
```

---

## Test Corpus

The profiling script auto-generates a reproducible test corpus at
`/tmp/ctd-test-50/`:

- 50 markdown files, ~1MB total
- 20 topics cycled (api, architecture, tutorial, etc.)
- 5-20 sections per file with random heading levels
- 50-300 words per section
- Produces ~4171 chunks (3058 summary, 775 standard, 338 detailed)

For stress testing, generate more:
```bash
mkdir -p /tmp/ctd-test-200
python3 -c "
import os, random
topics = ['api','guide','reference','tutorial','internals']
for i in range(200):
    topic = topics[i % len(topics)]
    sections = []
    for j in range(random.randint(5, 20)):
        words = random.randint(50, 300)
        content = ' '.join([f'w{k}' for k in range(words)])
        sections.append(f'## Section {j}\n\n{content}\n')
    with open(f'/tmp/ctd-test-200/{topic}_{i:03d}.md', 'w') as f:
        f.write(f'# {topic.title()} {i}\n\n' + '\n'.join(sections))
print('Generated 200 files')
"
```

---

## Regression Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Wall-clock (50 docs) | > 1.0s | > 1.5s |
| CHUNK step | > 0.4s | > 0.6s |
| INDEX+GRAPH step | > 0.8s | > 1.0s |
| Binary size | > 130MB | > 150MB |
| `snapshot_from_scrape/10k` | > 1.3ms | > 1.5ms |
| `compute_plan/all_added/10k` | > 2.0ms | > 2.5ms |

These thresholds are enforced by the profiling script's report card.

---

## Methodology Notes

- All timings taken on a warm machine (no cold-start bias)
- `hyperfine` uses 1 warmup run + 3 measured runs
- Criterion uses 100 samples with statistical analysis
- `cargo-bloat` numbers are estimates (documented in their output)
- The profiling profile preserves optimizations (LTO, opt-level=3) while
  adding `debug = "line-tables-only"` for symbol resolution
- Frame pointers enabled via `.cargo/config.toml` for stack walking
