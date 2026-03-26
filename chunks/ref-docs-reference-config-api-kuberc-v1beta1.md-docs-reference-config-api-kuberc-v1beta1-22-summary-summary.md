---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#22-summary
chunk_level: summary
chunk_type: prose
heading: `CredentialPluginPolicy`
token_count: 54
summary: credential plugins may run. If the policy is Allowlist, only those plugins meeting the criteria specified in the `credentialPluginAllowlist` field may run. If the policy is not `Allowlist` but one is...
---

credential plugins may run. If the policy is Allowlist, only those
plugins meeting the criteria specified in the `credentialPluginAllowlist`
field may run. If the policy is not `Allowlist` but one is provided, it
is considered a configuration error.