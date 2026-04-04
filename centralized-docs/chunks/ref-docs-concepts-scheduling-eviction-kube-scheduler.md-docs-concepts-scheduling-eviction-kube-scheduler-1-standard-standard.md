---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#1-standard
chunk_level: standard
chunk_type: prose
heading: Scheduling overview
token_count: 164
summary: # Kubernetes Scheduler In Kubernetes, *scheduling* refers to making sure that [Pods](/docs/concepts/workloads/pods/) are matched to [Nodes](/docs/concepts/architecture/nodes/) so that...
---

# Kubernetes Scheduler
In Kubernetes, *scheduling* refers to making sure that [Pods](/docs/concepts/workloads/pods/)
are matched to [Nodes](/docs/concepts/architecture/nodes/) so that
[Kubelet](/docs/reference/command-line-tools-reference/kubelet) can run them.
## Scheduling overview
A scheduler watches for newly created Pods that have no Node assigned. For
every Pod that the scheduler discovers, the scheduler becomes responsible
for finding the best Node for that Pod to run on. The scheduler reaches
this placement decision taking into account the scheduling principles
described below.
If you want to understand why Pods are placed onto a particular Node,
or if you're planning to implement a custom scheduler yourself, this
page will help you learn about scheduling.