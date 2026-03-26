---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 1001
summary: - [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches) - [- The URL path of an matchImages must be a prefix...
---

- [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches)
- [- The URL path of an matchImages must be a prefix of the target image URL path.](#--the-url-path-of-an-matchimages-must-be-a-prefix-of-the-target-image-url-path)
- [- If the matchImages contains a port, then the port must match in the image as well.](#--if-the-matchimages-contains-a-port-then-the-port-must-match-in-the-image-as-well)
- [- registry.io:8080/path](#--registryio8080path)
- [defaultCacheDuration is the default duration the plugin will cache credentials in-memory](#defaultcacheduration-is-the-default-duration-the-plugin-will-cache-credentials-in-memory)
- [if a cache duration is not provided in the plugin response. This field is required.](#if-a-cache-duration-is-not-provided-in-the-plugin-response-this-field-is-required)
- [Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse](#required-input-version-of-the-exec-credentialproviderrequest-the-returned-credentialproviderresponse)
- [MUST use the same encoding version as the input. Current supported values are:](#must-use-the-same-encoding-version-as-the-input-current-supported-values-are)
- [Arguments to pass to the command when executing it.](#arguments-to-pass-to-the-command-when-executing-it)
- [Env defines additional environment variables to expose to the process. These](#env-defines-additional-environment-variables-to-expose-to-the-process-these)
- [are unioned with the host's environment, as well as variables client-go uses](#are-unioned-with-the-hosts-environment-as-well-as-variables-client-go-uses)
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
- [returned credential is valid for all pods using the same service account.](#returned-credential-is-valid-for-all-pods-using-the-same-service-account)
- [requireServiceAccount indicates whether the plugin requires the pod to have a service account.](#requireserviceaccount-indicates-whether-the-plugin-requires-the-pod-to-have-a-service-account)
- [If set to true, kubelet will only invoke the plugin if the pod has a service account.](#if-set-to-true-kubelet-will-only-invoke-the-plugin-if-the-pod-has-a-service-account)