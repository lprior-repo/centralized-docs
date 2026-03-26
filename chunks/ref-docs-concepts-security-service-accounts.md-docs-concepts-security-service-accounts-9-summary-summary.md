---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#9-summary
chunk_level: summary
chunk_type: prose
heading: What are service accounts?
token_count: 121
summary: * **Portable:** A configuration bundle for a complex containerized workload might include service account definitions for the system's components. The lightweight nature of service accounts and the...
---

* **Portable:** A configuration bundle for a complex containerized workload
might include service account definitions for the system's components. The
lightweight nature of service accounts and the namespaced identities make
the configurations portable.
Service accounts are different from user accounts, which are authenticated
human users in the cluster. By default, user accounts don't exist in the Kubernetes
API server; instead, the API server treats user identities as opaque
data. You can authenticate as a user account using multiple methods. Some
Kubernetes distributions might add custom extension APIs to represent user
accounts in the API server.