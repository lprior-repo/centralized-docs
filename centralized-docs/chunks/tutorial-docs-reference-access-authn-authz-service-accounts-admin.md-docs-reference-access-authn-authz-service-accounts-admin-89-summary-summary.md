---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#89-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 104
summary: // \* If `--service-account-max-token-expiration` is not explicitly set, kube-apiserver defaults to `max\_token\_expiration\_seconds`. // \* If `--service-account-extend-token-expiration` is true,...
---

// \* If `--service-account-max-token-expiration` is not explicitly set, kube-apiserver defaults to `max\_token\_expiration\_seconds`.
// \* If `--service-account-extend-token-expiration` is true, the extended expiration is `min(1 year, max\_token\_expiration\_seconds)`.
//
// `max\_token\_expiration\_seconds` must be at least 600s.
int64 max\_token\_expiration\_seconds = 1;
}
`
```