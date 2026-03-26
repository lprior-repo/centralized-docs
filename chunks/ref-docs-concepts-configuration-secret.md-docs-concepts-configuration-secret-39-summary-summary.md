---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#39-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 126
summary: ### ServiceAccount token Secrets A `kubernetes.io/service-account-token` type of Secret is used to store a token credential that identifies a...
---

### ServiceAccount token Secrets
A `kubernetes.io/service-account-token` type of Secret is used to store a
token credential that identifies a
[ServiceAccount](/docs/tasks/configure-pod-container/configure-service-account/). This
is a legacy mechanism that provides long-lived ServiceAccount credentials to
Pods.
In Kubernetes v1.22 and later, the recommended approach is to obtain a
short-lived, automatically rotating ServiceAccount token by using the
[`TokenRequest`](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/)
API instead. You can get these short-lived tokens using the following methods: