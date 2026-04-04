---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 88
summary: *Topology Manager*) with these affinity hints. Based on both the hints and Topology Manager policy, the pod is rejected or admitted to the node. Moreover, the Memory Manager ensures that the memory...
---

*Topology Manager*) with these affinity hints.
Based on both the hints and Topology Manager policy, the pod is rejected or admitted to the node.
Moreover, the Memory Manager ensures that the memory which a pod requests
is allocated from a minimum number of NUMA nodes.
For background about memory resources for Pods, read
[Assign Memory Resources to Containers and Pods](/docs/tasks/configure-pod-container/assign-memory-resource/).