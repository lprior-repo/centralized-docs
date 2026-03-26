---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#65-summary
chunk_level: summary
chunk_type: table
heading: `KubeProxyIPVSConfiguration`
token_count: 128
summary: * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyIPVSConfiguration contains ipvs-related configuration details for the Kubernetes proxy server....
---

* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyIPVSConfiguration contains ipvs-related configuration
details for the Kubernetes proxy server.
|Field|Description|
|`syncPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
syncPeriod is an interval (e.g. '5s', '1m', '2h22m') indicating how frequently
various re-synchronizing and cleanup operations are performed. Must be greater