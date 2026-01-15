---
doc_id: graph-usage
chunk_id: graph-usage#4
chunk_level: standard
chunk_type: prose
heading: Usage
token_count: 271
summary: Node: sample#0 (Chunk). Title: Sample - Intro
---

```

Output:
```

Node: sample#0 (Chunk)
Title: Sample - Intro

Outgoing Edges (4):
  → sample#1 [Sequential, weight: 1.00]
     Sample - Installation
  → sample#1 [Related, weight: 1.00]
     Sample - Installation
  → sample#2 [Related, weight: 1.00]
     Sample - Basic Concepts
  → sample#3 [Related, weight: 1.00]
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
Reachable: 12 nodes
```

This shows how many nodes can be reached by following edges from the starting node.

#### 4. Explore with Chunk ID containing #

```bash
doc_transformer graph "doc-id#0" --index-dir output
```

Chunk IDs with `#` characters are properly handled.

