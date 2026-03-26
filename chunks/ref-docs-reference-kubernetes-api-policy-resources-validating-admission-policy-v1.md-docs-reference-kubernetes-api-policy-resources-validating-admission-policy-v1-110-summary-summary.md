---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#110-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 82
summary: * If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error. * If `paramKind` is namespace-scoped, the namespace of the object being evaluated...
---

* If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.
* If `paramKind` is namespace-scoped, the namespace of the object being evaluated for admission will be used when this field is left unset. Take care that if this is left empty the binding must not match any cluster-scoped resources, which will result in an error.