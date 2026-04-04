---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#15-summary
chunk_level: summary
chunk_type: prose
heading: How does the Memory Manager operate?
token_count: 117
summary: During this process, the Memory Manager updates its internal counters stored in [Node Map and Memory Maps][2] to manage guaranteed memory allocation. The memory manager activates during kubelet...
---

During this process, the Memory Manager updates its internal counters stored in
[Node Map and Memory Maps][2] to manage guaranteed memory allocation.
The memory manager activates during kubelet startup if a node administrator configures
`reservedMemory` for the kubelet (section [Reserved memory configuration](#reserved-memory-flag)).
In this case, the kubelet updates its node map to reflect this reservation.
When the `Static` policy is configured, you **must** configure reserved memory for the node
(for example, with the `reservedMemory` configuration field in the kubelet configuration).