---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#78-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 126
summary: placed on values, except an overall size limitation on the entire field. Other than these basic validations, the API server does not conduct any extra validations. The signer implementations should...
---

placed on values, except an overall size limitation on the entire field. Other
than these basic validations, the API server does not conduct any extra
validations. The signer implementations should be very careful when consuming
this data. Signers must not inherently trust this data without first
performing the appropriate verification steps. Signers should document the
keys and values they support. Signers should deny requests that contain keys
they do not recognize.
Nodes automatically receive permissions to create PodCertificateRequests and
read PodCertificateRequests related to them (as determined by the
`spec.nodeName` field). The `NodeRestriction`