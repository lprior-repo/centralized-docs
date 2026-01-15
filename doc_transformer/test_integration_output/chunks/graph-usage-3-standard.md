---
doc_id: graph-usage
chunk_id: graph-usage#3
chunk_level: standard
chunk_type: prose
heading: Usage
token_count: 518
summary: - **Documents** as nodes. - **Relationships** as weighted edges between nodes
---

- **Documents** as nodes
- **Relationships** as weighted edges between nodes

## Edge Types

- **Parent**: Document contains chunk (weight: 1.0)
- **Sequential**: Next chunk in document order (weight: 1.0)
- **Related**: Semantically similar content (weight: 0.3-1.0, based on Jaccard similarity)
- **References**: Explicit cross-references in documentation
- **ReferencedBy**: Backlinks from other documents

## Usage

### Basic Command

```bash
doc_transformer graph <NODE_ID> [OPTIONS]
```

### Options

- `<NODE_ID>`: The ID of the node to explore (required)
  - Document IDs: e.g., `"tutorial/general/getting-started"`
  - Chunk IDs: e.g., `"getting-started#0"`, `"doc-id#1"`
  
- `-i, --index-dir <DIR>`: Directory containing INDEX.json (default: current directory)

- `--reachable`: Show count of nodes reachable from this node (transitive closure)

### Examples

#### 1. Explore a Document Node

```bash
doc_transformer graph "tutorial/general/sample" --index-dir test_output
```

Output:
```
======================================================================
KNOWLEDGE GRAPH: tutorial/general/sample
======================================================================

Node: tutorial/general/sample (Document)
Title: Sample

Outgoing Edges: None

Incoming Edges: None

No relationships found

======================================================================
```

#### 2. Explore a Chunk Node

```bash
doc_transformer graph "sample#0" --index-dir test_output
```

Output:
```
======================================================================
KNOWLEDGE GRAPH: sample#0
======================================================================

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
