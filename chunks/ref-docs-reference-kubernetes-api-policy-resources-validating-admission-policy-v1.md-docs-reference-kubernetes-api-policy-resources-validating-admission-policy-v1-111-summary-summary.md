---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#111-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 104
summary: * **spec.paramRef.parameterNotFoundAction** (string) `parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no...
---

* **spec.paramRef.parameterNotFoundAction** (string)
`parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no parameters matched by the binding. If the value is set to `Allow`, then no matched parameters will be treated as successful validation by the binding. If set to `Deny`, then no matched parameters will be subject to the `failurePolicy` of the policy.
Allowed values are `Allow` or `Deny`
Required