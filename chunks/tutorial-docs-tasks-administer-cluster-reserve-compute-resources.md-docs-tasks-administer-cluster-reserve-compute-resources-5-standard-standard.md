---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#5-standard
chunk_level: standard
chunk_type: prose
heading: Node Allocatable
token_count: 300
summary: ### Explicitly Reserved CPU List FEATURE STATE: `Kubernetes v1.17 [stable]` **KubeletConfiguration Setting**: `reservedSystemCPUs:`. Example value `0-3` `reservedSystemCPUs` is meant to define an...
---

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