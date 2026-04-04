---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#1-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 331
summary: # Force Delete StatefulSet Pods This page shows how to delete Pods which are part of a [stateful set](/docs/concepts/workloads/controllers/statefulset/), and explains the considerations to keep in...
---

# Force Delete StatefulSet Pods
This page shows how to delete Pods which are part of a
[stateful set](/docs/concepts/workloads/controllers/statefulset/),
and explains the considerations to keep in mind when doing so.
## Before you begin
* This is a fairly advanced task and has the potential to violate some of the properties
inherent to StatefulSet.
* Before proceeding, make yourself familiar with the considerations enumerated below.## StatefulSet considerations
In normal operation of a StatefulSet, there is **never** a need to force delete a StatefulSet Pod.
The [StatefulSet controller](/docs/concepts/workloads/controllers/statefulset/) is responsible for
creating, scaling and deleting members of the StatefulSet. It tries to ensure that the specified
number of Pods from ordinal 0 through N-1 are alive and ready. StatefulSet ensures that, at any time,
there is at most one Pod with a given identity running in a cluster. This is referred to as
*at most one* semantics provided by a StatefulSet.
Manual force deletion should be undertaken with caution, as it has the potential to violate the
at most one semantics inherent to StatefulSet. StatefulSets may be used to run distributed and
clustered applications which have a need for a stable network identity and stable storage.
These applications often have configuration which relies on an ensemble of a fixed number of
members with fixed identities. Having multiple members with the same identity can be disastrous
and may lead to data loss (e.g. split brain scenario in quorum-based systems).