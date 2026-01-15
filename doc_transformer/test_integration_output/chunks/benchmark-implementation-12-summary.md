---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#12
chunk_level: summary
chunk_type: prose
heading: 5. Benchmark Execution Flow
token_count: 145
summary: ### Data Properties. All generators produce:
---

### Data Properties

All generators produce:
- **No false optimization** (data gen cannot be inlined/optimized away)

---

## 5. Benchmark Execution Flow

### When Running `cargo bench`

```
┌─────────────────────────────────────────────┐
│ 1. Initialize Criterion framework           │
│    └─ Create target/criterion/ directories  │
└─────────────────────────────────────────────┘
