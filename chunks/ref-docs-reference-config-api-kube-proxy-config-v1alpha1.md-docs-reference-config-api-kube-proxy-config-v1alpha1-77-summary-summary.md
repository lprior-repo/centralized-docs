---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#77-summary
chunk_level: summary
chunk_type: table
heading: `KubeProxyWinkernelConfiguration`
token_count: 124
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyWinkernelConfiguration contains Windows/HNS settings for the Kubernetes proxy server. |Field|Description|...
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyWinkernelConfiguration contains Windows/HNS settings for
the Kubernetes proxy server.
|Field|Description|
|`networkName`**[Required]**
`string`|
networkName is the name of the network kube-proxy will use
to create endpoints and policies
|
|`sourceVip`**[Required]**
`string`|
sourceVip is the IP address of the source VIP endpoint used for
NAT when loadbalancing
|
|