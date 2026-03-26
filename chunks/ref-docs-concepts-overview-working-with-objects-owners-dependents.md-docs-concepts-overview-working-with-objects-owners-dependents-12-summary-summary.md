---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#12-summary
chunk_level: summary
chunk_type: prose
heading: Ownership and finalizers
token_count: 128
summary: When you tell Kubernetes to delete a resource, the API server allows the managing controller to process any [finalizer rules](/docs/concepts/overview/working-with-objects/finalizers/) for the...
---

When you tell Kubernetes to delete a resource, the API server allows the
managing controller to process any [finalizer rules](/docs/concepts/overview/working-with-objects/finalizers/)
for the resource. [Finalizers](/docs/concepts/overview/working-with-objects/finalizers/)
prevent accidental deletion of resources your cluster may still need to function
correctly. For example, if you try to delete a [PersistentVolume](/docs/concepts/storage/persistent-volumes/) that is still
in use by a Pod, the deletion does not happen immediately because the
`PersistentVolume` has the