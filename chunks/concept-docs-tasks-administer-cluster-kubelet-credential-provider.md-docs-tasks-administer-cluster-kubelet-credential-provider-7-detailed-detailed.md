---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Configuring the Kubelet
token_count: 1019
summary: `apiVersion: kubelet.config.k8s.io/v1 kind: CredentialProviderConfig # providers is a list of credential provider helper plugins that will be enabled by the kubelet. # Multiple providers may match...
---

`apiVersion: kubelet.config.k8s.io/v1
kind: CredentialProviderConfig
# providers is a list of credential provider helper plugins that will be enabled by the kubelet.
# Multiple providers may match against a single image, in which case credentials
# from all providers will be returned to the kubelet. If multiple providers are called
# for a single image, the results are combined. If providers return overlapping
# auth keys, the value from the provider earlier in this list is used.
providers:
# name is the required name of the credential provider. It must match the name of the
# provider executable as seen by the kubelet. The executable must be in the kubelet's
# bin directory (set by the --image-credential-provider-bin-dir flag).
- name: ecr-credential-provider
# matchImages is a required list of strings used to match against images in order to
# determine if this provider should be invoked. If one of the strings matches the
# requested image from the kubelet, the plugin will be invoked and given a chance
# to provide credentials. Images are expected to contain the registry domain
# Each entry in matchImages is a pattern which can optionally contain a port and a path.
# Globs can be used in the domain, but not in the port or the path. Globs are supported
# as subdomains like '\*.k8s.io' or 'k8s.\*.io', and top-level-domains such as 'k8s.\*'.
# Matching partial subdomains like 'app\*.k8s.io' is also supported. Each glob can only match
# a single subdomain segment, so `\*.io` does \*\*not\*\* match `\*.k8s.io`.
# A match exists between an image and a matchImage when all of the below are true:
# - Both contain the same number of domain parts and each part matches.
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
# If set to true, kubelet will only invoke the plugin if the pod has a service account.
# If set to false, kubelet will invoke the plugin even if the pod does not have a service account
# and will not include a token in the CredentialProviderRequest. This is useful for plugins
# that are used to pull images for pods without service accounts (e.g., static pods).
# requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are required to be present in the service account.