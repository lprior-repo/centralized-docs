---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#6-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 81
summary: ## Signers Signers abstractly represent the entity or entities that might sign, or have signed, a security certificate. Any signer that is made available for outside a particular cluster should...
---

## Signers
Signers abstractly represent the entity or entities that might sign, or have
signed, a security certificate.
Any signer that is made available for outside a particular cluster should provide information
about how the signer works, so that consumers can understand what that means for CertificateSigningRequests
and (if enabled) [ClusterTrustBundles](#cluster-trust-bundles).
This includes: