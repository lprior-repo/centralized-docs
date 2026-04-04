---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#85-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 85
summary: #### Note: The kube-apiserver flags `--service-account-key-file` and `--service-account-signing-key-file` will continue to be used for reading from files unless `--service-account-signing-endpoint`...
---

#### Note:
The kube-apiserver flags `--service-account-key-file` and `--service-account-signing-key-file` will continue
to be used for reading from files unless `--service-account-signing-endpoint` is set; they are mutually
exclusive ways of supporting JWT signing and authentication.
An external signer provides a `v1.ExternalJWTSigner` gRPC service that implements 3 methods: