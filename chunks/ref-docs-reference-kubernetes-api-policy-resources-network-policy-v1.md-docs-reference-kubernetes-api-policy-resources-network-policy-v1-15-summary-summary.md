---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#15-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 126
summary: * **ingress.from.ipBlock.cidr** (string), required cidr is a string representing the IPBlock Valid examples are \"192.168.1.0/24\" or \"2001:db8::/64\" * **ingress.from.ipBlock.except** ([]string)...
---

* **ingress.from.ipBlock.cidr** (string), required
cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
* **ingress.from.ipBlock.except** ([]string)
*Atomic: will be replaced during a merge*
except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range