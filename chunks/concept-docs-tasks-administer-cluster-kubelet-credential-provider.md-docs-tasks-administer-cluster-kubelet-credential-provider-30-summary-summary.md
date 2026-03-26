---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#30-summary
chunk_level: summary
chunk_type: prose
heading: Service Account Token for Image Pulls
token_count: 127
summary: FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) Starting from Kubernetes v1.33, the kubelet can be configured to send a service account token bound to the pod for which the image pull is...
---

FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
Starting from Kubernetes v1.33,
the kubelet can be configured to send a service account token
bound to the pod for which the image pull is being performed
to the credential provider plugin.
This allows the plugin to exchange the token for credentials
to access the image registry.
To enable this feature,
the `KubeletServiceAccountTokenForCredentialProviders` feature gate
must be enabled on the kubelet,
and the `tokenAttributes` field must be set
in the `CredentialProviderConfig` file for the plugin.
The