---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#3
chunk_level: standard
chunk_type: prose
heading: 3. Benchmark Groups
token_count: 472
summary: **What Added:**. - Criterion framework with HTML report generation
---

[[bench]]
```

**What Added:**
- Criterion framework with HTML report generation
- Benchmark harness configuration (criterion runs, not libtest)

---

### 2. Benchmark Suite

**File:** `/home/lewis/src/centralized-docs/doc_transformer/benches/graph_bench.rs`

**Stats:**
- 254 lines of Rust code
- 4 benchmark groups
- 16 individual benchmarks
- 3 data generator functions
- 100% deterministic test data

---

## 3. Benchmark Groups

### Group 1: `dag_construction` (Primary)

Measures core DAG building performance across scales:

```
dag_construction/100   -> ~50-200ms   (baseline)
dag_construction/1000  -> ~200-1000ms (5-10x)
dag_construction/5000  -> ~1-5s       (10-25x)
dag_construction/10000 -> ~5-20s      (25-100x)
```

**What's Measured:**
- HNSW index creation time
- K-nearest neighbor queries
- Edge insertion into DAG

**Configuration:**
- Sample size: 10 runs per benchmark
- Measurement time: 30 seconds per benchmark

---

### Group 2: `dag_scaling` (Validation)

Detects non-linear scaling by testing larger datasets:

```
dag_scaling/5000   -> Time(5K)
dag_scaling/10000  -> Time(10K)
dag_scaling/20000  -> Time(20K)
```

**Scaling Proof:**
- If Time(20K) / Time(10K) ≈ 2.0-2.3x → O(n log n) ✓
- If Time(20K) / Time(10K) ≈ 4.0-5.0x → O(n²) detected ✗

**Configuration:**
- Sample size: 5 runs per benchmark (slower)
- Measurement time: 60 seconds per benchmark

---

### Group 3: `chunk_generation` (Overhead Analysis)

Isolates data generation cost:

```
chunk_generation/100
chunk_generation/1000
chunk_generation/5000
chunk_generation/10000
```

**Purpose:** Verify data gen is < 5% of total benchmark time

---

### Group 4: `tag_generation` (Overhead Analysis)

Measures tag creation overhead:

```
tag_generation/100
tag_generation/1000
tag_generation/5000
tag_generation/10000
```

**Purpose:** Verify tag prep is < 1% of total benchmark time

---

