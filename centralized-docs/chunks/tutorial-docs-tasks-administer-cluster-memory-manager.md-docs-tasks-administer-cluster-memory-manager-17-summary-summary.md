---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#17-summary
chunk_level: summary
chunk_type: prose
heading: Memory Manager configuration
token_count: 102
summary: ## Memory Manager configuration Other Managers should already be configured (see [resource alignment prerequisites](#resource-alignment-prerequisites). Set the `memoryManagerPolicy` configuration...
---

## Memory Manager configuration
Other Managers should already be configured (see [resource alignment prerequisites](#resource-alignment-prerequisites).
Set the `memoryManagerPolicy` configuration field within the [kubelet configuration](/docs/reference/config-api/kubelet-config.v1beta1/), to the name of your chosen [policy](#policies).
Optionally, some amount of memory can be reserved for system or kubelet processes to increase
node stability (section [Reserved memory configuration](#reserved-memory-flag)).