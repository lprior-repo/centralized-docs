---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#9-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 125
summary: The CertificateSigningRequest resource type allows a client to ask for an X.509 certificate be issued, based on a signing request. The CertificateSigningRequest object includes a PEM-encoded PKCS#10...
---

The CertificateSigningRequest resource type allows a client to ask for an X.509 certificate
be issued, based on a signing request.
The CertificateSigningRequest object includes a PEM-encoded PKCS#10 signing request in
the `spec.request` field. The CertificateSigningRequest denotes the signer (the
recipient that the request is being made to) using the `spec.signerName` field.
Note that `spec.signerName` is a required key after API version `certificates.k8s.io/v1`.
In Kubernetes v1.22 and later, clients may optionally set the `spec.expirationSeconds`