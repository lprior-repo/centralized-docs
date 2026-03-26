---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#61-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyIPTablesConfiguration`
token_count: 127
summary: [0, 31]. | |`masqueradeAll`**[Required]** `bool`| masqueradeAll tells kube-proxy to SNAT all traffic sent to Service cluster IPs, when using the iptables or ipvs proxy mode. This may be required with...
---

[0, 31].
|
|`masqueradeAll`**[Required]**
`bool`|
masqueradeAll tells kube-proxy to SNAT all traffic sent to Service cluster IPs,
when using the iptables or ipvs proxy mode. This may be required with some CNI
plugins.
|
|`localhostNodePorts`**[Required]**
`bool`|
localhostNodePorts, if false, tells kube-proxy to disable the legacy behavior
of allowing NodePort services to be accessed via localhost. (Applies only to
iptables mode and IPv4; localhost NodePorts are never allowed with other proxy