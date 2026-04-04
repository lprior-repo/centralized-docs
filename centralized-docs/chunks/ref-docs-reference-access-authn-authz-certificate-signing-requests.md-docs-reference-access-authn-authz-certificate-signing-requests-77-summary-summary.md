---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#77-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 116
summary: * `unverifiedUserAnnotations`: A map that allows the user to pass additional information to the signer implementation. It is copied verbatim from the `userAnnotations` field of the [podCertificate...
---

* `unverifiedUserAnnotations`: A map that allows the user to pass additional
information to the signer implementation. It is copied verbatim from the
`userAnnotations` field of the [podCertificate projected volume source](/docs/concepts/storage/projected-volumes/#podcertificate).
Entries are subject to the same validation as object metadata annotations,
with the addition that all keys must be domain-prefixed. No restrictions are
placed on values, except an overall size limitation on the entire field. Other
than these basic validations, the API server does not conduct any extra