---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#1-detailed
chunk_level: detailed
chunk_type: prose
heading: PriorityClass
token_count: 830
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
## PriorityClass
A PriorityClass is a non-namespaced object that defines a mapping from a
priority class name to the integer value of the priority. The name is specified
in the `name` field of the PriorityClass object's metadata. The value is
specified in the required `value` field. The higher the value, the higher the
priority.
The name of a PriorityClass object must be a valid
[DNS subdomain name](/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names),
and it cannot be prefixed with `system-`.
A PriorityClass object can have any 32-bit integer value smaller than or equal
to 1 billion. This means that the range of values for a PriorityClass object is
from -2147483648 to 1000000000 inclusive. Larger numbers are reserved for
built-in PriorityClasses that represent critical system Pods. A cluster
admin should create one PriorityClass object for each such mapping that they want.
PriorityClass also has two optional fields: `globalDefault` and `description`.
The `globalDefault` field indicates that the value of this PriorityClass should
be used for Pods without a `priorityClassName`. Only one PriorityClass with
`globalDefault` set to true can exist in the system. If there is no
PriorityClass with `globalDefault` set, the priority of Pods with no
`priorityClassName` is zero.
The `description` field is an arbitrary string. It is meant to tell users of the
cluster when they should use this PriorityClass.
### Notes about PodPriority and existing clusters
* If you upgrade an existing cluster without this feature, the priority
of your existing Pods is effectively zero.
* Addition of a PriorityClass with `globalDefault` set to `true` does not
change the priorities of existing Pods. The value of such a PriorityClass is
used only for Pods created after the PriorityClass is added.
* If you delete a PriorityClass, existing Pods that use the name of the
deleted PriorityClass remain unchanged, but you cannot create more Pods that
use the name of the deleted PriorityClass.
### Example PriorityClass
```
`apiVersion: scheduling.k8s.io/v1
kind: PriorityClass
metadata:
name: high-priority
value: 1000000
globalDefault: false
description: "This priority class should be used for XYZ service pods only."
`
```