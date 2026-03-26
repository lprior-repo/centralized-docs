---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#45-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 123
summary: # if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled, # kubelet will fail to start with invalid configuration error. #...
---

# if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,
# kubelet will fail to start with invalid configuration error.
# serviceAccountTokenAudience is the intended audience for the projected service account token.
# cacheType indicates the type of cache key use for caching the credentials returned by the plugin
# The most conservative option is to set this to "Token", which means the kubelet will cache
# returned credentials on a per-token basis. This should be set if the returned credential's
# lifetime is limited to the service account token's lifetime.