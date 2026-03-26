---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#14-summary
chunk_level: summary
chunk_type: prose
heading: Ownership and finalizers
token_count: 100
summary: [foreground or orphan cascading deletion](/docs/concepts/architecture/garbage-collection/#cascading-deletion). In foreground deletion, it adds the `foreground` finalizer so that the controller must...
---

[foreground or orphan cascading deletion](/docs/concepts/architecture/garbage-collection/#cascading-deletion).
In foreground deletion, it adds the `foreground` finalizer so that the
controller must delete dependent resources that also have
`ownerReferences.blockOwnerDeletion=true` before it deletes the owner. If you
specify an orphan deletion policy, Kubernetes adds the `orphan` finalizer so
that the controller ignores dependent resources after it deletes the owner
object.