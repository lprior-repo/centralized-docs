---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#6-standard
chunk_level: standard
chunk_type: prose
heading: Node Allocatable
token_count: 498
summary: ### Eviction Thresholds **KubeletConfiguration Setting**: `evictionHard: {memory.available: \"100Mi\", nodefs.available: \"10%\", nodefs.inodesFree: \"5%\", imagefs.available: \"15%\"}`. Example value:...
---

### Eviction Thresholds
**KubeletConfiguration Setting**: `evictionHard: {memory.available: "100Mi", nodefs.available: "10%", nodefs.inodesFree: "5%", imagefs.available: "15%"}`. Example value: `{memory.available: "&lt;500Mi"}`
Memory pressure at the node level leads to System OOMs which affects the entire
node and all pods running on it. Nodes can go offline temporarily until memory
has been reclaimed. To avoid (or reduce the probability of) system OOMs kubelet
provides [out of resource](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
management. Evictions are
supported for `memory` and `ephemeral-storage` only. By reserving some memory via
`evictionHard` setting, the `kubelet` attempts to evict pods whenever memory
availability on the node drops below the reserved value. Hypothetically, if
system daemons did not exist on a node, pods cannot use more than `capacity - eviction-hard`. For this reason, resources reserved for evictions are not
available for pods.
### Enforcing Node Allocatable
**KubeletConfiguration setting**: `enforceNodeAllocatable: [pods]`. Example value: `[pods,system-reserved,kube-reserved]`
The scheduler treats 'Allocatable' as the available `capacity` for pods.
`kubelet` enforce 'Allocatable' across pods by default. Enforcement is performed
by evicting pods whenever the overall usage across all pods exceeds
'Allocatable'. More details on eviction policy can be found
on the [node pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
page. This enforcement is controlled by
specifying `pods` value to the KubeletConfiguration setting `enforceNodeAllocatable`.
Optionally, `kubelet` can be made to enforce `kubeReserved` and
`systemReserved` by specifying `kube-reserved` &amp; `system-reserved` values in
the same setting. Additionally, only compressible resources may be enforced by
specifying `kube-reserved-compressible` and `system-reserved-compressible`.
Note that to enforce `kubeReserved` or `systemReserved`,
`kubeReservedCgroup` or `systemReservedCgroup` needs to be specified
respectively.