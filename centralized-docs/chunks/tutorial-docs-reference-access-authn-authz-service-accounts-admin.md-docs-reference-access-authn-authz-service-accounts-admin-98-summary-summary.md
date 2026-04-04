---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#98-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 128
summary: // kid must be non-empty, &lt;=1024 characters, and its corresponding public key should not be excluded from OIDC discovery. // alg must be one of the algorithms supported by kube-apiserver...
---

// kid must be non-empty, &lt;=1024 characters, and its corresponding public key should not be excluded from OIDC discovery.
// alg must be one of the algorithms supported by kube-apiserver (currently RS256, ES256, ES384, ES512).
// header cannot have any additional data that kube-apiserver does not recognize.
// Already wrapped in URL-safe base64, exactly as it appears in the first segment of the JWT.
string header = 1;
// The signature for the JWT.
// Already wrapped in URL-safe base64, exactly as it appears in the final segment of the JWT.