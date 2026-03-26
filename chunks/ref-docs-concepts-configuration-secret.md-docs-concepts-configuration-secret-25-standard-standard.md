---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#25-standard
chunk_level: standard
chunk_type: prose
heading: Information security for Secrets
token_count: 431
summary: ## Information security for Secrets Although ConfigMap and Secret work similarly, Kubernetes applies some additional protection for Secret objects. Secrets often hold values that span a spectrum of...
---

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