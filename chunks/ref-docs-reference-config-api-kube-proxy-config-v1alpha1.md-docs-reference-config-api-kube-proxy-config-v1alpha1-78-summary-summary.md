---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#78-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyWinkernelConfiguration`
token_count: 121
summary: `string`| sourceVip is the IP address of the source VIP endpoint used for NAT when loadbalancing | |`enableDSR`**[Required]** `bool`| enableDSR tells kube-proxy whether HNS policies should be created...
---

`string`|
sourceVip is the IP address of the source VIP endpoint used for
NAT when loadbalancing
|
|`enableDSR`**[Required]**
`bool`|
enableDSR tells kube-proxy whether HNS policies should be created
with DSR
|
|`rootHnsEndpointName`**[Required]**
`string`|
rootHnsEndpointName is the name of hnsendpoint that is attached to
l2bridge for root network namespace
|
|`forwardHealthCheckVip`**[Required]**
`bool`|