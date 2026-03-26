---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#1-standard
chunk_level: standard
chunk_type: prose
heading: How to use priority and preemption
token_count: 329
summary: # Pod Priority and Preemption FEATURE STATE: `Kubernetes v1.14 [stable]` [Pods](/docs/concepts/workloads/pods/) can have *priority*. Priority indicates the importance of a Pod relative to other Pods....
---

# Pod Priority and Preemption
FEATURE STATE:
`Kubernetes v1.14 [stable]`
[Pods](/docs/concepts/workloads/pods/) can have *priority*. Priority indicates the
importance of a Pod relative to other Pods. If a Pod cannot be scheduled, the
scheduler tries to preempt (evict) lower priority Pods to make scheduling of the
pending Pod possible.
#### Warning:
In a cluster where not all users are trusted, a malicious user could create Pods
at the highest possible priorities, causing other Pods to be evicted/not get
scheduled.
An administrator can use ResourceQuota to prevent users from creating pods at
high priorities.
See [limit Priority Class consumption by default](/docs/concepts/policy/resource-quotas/#limit-priority-class-consumption-by-default)
for details.
## How to use priority and preemption
To use priority and preemption:
1. Add one or more [PriorityClasses](#priorityclass).
2. Create Pods with[`priorityClassName`](#pod-priority) set to one of the added
PriorityClasses. Of course you do not need to create the Pods directly;
normally you would add `priorityClassName` to the Pod template of a
collection object like a Deployment.
Keep reading for more information about these steps.
#### Note:
Kubernetes already ships with two PriorityClasses:
`system-cluster-critical` and `system-node-critical`.
These are common classes and are used to [ensure that critical components are always scheduled first](/docs/tasks/administer-cluster/guaranteed-scheduling-critical-addon-pods/).