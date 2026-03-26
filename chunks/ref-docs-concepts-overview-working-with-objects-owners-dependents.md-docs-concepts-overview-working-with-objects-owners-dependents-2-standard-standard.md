---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#2-standard
chunk_level: standard
chunk_type: prose
heading: Owner references in object specifications
token_count: 301
summary: ## Owner references in object specifications Dependent objects have a `metadata.ownerReferences` field that references their owner object. A valid owner reference consists of the object name and a...
---

## Owner references in object specifications
Dependent objects have a `metadata.ownerReferences` field that references their
owner object. A valid owner reference consists of the object name and a [UID](/docs/concepts/overview/working-with-objects/names)
within the same [namespace](/docs/concepts/overview/working-with-objects/namespaces) as the dependent object. Kubernetes sets the value of
this field automatically for objects that are dependents of other objects like
ReplicaSets, DaemonSets, Deployments, Jobs and CronJobs, and ReplicationControllers.
You can also configure these relationships manually by changing the value of
this field. However, you usually don't need to and can allow Kubernetes to
automatically manage the relationships.
Dependent objects also have an `ownerReferences.blockOwnerDeletion` field that
takes a boolean value and controls whether specific dependents can block garbage
collection from deleting their owner object. Kubernetes automatically sets this
field to `true` if a [controller](/docs/concepts/architecture/controller/)
(for example, the Deployment controller) sets the value of the
`metadata.ownerReferences` field. You can also set the value of the
`blockOwnerDeletion` field manually to control which dependents block garbage
collection.
A Kubernetes admission controller controls user access to change this field for
dependent resources, based on the delete permissions of the owner. This control
prevents unauthorized users from delaying owner object deletion.