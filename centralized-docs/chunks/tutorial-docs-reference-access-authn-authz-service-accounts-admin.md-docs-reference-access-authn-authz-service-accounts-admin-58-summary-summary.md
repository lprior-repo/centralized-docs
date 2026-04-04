---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#58-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 118
summary: * If the spec of the incoming Pod doesn't already contain any `imagePullSecrets`, then the admission controller adds `imagePullSecrets`, copying them from the `ServiceAccount`.### Legacy...
---

* If the spec of the incoming Pod doesn't already contain any `imagePullSecrets`, then the
admission controller adds `imagePullSecrets`, copying them from the `ServiceAccount`.### Legacy ServiceAccount token tracking controller
FEATURE STATE:
`Kubernetes v1.28 [stable]`(enabled by default)
This controller generates a ConfigMap called
`kube-system/kube-apiserver-legacy-service-account-token-tracking` in the
`kube-system` namespace. The ConfigMap records the timestamp when legacy service
account tokens began to be monitored by the system.