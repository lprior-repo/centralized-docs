---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#27-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 123
summary: As an administrator, you can configure the total amount of reserved memory for a node. This pre-configured value is subsequently utilized to calculate the real amount of [node...
---

As an administrator, you can configure the total amount of reserved memory
for a node. This pre-configured value is subsequently utilized to calculate
the real amount of [node allocatable](/docs/tasks/administer-cluster/reserve-compute-resources/#node-allocatable) memory available to pods.
The Kubernetes scheduler incorporates allocatable memory information to optimise pod
[scheduling](/docs/concepts/scheduling-eviction/).
. The *node allocatable* mechanism is commonly used by node administrators to reserve K8s node
system resources for the kubelet or operating system processes to help assure node stability.