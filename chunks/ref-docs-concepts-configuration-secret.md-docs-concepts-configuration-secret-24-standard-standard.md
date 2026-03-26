---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#24-standard
chunk_level: standard
chunk_type: prose
heading: Immutable Secrets
token_count: 265
summary: ## Immutable Secrets FEATURE STATE: `Kubernetes v1.21 [stable]` Kubernetes lets you mark specific Secrets (and ConfigMaps) as *immutable*. Preventing changes to the data of an existing Secret has the...
---

## Immutable Secrets
FEATURE STATE:
`Kubernetes v1.21 [stable]`
Kubernetes lets you mark specific Secrets (and ConfigMaps) as *immutable*.
Preventing changes to the data of an existing Secret has the following benefits:
* protects you from accidental (or unwanted) updates that could cause applications outages
* (for clusters that extensively use Secrets - at least tens of thousands of unique Secret
to Pod mounts), switching to immutable Secrets improves the performance of your cluster
by significantly reducing load on kube-apiserver. The kubelet does not need to maintain
a [watch] on any Secrets that are marked as immutable.### Marking a Secret as immutable
You can create an immutable Secret by setting the `immutable` field to `true`. For example,
```
`apiVersion: v1
kind: Secret
metadata: ...
data: ...
immutable: true
`
```
You can also update any existing mutable Secret to make it immutable.
#### Note:
Once a Secret or ConfigMap is marked as immutable, it is *not* possible to revert this change
nor to mutate the contents of the `data` field. You can only delete and recreate the Secret.
Existing Pods maintain a mount point to the deleted Secret - it is recommended to recreate
these pods.