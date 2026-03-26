---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#5-summary
chunk_level: summary
chunk_type: prose
heading: Owner references in object specifications
token_count: 124
summary: Dependent objects have a `metadata.ownerReferences` field that references their owner object. A valid owner reference consists of the object name and a...
---

Dependent objects have a `metadata.ownerReferences` field that references their
owner object. A valid owner reference consists of the object name and a [UID](/docs/concepts/overview/working-with-objects/names)
within the same [namespace](/docs/concepts/overview/working-with-objects/namespaces) as the dependent object. Kubernetes sets the value of
this field automatically for objects that are dependents of other objects like
ReplicaSets, DaemonSets, Deployments, Jobs and CronJobs, and ReplicationControllers.
You can also configure these relationships manually by changing the value of