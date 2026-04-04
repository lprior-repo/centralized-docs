---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#21-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 317
summary: ### Sign Sign takes a serialized JWT payload, and returns the serialized header and signature. `kube-apiserver` then assembles the JWT from the header, payload, and signature. ``` `rpc...
---

### Sign
Sign takes a serialized JWT payload, and returns the serialized header and
signature. `kube-apiserver` then assembles the JWT from the header, payload,
and signature.
```
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
// alg must be one of the algorithms supported by kube-apiserver (currently RS256, ES256, ES384, ES512).
// header cannot have any additional data that kube-apiserver does not recognize.
// Already wrapped in URL-safe base64, exactly as it appears in the first segment of the JWT.
string header = 1;
// The signature for the JWT.
// Already wrapped in URL-safe base64, exactly as it appears in the final segment of the JWT.
string signature = 2;
}
`
```
## Clean up
If you created a namespace `examplens` to experiment with, you can remove it:
```
`kubectl delete namespace examplens
`
```
## What's next
* Read more details about [projected volumes](/docs/concepts/storage/projected-volumes/).