---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 684
summary: ## Use cases for Kubernetes service accounts As a general guideline, you can use service accounts to provide identities in the following scenarios: * Your Pods need to communicate with the Kubernetes...
---

## Use cases for Kubernetes service accounts
As a general guideline, you can use service accounts to provide identities in
the following scenarios:
* Your Pods need to communicate with the Kubernetes API server, for example in
situations such as the following:
* Providing read-only access to sensitive information stored in Secrets.
* Granting [cross-namespace access](#cross-namespace), such as allowing a
Pod in namespace `example` to read, list, and watch for Lease objects in
the `kube-node-lease` namespace.
* Your Pods need to communicate with an external service. For example, a
workload Pod requires an identity for a commercially available cloud API,
and the commercial provider allows configuring a suitable trust relationship.
* [Authenticating to a private image registry using an `imagePullSecret`](/docs/tasks/configure-pod-container/configure-service-account/#add-imagepullsecrets-to-a-service-account).
* An external service needs to communicate with the Kubernetes API server. For
example, authenticating to the cluster as part of a CI/CD pipeline.
* You use third-party security software in your cluster that relies on the
ServiceAccount identity of different Pods to group those Pods into different
contexts.## How to use service accounts
To use a Kubernetes service account, you do the following:
1. Create a ServiceAccount object using a Kubernetes
client like `kubectl` or a manifest that defines the object.
2. Grant permissions to the ServiceAccount object using an authorization
mechanism such as
[RBAC](/docs/reference/access-authn-authz/rbac/).
3. Assign the ServiceAccount object to Pods during Pod creation.
If you're using the identity from an external service,
[retrieve the ServiceAccount token](#get-a-token) and use it from that
service instead.
For instructions, refer to
[Configure Service Accounts for Pods](/docs/tasks/configure-pod-container/configure-service-account/).
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