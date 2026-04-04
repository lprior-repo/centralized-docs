---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#79-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 128
summary: read PodCertificateRequests related to them (as determined by the `spec.nodeName` field). The `NodeRestriction` admission plugin, if enabled, ensures that nodes can only create PodCertificateRequests...
---

read PodCertificateRequests related to them (as determined by the
`spec.nodeName` field). The `NodeRestriction` admission plugin, if enabled,
ensures that nodes can only create PodCertificateRequests that correspond to a
real pod that is currently running on the node.
After creation, the `spec` of a PodCertificateRequest is immutable.
Unlike CSRs, PodCertificateRequests do not have an
approval phase. Once the PodCertificateRequest is created, the signer's
controller directly decides to issue or deny the request. It also has the
option to mark the request as failed, if it encountered a permanent error when