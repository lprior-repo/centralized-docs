---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#44-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 127
summary: | nftables contains nftables-related configuration options. | |`winkernel`**[Required]** [`KubeProxyWinkernelConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration)|...
---

|
nftables contains nftables-related configuration options.
|
|`winkernel`**[Required]**
[`KubeProxyWinkernelConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration)|
winkernel contains winkernel-related configuration options.
|
|`detectLocalMode`**[Required]**
[`LocalMode`](#kubeproxy-config-k8s-io-v1alpha1-LocalMode)|
detectLocalMode determines mode to use for detecting local traffic, defaults to ClusterCIDR
|
|`detectLocal`