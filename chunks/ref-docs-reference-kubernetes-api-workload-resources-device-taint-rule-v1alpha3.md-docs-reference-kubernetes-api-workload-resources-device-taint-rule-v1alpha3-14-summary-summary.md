---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#14-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 121
summary: * **taint.key** (string), required The taint key to be applied to a device. Must be a label name. * **taint.timeAdded** (Time) TimeAdded represents the time at which the taint was added. Added...
---

* **taint.key** (string), required
The taint key to be applied to a device. Must be a label name.
* **taint.timeAdded** (Time)
TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **taint.value** (string)
The taint value corresponding to the taint key. Must be a label value.