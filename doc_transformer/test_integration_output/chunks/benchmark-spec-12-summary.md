---
doc_id: benchmark-spec
chunk_id: benchmark-spec#12
chunk_level: summary
chunk_type: prose
heading: 4. Implementation Details
token_count: 128
summary: Warmup: Yes (automatic). Outlier filtering: Yes (automatic)
---



```
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
