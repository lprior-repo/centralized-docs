---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#15-summary
chunk_level: summary
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 122
summary: * Your Pods need to communicate with the Kubernetes API server, for example in situations such as the following: * Providing read-only access to sensitive information stored in Secrets. * Granting...
---

* Your Pods need to communicate with the Kubernetes API server, for example in
situations such as the following:
* Providing read-only access to sensitive information stored in Secrets.
* Granting [cross-namespace access](#cross-namespace), such as allowing a
Pod in namespace `example` to read, list, and watch for Lease objects in
the `kube-node-lease` namespace.
* Your Pods need to communicate with an external service. For example, a
workload Pod requires an identity for a commercially available cloud API,
and the commercial provider allows configuring a suitable trust relationship.