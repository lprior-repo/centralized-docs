---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#21-summary
chunk_level: summary
chunk_type: prose
heading: Non-preempting PriorityClass
token_count: 76
summary: ### Example Non-preempting PriorityClass ``` `apiVersion: scheduling.k8s.io/v1 kind: PriorityClass metadata: name: high-priority-nonpreempting value: 1000000 preemptionPolicy: Never globalDefault:...
---

### Example Non-preempting PriorityClass
```
`apiVersion: scheduling.k8s.io/v1
kind: PriorityClass
metadata:
name: high-priority-nonpreempting
value: 1000000
preemptionPolicy: Never
globalDefault: false
description: "This priority class will not cause other pods to be preempted."
`
```