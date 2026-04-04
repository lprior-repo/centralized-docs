---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#10-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 127
summary: `certificates.k8s.io/v1`. In Kubernetes v1.22 and later, clients may optionally set the `spec.expirationSeconds` field to request a particular lifetime for the issued certificate. The minimum valid...
---

`certificates.k8s.io/v1`.
In Kubernetes v1.22 and later, clients may optionally set the `spec.expirationSeconds`
field to request a particular lifetime for the issued certificate. The minimum valid
value for this field is `600`, i.e. ten minutes.
Once created, a CertificateSigningRequest must be approved before it can be signed.
Depending on the signer selected, a CertificateSigningRequest may be automatically approved
by a [controller](/docs/concepts/architecture/controller/).
Otherwise, a CertificateSigningRequest must be manually approved either via the REST API (or client-go)
or by running