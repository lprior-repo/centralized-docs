---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#5-summary
chunk_level: summary
chunk_type: prose
heading: Sources of troubleshooting information
token_count: 119
summary: * *Pod status* - indicates topology affinity errors * *system logs* - include valuable information for debugging; for example, about generated hints * *kubelet state file* - the dump of internal...
---

* *Pod status* - indicates topology affinity errors
* *system logs* - include valuable information for debugging; for example, about generated hints
* *kubelet state file* - the dump of internal state of the Memory Manager
(including the *node map* and *memory maps*)
* You can use the [device plugin resource API](#device-plugin-resource-api)
to retrieve information about the memory reserved for containers## Troubleshoot `TopologyAffinityError`
This error typically occurs in the following situations:
* a node has not enough resources available to satisfy the pod's request