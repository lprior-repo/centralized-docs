---
doc_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness
chunk_id: ref/docs-concepts-scheduling-eviction-pod-scheduling-readiness.md/docs-concepts-scheduling-eviction-pod-scheduling-readiness#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: # Pod Scheduling Readiness FEATURE STATE: `Kubernetes v1.30 [stable]` Pods were considered ready for scheduling once created. Kubernetes scheduler does its due diligence to find nodes to place all...
---

# Pod Scheduling Readiness
FEATURE STATE:
`Kubernetes v1.30 [stable]`
Pods were considered ready for scheduling once created. Kubernetes scheduler
does its due diligence to find nodes to place all pending Pods. However, in a
real-world case, some Pods may stay in a "miss-essential-resources" state for a long period.
These Pods actually churn the scheduler (and downstream integrators like Cluster AutoScaler)
in an unnecessary manner.
By specifying/removing a Pod's `.spec.schedulingGates`, you can control when a Pod is ready
to be considered for scheduling.