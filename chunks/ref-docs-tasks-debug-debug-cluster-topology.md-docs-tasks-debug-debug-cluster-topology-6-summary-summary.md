---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#6-summary
chunk_level: summary
chunk_type: prose
heading: Sources of troubleshooting information
token_count: 107
summary: * a node has not enough resources available to satisfy the pod's request * the pod's request is rejected due to particular Topology Manager policy constraints The error appears in the status of a...
---

* a node has not enough resources available to satisfy the pod's request
* the pod's request is rejected due to particular Topology Manager policy constraints
The error appears in the status of a pod:
```
`kubectl get pods
`
```
```
`NAME READY STATUS RESTARTS AGE
guaranteed 0/1 TopologyAffinityError 0 113s
`
```
Use `kubectl describe pod &lt;id&gt;` or `kubectl events` to obtain a detailed error message: