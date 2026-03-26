---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#21-summary
chunk_level: summary
chunk_type: prose
heading: `CredentialPluginPolicy`
token_count: 123
summary: * [Preference](#kubectl-config-k8s-io-v1beta1-Preference) CredentialPluginPolicy specifies the policy governing which, if any, client-go credential plugins may be executed. It MUST be one of { \"\",...
---

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
CredentialPluginPolicy specifies the policy governing which, if any, client-go
credential plugins may be executed. It MUST be one of { "", "AllowAll", "DenyAll", "Allowlist" }.
If the policy is "", then it falls back to "AllowAll" (this is required
to maintain backward compatibility). If the policy is DenyAll, no
credential plugins may run. If the policy is Allowlist, only those
plugins meeting the criteria specified in the `credentialPluginAllowlist`