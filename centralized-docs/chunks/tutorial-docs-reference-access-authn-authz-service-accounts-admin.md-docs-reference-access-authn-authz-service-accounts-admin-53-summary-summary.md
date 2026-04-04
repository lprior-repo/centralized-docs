---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#53-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 128
summary: * watches for Secret deletion and removes a reference from the corresponding ServiceAccount if needed. You must pass a service account private key file to the token controller in the...
---

* watches for Secret deletion and removes a reference from the corresponding
ServiceAccount if needed.
You must pass a service account private key file to the token controller in
the `kube-controller-manager` using the `--service-account-private-key-file`
flag. The private key is used to sign generated service account tokens.
Similarly, you must pass the corresponding public key to the `kube-apiserver`
using the `--service-account-key-file` flag. The public key will be used to
verify the tokens during authentication.
FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
An alternate setup to setting