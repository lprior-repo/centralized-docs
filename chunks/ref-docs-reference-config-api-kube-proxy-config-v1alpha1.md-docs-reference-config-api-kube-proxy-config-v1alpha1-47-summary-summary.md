---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#47-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 128
summary: 'primary'. If set to a list of CIDRs, connections to NodePort services will only be accepted on node IPs in one of the indicated ranges. If set to 'primary', NodePort services will only be accepted...
---

'primary'. If set to a list of CIDRs,
connections to NodePort services will only be accepted on node IPs in one of
the indicated ranges. If set to 'primary', NodePort services will only be
accepted on the node's primary IPv4 and/or IPv6 address according to the Node
object. If unset, NodePort connections will be accepted on all local IPs.
|
|`oomScoreAdj`**[Required]**
`int32`|
oomScoreAdj is the oom-score-adj value for kube-proxy process. Values must be within
the range [-1000, 1000]
|