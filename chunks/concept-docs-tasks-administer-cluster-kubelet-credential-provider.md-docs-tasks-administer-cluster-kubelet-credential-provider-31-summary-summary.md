---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#31-summary
chunk_level: summary
chunk_type: prose
heading: Service Account Token for Image Pulls
token_count: 79
summary: and the `tokenAttributes` field must be set in the `CredentialProviderConfig` file for the plugin. The `tokenAttributes` field contains information about the service account token that will be passed...
---

and the `tokenAttributes` field must be set
in the `CredentialProviderConfig` file for the plugin.
The `tokenAttributes` field contains information
about the service account token that will be passed to the plugin,
including the intended audience for the token
and whether the plugin requires the pod to have a service account.
Using service account token credentials can enable the following use-cases: