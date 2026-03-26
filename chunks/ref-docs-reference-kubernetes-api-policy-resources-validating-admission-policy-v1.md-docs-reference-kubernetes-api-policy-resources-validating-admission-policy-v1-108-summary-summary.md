---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#108-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 119
summary: *ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.* * **spec.paramRef.name** (string) name is the name of the resource being...
---

*ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.*
* **spec.paramRef.name** (string)
name is the name of the resource being referenced.
One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
A single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.