---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#10-summary
chunk_level: summary
chunk_type: table
heading: What are service accounts?
token_count: 99
summary: Kubernetes distributions might add custom extension APIs to represent user accounts in the API server. Comparison between service accounts and users|Description|ServiceAccount|User or group|...
---

Kubernetes distributions might add custom extension APIs to represent user
accounts in the API server.
Comparison between service accounts and users|Description|ServiceAccount|User or group|
|Location|Kubernetes API (ServiceAccount object)|External|
|Access control|Kubernetes RBAC or other [authorization mechanisms](/docs/reference/access-authn-authz/authorization/#authorization-modules)|Kubernetes RBAC or other identity and access management mechanisms|
|Intended use|Workloads, automation|People|