---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#36-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 126
summary: **KubeletConfiguration setting**: `enforceNodeAllocatable: [pods]`. Example value: `[pods,system-reserved,kube-reserved]` The scheduler treats 'Allocatable' as the available `capacity` for pods....
---

**KubeletConfiguration setting**: `enforceNodeAllocatable: [pods]`. Example value: `[pods,system-reserved,kube-reserved]`
The scheduler treats 'Allocatable' as the available `capacity` for pods.
`kubelet` enforce 'Allocatable' across pods by default. Enforcement is performed
by evicting pods whenever the overall usage across all pods exceeds
'Allocatable'. More details on eviction policy can be found
on the [node pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
page. This enforcement is controlled by
specifying