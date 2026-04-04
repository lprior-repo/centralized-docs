---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#19-standard
chunk_level: standard
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 256
summary: ### Metadata Metadata is meant to be called once by `kube-apiserver` on startup. This enables the external signer to share metadata with kube-apiserver, like the max token lifetime that signer...
---

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