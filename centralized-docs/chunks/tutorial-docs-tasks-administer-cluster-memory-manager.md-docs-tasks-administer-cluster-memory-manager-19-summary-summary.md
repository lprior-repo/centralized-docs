---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#19-summary
chunk_level: summary
chunk_type: prose
heading: Memory Manager configuration
token_count: 110
summary: * [`None`](#policy-none) (default) * [`Static`](#policy-static) (Linux only) * [`BestEffort`](#policy-best-effort) (Windows only)#### None policy This is the default policy and does not affect the...
---

* [`None`](#policy-none) (default)
* [`Static`](#policy-static) (Linux only)
* [`BestEffort`](#policy-best-effort) (Windows only)#### None policy
This is the default policy and does not affect the memory allocation in any way.
It acts the same as if the Memory Manager is not present at all.
The `None` policy returns default topology hint. This special hint denotes that Hint Provider
(Memory Manager in this case) has no preference for NUMA affinity with any resource.