---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#48-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 126
summary: the range [-1000, 1000] | |`conntrack`**[Required]** [`KubeProxyConntrackConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConntrackConfiguration)| conntrack contains conntrack-related...
---

the range [-1000, 1000]
|
|`conntrack`**[Required]**
[`KubeProxyConntrackConfiguration`](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConntrackConfiguration)|
conntrack contains conntrack-related configuration options.
|
|`configSyncPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
configSyncPeriod is how often configuration from the apiserver is refreshed. Must be greater
than 0.
|
|`portRange`