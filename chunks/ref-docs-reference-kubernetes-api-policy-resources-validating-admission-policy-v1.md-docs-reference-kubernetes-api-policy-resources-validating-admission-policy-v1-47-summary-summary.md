---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#47-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 123
summary: * `\"Namespaced\"` means that scope is limited to namespaced objects. * **spec.paramKind** (ParamKind) ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are...
---

* `"Namespaced"` means that scope is limited to namespaced objects.
* **spec.paramKind** (ParamKind)
ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
*ParamKind is a tuple of Group Kind and Version.*