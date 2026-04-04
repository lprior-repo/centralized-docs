---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#30-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 108
summary: You can omit this parameter, however, you should be aware that the quantity of reserved memory from all NUMA nodes should be equal to the quantity of *node allocatable* memory. If at least one node...
---

You can omit this parameter, however, you should be aware that the quantity of reserved memory
from all NUMA nodes should be equal to the quantity of *node allocatable* memory.
If at least one node allocatable parameter is non-zero, you will need to specify
`reservedMemory` for at least one NUMA node.
In fact, the `evictionHard` threshold value is equal to `100Mi` by default, so
if you use the `Static` policy, specifying `reservedMemory` is obligatory.