---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#12-standard
chunk_level: standard
chunk_type: table
heading: `KubeProxyConntrackConfiguration`
token_count: 509
summary: **Appears in:** * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) KubeProxyConntrackConfiguration contains conntrack settings for the Kubernetes proxy server....
---

**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
KubeProxyConntrackConfiguration contains conntrack settings for
the Kubernetes proxy server.
|Field|Description|
|`maxPerCore`**[Required]**
`int32`|
maxPerCore is the maximum number of NAT connections to track
per CPU core (0 to leave the limit as-is and ignore min).
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
|`tcpCloseWaitTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
tcpCloseWaitTimeout is how long an idle conntrack entry
in CLOSE\_WAIT state will remain in the conntrack
table. (e.g. '60s'). Must be greater than 0 to set.
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
(e.g. '30s'). Must be greater than 0 to set.
|
|`udpStreamTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
udpStreamTimeout is how long an idle UDP conntrack entry in
ASSURED state will remain in the conntrack table
(e.g. '300s'). Must be greater than 0 to set.
|