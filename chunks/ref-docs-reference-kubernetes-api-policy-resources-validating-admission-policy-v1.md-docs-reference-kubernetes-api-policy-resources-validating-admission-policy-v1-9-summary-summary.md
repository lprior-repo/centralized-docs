---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 122
summary: * **spec** (ValidatingAdmissionPolicySpec) Specification of the desired behavior of the ValidatingAdmissionPolicy. *ValidatingAdmissionPolicySpec is the specification of the desired behavior of the...
---

* **spec** (ValidatingAdmissionPolicySpec)
Specification of the desired behavior of the ValidatingAdmissionPolicy.
*ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.*
* **spec.auditAnnotations** ([]AuditAnnotation)
*Atomic: will be replaced during a merge*
auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.
*AuditAnnotation describes how to produce an audit annotation for an API request.*