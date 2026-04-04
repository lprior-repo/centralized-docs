---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#7-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 474
summary: 1. **Trust distribution**: how trust anchors (CA certificates or certificate bundles) are distributed. 2. **Permitted subjects**: any restrictions on and behavior when a disallowed subject is...
---

1. **Trust distribution**: how trust anchors (CA certificates or certificate bundles) are distributed.
2. **Permitted subjects**: any restrictions on and behavior when a disallowed subject is requested.
3. **Permitted x509 extensions**: including IP subjectAltNames, DNS subjectAltNames,
Email subjectAltNames, URI subjectAltNames etc, and behavior when a disallowed extension is requested.
4. **Permitted key usages / extended key usages**: any restrictions on and behavior
when usages different than the signer-determined usages are specified in the CSR.
5. **Expiration/certificate lifetime**: whether it is fixed by the signer, configurable by the admin, determined by the CSR `spec.expirationSeconds` field, etc
and the behavior when the signer-determined expiration is different from the CSR `spec.expirationSeconds` field.
6. **CA bit allowed/disallowed**: and behavior if a CSR contains a request for a CA certificate when the signer does not permit it.
Commonly, the `status.certificate` field of a CertificateSigningRequest contains a
single PEM-encoded X.509 certificate once the CSR is approved and the certificate is issued.
Some signers store multiple certificates into the `status.certificate` field. In
that case, the documentation for the signer should specify the meaning of
additional certificates; for example, this might be the certificate plus
intermediates to be presented during TLS handshakes.
If you want to make the *trust anchor* (root certificate) available, this should be done
separately from a CertificateSigningRequest and its `status.certificate` field. For example,
you could use a ClusterTrustBundle.
The PKCS#10 signing request format does not have a standard mechanism to specify a
certificate expiration or lifetime. The expiration or lifetime therefore has to be set
through the `spec.expirationSeconds` field of the CSR object. The built-in signers
use the `ClusterSigningDuration` configuration option, which defaults to 1 year,
(the `--cluster-signing-duration` command-line flag of the kube-controller-manager)
as the default when no `spec.expirationSeconds` is specified. When `spec.expirationSeconds`
is specified, the minimum of `spec.expirationSeconds` and `ClusterSigningDuration` is
used.