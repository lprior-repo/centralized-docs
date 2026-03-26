---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#15-summary
chunk_level: summary
chunk_type: prose
heading: PriorityClass
token_count: 118
summary: ### Notes about PodPriority and existing clusters * If you upgrade an existing cluster without this feature, the priority of your existing Pods is effectively zero. * Addition of a PriorityClass with...
---

### Notes about PodPriority and existing clusters
* If you upgrade an existing cluster without this feature, the priority
of your existing Pods is effectively zero.
* Addition of a PriorityClass with `globalDefault` set to `true` does not
change the priorities of existing Pods. The value of such a PriorityClass is
used only for Pods created after the PriorityClass is added.
* If you delete a PriorityClass, existing Pods that use the name of the
deleted PriorityClass remain unchanged, but you cannot create more Pods that
use the name of the deleted PriorityClass.