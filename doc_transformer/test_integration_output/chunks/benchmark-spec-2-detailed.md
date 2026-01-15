---
doc_id: benchmark-spec
chunk_id: benchmark-spec#2
chunk_level: detailed
chunk_type: table
heading: 1. Domain Research & Contracts
token_count: 363
summary: # HNSW Performance Benchmark Specification. ## Task: centralized-docs-8lg
---

# HNSW Performance Benchmark Specification

## Task: centralized-docs-8lg

Missing benchmark for HNSW performance at scale. This document specifies the complete benchmark suite with contracts, test data generators, and expected performance characteristics.

---

## 1. Domain Research & Contracts

### Benchmark Objectives

The benchmark validates that `build_knowledge_dag()` scales linearly O(n log n) or better, proving the HNSW-based similarity detection is efficient at scale.

### Key Performance Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| **Time per N** | Wall-clock time to build DAG | < 1s (100 chunks), < 5s (1K), < 15s (10K) |
| **Scaling Factor** | Time(2N) / Time(N) | < 2.5x (sub-quadratic proof) |
| **Edges per second** | (edges_count / execution_time_ms) | Higher is better |
| **Memory usage** | Peak RSS during build | Proportional to N, no spikes |

### Design by Contract (DbC)

```
Preconditions:
- N chunks with valid structure (chunk_id, doc_id, tags)
- Criterion framework installed and configured
- Test data generators produce consistent, reproducible data

Postconditions:
- Benchmark completes without OOM or panic
- Results stored in target/criterion/
- HTML reports generated for trend analysis
- Edge count grows ≤ O(n log n)

Invariants:
- DAG property maintained (no cycles)
- Each chunk has ≤ max_related_chunks edges
- All relationships are deterministic (seeded RNG)
```

---

