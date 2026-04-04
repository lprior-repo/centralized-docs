---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#86-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 43
summary: ### Metadata Metadata is meant to be called once by `kube-apiserver` on startup. This enables the external signer to share metadata with kube-apiserver, like the max token lifetime that signer...
---

### Metadata
Metadata is meant to be called once by `kube-apiserver` on startup.
This enables the external signer to share metadata with kube-apiserver, like the max token lifetime that signer supports.