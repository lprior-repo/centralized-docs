---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 777
summary: ## Auto-generated legacy ServiceAccount token clean up Before version 1.24, Kubernetes automatically generated Secret-based tokens for ServiceAccounts. To distinguish between automatically generated...
---

## Auto-generated legacy ServiceAccount token clean up
Before version 1.24, Kubernetes automatically generated Secret-based tokens for
ServiceAccounts. To distinguish between automatically generated tokens and
manually created ones, Kubernetes checks for a reference from the
ServiceAccount's secrets field. If the Secret is referenced in the `secrets`
field, it is considered an auto-generated legacy token. Otherwise, it is
considered a manually created legacy token. For example:
```
`apiVersion: v1
kind: ServiceAccount
metadata:
name: build-robot
namespace: default
secrets:
- name: build-robot-secret # usually NOT present for a manually generated token
`
```
Beginning from version 1.29, legacy ServiceAccount tokens that were generated
automatically will be marked as invalid if they remain unused for a certain
period of time (set to default at one year). Tokens that continue to be unused
for this defined period (again, by default, one year) will subsequently be
purged by the control plane.
If users use an invalidated auto-generated token, the token validator will
1. add an audit annotation for the key-value pair
`authentication.k8s.io/legacy-token-invalidated: &lt;secret name&gt;/&lt;namespace&gt;`,
2. increment the `invalid\_legacy\_auto\_token\_uses\_total` metric count,
3. update the Secret label `kubernetes.io/legacy-token-last-used` with the new
date,
4. return an error indicating that the token has been invalidated.
When receiving this validation error, users can update the Secret to remove the
`kubernetes.io/legacy-token-invalid-since` label to temporarily allow use of
this token.
Here's an example of an auto-generated legacy token that has been marked with the
`kubernetes.io/legacy-token-last-used` and `kubernetes.io/legacy-token-invalid-since`
labels:
```
`apiVersion: v1
kind: Secret
metadata:
name: build-robot-secret
namespace: default
labels:
kubernetes.io/legacy-token-last-used: 2022-10-24
kubernetes.io/legacy-token-invalid-since: 2023-10-25
annotations:
kubernetes.io/service-account.name: build-robot
type: kubernetes.io/service-account-token
`
```
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