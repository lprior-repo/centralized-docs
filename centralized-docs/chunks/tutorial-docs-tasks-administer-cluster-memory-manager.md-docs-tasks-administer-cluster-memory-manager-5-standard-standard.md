---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#5-standard
chunk_level: standard
chunk_type: prose
heading: Memory Manager configuration
token_count: 470
summary: ### Policies Kubernetes' memory manager provides three policies. You can select a policy via the `memoryManagerPolicy` configuration field in the kubelet configuration; the values available in...
---

### Policies
Kubernetes' memory manager provides three policies. You can select a policy via the `memoryManagerPolicy` configuration field
in the kubelet configuration; the values available in Kubernetes 1.35 are:
* [`None`](#policy-none) (default)
* [`Static`](#policy-static) (Linux only)
* [`BestEffort`](#policy-best-effort) (Windows only)#### None policy
This is the default policy and does not affect the memory allocation in any way.
It acts the same as if the Memory Manager is not present at all.
The `None` policy returns default topology hint. This special hint denotes that Hint Provider
(Memory Manager in this case) has no preference for NUMA affinity with any resource.
#### Static policy
FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
**This policy is only supported on Linux.**
In the case of the `Guaranteed` pod, the `Static` Memory Manager policy returns topology hints
relating to the set of NUMA nodes where the memory can be guaranteed,
and reserves the memory through updating the internal [NodeMap][2] object.
In the case of the `BestEffort` or `Burstable` pod, the `Static` Memory Manager policy sends back
the default topology hint as there is no request for the guaranteed memory,
and does not reserve the memory in the internal [NodeMap][2] object.
This policy is only supported on Linux.
#### BestEffort policy
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
**This policy is only supported on Windows.**
On Windows, NUMA node assignment works differently than Linux.
There is no mechanism to ensure that Memory access only comes from a specific NUMA node.
Instead the Windows operating system scheduler selects the most optimal NUMA node based on the CPU(s) assignments.
It is possible that Windows might use other NUMA nodes if the Windows scheduler deems them optimal.
The policy does track the amount of memory available and requested through the internal *node map*.
The memory manager makes a best effort at ensuring that enough memory is available on a NUMA node before making
a resource assignment.
This means that in most cases memory assignment should function as specified.