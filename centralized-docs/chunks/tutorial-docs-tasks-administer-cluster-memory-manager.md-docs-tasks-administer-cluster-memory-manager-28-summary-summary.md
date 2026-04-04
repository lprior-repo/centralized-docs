---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#28-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 114
summary: mechanism is commonly used by node administrators to reserve K8s node system resources for the kubelet or operating system processes to help assure node stability. The relevant kubelet settings...
---

 mechanism is commonly used by node administrators to reserve K8s node
system resources for the kubelet or operating system processes to help assure node stability.
The relevant kubelet settings include `kubeReserved`, `systemReserved` and `reservedMemory`.
The `reservedMemory` setting allows you to split the total reserved memory and assign it
across many NUMA nodes.
You specify a comma-separated list of memory reservations, of different
memory types, per NUMA node.
You can also specify reservations that span multiple NUMA nodes, using a semicolon as separator.