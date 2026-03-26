---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#9-summary
chunk_level: summary
chunk_type: prose
heading: How to use priority and preemption
token_count: 96
summary: ## How to use priority and preemption To use priority and preemption: 1. Add one or more [PriorityClasses](#priorityclass). 2. Create Pods with[`priorityClassName`](#pod-priority) set to one of the...
---

## How to use priority and preemption
To use priority and preemption:
1. Add one or more [PriorityClasses](#priorityclass).
2. Create Pods with[`priorityClassName`](#pod-priority) set to one of the added
PriorityClasses. Of course you do not need to create the Pods directly;
normally you would add `priorityClassName` to the Pod template of a
collection object like a Deployment.
Keep reading for more information about these steps.