---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#38-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 127
summary: `hostnameOverride`**[Required]** `string`| hostnameOverride, if non-empty, will be used as the name of the Node that kube-proxy is running on. If unset, the node name is assumed to be the same as the...
---

`hostnameOverride`**[Required]**
`string`|
hostnameOverride, if non-empty, will be used as the name of the Node that
kube-proxy is running on. If unset, the node name is assumed to be the same as
the node's hostname.
|
|`bindAddress`**[Required]**
`string`|
bindAddress can be used to override kube-proxy's idea of what its node's
primary IP is. Note that the name is a historical artifact, and kube-proxy does
not actually bind any sockets to this IP.
|
|`healthzBindAddress`**[Required]**