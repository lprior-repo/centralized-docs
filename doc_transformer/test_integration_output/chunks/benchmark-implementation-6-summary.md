---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#6
chunk_level: summary
chunk_type: prose
heading: 3. Benchmark Groups
token_count: 136
summary: dag_scaling/20000  -> Time(20K). **Scaling Proof:**
---



```
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
