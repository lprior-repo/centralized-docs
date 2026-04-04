---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Manual Secret management for ServiceAccounts
token_count: 257
summary: ## Manual Secret management for ServiceAccounts Versions of Kubernetes before v1.22 automatically created credentials for accessing the Kubernetes API. This older mechanism was based on creating...
---

## Manual Secret management for ServiceAccounts
Versions of Kubernetes before v1.22 automatically created credentials for accessing
the Kubernetes API. This older mechanism was based on creating token Secrets that
could then be mounted into running Pods.
In more recent versions, including Kubernetes v1.35, API credentials
are [obtained directly](#bound-service-account-token-volume) using the
[TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/) API,
and are mounted into Pods using a projected volume.
The tokens obtained using this method have bounded lifetimes, and are automatically
invalidated when the Pod they are mounted into is deleted.
You can still [manually create](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount)
a Secret to hold a service account token; for example, if you need a token that never expires.
Once you manually create a Secret and link it to a ServiceAccount,
the Kubernetes control plane automatically populates the token into that Secret.
#### Note:
Although the manual mechanism for creating a long-lived ServiceAccount token exists,
using [TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/)
to obtain short-lived API access tokens is recommended instead.