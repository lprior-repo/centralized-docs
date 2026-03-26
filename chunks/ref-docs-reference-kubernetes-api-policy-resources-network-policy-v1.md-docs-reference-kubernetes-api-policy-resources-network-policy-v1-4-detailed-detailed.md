---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#4-detailed
chunk_level: detailed
chunk_type: prose
heading: NetworkPolicySpec
token_count: 967
summary: * **ingress.from.ipBlock.except** ([]string) *Atomic: will be replaced during a merge* except is a slice of CIDRs that should not be included within an IPBlock Valid examples are \"192.168.1.0/24\" or...
---

* **ingress.from.ipBlock.except** ([]string)
*Atomic: will be replaced during a merge*
except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
* **ingress.from.namespaceSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
* **ingress.from.podSelector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
* **ingress.ports** ([]NetworkPolicyPort)
*Atomic: will be replaced during a merge*
ports is a list of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
*NetworkPolicyPort describes a port to allow traffic on*
* **ingress.ports.port** (IntOrString)
port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
*IntOrString is a type that can hold an int32 or a string. When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type. This allows you to have, for example, a JSON field that can accept a name or number.*
* **ingress.ports.endPort** (int32)
endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
* **ingress.ports.protocol** (string)
protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
Possible enum values:
* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.
* **egress** ([]NetworkPolicyEgressRule)
*Atomic: will be replaced during a merge*
egress is a list of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
*NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8*
* **egress.to** ([]NetworkPolicyPeer)
*Atomic: will be replaced during a merge*
to is a list of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
*NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed*