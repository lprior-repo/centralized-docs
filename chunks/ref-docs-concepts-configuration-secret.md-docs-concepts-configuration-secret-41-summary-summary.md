---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#41-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 104
summary: * Request a mounted token in a [projected volume](/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-token-volume) in your Pod manifest. Kubernetes creates the token and...
---

* Request a mounted token in a
[projected volume](/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-token-volume)
in your Pod manifest. Kubernetes creates the token and mounts it in the Pod.
The token is automatically invalidated when the Pod that it's mounted in is
deleted. For details, see
[Launch a Pod using service account token projection](/docs/tasks/configure-pod-container/configure-service-account/#launch-a-pod-using-service-account-token-projection).