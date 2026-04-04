---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#90-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 78
summary: ### FetchKeys FetchKeys returns the set of public keys that are trusted to sign Kubernetes service account tokens. Kube-apiserver will call this RPC: * Every time it tries to validate a JWT from the...
---

### FetchKeys
FetchKeys returns the set of public keys that are trusted to sign
Kubernetes service account tokens. Kube-apiserver will call this RPC:
* Every time it tries to validate a JWT from the service account issuer with an unknown key ID, and
* Periodically, so it can serve reasonably-up-to-date keys from the OIDC JWKs endpoint.