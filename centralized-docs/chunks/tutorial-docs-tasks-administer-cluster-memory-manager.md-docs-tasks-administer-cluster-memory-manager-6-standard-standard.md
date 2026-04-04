---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#6-standard
chunk_level: standard
chunk_type: prose
heading: Reserved memory configuration
token_count: 397
summary: ## Reserved memory configuration As an administrator, you can configure the total amount of reserved memory for a node. This pre-configured value is subsequently utilized to calculate the real amount...
---

## Reserved memory configuration
As an administrator, you can configure the total amount of reserved memory
for a node. This pre-configured value is subsequently utilized to calculate
the real amount of [node allocatable](/docs/tasks/administer-cluster/reserve-compute-resources/#node-allocatable) memory available to pods.
The Kubernetes scheduler incorporates allocatable memory information to optimise pod
[scheduling](/docs/concepts/scheduling-eviction/).
. The *node allocatable* mechanism is commonly used by node administrators to reserve K8s node
system resources for the kubelet or operating system processes to help assure node stability.
The relevant kubelet settings include `kubeReserved`, `systemReserved` and `reservedMemory`.
The `reservedMemory` setting allows you to split the total reserved memory and assign it
across many NUMA nodes.
You specify a comma-separated list of memory reservations, of different
memory types, per NUMA node.
You can also specify reservations that span multiple NUMA nodes, using a semicolon as separator.
The Memory Manager will not use this reserved memory for running container workloads.
For example, if you have a NUMA node "NUMA0" with 10GiB of memory available, and
you configure `reservedMemory` to reserve `1Gi` (of memory) for NUMA0,
the Memory Manager assumes that only 9GiB is available for pods.
You can omit this parameter, however, you should be aware that the quantity of reserved memory
from all NUMA nodes should be equal to the quantity of *node allocatable* memory.
If at least one node allocatable parameter is non-zero, you will need to specify
`reservedMemory` for at least one NUMA node.
In fact, the `evictionHard` threshold value is equal to `100Mi` by default, so
if you use the `Static` policy, specifying `reservedMemory` is obligatory.