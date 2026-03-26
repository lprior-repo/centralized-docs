---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#87-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicyBinding
token_count: 122
summary: * **spec.matchResources.excludeResourceRules.operations** ([]string) *Atomic: will be replaced during a merge* Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE,...
---

* **spec.matchResources.excludeResourceRules.operations** ([]string)
*Atomic: will be replaced during a merge*
Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or \* for all of those operations and any future admission operations that are added. If '\*' is present, the length of the slice must be one. Required.
* **spec.matchResources.excludeResourceRules.resourceNames** ([]string)
*Atomic: will be replaced during a merge*
ResourceNames is an optional white list of names that the rule applies to. An empty set means that everything is allowed.