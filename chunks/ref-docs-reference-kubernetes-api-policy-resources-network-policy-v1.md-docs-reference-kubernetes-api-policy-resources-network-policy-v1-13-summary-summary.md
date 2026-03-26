---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#13-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 123
summary: * **ingress.from** ([]NetworkPolicyPeer) *Atomic: will be replaced during a merge* from is a list of sources which should be able to access the pods selected for this rule. Items in this list are...
---

* **ingress.from** ([]NetworkPolicyPeer)
*Atomic: will be replaced during a merge*
from is a list of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.
*NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed*