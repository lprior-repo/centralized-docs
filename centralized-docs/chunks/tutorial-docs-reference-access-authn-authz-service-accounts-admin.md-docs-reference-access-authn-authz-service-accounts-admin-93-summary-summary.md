---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#93-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 123
summary: // any value &lt;= 0 is considered a misconfiguration. int64 refresh\_hint\_seconds = 3; } message Key { // A unique identifier for this key. // Length must be &lt;=1024. string key\_id = 1; // The...
---

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