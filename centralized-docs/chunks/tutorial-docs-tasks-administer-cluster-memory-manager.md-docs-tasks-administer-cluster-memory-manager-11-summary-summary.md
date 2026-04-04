---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: * the Topology Manager should be enabled and proper Topology Manager policy should be configured on a Node. See [control Topology Management...
---

* the Topology Manager should be enabled and proper Topology Manager policy should be configured on a Node.
See [control Topology Management Policies](/docs/tasks/administer-cluster/topology-manager/).### Windows support
FEATURE STATE:
`Kubernetes v1.32 [alpha]`(disabled by default)
Windows support can be enabled via the `WindowsCPUAndMemoryAffinity` feature gate
and it requires support in the container runtime.
Only the [None](#policy-none) and [BestEffort](#policy-best-effort) policies are supported on Windows.