---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#72-summary
chunk_level: summary
chunk_type: table
heading: `KubeProxyNFTablesConfiguration`
token_count: 127
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyNFTablesConfiguration contains nftables-related configuration details for the Kubernetes proxy server....
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyNFTablesConfiguration contains nftables-related configuration
details for the Kubernetes proxy server.
|Field|Description|
|`masqueradeBit`**[Required]**
`int32`|
masqueradeBit is the bit of the iptables fwmark space to use for SNAT if using
the nftables proxy mode. Values must be within the range [0, 31].
|
|`masqueradeAll`**[Required]**
`bool`|