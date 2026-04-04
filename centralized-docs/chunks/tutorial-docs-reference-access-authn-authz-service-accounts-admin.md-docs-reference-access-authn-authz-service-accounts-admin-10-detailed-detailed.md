---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#10-detailed
chunk_level: detailed
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 888
summary: ## External ServiceAccount token signing and key management FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) The kube-apiserver can be configured to use external signer for token signing...
---

## External ServiceAccount token signing and key management
FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
The kube-apiserver can be configured to use external signer for token signing and token verifying key management.
This feature enables kubernetes distributions to integrate with key management solutions of their choice
(for example, HSMs, cloud KMSes) for service account credential signing and verification.
To configure kube-apiserver to use external-jwt-signer set the `--service-account-signing-endpoint` flag
to the location of a Unix domain socket (UDS) on a filesystem, or be prefixed with an @ symbol and name
a UDS in the abstract socket namespace. At the configured UDS shall be an RPC server which implements
an `ExternalJWTSigner` gRPC service.
The external-jwt-signer must be healthy and be ready to serve supported service account keys for the kube-apiserver to start.
#### Note:
The kube-apiserver flags `--service-account-key-file` and `--service-account-signing-key-file` will continue
to be used for reading from files unless `--service-account-signing-endpoint` is set; they are mutually
exclusive ways of supporting JWT signing and authentication.
An external signer provides a `v1.ExternalJWTSigner` gRPC service that implements 3 methods:
### Metadata
Metadata is meant to be called once by `kube-apiserver` on startup.
This enables the external signer to share metadata with kube-apiserver, like the max token lifetime that signer supports.
```
`rpc Metadata(MetadataRequest) returns (MetadataResponse) {}
message MetadataRequest {}
message MetadataResponse {
// used by kube-apiserver for defaulting/validation of JWT lifetime while accounting for configuration flag values:
// 1. `--service-account-max-token-expiration`
// 2. `--service-account-extend-token-expiration`
//
// \* If `--service-account-max-token-expiration` is greater than `max\_token\_expiration\_seconds`, kube-apiserver treats that as misconfiguration and exits.
// \* If `--service-account-max-token-expiration` is not explicitly set, kube-apiserver defaults to `max\_token\_expiration\_seconds`.
// \* If `--service-account-extend-token-expiration` is true, the extended expiration is `min(1 year, max\_token\_expiration\_seconds)`.
//
// `max\_token\_expiration\_seconds` must be at least 600s.
int64 max\_token\_expiration\_seconds = 1;
}
`
```
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