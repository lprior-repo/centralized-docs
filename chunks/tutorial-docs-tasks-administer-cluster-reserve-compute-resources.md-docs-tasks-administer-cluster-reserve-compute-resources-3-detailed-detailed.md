---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Node Allocatable
token_count: 888
summary: ### System Reserved * **KubeletConfiguration Setting**: `systemReserved: {}`. Example value `{cpu: 100m, memory: 100Mi, ephemeral-storage: 1Gi, pid=1000}` * **KubeletConfiguration Setting**:...
---

### System Reserved
* **KubeletConfiguration Setting**: `systemReserved: {}`. Example value `{cpu: 100m, memory: 100Mi, ephemeral-storage: 1Gi, pid=1000}`
* **KubeletConfiguration Setting**: `systemReservedCgroup: ""`
`systemReserved` is meant to capture resource reservation for OS system daemons
like `sshd`, `udev`, etc. `systemReserved` should reserve `memory` for the
`kernel` too since `kernel` memory is not accounted to pods in Kubernetes at this time.
Reserving resources for user login sessions is also recommended (`user.slice` in
systemd world).
In addition to `cpu`, `memory`, and `ephemeral-storage`, `pid` may be
specified to reserve the specified number of process IDs for OS system
daemons.
To optionally enforce `systemReserved` on system daemons, specify the parent
control group for OS system daemons as the value for `systemReservedCgroup` setting,
and [add `system-reserved` to `enforceNodeAllocatable`](#enforcing-node-allocatable).
It is recommended that the OS system daemons are placed under a top level
control group (`system.slice` on systemd machines for example).
Note that `kubelet` **does not** create `systemReservedCgroup` if it doesn't
exist. `kubelet` will fail if an invalid cgroup is specified. With `systemd`
cgroup driver, you should follow a specific pattern for the name of the cgroup you
define: the name should be the value you set for `systemReservedCgroup`,
with `.slice` appended.
### Explicitly Reserved CPU List
FEATURE STATE:
`Kubernetes v1.17 [stable]`
**KubeletConfiguration Setting**: `reservedSystemCPUs:`. Example value `0-3`
`reservedSystemCPUs` is meant to define an explicit CPU set for OS system daemons and
kubernetes system daemons. `reservedSystemCPUs` is for systems that do not intend to
define separate top level cgroups for OS system daemons and kubernetes system daemons
with regard to cpuset resource.
If the Kubelet **does not** have `kubeReservedCgroup` and `systemReservedCgroup`,
the explicit cpuset provided by `reservedSystemCPUs` will take precedence over the CPUs
defined by `kubeReservedCgroup` and `systemReservedCgroup` options.
This option is specifically designed for Telco/NFV use cases where uncontrolled
interrupts/timers may impact the workload performance. you can use this option
to define the explicit cpuset for the system/kubernetes daemons as well as the
interrupts/timers, so the rest CPUs on the system can be used exclusively for
workloads, with less impact from uncontrolled interrupts/timers. To move the
system daemon, kubernetes daemons and interrupts/timers to the explicit cpuset
defined by this option, other mechanism outside Kubernetes should be used.
For example: in Centos, you can do this using the tuned toolset.
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