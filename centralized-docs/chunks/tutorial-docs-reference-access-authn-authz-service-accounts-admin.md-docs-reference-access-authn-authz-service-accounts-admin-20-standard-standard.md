---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#20-standard
chunk_level: standard
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 349
summary: ### FetchKeys FetchKeys returns the set of public keys that are trusted to sign Kubernetes service account tokens. Kube-apiserver will call this RPC: * Every time it tries to validate a JWT from the...
---

### FetchKeys
FetchKeys returns the set of public keys that are trusted to sign
Kubernetes service account tokens. Kube-apiserver will call this RPC:
* Every time it tries to validate a JWT from the service account issuer with an unknown key ID, and
* Periodically, so it can serve reasonably-up-to-date keys from the OIDC JWKs endpoint.
```
`rpc FetchKeys(FetchKeysRequest) returns (FetchKeysResponse) {}
message FetchKeysRequest {}
message FetchKeysResponse {
repeated Key keys = 1;
// The timestamp when this data was pulled from the authoritative source of
// truth for verification keys.
// kube-apiserver can export this from metrics, to enable end-to-end SLOs.
google.protobuf.Timestamp data\_timestamp = 2;
// refresh interval for verification keys to pick changes if any.
// any value &lt;= 0 is considered a misconfiguration.
int64 refresh\_hint\_seconds = 3;
}
message Key {
// A unique identifier for this key.
// Length must be &lt;=1024.
string key\_id = 1;
// The public key, PKIX-serialized.
// must be a public key supported by kube-apiserver (currently RSA 256 or ECDSA 256/384/521)
bytes key = 2;
// Set only for keys that are not used to sign bound tokens.
// eg: supported keys for legacy tokens.
// If set, key is used for verification but excluded from OIDC discovery docs.
// if set, external signer should not use this key to sign a JWT.
bool exclude\_from\_oidc\_discovery = 3;
}
`
```