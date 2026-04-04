---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#11-standard
chunk_level: standard
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 290
summary: ### ServiceAccount controller A ServiceAccount controller manages the ServiceAccounts inside namespaces, and ensures a ServiceAccount named \"default\" exists in every active namespace. ### Token...
---

### ServiceAccount controller
A ServiceAccount controller manages the ServiceAccounts inside namespaces, and
ensures a ServiceAccount named "default" exists in every active namespace.
### Token controller
The service account token controller runs as part of `kube-controller-manager`.
This controller acts asynchronously. It:
* watches for ServiceAccount deletion and deletes all corresponding ServiceAccount
token Secrets.
* watches for ServiceAccount token Secret addition, and ensures the referenced
ServiceAccount exists, and adds a token to the Secret if needed.
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
An alternate setup to setting `--service-account-private-key-file` and `--service-account-key-file` flags is
to configure an external JWT signer for [external ServiceAccount token signing and key management](#external-serviceaccount-token-signing-and-key-management).
Note that these setups are mutually exclusive and cannot be configured together.