---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#24-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 120
summary: * **egress.to** ([]NetworkPolicyPeer) *Atomic: will be replaced during a merge* to is a list of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using...
---

* **egress.to** ([]NetworkPolicyPeer)
*Atomic: will be replaced during a merge*
to is a list of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
*NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed*