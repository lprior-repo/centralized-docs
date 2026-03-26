---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#21-standard
chunk_level: standard
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 450
summary: * **spec.matchResources.excludeResourceRules** ([]NamedRuleWithOperations) *Atomic: will be replaced during a merge* ExcludeResourceRules describes what operations on what resources/subresources the...
---

* **spec.matchResources.excludeResourceRules** ([]NamedRuleWithOperations)
*Atomic: will be replaced during a merge*
ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
*NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.*
* **spec.matchResources.excludeResourceRules.apiGroups** ([]string)
*Atomic: will be replaced during a merge*
APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
* **spec.matchResources.excludeResourceRules.apiVersions** ([]string)
*Atomic: will be replaced during a merge*
APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
* **spec.matchResources.excludeResourceRules.operations** ([]string)
*Atomic: will be replaced during a merge*
Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or \* for all of those operations and any future admission operations that are added. If '\*' is present, the length of the slice must be one. Required.
* **spec.matchResources.excludeResourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.
* **spec.matchResources.excludeResourceRules.resources** ([]string)
*Atomic: will be replaced during a merge*
Resources is a list of resources this rule applies to.
For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/\*' means all resources and their subresources.
If wildcard is present, the validation rule will ensure resources do not overlap with each other.
Depending on the enclosing object, subresources might not be allowed. Required.