---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#17-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 192
summary: * **status.typeChecking** (TypeChecking) The results of type checking for each expression. Presence of this field indicates the completion of the type checking. *TypeChecking contains results of type...
---

* **status.typeChecking** (TypeChecking)
The results of type checking for each expression. Presence of this field indicates the completion of the type checking.
*TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy*
* **status.typeChecking.expressionWarnings** ([]ExpressionWarning)
*Atomic: will be replaced during a merge*
The type checking warnings for each expression.
*ExpressionWarning is a warning information that targets a specific expression.*
* **status.typeChecking.expressionWarnings.fieldRef** (string), required
The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is "spec.validations[0].expression"
* **status.typeChecking.expressionWarnings.warning** (string), required
The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.