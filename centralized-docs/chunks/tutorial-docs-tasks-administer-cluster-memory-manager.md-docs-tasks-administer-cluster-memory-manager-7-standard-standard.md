---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#7-standard
chunk_level: standard
chunk_type: prose
heading: Reserved memory configuration
token_count: 145
summary: ### Memory manager reserved memory syntax Here are some examples of how to set the `reservedMemory` configuration for the kubelet. ``` ` # Example 1 reservedMemory: - numaNode: 0 # NUMA node index...
---

### Memory manager reserved memory syntax
Here are some examples of how to set the `reservedMemory` configuration for the kubelet.
```
` # Example 1
reservedMemory:
- numaNode: 0 # NUMA node index
limits:
memory: "1Gi" # byte quantity
- numaNode: 1
limits:
memory: "2Gi" # byte quantity
`
```
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