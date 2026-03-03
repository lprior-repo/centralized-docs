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

## Data Flow Pipeline

```text
INPUT                    PIPELINE STAGES                    OUTPUT
═════                    ═══════════════                    ══════

Source                        ┏━━━━━━━━━━━━━━━━━┓
Directory ───────────────────►┃ 1. DISCOVER     ┃
(*.md, *.mdx)                 ┃  discover.rs    ┃
                              ┗━━━━━━━┬━━━━━━━━━┛
                                      │
                          Vec<DiscoveryFile>
                                      │
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 2. ANALYZE      ┃
                              ┃  analyze.rs     ┃──► Extract:
                              ┗━━━━━━━┬━━━━━━━━━┛    • Title
                                      │              • Headings
                              Vec<Analysis>          • Links
                                      │              • Category
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 3. ASSIGN IDs   ┃
                              ┃  assign.rs      ┃──► Generate:
                              ┗━━━━━━━┬━━━━━━━━━┛    • SHA256 IDs
                                      │              • Filenames
                        (Vec<Analysis>, LinkMap)     • Link map
                                      │
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 4. TRANSFORM    ┃
                              ┃  transform.rs   ┃──► Rewrite:
                              ┗━━━━━━━┬━━━━━━━━━┛    • Headings
                                      │              • Links
                           TransformResult           • Frontmatter
                                      │
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 5. CHUNK        ┃
                              ┃  chunk.rs       ┃──► Split on:
                              ┗━━━━━━━┬━━━━━━━━━┛    • H2 boundaries
                                      │              • ~170 tokens
                            ChunksResult             • Add context
                                      │
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 6. INDEX        ┃
                              ┃  index.rs       ┃──► Build:
                              ┗━━━━━━━┬━━━━━━━━━┛    • INDEX.json
                                      │              • COMPASS.md
                                 Index Data          • Graph (DAG)
                                      │
                              ┏━━━━━━━▼━━━━━━━━━┓
                              ┃ 7. VALIDATE     ┃
                              ┃  validate.rs    ┃──► Check:     Indexed
                              ┗━━━━━━━┬━━━━━━━━━┛    • Frontmatter    Docs
                                      │              • Headings       ──────►
                          ValidationResult           • Links       output_dir/
                                      │                              ├─ docs/
                                      ▼                              ├─ chunks/
                                   SUCCESS                           ├─ INDEX.json
                                                                     └─ COMPASS.md
```
