---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#4-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 492
summary: - [tokenAttributes is the configuration for the service account token that will be passed to the...
---

- [tokenAttributes is the configuration for the service account token that will be passed to the plugin.](#tokenattributes-is-the-configuration-for-the-service-account-token-that-will-be-passed-to-the-plugin)
- [The credential provider opts in to using service account tokens for image pull by setting this field.](#the-credential-provider-opts-in-to-using-service-account-tokens-for-image-pull-by-setting-this-field)
- [if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,](#if-this-field-is-set-without-the-kubeletserviceaccounttokenforcredentialproviders-feature-gate-enabled)
- [kubelet will fail to start with invalid configuration error.](#kubelet-will-fail-to-start-with-invalid-configuration-error)
- [serviceAccountTokenAudience is the intended audience for the projected service account token.](#serviceaccounttokenaudience-is-the-intended-audience-for-the-projected-service-account-token)
- [cacheType indicates the type of cache key use for caching the credentials returned by the plugin](#cachetype-indicates-the-type-of-cache-key-use-for-caching-the-credentials-returned-by-the-plugin)
- [The most conservative option is to set this to "Token", which means the kubelet will cache](#the-most-conservative-option-is-to-set-this-to-token-which-means-the-kubelet-will-cache)
- [returned credentials on a per-token basis. This should be set if the returned credential's](#returned-credentials-on-a-per-token-basis-this-should-be-set-if-the-returned-credentials)
- [lifetime is limited to the service account token's lifetime.](#lifetime-is-limited-to-the-service-account-tokens-lifetime)
- [If the plugin's credential retrieval logic depends only on the service account and not on](#if-the-plugins-credential-retrieval-logic-depends-only-on-the-service-account-and-not-on)
- [pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the](#pod-specific-claims-then-the-plugin-can-set-this-to-serviceaccount-in-this-case-the)
- [kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the](#kubelet-will-cache-returned-credentials-on-a-per-serviceaccount-basis-use-this-when-the)