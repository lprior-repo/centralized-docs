---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#13-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the Kubelet
token_count: 503
summary: # - The URL path of an matchImages must be a prefix of the target image URL path. # - If the matchImages contains a port, then the port must match in the image as well. # - registry.io:8080/path...
---

# - The URL path of an matchImages must be a prefix of the target image URL path.
# - If the matchImages contains a port, then the port must match in the image as well.
# - registry.io:8080/path
matchImages:
- "\*.dkr.ecr.\*.amazonaws.com"
- "\*.dkr.ecr.\*.amazonaws.com.cn"
- "\*.dkr.ecr-fips.\*.amazonaws.com"
- "\*.dkr.ecr.us-iso-east-1.c2s.ic.gov"
- "\*.dkr.ecr.us-isob-east-1.sc2s.sgov.gov"
# defaultCacheDuration is the default duration the plugin will cache credentials in-memory
# if a cache duration is not provided in the plugin response. This field is required.
defaultCacheDuration: "12h"
# Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse
# MUST use the same encoding version as the input. Current supported values are:
# Arguments to pass to the command when executing it.
# Env defines additional environment variables to expose to the process. These
# are unioned with the host's environment, as well as variables client-go uses
# tokenAttributes is the configuration for the service account token that will be passed to the plugin.
# The credential provider opts in to using service account tokens for image pull by setting this field.
# if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,
# kubelet will fail to start with invalid configuration error.
# serviceAccountTokenAudience is the intended audience for the projected service account token.
# cacheType indicates the type of cache key use for caching the credentials returned by the plugin
# The most conservative option is to set this to "Token", which means the kubelet will cache
# returned credentials on a per-token basis. This should be set if the returned credential's
# lifetime is limited to the service account token's lifetime.
# If the plugin's credential retrieval logic depends only on the service account and not on
# pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the
# kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the
# returned credential is valid for all pods using the same service account.
# requireServiceAccount indicates whether the plugin requires the pod to have a service account.