---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#14-summary
chunk_level: summary
chunk_type: prose
heading: Bootstrap Token Secret Format
token_count: 103
summary: * `usage-bootstrap-signing` indicates that the token may be used to sign the `cluster-info` ConfigMap as described below. The `expiration` field controls the expiry of the token. Expired tokens are...
---

* `usage-bootstrap-signing` indicates that the token may be used to sign the
`cluster-info` ConfigMap as described below.
The `expiration` field controls the expiry of the token. Expired tokens are
rejected when used for authentication and ignored during ConfigMap signing.
The expiry value is encoded as an absolute UTC time using [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339). Enable the
`tokencleaner` controller to automatically delete expired tokens.