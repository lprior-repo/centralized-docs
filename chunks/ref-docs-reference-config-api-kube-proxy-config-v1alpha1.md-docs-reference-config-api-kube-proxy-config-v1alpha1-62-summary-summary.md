---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#62-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyIPTablesConfiguration`
token_count: 116
summary: iptables mode and IPv4; localhost NodePorts are never allowed with other proxy modes or with IPv6.) | |`syncPeriod`**[Required]**...
---

iptables mode and IPv4; localhost NodePorts are never allowed with other proxy
modes or with IPv6.)
|
|`syncPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
syncPeriod is an interval (e.g. '5s', '1m', '2h22m') indicating how frequently
various re-synchronizing and cleanup operations are performed. Must be greater
than 0.
|
|`minSyncPeriod`**[Required]**