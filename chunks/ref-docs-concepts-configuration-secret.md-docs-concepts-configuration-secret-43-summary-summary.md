---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#43-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 120
summary: You should only create a ServiceAccount token Secret if you can't use the `TokenRequest` API to obtain a token, and the security exposure of persisting a non-expiring token credential in a readable...
---

You should only create a ServiceAccount token Secret
if you can't use the `TokenRequest` API to obtain a token,
and the security exposure of persisting a non-expiring token credential
in a readable API object is acceptable to you. For instructions, see
[Manually create a long-lived API token for a ServiceAccount](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount).
When using this Secret type, you need to ensure that the
`kubernetes.io/service-account.name` annotation is set to an existing