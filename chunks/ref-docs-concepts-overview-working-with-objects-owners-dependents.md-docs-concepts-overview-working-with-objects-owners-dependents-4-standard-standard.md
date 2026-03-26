---
doc_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents
chunk_id: ref/docs-concepts-overview-working-with-objects-owners-dependents.md/docs-concepts-overview-working-with-objects-owners-dependents#4-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 385
summary: ## Ownership and finalizers When you tell Kubernetes to delete a resource, the API server allows the managing controller to process any [finalizer...
---

## Ownership and finalizers
When you tell Kubernetes to delete a resource, the API server allows the
managing controller to process any [finalizer rules](/docs/concepts/overview/working-with-objects/finalizers/)
for the resource. [Finalizers](/docs/concepts/overview/working-with-objects/finalizers/)
prevent accidental deletion of resources your cluster may still need to function
correctly. For example, if you try to delete a [PersistentVolume](/docs/concepts/storage/persistent-volumes/) that is still
in use by a Pod, the deletion does not happen immediately because the
`PersistentVolume` has the `kubernetes.io/pv-protection` finalizer on it.
Instead, the [volume](/docs/concepts/storage/volumes/) remains in the `Terminating` status until Kubernetes clears
the finalizer, which only happens after the `PersistentVolume` is no longer
bound to a Pod.
Kubernetes also adds finalizers to an owner resource when you use either
[foreground or orphan cascading deletion](/docs/concepts/architecture/garbage-collection/#cascading-deletion).
In foreground deletion, it adds the `foreground` finalizer so that the
controller must delete dependent resources that also have
`ownerReferences.blockOwnerDeletion=true` before it deletes the owner. If you
specify an orphan deletion policy, Kubernetes adds the `orphan` finalizer so
that the controller ignores dependent resources after it deletes the owner
object.
## What's next
* Learn more about [Kubernetes finalizers](/docs/concepts/overview/working-with-objects/finalizers/).
* Learn about [garbage collection](/docs/concepts/architecture/garbage-collection/).
* Read the API reference for [object metadata](/docs/reference/kubernetes-api/common-definitions/object-meta/#System).