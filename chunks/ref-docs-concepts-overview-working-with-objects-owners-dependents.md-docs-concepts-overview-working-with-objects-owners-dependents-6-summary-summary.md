---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#6-summary
chunk_level: summary
chunk_type: prose
heading: Owner references in object specifications
token_count: 125
summary: You can also configure these relationships manually by changing the value of this field. However, you usually don't need to and can allow Kubernetes to automatically manage the relationships....
---

You can also configure these relationships manually by changing the value of
this field. However, you usually don't need to and can allow Kubernetes to
automatically manage the relationships.
Dependent objects also have an `ownerReferences.blockOwnerDeletion` field that
takes a boolean value and controls whether specific dependents can block garbage
collection from deleting their owner object. Kubernetes automatically sets this
field to `true` if a [controller](/docs/concepts/architecture/controller/)
(for example, the Deployment controller) sets the value of the
`metadata.ownerReferences` field. You can also set the value of the