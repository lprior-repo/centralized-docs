---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#38-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 124
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