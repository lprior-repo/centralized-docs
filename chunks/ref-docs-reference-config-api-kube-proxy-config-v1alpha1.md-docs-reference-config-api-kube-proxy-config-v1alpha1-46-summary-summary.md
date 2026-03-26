---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#46-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyConfiguration`
token_count: 122
summary: clusters, this can be a comma-separated dual-stack pair of CIDR ranges.). When DetectLocalMode is set to ClusterCIDR, kube-proxy will consider traffic to be local if its source IP is in this range....
---

clusters, this can be a comma-separated dual-stack pair of CIDR ranges.). When
DetectLocalMode is set to ClusterCIDR, kube-proxy will consider
traffic to be local if its source IP is in this range. (Otherwise it is not
used.)
|
|`nodePortAddresses`**[Required]**
`[]string`|
nodePortAddresses is a list of CIDR ranges that contain valid node IPs, or
alternatively, the single string 'primary'. If set to a list of CIDRs,
connections to NodePort services will only be accepted on node IPs in one of