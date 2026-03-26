---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#25-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 82
summary: * **spec.matchConstraints.excludeResourceRules** ([]NamedRuleWithOperations) *Atomic: will be replaced during a merge* ExcludeResourceRules describes what operations on what resources/subresources...
---

* **spec.matchConstraints.excludeResourceRules** ([]NamedRuleWithOperations)
*Atomic: will be replaced during a merge*
ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
*NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.*