---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#124-summary
chunk_level: summary
chunk_type: prose
heading: Information security for Secrets
token_count: 55
summary: ### Configure least-privilege access to Secrets To enhance the security measures around Secrets, use separate namespaces to isolate access to mounted secrets. #### Warning: Any containers that run...
---

### Configure least-privilege access to Secrets
To enhance the security measures around Secrets, use separate namespaces to isolate access to mounted secrets.
#### Warning:
Any containers that run with `privileged: true` on a node can access all
Secrets used on that node.