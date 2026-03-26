---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#6-standard
chunk_level: standard
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 203
summary: ### Assign a ServiceAccount to a Pod To assign a ServiceAccount to a Pod, you set the `spec.serviceAccountName` field in the Pod specification. Kubernetes then automatically provides the credentials...
---

### Assign a ServiceAccount to a Pod
To assign a ServiceAccount to a Pod, you set the `spec.serviceAccountName`
field in the Pod specification. Kubernetes then automatically provides the
credentials for that ServiceAccount to the Pod. In v1.22 and later, Kubernetes
gets a short-lived, **automatically rotating** token using the `TokenRequest`
API and mounts the token as a
[projected volume](/docs/concepts/storage/projected-volumes/#serviceaccounttoken).
By default, Kubernetes provides the Pod
with the credentials for an assigned ServiceAccount, whether that is the
`default` ServiceAccount or a custom ServiceAccount that you specify.
To prevent Kubernetes from automatically injecting
credentials for a specified ServiceAccount or the `default` ServiceAccount, set the
`automountServiceAccountToken` field in your Pod specification to `false`.
In versions earlier than 1.22, Kubernetes provides a long-lived, static token
to the Pod as a Secret.