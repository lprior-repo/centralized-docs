---
doc_id: benchmark-spec
chunk_id: benchmark-spec#15
chunk_level: summary
chunk_type: table
heading: 4. Implementation Details
token_count: 132
summary: **What's measured:**. #### Overhead: `benchmark_chunk_generation()`
---

```

**What's measured:**

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
