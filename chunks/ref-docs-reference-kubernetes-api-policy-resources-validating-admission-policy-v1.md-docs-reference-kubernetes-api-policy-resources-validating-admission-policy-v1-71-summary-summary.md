---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#71-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 100
summary: * **status.typeChecking.expressionWarnings.fieldRef** (string), required The path to the field that refers the expression. For example, the reference to the expression of the first item of...
---

* **status.typeChecking.expressionWarnings.fieldRef** (string), required
The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is "spec.validations[0].expression"
* **status.typeChecking.expressionWarnings.warning** (string), required
The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.