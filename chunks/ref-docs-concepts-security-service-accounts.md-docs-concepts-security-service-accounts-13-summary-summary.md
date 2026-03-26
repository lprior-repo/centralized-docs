---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#13-summary
chunk_level: summary
chunk_type: prose
heading: What are service accounts?
token_count: 86
summary: `default` ServiceAccount object in a namespace, the [control plane](/docs/reference/glossary/?all=true#term-control-plane) replaces it with a new one. If you deploy a Pod in a namespace, and you...
---

`default` ServiceAccount object in a namespace, the
[control plane](/docs/reference/glossary/?all=true#term-control-plane)
replaces it with a new one.
If you deploy a Pod in a namespace, and you don't
[manually assign a ServiceAccount to the Pod](#assign-to-pod), Kubernetes
assigns the `default` ServiceAccount for that namespace to the Pod.