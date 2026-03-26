---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: NetworkPolicySpec
token_count: 449
summary: * **ingress.from** ([]NetworkPolicyPeer) *Atomic: will be replaced during a merge* from is a list of sources which should be able to access the pods selected for this rule. Items in this list are...
---

* **ingress.from** ([]NetworkPolicyPeer)
*Atomic: will be replaced during a merge*
from is a list of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.
*NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed*
* **ingress.from.ipBlock** (IPBlock)
ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
*IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.*
* **ingress.from.ipBlock.cidr** (string), required
cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
* **ingress.from.ipBlock.except** ([]string)
*Atomic: will be replaced during a merge*
except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
* **ingress.from.namespaceSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.