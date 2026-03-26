---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#57-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 85
summary: * If set to `true`, kubelet will only invoke the plugin if the pod has a service account. * If set to `false`, kubelet will invoke the plugin even if the pod does not have a service account and will...
---

* If set to `true`, kubelet will only invoke the plugin
if the pod has a service account.
* If set to `false`, kubelet will invoke the plugin
even if the pod does not have a service account
and will not include a token in the `CredentialProviderRequest`.
This is useful for plugins that are used
to pull images for pods without service accounts
(e.g., static pods).