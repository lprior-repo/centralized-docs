---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#8-standard
chunk_level: standard
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 506
summary: on, defaulting to \"127.0.0.1:10249\" (if bindAddress is unset or IPv4), or \"[::1]:10249\" (if bindAddress is IPv6). (Set to \"0.0.0.0:10249\" / \"[::]:10249\" to bind on all interfaces.) |...
---

on, defaulting to "127.0.0.1:10249" (if bindAddress is unset or IPv4), or
"[::1]:10249" (if bindAddress is IPv6). (Set to "0.0.0.0:10249" / "[::]:10249"
to bind on all interfaces.)
|
|`bindAddressHardFail`**[Required]**
`bool`|
bindAddressHardFail, if true, tells kube-proxy to treat failure to bind to a
port as fatal and exit
|
|`enableProfiling`**[Required]**
`bool`|
enableProfiling enables profiling via web interface on /debug/pprof handler.
Profiling handlers will be handled by metrics server.
|
|`showHiddenMetricsForVersion`**[Required]**
`string`|
showHiddenMetricsForVersion is the version for which you want to show hidden metrics.
|
|`mode`**[Required]**
[`ProxyMode`](#kubeproxy-config-k8s-io-v1alpha1-ProxyMode)|
mode specifies which proxy mode to use.
|
|`iptables`**[Required]**
[`KubeProxyIPTablesConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyIPTablesConfiguration)|
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
[`KubeProxyWinkernelConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyWinkernelConfiguration)|
winkernel contains winkernel-related configuration options.
|
|`detectLocalMode`**[Required]**
[`LocalMode`](#kubeproxy-config-k8s-io-v1alpha1-LocalMode)|
detectLocalMode determines mode to use for detecting local traffic, defaults to ClusterCIDR
|
|`detectLocal`**[Required]**