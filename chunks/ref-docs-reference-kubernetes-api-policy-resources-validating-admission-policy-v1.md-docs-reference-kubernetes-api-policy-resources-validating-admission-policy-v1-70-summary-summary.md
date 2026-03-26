---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#70-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 110
summary: * **status.observedGeneration** (int64) The generation observed by the controller. * **status.typeChecking** (TypeChecking) The results of type checking for each expression. Presence of this field...
---

* **status.observedGeneration** (int64)
The generation observed by the controller.
* **status.typeChecking** (TypeChecking)
The results of type checking for each expression. Presence of this field indicates the completion of the type checking.
*TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy*
* **status.typeChecking.expressionWarnings** ([]ExpressionWarning)
*Atomic: will be replaced during a merge*
The type checking warnings for each expression.
*ExpressionWarning is a warning information that targets a specific expression.*