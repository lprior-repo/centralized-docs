---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#1-detailed
chunk_level: detailed
chunk_type: prose
heading: System Overview
token_count: 492
summary: # Architecture Diagrams. Visual representations of the hexagonal architecture and data flow in `doc_transformer`
---

# Architecture Diagrams

Visual representations of the hexagonal architecture and data flow in `doc_transformer`.

## System Overview

```text
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                         DOC TRANSFORMER SYSTEM                       ┃
┃                      Hexagonal Architecture (Rust)                   ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

                                    │
                                    │ CLI Commands
                                    ▼
              ┌─────────────────────────────────────────┐
              │      PRESENTATION LAYER (main.rs)       │
              │  • Parse CLI args (clap)                │
              │  • Display results                      │
              │  • Manage async runtime (tokio)         │
              └──────────────┬──────────────────────────┘
                             │
                             │ Function Calls
                             ▼
┌────────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER (Core Logic)                   │
│                                                                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │transform │  │  chunk   │  │  index   │  │ validate │           │
│  │   .rs    │  │   .rs    │  │   .rs    │  │   .rs    │           │
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

