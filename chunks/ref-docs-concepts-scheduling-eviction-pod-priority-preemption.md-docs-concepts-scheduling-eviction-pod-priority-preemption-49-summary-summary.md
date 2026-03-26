---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#49-summary
chunk_level: summary
chunk_type: prose
heading: Troubleshooting
token_count: 120
summary: When pods are preempted, they receive their requested graceful termination period, which is by default 30 seconds. If the victim Pods do not terminate within this period, they are forcibly...
---

When pods are preempted, they receive their requested graceful termination
period, which is by default 30 seconds. If the victim Pods do not terminate within
this period, they are forcibly terminated. Once all the victims go away, the
preemptor Pod can be scheduled.
While the preemptor Pod is waiting for the victims to go away, a higher priority
Pod may be created that fits on the same Node. In this case, the scheduler will
schedule the higher priority Pod instead of the preemptor.
This is expected behavior: the Pod with the higher priority should take the place