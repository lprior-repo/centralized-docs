---
doc_id: concept/architecture-diagram.md/architecture-diagram
chunk_id: concept/architecture-diagram.md/architecture-diagram#6-summary
chunk_level: summary
chunk_type: prose
heading: Data Flow Pipeline
token_count: 128
summary:                               ┏━━━━━━━▼━━━━━━━━━┓.  TRANSFORM    ┃
---

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
