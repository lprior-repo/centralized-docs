---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#32-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 62
summary: ``` ` # Example 2 reservedMemory: - numaNode: 0 limits: \"memory\": \"512Gi\" - numaNode: 1 limits: \"memory\": \"512Gi\" \"hugepages-1Gi\": \"2Gi\" # only relevant on Linux ` ```
---

```
` # Example 2
reservedMemory:
- numaNode: 0
limits:
"memory": "512Gi"
- numaNode: 1
limits:
"memory": "512Gi"
"hugepages-1Gi": "2Gi" # only relevant on Linux
`
```