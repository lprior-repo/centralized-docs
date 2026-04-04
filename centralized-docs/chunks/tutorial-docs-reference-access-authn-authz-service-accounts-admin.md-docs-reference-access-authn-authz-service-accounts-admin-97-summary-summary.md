---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#97-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 105
summary: `rpc Sign(SignJWTRequest) returns (SignJWTResponse) {} message SignJWTRequest { // URL-safe base64 wrapped payload to be signed. // Exactly as it appears in the second segment of the JWT string...
---

`rpc Sign(SignJWTRequest) returns (SignJWTResponse) {}
message SignJWTRequest {
// URL-safe base64 wrapped payload to be signed.
// Exactly as it appears in the second segment of the JWT
string claims = 1;
}
message SignJWTResponse {
// header must contain only alg, kid, typ claims.
// typ must be “JWT”.
// kid must be non-empty, &lt;=1024 characters, and its corresponding public key should not be excluded from OIDC discovery.