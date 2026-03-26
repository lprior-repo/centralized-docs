---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#9-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 283
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
* Call the `TokenRequest` API either directly or by using an API client like
`kubectl`. For example, you can use the
[`kubectl create token`](/docs/reference/generated/kubectl/kubectl-commands#-em-token-em-)
command.
* Request a mounted token in a
[projected volume](/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-token-volume)
in your Pod manifest. Kubernetes creates the token and mounts it in the Pod.
The token is automatically invalidated when the Pod that it's mounted in is
deleted. For details, see
[Launch a Pod using service account token projection](/docs/tasks/configure-pod-container/configure-service-account/#launch-a-pod-using-service-account-token-projection).