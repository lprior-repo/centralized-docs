---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#13-summary
chunk_level: summary
chunk_type: prose
heading: PriorityClass
token_count: 126
summary: , and it cannot be prefixed with `system-`. A PriorityClass object can have any 32-bit integer value smaller than or equal to 1 billion. This means that the range of values for a PriorityClass object...
---

,
and it cannot be prefixed with `system-`.
A PriorityClass object can have any 32-bit integer value smaller than or equal
to 1 billion. This means that the range of values for a PriorityClass object is
from -2147483648 to 1000000000 inclusive. Larger numbers are reserved for
built-in PriorityClasses that represent critical system Pods. A cluster
admin should create one PriorityClass object for each such mapping that they want.
PriorityClass also has two optional fields: `globalDefault` and `description`.
The `globalDefault` field indicates that the value of this PriorityClass should