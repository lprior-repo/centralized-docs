---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 188
summary: # Control Memory Management Policies on a Node FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages)...
---

# Control Memory Management Policies on a Node
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
The Kubernetes *Memory Manager* enables the feature of guaranteed memory (and hugepages)
allocation for pods in the `Guaranteed` [QoS class](/docs/concepts/workloads/pods/pod-qos/).
The Memory Manager employs a hint generation protocol to yield the most suitable NUMA affinity for a pod.
The Memory Manager feeds the central manager (*Topology Manager*) with these affinity hints.
Based on both the hints and Topology Manager policy, the pod is rejected or admitted to the node.
Moreover, the Memory Manager ensures that the memory which a pod requests
is allocated from a minimum number of NUMA nodes.
For background about memory resources for Pods, read
[Assign Memory Resources to Containers and Pods](/docs/tasks/configure-pod-container/assign-memory-resource/).