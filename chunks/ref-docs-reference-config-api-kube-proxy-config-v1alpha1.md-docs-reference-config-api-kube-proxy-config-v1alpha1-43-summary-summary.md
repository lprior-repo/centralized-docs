---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#43-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 125
summary: | iptables contains iptables-related configuration options. | |`ipvs`**[Required]** [`KubeProxyIPVSConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyIPVSConfiguration)| ipvs contains...
---

|
iptables contains iptables-related configuration options.
|
|`ipvs`**[Required]**
[`KubeProxyIPVSConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyIPVSConfiguration)|
ipvs contains ipvs-related configuration options.
|
|`nftables`**[Required]**
[`KubeProxyNFTablesConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyNFTablesConfiguration)|
nftables contains nftables-related configuration options.
|
|`winkernel`**[Required]**