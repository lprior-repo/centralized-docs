---
doc_id: benchmark-spec
chunk_id: benchmark-spec#5
chunk_level: standard
chunk_type: table
heading: 4. Implementation Details
token_count: 518
summary: ### Handled Scenarios. | **Extra-large** | 20,000 | Proves scaling up to limit | 20-60 seconds |
---

---


### Handled Scenarios

| **Extra-large** | 20,000 | Proves scaling up to limit | 20-60 seconds |

### Boundary Conditions

- **N=100**: Minimum meaningful benchmark (avoids noise)
- **N=20,000**: Maximum before OOM risk on 8GB RAM
- **Chunk size**: Fixed ~256-512 tokens per chunk
- **Tags per chunk**: 5 tags (no variation)
- **Documents per run**: sqrt(N) (distributes chunks naturally)

---

## 4. Implementation Details

### Benchmark Groups

#### Group 1: `dag_construction` - Core Benchmark

**Benchmarks:**
- `dag_construction/100`
- `dag_construction/1000`
- `dag_construction/5000`
- `dag_construction/10000`

**Configuration:**
```
Sample size: 10 runs per benchmark
Measurement time: 30 seconds per benchmark
Warmup: Yes (automatic)
Outlier filtering: Yes (automatic)
```

#### Group 2: `dag_scaling` - Scaling Validation

**Benchmarks:**
- `dag_scaling/5000`
- `dag_scaling/10000`
- `dag_scaling/20000`

**Configuration:**
```
Sample size: 5 runs per benchmark (slower)
Measurement time: 60 seconds per benchmark
Purpose: Detect non-linear scaling patterns
```

#### Group 3: `chunk_generation` - Data Gen Overhead

**Benchmarks:**
- `chunk_generation/100` through `chunk_generation/10000`

**Configuration:**
```
Sample size: 10 runs per benchmark
Purpose: Isolate data generation cost from DAG build cost
```

#### Group 4: `tag_generation` - Tag Gen Overhead

**Benchmarks:**
- `tag_generation/100` through `tag_generation/10000`

**Configuration:**
```
Sample size: 10 runs per benchmark
Purpose: Measure tag preparation cost separately
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
    b.iter(|| generate_test_chunks(n))
