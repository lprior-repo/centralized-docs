---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#57-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConntrackConfiguration`
token_count: 128
summary: '60s'). Must be greater than 0 to set. | |`tcpBeLiberal`**[Required]** `bool`| tcpBeLiberal, if true, kube-proxy will configure conntrack to run in liberal mode for TCP connections and packets with...
---

'60s'). Must be greater than 0 to set.
|
|`tcpBeLiberal`**[Required]**
`bool`|
tcpBeLiberal, if true, kube-proxy will configure conntrack
to run in liberal mode for TCP connections and packets with
out-of-window sequence numbers won't be marked INVALID.
|
|`udpTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
udpTimeout is how long an idle UDP conntrack entry in
UNREPLIED state will remain in the conntrack table