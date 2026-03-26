---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#41-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 110
summary: * **spec.matchConstraints.resourceRules.apiGroups** ([]string) *Atomic: will be replaced during a merge* APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the...
---

* **spec.matchConstraints.resourceRules.apiGroups** ([]string)
*Atomic: will be replaced during a merge*
APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
* **spec.matchConstraints.resourceRules.apiVersions** ([]string)
*Atomic: will be replaced during a merge*
APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.