---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#16-summary
chunk_level: summary
chunk_type: prose
heading: How does the Memory Manager operate?
token_count: 85
summary: **must** configure reserved memory for the node (for example, with the `reservedMemory` configuration field in the kubelet configuration). An important topic in the context of Memory Manager...
---

**must** configure reserved memory for the node
(for example, with the `reservedMemory` configuration field in the kubelet configuration).
An important topic in the context of Memory Manager operation is the management of NUMA groups.
Each time pod's memory request is in excess of single NUMA node capacity, the Memory Manager
attempts to create a group that comprises several NUMA nodes and that features extended memory
capacity.