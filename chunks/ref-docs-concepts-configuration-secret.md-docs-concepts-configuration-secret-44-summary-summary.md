---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#44-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 126
summary: . When using this Secret type, you need to ensure that the `kubernetes.io/service-account.name` annotation is set to an existing ServiceAccount name. If you are creating both the ServiceAccount and...
---

.
When using this Secret type, you need to ensure that the
`kubernetes.io/service-account.name` annotation is set to an existing
ServiceAccount name. If you are creating both the ServiceAccount and
the Secret objects, you should create the ServiceAccount object first.
After the Secret is created, a Kubernetes [controller](/docs/concepts/architecture/controller/)
fills in some other fields such as the `kubernetes.io/service-account.uid` annotation, and the
`token` key in the `data` field, which is populated with an authentication token.
The following example configuration declares a ServiceAccount token Secret: