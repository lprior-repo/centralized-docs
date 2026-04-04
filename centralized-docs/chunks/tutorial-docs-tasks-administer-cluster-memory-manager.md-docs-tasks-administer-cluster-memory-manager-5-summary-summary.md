---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 121
summary: FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages) allocation for pods in the `Guaranteed` [QoS...
---

FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages)
allocation for pods in the `Guaranteed` [QoS class](/docs/concepts/workloads/pods/pod-qos/).
The Memory Manager employs a hint generation protocol to yield the most suitable NUMA affinity for a pod.
The Memory Manager feeds the central manager (*Topology Manager*) with these affinity hints.
Based on both the hints and Topology Manager policy, the pod is rejected or admitted to the node.