---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#2-standard
chunk_level: standard
chunk_type: prose
heading: Certificate signing requests
token_count: 75
summary: ## Certificate signing requests FEATURE STATE: `Kubernetes v1.19 [stable]` A [CertificateSigningRequest](/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/) (CSR)...
---

## Certificate signing requests
FEATURE STATE:
`Kubernetes v1.19 [stable]`
A [CertificateSigningRequest](/docs/reference/kubernetes-api/authentication-resources/certificate-signing-request-v1/)
(CSR) resource is used to request that a certificate be signed
by a denoted signer, after which the request may be approved or denied before
finally being signed.