---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#54-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 89
summary: verify the tokens during authentication. FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) An alternate setup to setting `--service-account-private-key-file` and...
---

verify the tokens during authentication.
FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
An alternate setup to setting `--service-account-private-key-file` and `--service-account-key-file` flags is
to configure an external JWT signer for [external ServiceAccount token signing and key management](#external-serviceaccount-token-signing-and-key-management).
Note that these setups are mutually exclusive and cannot be configured together.