---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#55-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConntrackConfiguration`
token_count: 125
summary: | |`min`**[Required]** `int32`| min is the minimum value of connect-tracking records to allocate, regardless of maxPerCore (set maxPerCore=0 to leave the limit as-is). |...
---

|
|`min`**[Required]**
`int32`|
min is the minimum value of connect-tracking records to allocate,
regardless of maxPerCore (set maxPerCore=0 to leave the limit as-is).
|
|`tcpEstablishedTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
tcpEstablishedTimeout is how long an idle TCP connection will be kept open
(e.g. '2s'). Must be greater than 0 to set.
|
|`tcpCloseWaitTimeout`