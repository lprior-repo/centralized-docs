---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#4-standard
chunk_level: standard
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 400
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