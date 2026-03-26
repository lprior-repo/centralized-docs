---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#12-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the Kubelet
token_count: 497
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