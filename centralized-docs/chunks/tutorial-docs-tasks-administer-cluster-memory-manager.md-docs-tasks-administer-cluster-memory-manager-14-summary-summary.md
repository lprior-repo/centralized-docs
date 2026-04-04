---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#14-summary
chunk_level: summary
chunk_type: prose
heading: How does the Memory Manager operate?
token_count: 121
summary: . The Memory Manager is a hint provider, and it provides topology hints for the Topology Manager which then aligns the requested resources according to these topology hints. On Linux, it also...
---

.
The Memory Manager is a hint provider, and it provides topology hints for
the Topology Manager which then aligns the requested resources according to these topology hints.
On Linux, it also enforces `cgroups` (specifically, `cpuset.mems`) for Pods.
The complete flow diagram concerning pod admission and deployment process is illustrated
below:
![Memory Manager in the pod admission and deployment process](/images/docs/memory-manager-diagram.svg)
During this process, the Memory Manager updates its internal counters stored in
[Node Map and Memory Maps][2] to manage guaranteed memory allocation.