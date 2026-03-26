---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#79-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 119
summary: * `token-id`: A random 6 character string as the token identifier. Required. * `token-secret`: A random 16 character string as the actual token Secret. Required. * `description`: A human-readable...
---

* `token-id`: A random 6 character string as the token identifier. Required.
* `token-secret`: A random 16 character string as the actual token Secret. Required.
* `description`: A human-readable string that describes what the token is
used for. Optional.
* `expiration`: An absolute UTC time using [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) specifying when the token
should be expired. Optional.
* `usage-bootstrap-&lt;usage&gt;`: A boolean flag indicating additional usage for
the bootstrap token.