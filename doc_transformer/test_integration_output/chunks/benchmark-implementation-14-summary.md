---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#14
chunk_level: summary
chunk_type: prose
heading: 5. Benchmark Execution Flow
token_count: 137
summary:  Run benchmark_chunk_generation           │. │    └─ Measure allocate_chunks time          │
---

                    ↓
│ 2. Run benchmark_chunk_generation           │
│    └─ Measure allocate_chunks time          │
                    ↓
┌─────────────────────────────────────────────┐
│ 3. Run benchmark_tag_generation             │
│    └─ Measure create_tags time              │
└─────────────────────────────────────────────┘
