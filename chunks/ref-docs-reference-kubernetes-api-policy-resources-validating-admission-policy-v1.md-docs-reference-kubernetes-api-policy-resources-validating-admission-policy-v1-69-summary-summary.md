---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#69-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 122
summary: * **status.conditions.status** (string), required status of the condition, one of True, False, Unknown. * **status.conditions.type** (string), required type of condition in CamelCase or in...
---

* **status.conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **status.conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **status.conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.