---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#88-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 107
summary: `rpc Metadata(MetadataRequest) returns (MetadataResponse) {} message MetadataRequest {} message MetadataResponse { // used by kube-apiserver for defaulting/validation of JWT lifetime while accounting...
---

`rpc Metadata(MetadataRequest) returns (MetadataResponse) {}
message MetadataRequest {}
message MetadataResponse {
// used by kube-apiserver for defaulting/validation of JWT lifetime while accounting for configuration flag values:
// 1. `--service-account-max-token-expiration`
// 2. `--service-account-extend-token-expiration`
//
// \* If `--service-account-max-token-expiration` is greater than `max\_token\_expiration\_seconds`, kube-apiserver treats that as misconfiguration and exits.