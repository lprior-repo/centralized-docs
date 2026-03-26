---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#18-summary
chunk_level: summary
chunk_type: prose
heading: Examine system logs
token_count: 128
summary: Pinned term means that pod's memory consumption is constrained (through `cgroups` configuration) to these NUMA nodes. This automatically implies that Memory Manager instantiated a new group that...
---

Pinned term means that pod's memory consumption is constrained (through `cgroups` configuration)
to these NUMA nodes.
This automatically implies that Memory Manager instantiated a new group that
comprises these two NUMA nodes, i.e. `0` and `1` indexed NUMA nodes.
In order to analyse memory resources available in a group,the corresponding entries from
NUMA nodes belonging to the group must be added up.
For example, the total amount of free "conventional" memory in the group can be computed
by adding up the free memory available at every NUMA node in the group,
i.e., in the