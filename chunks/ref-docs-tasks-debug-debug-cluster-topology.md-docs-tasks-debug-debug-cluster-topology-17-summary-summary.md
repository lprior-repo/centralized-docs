---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#17-summary
chunk_level: summary
chunk_type: prose
heading: Examine system logs
token_count: 41
summary: It can be deduced from the state file that the pod was pinned to both NUMA nodes, i.e.: ``` `\"numaAffinity\":[ 0, 1 ], ` ```
---

It can be deduced from the state file that the pod was pinned to both NUMA nodes, i.e.:
```
`"numaAffinity":[
0,
1
],
`
```