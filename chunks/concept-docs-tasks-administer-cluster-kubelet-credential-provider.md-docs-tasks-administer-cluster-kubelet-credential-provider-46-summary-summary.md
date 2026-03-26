---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#46-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 112
summary: This should be set if the returned credential's # lifetime is limited to the service account token's lifetime. # If the plugin's credential retrieval logic depends only on the service account and not...
---

This should be set if the returned credential's
# lifetime is limited to the service account token's lifetime.
# If the plugin's credential retrieval logic depends only on the service account and not on
# pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the
# kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the
# returned credential is valid for all pods using the same service account.
# requireServiceAccount indicates whether the plugin requires the pod to have a service account.