---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#40-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 100
summary: ``` ` # this snippet relies on the default value of evictionHard memoryManagerPolicy: Static kubeReserved: { cpu: \"4\", memory: \"4Gi\" } systemReserved: { cpu: \"1\", memory: \"1Gi\" } reservedMemory: -...
---

```
` # this snippet relies on the default value of evictionHard
memoryManagerPolicy: Static
kubeReserved: { cpu: "4", memory: "4Gi" }
systemReserved: { cpu: "1", memory: "1Gi" }
reservedMemory:
- numaNode: 0
limits:
memory: "3Gi"
- numaNode: 1
limits:
memory: "2148Mi" # 3GiB minus 100MiB
`
```