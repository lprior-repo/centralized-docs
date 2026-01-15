---
doc_id: benchmark-spec
chunk_id: benchmark-spec#2
chunk_level: summary
chunk_type: table
heading: 1. Domain Research & Contracts
token_count: 141
summary: # HNSW Performance Benchmark Specification. ## Task: centralized-docs-8lg
---

# HNSW Performance Benchmark Specification

## Task: centralized-docs-8lg


---

## 1. Domain Research & Contracts

### Benchmark Objectives

The benchmark validates that `build_knowledge_dag()` scales linearly O(n log n) or better, proving the HNSW-based similarity detection is efficient at scale.

### Key Performance Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| **Time per N** | Wall-clock time to build DAG | < 1s (100 chunks), < 5s (1K), < 15s (10K) |
| **Scaling Factor** | Time(2N) / Time(N) | < 2.5x (sub-quadratic proof) |
