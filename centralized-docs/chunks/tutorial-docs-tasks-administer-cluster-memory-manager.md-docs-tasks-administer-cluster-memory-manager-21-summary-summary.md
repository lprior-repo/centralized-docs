---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#21-summary
chunk_level: summary
chunk_type: prose
heading: Memory Manager configuration
token_count: 124
summary: FEATURE STATE: `Kubernetes v1.32 [stable]`(enabled by default) **This policy is only supported on Linux.** In the case of the `Guaranteed` pod, the `Static` Memory Manager policy returns topology...
---

FEATURE STATE:
`Kubernetes v1.32 [stable]`(enabled by default)
**This policy is only supported on Linux.**
In the case of the `Guaranteed` pod, the `Static` Memory Manager policy returns topology hints
relating to the set of NUMA nodes where the memory can be guaranteed,
and reserves the memory through updating the internal [NodeMap][2] object.
In the case of the `BestEffort` or `Burstable` pod, the `Static` Memory Manager policy sends back
the default topology hint as there is no request for the guaranteed memory,