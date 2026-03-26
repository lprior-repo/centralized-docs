---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#64-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 65
summary: *Variable is the definition of a variable that is used for composition. A variable is defined as a named expression.* * **spec.variables.expression** (string), required Expression is the expression...
---

*Variable is the definition of a variable that is used for composition. A variable is defined as a named expression.*
* **spec.variables.expression** (string), required
Expression is the expression that will be evaluated as the value of the variable. The CEL expression has access to the same identifiers as the CEL expressions in Validation.