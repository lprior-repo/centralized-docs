---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#7
chunk_level: summary
chunk_type: prose
heading: 3. Benchmark Groups
token_count: 90
summary: chunk_generation/5000. chunk_generation/10000
---



```
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

