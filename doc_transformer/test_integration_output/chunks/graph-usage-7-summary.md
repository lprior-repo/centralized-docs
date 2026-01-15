---
doc_id: graph-usage
chunk_id: graph-usage#7
chunk_level: summary
chunk_type: prose
heading: Usage
token_count: 128
summary:      Sample - Installation.      Sample - Writing Your First Program
---

     Sample - Installation
     Sample - Writing Your First Program

Incoming Edges (3):
  ← sample#1 [Related, weight: 1.00]
     Sample - Installation
  ← sample#2 [Related, weight: 1.00]
     Sample - Basic Concepts
  ← sample#3 [Related, weight: 1.00]
     Sample - Writing Your First Program

======================================================================
```

#### 3. Show Reachable Nodes

```bash
doc_transformer graph "sample#0" --index-dir test_output --reachable
```

Output includes:
```
