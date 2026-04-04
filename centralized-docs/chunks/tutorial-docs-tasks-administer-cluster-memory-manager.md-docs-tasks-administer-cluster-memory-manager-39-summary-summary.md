---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#39-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 72
summary: #### Note: The default hard eviction threshold is 100MiB, and **not** zero. Remember to increase the quantity of memory that you reserve by setting `reservedMemory` by that hard eviction threshold....
---

#### Note:
The default hard eviction threshold is 100MiB, and **not** zero.
Remember to increase the quantity of memory that you reserve by setting `reservedMemory`
by that hard eviction threshold. Otherwise, the kubelet will not start Memory Manager and
display an error.
Here is an example of a correct configuration that uses `reservedMemory`: