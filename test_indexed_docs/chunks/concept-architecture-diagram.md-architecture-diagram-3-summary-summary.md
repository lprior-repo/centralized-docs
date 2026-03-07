---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#3-summary
chunk_level: summary
chunk_type: prose
heading: System Overview
token_count: 136
summary: │  • Result<T, anyhow::Error>                                         │. │  • Option<T>                                                         │
---

│  • Result<T, anyhow::Error>                                         │
│  • Option<T>                                                         │
└──────────────┬─────────────────────────────────────┬────────────────┘
               │                                     │
               │ Implements                          │ Provides Data
               ▼                                     ▼
┌────────────────────────────────────────────────────────────────────┐
│                   ADAPTERS LAYER (External I/O)                     │
│                                                                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │ discover │  │ analyze  │  │  graph   │  │  assign  │           │
│  │   .rs    │  │   .rs    │  │   .rs    │  │   .rs    │           │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │
│                                                                      │
│  Responsibilities:                                                   │
│  • File system I/O (walkdir, std::fs)                               │
│  • Regex matching (lazy statics)                                    │
