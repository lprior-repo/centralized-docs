---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#34-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 49
summary: When you specify values for `reservedMemory`, this must be compatible with the `kubeReserved` and `systemReserved` values that are in effect, along with any `memory.available` setting you make as...
---

When you specify values for `reservedMemory`, this must be compatible with the `kubeReserved`
and `systemReserved` values that are in effect, along with any `memory.available` setting
you make as part of `evictionHard`.