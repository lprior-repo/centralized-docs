---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#2-summary
chunk_level: summary
chunk_type: prose
heading: System Overview
token_count: 129
summary: │  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │. │                                                                      │
---

│  └──────────┘  └──────────┘  └──────────┘  └──────────┘           │
│                                                                      │
│  Characteristics:                                                    │
│  • Pure functions (deterministic)                                   │
│  • Immutable data structures                                        │
│  • Result<T, E> for error handling                                  │
│  • No side effects (delegates I/O)                                  │
└──────────────┬─────────────────────────────────────┬────────────────┘
               │                                     │
               │ Uses Data Types                     │ Calls Functions
               ▼                                     ▼
┌────────────────────────────────────────────────────────────────────┐
│                   PORTS LAYER (Contracts)                           │
│                                                                      │
│  Data Structures:          Function Signatures:                     │
│  • Analysis                • discover_files()                       │
│  • Chunk                   • analyze_files()                        │
│  • GraphNode               • chunk_all()                            │
│  • IndexDocument           • validate_all()                         │
│                                                                      │
│  Result Types:                                                       │
