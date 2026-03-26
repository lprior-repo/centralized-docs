---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#35-summary
chunk_level: summary
chunk_type: table
heading: Types of Secret
token_count: 124
summary: |`kubernetes.io/tls`|data for a TLS client or server| |`bootstrap.kubernetes.io/token`|bootstrap token data| You can define and use your own Secret type by assigning a non-empty string as the `type`...
---

|`kubernetes.io/tls`|data for a TLS client or server|
|`bootstrap.kubernetes.io/token`|bootstrap token data|
You can define and use your own Secret type by assigning a non-empty string as the
`type` value for a Secret object (an empty string is treated as an `Opaque` type).
Kubernetes doesn't impose any constraints on the type name. However, if you
are using one of the built-in types, you must meet all the requirements defined
for that type.
If you are defining a type of Secret that's for public use, follow the convention