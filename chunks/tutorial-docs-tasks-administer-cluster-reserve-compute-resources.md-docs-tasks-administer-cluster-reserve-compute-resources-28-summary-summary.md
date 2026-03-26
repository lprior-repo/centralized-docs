---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#28-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 125
summary: FEATURE STATE: `Kubernetes v1.17 [stable]` **KubeletConfiguration Setting**: `reservedSystemCPUs:`. Example value `0-3` `reservedSystemCPUs` is meant to define an explicit CPU set for OS system...
---

FEATURE STATE:
`Kubernetes v1.17 [stable]`
**KubeletConfiguration Setting**: `reservedSystemCPUs:`. Example value `0-3`
`reservedSystemCPUs` is meant to define an explicit CPU set for OS system daemons and
kubernetes system daemons. `reservedSystemCPUs` is for systems that do not intend to
define separate top level cgroups for OS system daemons and kubernetes system daemons
with regard to cpuset resource.
If the Kubelet **does not** have `kubeReservedCgroup` and `systemReservedCgroup`,