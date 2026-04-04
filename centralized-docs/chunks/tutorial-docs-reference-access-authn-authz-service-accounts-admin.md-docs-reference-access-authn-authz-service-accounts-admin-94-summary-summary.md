---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#94-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 79
summary: bytes key = 2; // Set only for keys that are not used to sign bound tokens. // eg: supported keys for legacy tokens. // If set, key is used for verification but excluded from OIDC discovery docs. //...
---

bytes key = 2;
// Set only for keys that are not used to sign bound tokens.
// eg: supported keys for legacy tokens.
// If set, key is used for verification but excluded from OIDC discovery docs.
// if set, external signer should not use this key to sign a JWT.
bool exclude\_from\_oidc\_discovery = 3;
}
`
```