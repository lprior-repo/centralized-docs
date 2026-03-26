---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#67-summary
chunk_level: summary
chunk_type: prose
heading: `KubeProxyIPVSConfiguration`
token_count: 126
summary: '2h22m'). A value of 0 means every Service or EndpointSlice change will result in an immediate IPVS resync. | |`scheduler`**[Required]** `string`| scheduler is the IPVS scheduler to use |...
---

'2h22m'). A value of 0 means every Service or EndpointSlice change will result
in an immediate IPVS resync.
|
|`scheduler`**[Required]**
`string`|
scheduler is the IPVS scheduler to use
|
|`excludeCIDRs`**[Required]**
`[]string`|
excludeCIDRs is a list of CIDRs which the ipvs proxier should not touch
when cleaning up ipvs services.
|
|`strictARP`**[Required]**
`bool`|
strictARP configures arp\_ignore and arp\_announce to avoid answering ARP queries