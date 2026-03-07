---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Data Flow Pipeline
token_count: 448
summary: ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓. ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
---




┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

┌────────────────────────────────────────────────────────────────────┐
└──────────────┬─────────────────────────────────────┬────────────────┘
┌────────────────────────────────────────────────────────────────────┐
└──────────────┬─────────────────────────────────────┬────────────────┘
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
