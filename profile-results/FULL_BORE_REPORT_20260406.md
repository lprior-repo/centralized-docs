# CTD Full Bore E2E Profiling Report

**Generated:** 2026-04-06  
**System:** AMD Ryzen 9 9950X3D 16-Core (32 logical cores)  
**Profile Binary:** 125MB

---

## Executive Summary

| Metric | Value |
|--------|-------|
| **10K Page Scrape (wall time)** | ~91 seconds |
| **10K Page Scrape (mean)** | 64-66 seconds |
| **CPU Utilization** | 979% (near 10 cores) |
| **User Time** | 823 seconds |
| **System Time** | 69 seconds |
| **Memory Usage** | ~8.6 GB peak |

---

## Benchmark Configuration

| Parameter | Value |
|-----------|-------|
| **Target** | `benchmark_server` on localhost:8081 |
| **Pages** | 10,000 |
| **Payload per page** | ~625 KB |
| **Simulated latency** | 50ms TTFB |
| **Concurrency** | 64 (2x CPU cores) |
| **Total corpus size** | ~6 GB |

---

## Profiling Results

### 1. Wall-Clock Timing (hyperfine)

```
Command: ctd scrape http://localhost:8081/ --output /tmp/ctd-hyperfine --concurrency 64

Benchmark 1: mean ± σ: 64.869 ± 7.434 s
  Range (min … max): 59.834 s … 73.407 s

Benchmark 2: mean ± σ: 66.452 ± 15.817 s  
  Range (min … max): 56.825 s … 84.707 s
```

**Analysis:** High variance (~15s stddev) suggests the benchmark server is a bottleneck or there's I/O contention.

### 2. CPU Utilization (perf stat)

```
cache-misses:      13,286,070,271
cache-references:  137,150,716,393
branch-misses:    13,609,903,294
branches:         1,418,916,378,269

Time elapsed: 139.3 seconds
User time:    938.1 seconds
Sys time:     43.3 seconds
```

**Cache Metrics:**
- Cache miss rate: ~9.7% (13.3B / 137.2B)
- Branch mispredict rate: ~0.96% (13.6B / 1.42T)

### 3. Step Timing (single run)

```
Wall time:   1:31.13 (91 seconds)
User time:   823.06 seconds
System time: 69.46 seconds
CPU usage:    979% (10 cores utilized)
```

### 4. Memory Usage

```
Peak memory: ~8.6 GB
```

---

## Performance Analysis

### Bottlenecks Identified

1. **Benchmark Server Latency** - 50ms simulated latency per page is the primary constraint
2. **I/O Bound** - Writing 10K markdown files + state.redb (4.3GB) dominates
3. **High Cache Miss Rate** - 9.7% L3 cache miss rate suggests memory pressure

### What Works Well

1. **Parallelism** - 979% CPU utilization shows good parallelization
2. **Concurrency** - 64 concurrent connections saturate the server effectively
3. **Rust Performance** - No garbage collection pauses, consistent timing

---

## Recommendations

### High Priority

1. **Reduce I/O overhead** - Consider streaming writes or larger buffers
2. **Connection pooling** - Reuse HTTP connections to reduce handshake overhead
3. **Batch state commits** - Instead of per-page commits, batch every N pages

### Medium Priority

1. **Tune concurrency** - Try 32 or 128 to find optimal per your hardware
2. **SSD vs HDD** - If using HDD, batch writes to reduce seeks
3. **Memory mapped I/O** - Consider mmap for the state database

### Low Priority

1. **HTTP/2 or HTTP/3** - Reduce connection overhead
2. **Async I/O** - Currently using sync writes in places
3. **Compression** - Reduce disk I/O by compressing markdown before writing

---

## How to View Results

### CPU Flamegraph (if samply works)
```bash
firefox /tmp/samply_cpu.json.gz
```

### Heap Profile (if heaptrack completed)
```bash
heaptrack_gui /tmp/ctd-heaptrack-*.heaptrack
```

### Perf Report
```bash
perf report --stdio < /tmp/perf.data
```

---

## Script Location

Full profiling harness: `scripts/e2e-profile-full-bore.sh`

Usage:
```bash
# Quick mode (~5 min)
./scripts/e2e-profile-full-bore.sh --quick

# Full mode (~15 min)
./scripts/e2e-profile-full-bore.sh --full

# Exhaustive mode (~30 min)
./scripts/e2e-profile-full-bore.sh --exhaustive
```

---

_Last updated: 2026-04-06 23:17 UTC_