---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#2-summary
chunk_level: summary
chunk_type: prose
heading: Scheduling overview
token_count: 98
summary: ## Scheduling overview A scheduler watches for newly created Pods that have no Node assigned. For every Pod that the scheduler discovers, the scheduler becomes responsible for finding the best Node...
---

## Scheduling overview
A scheduler watches for newly created Pods that have no Node assigned. For
every Pod that the scheduler discovers, the scheduler becomes responsible
for finding the best Node for that Pod to run on. The scheduler reaches
this placement decision taking into account the scheduling principles
described below.
If you want to understand why Pods are placed onto a particular Node,
or if you're planning to implement a custom scheduler yourself, this
page will help you learn about scheduling.