---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#92-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 126
summary: `rpc FetchKeys(FetchKeysRequest) returns (FetchKeysResponse) {} message FetchKeysRequest {} message FetchKeysResponse { repeated Key keys = 1; // The timestamp when this data was pulled from the...
---

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