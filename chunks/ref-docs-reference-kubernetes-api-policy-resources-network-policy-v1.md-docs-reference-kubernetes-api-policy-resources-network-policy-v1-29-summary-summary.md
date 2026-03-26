---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#29-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 107
summary: * **egress.ports** ([]NetworkPolicyPort) *Atomic: will be replaced during a merge* ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If...
---

* **egress.ports** ([]NetworkPolicyPort)
*Atomic: will be replaced during a merge*
ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
*NetworkPolicyPort describes a port to allow traffic on*