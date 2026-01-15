---
doc_id: benchmark-spec
chunk_id: benchmark-spec#6
chunk_level: standard
chunk_type: table
heading: 4. Implementation Details
token_count: 203
summary: iter(|| build_dag_for_benchmark(&chunks, &documents, &tags)). **What's measured:**
---




```rust
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
    b.iter(|| generate_test_chunks(n))
}
```

**What's measured:**
- Time to allocate and populate N chunks
- Validates data gen is not the bottleneck
- Should be < 5% of total time

#### Overhead: `benchmark_tag_generation()`

```rust
for n in [100, 1_000, 5_000, 10_000] {
    b.iter(|| generate_test_tags(&chunks))
}
```

**What's measured:**
- Time to create tag metadata
- Should be O(n) and very fast
- Should be < 1% of DAG build time

---

