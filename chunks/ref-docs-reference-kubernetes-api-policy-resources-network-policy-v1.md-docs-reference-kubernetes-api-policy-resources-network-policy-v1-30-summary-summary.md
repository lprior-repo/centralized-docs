---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#30-summary
chunk_level: summary
chunk_type: prose
heading: NetworkPolicySpec
token_count: 123
summary: * **egress.ports.port** (IntOrString) port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names...
---

* **egress.ports.port** (IntOrString)
port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
*IntOrString is a type that can hold an int32 or a string. When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type. This allows you to have, for example, a JSON field that can accept a name or number.*