---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#24-summary
chunk_level: summary
chunk_type: prose
heading: Memory Manager configuration
token_count: 117
summary: FEATURE STATE: `Kubernetes v1.32 [alpha]`(disabled by default) **This policy is only supported on Windows.** On Windows, NUMA node assignment works differently than Linux. There is no mechanism to...
---

FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
**This policy is only supported on Windows.**
On Windows, NUMA node assignment works differently than Linux.
There is no mechanism to ensure that Memory access only comes from a specific NUMA node.
Instead the Windows operating system scheduler selects the most optimal NUMA node based on the CPU(s) assignments.
It is possible that Windows might use other NUMA nodes if the Windows scheduler deems them optimal.
The policy does track the amount of memory available and requested through the internal *node map*.