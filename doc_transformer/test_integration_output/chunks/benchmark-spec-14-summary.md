---
doc_id: benchmark-spec
chunk_id: benchmark-spec#14
chunk_level: summary
chunk_type: table
heading: 4. Implementation Details
token_count: 128
summary: Sample size: 10 runs per benchmark. ### Benchmark Functions
---


```
Sample size: 10 runs per benchmark
```

### Benchmark Functions

#### Core: `benchmark_dag_construction()`

```rust
for n in [100, 1_000, 5_000, 10_000] {
    b.iter(|| build_dag_for_benchmark(&chunks, &documents, &tags))
}
```

**What's measured:**
- Time from DAG initialization to final edge insertion
- Includes HNSW index build + query + edge insertion
- Does NOT include data generation (measured separately)

#### Overhead: `benchmark_chunk_generation()`

```rust
for n in [100, 1_000, 5_000, 10_000] {
