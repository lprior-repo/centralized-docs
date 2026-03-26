---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#3-standard
chunk_level: standard
chunk_type: prose
heading: What are service accounts?
token_count: 182
summary: ### Default service accounts When you create a cluster, Kubernetes automatically creates a ServiceAccount object named `default` for every namespace in your cluster. The `default` service accounts in...
---

### Default service accounts
When you create a cluster, Kubernetes automatically creates a ServiceAccount
object named `default` for every namespace in your cluster. The `default`
service accounts in each namespace get no permissions by default other than the
[default API discovery permissions](/docs/reference/access-authn-authz/rbac/#default-roles-and-role-bindings)
that Kubernetes grants to all authenticated principals if role-based access control (RBAC) is enabled.
If you delete the `default` ServiceAccount object in a namespace, the
[control plane](/docs/reference/glossary/?all=true#term-control-plane)
replaces it with a new one.
If you deploy a Pod in a namespace, and you don't
[manually assign a ServiceAccount to the Pod](#assign-to-pod), Kubernetes
assigns the `default` ServiceAccount for that namespace to the Pod.