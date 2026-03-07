---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#4-summary
chunk_level: summary
chunk_type: prose
heading: System Overview
token_count: 77
summary: │  • Graph algorithms (petgraph)                                      │. │  • JSON serialization (serde)                                       │
---

│  • Graph algorithms (petgraph)                                      │
│  • JSON serialization (serde)                                       │
└──────────────┬─────────────────────────────────────┬────────────────┘
               │                                     │
               ▼                                     ▼
      ┌────────────────┐                  ┌────────────────┐
      │  File System   │                  │  External Libs │
      │  • Read files  │                  │  • petgraph    │
      │  • Write files │                  │  • regex       │
      │  • Walk dirs   │                  │  • serde       │
      └────────────────┘                  └────────────────┘
```

