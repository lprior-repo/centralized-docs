---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#19-summary
chunk_level: summary
chunk_type: prose
heading: Examine system logs
token_count: 128
summary: \" memory in the group can be computed by adding up the free memory available at every NUMA node in the group, i.e., in the `\"memory\"` section of NUMA node `0` (`\"free\":0`) and NUMA node `1`...
---

" memory in the group can be computed
by adding up the free memory available at every NUMA node in the group,
i.e., in the `"memory"` section of NUMA node `0` (`"free":0`) and NUMA node `1` (`"free":103739236352`).
So, the total amount of free "conventional" memory in this group is equal to `0 + 103739236352` bytes.
The line `"systemReserved":3221225472` indicates that the administrator of this node reserved
`3221225472` bytes (i.e. `3Gi`