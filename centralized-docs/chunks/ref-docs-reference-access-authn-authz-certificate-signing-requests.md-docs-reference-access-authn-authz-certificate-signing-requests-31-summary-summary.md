---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#31-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 121
summary: intermediates to be presented during TLS handshakes. If you want to make the *trust anchor* (root certificate) available, this should be done separately from a CertificateSigningRequest and its...
---

intermediates to be presented during TLS handshakes.
If you want to make the *trust anchor* (root certificate) available, this should be done
separately from a CertificateSigningRequest and its `status.certificate` field. For example,
you could use a ClusterTrustBundle.
The PKCS#10 signing request format does not have a standard mechanism to specify a
certificate expiration or lifetime. The expiration or lifetime therefore has to be set
through the `spec.expirationSeconds` field of the CSR object. The built-in signers
use the `ClusterSigningDuration`