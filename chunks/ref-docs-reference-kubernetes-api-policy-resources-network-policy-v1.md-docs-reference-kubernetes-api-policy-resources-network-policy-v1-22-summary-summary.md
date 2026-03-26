---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#22-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 127
summary: * **egress** ([]NetworkPolicyEgressRule) *Atomic: will be replaced during a merge* egress is a list of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no...
---

* **egress** ([]NetworkPolicyEgressRule)
*Atomic: will be replaced during a merge*
egress is a list of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8