---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#11-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 123
summary: . Otherwise, a CertificateSigningRequest must be manually approved either via the REST API (or client-go) or by running `kubectl certificate approve`. Likewise, a CertificateSigningRequest may also...
---

.
Otherwise, a CertificateSigningRequest must be manually approved either via the REST API (or client-go)
or by running `kubectl certificate approve`. Likewise, a CertificateSigningRequest may also be denied,
which tells the configured signer that it must not sign the request.
For certificates that have been approved, the next step is signing. The relevant signing controller
first validates that the signing conditions are met and then creates a certificate.
The signing controller then updates the CertificateSigningRequest, storing the new certificate into
the `status.certificate` field of the existing CertificateSigningRequest object. The
`status.certificate`