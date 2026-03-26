---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#8-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 105
summary: | |`credentialPluginAllowlist` [`[]AllowlistEntry`](#kubectl-config-k8s-io-v1beta1-AllowlistEntry)| Allowlist is a slice of allowlist entries. If any of them is a match, then the executable in...
---

|
|`credentialPluginAllowlist`
[`[]AllowlistEntry`](#kubectl-config-k8s-io-v1beta1-AllowlistEntry)|
Allowlist is a slice of allowlist entries. If any of them is a match,
then the executable in question may execute. That is, the result is the
logical OR of all entries in the allowlist. This list MUST NOT be
supplied if the policy is not "Allowlist".
e.g.
credentialPluginAllowlist:
* name: cloud-provider-plugin