---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#6-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 124
summary: * node \"kubectl getn control-plane-1\" expands to \"kubectl get node control-plane-1 --output=wide\" \"kubectl getn control-plane-1 --output=json\" expands to \"kubectl get node --output=json...
---

* node
"kubectl getn control-plane-1" expands to "kubectl get node control-plane-1 --output=wide"
"kubectl getn control-plane-1 --output=json" expands to "kubectl get node --output=json control-plane-1"|
|`credentialPluginPolicy`
[`CredentialPluginPolicy`](#kubectl-config-k8s-io-v1beta1-CredentialPluginPolicy)|
credentialPluginPolicy specifies the policy governing which, if any, client-go
credential plugins may be executed. It MUST be one of { "", "AllowAll", "DenyAll", "Allowlist" }.