---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#11-detailed
chunk_level: detailed
chunk_type: prose
heading: What's next
token_count: 835
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
## Information security for Secrets
Although ConfigMap and Secret work similarly, Kubernetes applies some additional
protection for Secret objects.
Secrets often hold values that span a spectrum of importance, many of which can
cause escalations within Kubernetes (e.g. service account tokens) and to
external systems. Even if an individual app can reason about the power of the
Secrets it expects to interact with, other apps within the same namespace can
render those assumptions invalid.
Authorization configuration affects how Secret data can be accessed within a namespace.
For example, granting **list** or **watch** permissions on Secrets allows a subject
to read all Secret data in that namespace, not only the Secrets explicitly
referenced by its Pods. Restrict access to the minimum set of permissions
required for a workload to function, and avoid granting broad roles such as
`cluster-admin` unless required for administrative purposes.
Also see the [Authorization documentation](/docs/reference/access-authn-authz/rbac/).
A Secret is only sent to a node if a Pod on that node requires it.
For mounting Secrets into Pods, the kubelet stores a copy of the data into a `tmpfs`
so that the confidential data is not written to durable storage.
Once the Pod that depends on the Secret is deleted, the kubelet deletes its local copy
of the confidential data from the Secret.
There may be several containers in a Pod. By default, containers you define
only have access to the default ServiceAccount and its related Secret.
You must explicitly define environment variables or map a volume into a
container in order to provide access to any other Secret.
There may be Secrets for several Pods on the same node. However, only the
Secrets that a Pod requests are potentially visible within its containers.
Therefore, one Pod does not have access to the Secrets of another Pod.
### Configure least-privilege access to Secrets
To enhance the security measures around Secrets, use separate namespaces to isolate access to mounted secrets.
#### Warning:
Any containers that run with `privileged: true` on a node can access all
Secrets used on that node.
## What's next
* For guidelines to manage and improve the security of your Secrets, refer to
[Good practices for Kubernetes Secrets](/docs/concepts/security/secrets-good-practices/).
* Learn how to [manage Secrets using `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/)
* Learn how to [manage Secrets using config file](/docs/tasks/configmap-secret/managing-secret-using-config-file/)
* Learn how to [manage Secrets using kustomize](/docs/tasks/configmap-secret/managing-secret-using-kustomize/)
* Read the [API reference](/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/) for `Secret`