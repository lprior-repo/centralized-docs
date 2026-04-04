---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#29-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 122
summary: memory types, per NUMA node. You can also specify reservations that span multiple NUMA nodes, using a semicolon as separator. The Memory Manager will not use this reserved memory for running...
---

memory types, per NUMA node.
You can also specify reservations that span multiple NUMA nodes, using a semicolon as separator.
The Memory Manager will not use this reserved memory for running container workloads.
For example, if you have a NUMA node "NUMA0" with 10GiB of memory available, and
you configure `reservedMemory` to reserve `1Gi` (of memory) for NUMA0,
the Memory Manager assumes that only 9GiB is available for pods.
You can omit this parameter, however, you should be aware that the quantity of reserved memory