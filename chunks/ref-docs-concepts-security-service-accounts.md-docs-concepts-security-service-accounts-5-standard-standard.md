---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#5-standard
chunk_level: standard
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 284
summary: ### Grant permissions to a ServiceAccount You can use the built-in Kubernetes [role-based access control (RBAC)](/docs/reference/access-authn-authz/rbac/) mechanism to grant the minimum permissions...
---

### Grant permissions to a ServiceAccount
You can use the built-in Kubernetes
[role-based access control (RBAC)](/docs/reference/access-authn-authz/rbac/)
mechanism to grant the minimum permissions required by each service account.
You create a *role*, which grants access, and then *bind* the role to your
ServiceAccount. RBAC lets you define a minimum set of permissions so that the
service account permissions follow the principle of least privilege. Pods that
use that service account don't get more permissions than are required to
function correctly.
For instructions, refer to
[ServiceAccount permissions](/docs/reference/access-authn-authz/rbac/#service-account-permissions).
#### Cross-namespace access using a ServiceAccount
You can use RBAC to allow service accounts in one namespace to perform actions
on resources in a different namespace in the cluster. For example, consider a
scenario where you have a service account and Pod in the `dev` namespace and
you want your Pod to see Jobs running in the `maintenance` namespace. You could
create a Role object that grants permissions to list Job objects. Then,
you'd create a RoleBinding object in the `maintenance` namespace to bind the
Role to the ServiceAccount object. Now, Pods in the `dev` namespace can list
Job objects in the `maintenance` namespace using that service account.