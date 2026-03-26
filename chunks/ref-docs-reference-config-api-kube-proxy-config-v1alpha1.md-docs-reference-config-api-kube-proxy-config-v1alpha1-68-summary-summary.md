---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#68-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyIPVSConfiguration`
token_count: 116
summary: |`strictARP`**[Required]** `bool`| strictARP configures arp\_ignore and arp\_announce to avoid answering ARP queries from kube-ipvs0 interface | |`tcpTimeout`**[Required]**...
---

|`strictARP`**[Required]**
`bool`|
strictARP configures arp\_ignore and arp\_announce to avoid answering ARP queries
from kube-ipvs0 interface
|
|`tcpTimeout`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|
tcpTimeout is the timeout value used for idle IPVS TCP sessions.
The default value is 0, which preserves the current timeout value on the system.
|
|`tcpFinTimeout`**[Required]**