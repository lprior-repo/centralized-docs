---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#8-summary
chunk_level: summary
chunk_type: prose
heading: What are service accounts?
token_count: 84
summary: * **Namespaced:** Each service account is bound to a Kubernetes [namespace](/docs/concepts/overview/working-with-objects/namespaces). Every namespace gets a [`default`...
---

* **Namespaced:** Each service account is bound to a Kubernetes
[namespace](/docs/concepts/overview/working-with-objects/namespaces). Every namespace
gets a [`default` ServiceAccount](#default-service-accounts) upon creation.
* **Lightweight:** Service accounts exist in the cluster and are
defined in the Kubernetes API. You can quickly create service accounts to
enable specific tasks.