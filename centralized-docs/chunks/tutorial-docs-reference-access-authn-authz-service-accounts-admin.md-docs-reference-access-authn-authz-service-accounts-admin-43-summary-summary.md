---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#43-summary
chunk_level: summary
chunk_type: prose
heading: Manual Secret management for ServiceAccounts
token_count: 107
summary: The tokens obtained using this method have bounded lifetimes, and are automatically invalidated when the Pod they are mounted into is deleted. You can still [manually...
---

The tokens obtained using this method have bounded lifetimes, and are automatically
invalidated when the Pod they are mounted into is deleted.
You can still [manually create](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount)
a Secret to hold a service account token; for example, if you need a token that never expires.
Once you manually create a Secret and link it to a ServiceAccount,
the Kubernetes control plane automatically populates the token into that Secret.