---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#42-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 122
summary: `string`| showHiddenMetricsForVersion is the version for which you want to show hidden metrics. | |`mode`**[Required]** [`ProxyMode`](#kubeproxy-config-k8s-io-v1alpha1-ProxyMode)| mode specifies...
---

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