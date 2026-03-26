---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#36-summary
chunk_level: summary
chunk_type: table
heading: `KubeProxyConfiguration`
token_count: 117
summary: KubeProxyConfiguration contains everything necessary to configure the Kubernetes proxy server. |Field|Description| |`apiVersion` string|`kubeproxy.config.k8s.io/v1alpha1`| |`kind`...
---

KubeProxyConfiguration contains everything necessary to configure the
Kubernetes proxy server.
|Field|Description|
|`apiVersion`
string|`kubeproxy.config.k8s.io/v1alpha1`|
|`kind`
string|`KubeProxyConfiguration`|
|`featureGates`**[Required]**
`map[string]bool`|
featureGates is a map of feature names to bools that enable or disable alpha/experimental features.
|
|`clientConnection`**[Required]**
[`ClientConnectionConfiguration`](#ClientConnectionConfiguration)|