---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 112
summary: * Before proceeding, make yourself familiar with the considerations enumerated below.## StatefulSet considerations In normal operation of a StatefulSet, there is **never** a need to force delete a...
---

* Before proceeding, make yourself familiar with the considerations enumerated below.## StatefulSet considerations
In normal operation of a StatefulSet, there is **never** a need to force delete a StatefulSet Pod.
The [StatefulSet controller](/docs/concepts/workloads/controllers/statefulset/) is responsible for
creating, scaling and deleting members of the StatefulSet. It tries to ensure that the specified
number of Pods from ordinal 0 through N-1 are alive and ready. StatefulSet ensures that, at any time,