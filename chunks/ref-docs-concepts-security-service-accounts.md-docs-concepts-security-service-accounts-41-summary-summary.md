---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#41-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating service account credentials
token_count: 126
summary: ServiceAccounts use signed [JSON Web Tokens](https://www.rfc-editor.org/rfc/rfc7519) (JWTs) to authenticate to the Kubernetes API server, and to any other system where a trust relationship exists....
---

ServiceAccounts use signed
[JSON Web Tokens](https://www.rfc-editor.org/rfc/rfc7519) (JWTs)
to authenticate to the Kubernetes API server, and to any other system where a
trust relationship exists. Depending on how the token was issued
(either time-limited using a `TokenRequest` or using a legacy mechanism with
a Secret), a ServiceAccount token might also have an expiry time, an audience,
and a time after which the token *starts* being valid. When a client that is
acting as a ServiceAccount tries to communicate with the Kubernetes API server,
the client includes an