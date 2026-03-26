---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#73-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyNFTablesConfiguration`
token_count: 121
summary: [0, 31]. | |`masqueradeAll`**[Required]** `bool`| masqueradeAll tells kube-proxy to SNAT all traffic sent to Service cluster IPs, when using the nftables mode. This may be required with some CNI...
---

[0, 31].
|
|`masqueradeAll`**[Required]**
`bool`|
masqueradeAll tells kube-proxy to SNAT all traffic sent to Service cluster IPs,
when using the nftables mode. This may be required with some CNI plugins.
|
|`syncPeriod`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
syncPeriod is an interval (e.g. '5s', '1m', '2h22m') indicating how frequently