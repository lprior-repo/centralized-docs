---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#14-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 98
summary: * **ingress.from.ipBlock** (IPBlock) ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be. *IPBlock describes a particular CIDR (Ex....
---

* **ingress.from.ipBlock** (IPBlock)
ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
*IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.*