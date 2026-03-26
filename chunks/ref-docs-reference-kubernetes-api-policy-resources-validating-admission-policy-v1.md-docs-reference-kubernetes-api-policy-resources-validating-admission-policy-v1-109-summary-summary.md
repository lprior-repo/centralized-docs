---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#109-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 95
summary: * **spec.paramRef.namespace** (string) namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector`...
---

* **spec.paramRef.namespace** (string)
namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.
A per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.
* If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.