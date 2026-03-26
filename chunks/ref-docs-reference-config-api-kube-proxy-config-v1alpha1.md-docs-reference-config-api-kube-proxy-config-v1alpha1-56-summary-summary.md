---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#56-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConntrackConfiguration`
token_count: 115
summary: (e.g. '2s'). Must be greater than 0 to set. | |`tcpCloseWaitTimeout`**[Required]** [`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)| tcpCloseWaitTimeout is how...
---

(e.g. '2s'). Must be greater than 0 to set.
|
|`tcpCloseWaitTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
tcpCloseWaitTimeout is how long an idle conntrack entry
in CLOSE\_WAIT state will remain in the conntrack
table. (e.g. '60s'). Must be greater than 0 to set.
|
|`tcpBeLiberal`**[Required]**
`bool`|