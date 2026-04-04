---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#9-standard
chunk_level: standard
chunk_type: prose
heading: Reserved memory configuration
token_count: 173
summary: #### Note: The default hard eviction threshold is 100MiB, and **not** zero. Remember to increase the quantity of memory that you reserve by setting `reservedMemory` by that hard eviction threshold....
---

#### Note:
The default hard eviction threshold is 100MiB, and **not** zero.
Remember to increase the quantity of memory that you reserve by setting `reservedMemory`
by that hard eviction threshold. Otherwise, the kubelet will not start Memory Manager and
display an error.
Here is an example of a correct configuration that uses `reservedMemory`:
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