---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#48-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 123
summary: * **spec.paramKind.apiVersion** (string) APIVersion is the API group version the resources belong to. In format of \"group/version\". Required. * **spec.paramKind.kind** (string) Kind is the API kind...
---

* **spec.paramKind.apiVersion** (string)
APIVersion is the API group version the resources belong to. In format of "group/version". Required.
* **spec.paramKind.kind** (string)
Kind is the API kind the resources belong to. Required.
* **spec.validations** ([]Validation)
*Atomic: will be replaced during a merge*
Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.
*Validation specifies the CEL expression which is used to apply the validation.*