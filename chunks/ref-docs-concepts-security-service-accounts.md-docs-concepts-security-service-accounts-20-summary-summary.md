---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#20-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 114
summary: You can use the built-in Kubernetes [role-based access control (RBAC)](/docs/reference/access-authn-authz/rbac/) mechanism to grant the minimum permissions required by each service account. You...
---

You can use the built-in Kubernetes
[role-based access control (RBAC)](/docs/reference/access-authn-authz/rbac/)
mechanism to grant the minimum permissions required by each service account.
You create a *role*, which grants access, and then *bind* the role to your
ServiceAccount. RBAC lets you define a minimum set of permissions so that the
service account permissions follow the principle of least privilege. Pods that
use that service account don't get more permissions than are required to
function correctly.
For instructions, refer to