---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#5-summary
chunk_level: summary
chunk_type: prose
heading: Data Flow Pipeline
token_count: 137
summary: └──────────────┬─────────────────────────────────────┬────────────────┘.       │  File System   │                  │  External Libs │
---

└──────────────┬─────────────────────────────────────┬────────────────┘
      │  File System   │                  │  External Libs │
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
