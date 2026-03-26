---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#42-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating service account credentials
token_count: 62
summary: *starts* being valid. When a client that is acting as a ServiceAccount tries to communicate with the Kubernetes API server, the client includes an `Authorization: Bearer &lt;token&gt;` header with...
---

*starts* being valid. When a client that is
acting as a ServiceAccount tries to communicate with the Kubernetes API server,
the client includes an `Authorization: Bearer &lt;token&gt;` header with the HTTP
request. The API server checks the validity of that bearer token as follows: