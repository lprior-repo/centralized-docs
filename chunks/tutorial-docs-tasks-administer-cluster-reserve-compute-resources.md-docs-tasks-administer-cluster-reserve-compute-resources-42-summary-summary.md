---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#42-summary
chunk_level: summary
chunk_type: prose
heading: General Guidelines
token_count: 51
summary: ability to recover if any process in that group is oom-killed. Enforcing only compressible resources for `kubeReserved` and `systemReserved` is less likely to cause disruption while ensuring that the...
---

ability to recover if any process in that group is oom-killed.
Enforcing only compressible resources for `kubeReserved` and `systemReserved`
is less likely to cause disruption while ensuring that the resource is
allocated appropriately when there is contention.