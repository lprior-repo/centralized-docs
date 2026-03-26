---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#67-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 121
summary: The `kubernetes.io/tls` Secret type is for storing a certificate and its associated key that are typically used for TLS. One common use for TLS Secrets is to configure encryption in transit for an...
---

The `kubernetes.io/tls` Secret type is for storing
a certificate and its associated key that are typically used for TLS.
One common use for TLS Secrets is to configure encryption in transit for
an [Ingress](/docs/concepts/services-networking/ingress/), but you can also use it
with other resources or directly in your workload.
When using this type of Secret, the `tls.key` and the `tls.crt` key must be provided
in the `data` (or `stringData`) field of the Secret configuration, although the API
server doesn'