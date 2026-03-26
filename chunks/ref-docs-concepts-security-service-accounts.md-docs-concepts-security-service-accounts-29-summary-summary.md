---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#29-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 87
summary: * [TokenRequest API](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/) (recommended): Request a short-lived service account token from within your own *application code*. The...
---

* [TokenRequest API](/docs/reference/kubernetes-api/authentication-resources/token-request-v1/)
(recommended): Request a short-lived service account token from within
your own *application code*. The token expires automatically and can rotate
upon expiration.
If you have a legacy application that is not aware of Kubernetes, you
could use a sidecar container within the same pod to fetch these tokens
and make them available to the application workload.