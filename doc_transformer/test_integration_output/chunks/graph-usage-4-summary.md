---
doc_id: graph-usage
chunk_id: graph-usage#4
chunk_level: summary
chunk_type: prose
heading: Usage
token_count: 130
summary: ### Options. - `--reachable`: Show count of nodes reachable from this node (transitive closure)
---



```bash
```

### Options

  

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
