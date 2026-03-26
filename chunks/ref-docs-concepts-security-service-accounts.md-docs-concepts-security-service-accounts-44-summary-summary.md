---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#44-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating service account credentials
token_count: 123
summary: 5. Checks the audience claims. The TokenRequest API produces *bound tokens* for a ServiceAccount. This binding is linked to the lifetime of the client, such as a Pod, that is acting as that...
---

5. Checks the audience claims.
The TokenRequest API produces *bound tokens* for a ServiceAccount. This
binding is linked to the lifetime of the client, such as a Pod, that is acting
as that ServiceAccount. See [Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection)
for an example of a bound pod service account token's JWT schema and payload.
For tokens issued using the `TokenRequest` API, the API server also checks that
the specific object reference that is using the ServiceAccount still exists,
matching by the