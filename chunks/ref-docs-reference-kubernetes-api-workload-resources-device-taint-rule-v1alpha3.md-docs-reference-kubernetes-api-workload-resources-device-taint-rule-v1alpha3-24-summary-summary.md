---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#24-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleStatus
token_count: 119
summary: * **conditions.status** (string), required status of the condition, one of True, False, Unknown. * **conditions.type** (string), required type of condition in CamelCase or in...
---

* **conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.