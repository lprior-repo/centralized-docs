---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#25-summary
chunk_level: summary
chunk_type: prose
heading: Memory Manager configuration
token_count: 58
summary: The policy does track the amount of memory available and requested through the internal *node map*. The memory manager makes a best effort at ensuring that enough memory is available on a NUMA node...
---

The policy does track the amount of memory available and requested through the internal *node map*.
The memory manager makes a best effort at ensuring that enough memory is available on a NUMA node before making
a resource assignment.
This means that in most cases memory assignment should function as specified.