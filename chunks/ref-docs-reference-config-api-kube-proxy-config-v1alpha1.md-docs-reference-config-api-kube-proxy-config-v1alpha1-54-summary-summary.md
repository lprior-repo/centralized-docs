---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#54-summary
chunk_level: summary
chunk_type: table
heading: `KubeProxyConntrackConfiguration`
token_count: 120
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyConntrackConfiguration contains conntrack settings for the Kubernetes proxy server. |Field|Description|...
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyConntrackConfiguration contains conntrack settings for
the Kubernetes proxy server.
|Field|Description|
|`maxPerCore`**[Required]**
`int32`|
maxPerCore is the maximum number of NAT connections to track
per CPU core (0 to leave the limit as-is and ignore min).
|
|`min`**[Required]**
`int32`|
min is the minimum value of connect-tracking records to allocate,