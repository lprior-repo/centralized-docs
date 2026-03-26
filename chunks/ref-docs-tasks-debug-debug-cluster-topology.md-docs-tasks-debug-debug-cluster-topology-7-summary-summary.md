---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#7-summary
chunk_level: summary
chunk_type: prose
heading: Sources of troubleshooting information
token_count: 52
summary: Use `kubectl describe pod &lt;id&gt;` or `kubectl events` to obtain a detailed error message: ``` `Warning TopologyAffinityError 10m kubelet, dell8 Resources cannot be allocated with Topology...
---

Use `kubectl describe pod &lt;id&gt;` or `kubectl events` to obtain a detailed error message:
```
`Warning TopologyAffinityError 10m kubelet, dell8 Resources cannot be allocated with Topology locality
`
```