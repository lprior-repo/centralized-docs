---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#51-summary
chunk_level: summary
chunk_type: table
heading: `DetectLocalConfiguration`
token_count: 128
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) DetectLocalConfiguration contains optional settings related to DetectLocalMode option |Field|Description|...
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
DetectLocalConfiguration contains optional settings related to DetectLocalMode option
|Field|Description|
|`bridgeInterface`**[Required]**
`string`|
bridgeInterface is a bridge interface name. When DetectLocalMode is set to
LocalModeBridgeInterface, kube-proxy will consider traffic to be local if
it originates from this bridge.
|
|`interfaceNamePrefix`**[Required]**
`string`|
interfaceNamePrefix is an interface name prefix. When DetectLocalMode is set to