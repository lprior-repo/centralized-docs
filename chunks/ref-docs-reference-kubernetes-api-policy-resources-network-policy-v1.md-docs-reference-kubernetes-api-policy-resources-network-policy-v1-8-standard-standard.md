---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#8-standard
chunk_level: standard
chunk_type: prose
heading: NetworkPolicySpec
token_count: 498
summary: * **egress.ports** ([]NetworkPolicyPort) *Atomic: will be replaced during a merge* ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If...
---

* **egress.ports** ([]NetworkPolicyPort)
*Atomic: will be replaced during a merge*
ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
*NetworkPolicyPort describes a port to allow traffic on*
* **egress.ports.port** (IntOrString)
port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
*IntOrString is a type that can hold an int32 or a string. When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type. This allows you to have, for example, a JSON field that can accept a name or number.*
* **egress.ports.endPort** (int32)
endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
* **egress.ports.protocol** (string)
protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
Possible enum values:
* `"SCTP"` is the SCTP protocol.
* `"TCP"` is the TCP protocol.
* `"UDP"` is the UDP protocol.## NetworkPolicyList
NetworkPolicyList is a list of NetworkPolicy objects.
* **apiVersion**: networking.k8s.io/v1
* **kind**: NetworkPolicyList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)