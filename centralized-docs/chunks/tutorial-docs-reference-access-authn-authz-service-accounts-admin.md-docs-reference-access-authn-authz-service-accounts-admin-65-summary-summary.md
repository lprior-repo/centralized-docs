---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#65-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 100
summary: FEATURE STATE: `Kubernetes v1.22 [stable]` You use the [TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/) subresource of a ServiceAccount to obtain a time-bound...
---

FEATURE STATE:
`Kubernetes v1.22 [stable]`
You use the [TokenRequest](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/)
subresource of a ServiceAccount to obtain a time-bound token for that ServiceAccount.
You don't need to call this to obtain an API token for use within a container, since
the kubelet sets this up for you using a *projected volume*.
If you want to use the TokenRequest API from `kubectl`, see