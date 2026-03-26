---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#13-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 113
summary: * `\"NoExecute\"` Evict any already-running pods that do not tolerate the device taint. * `\"NoSchedule\"` Do not allow new pods to schedule which use a tainted device unless they tolerate the taint, but...
---

* `"NoExecute"` Evict any already-running pods that do not tolerate the device taint.
* `"NoSchedule"` Do not allow new pods to schedule which use a tainted device unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running.
* `"None"` No effect, the taint is purely informational.
* **taint.key** (string), required
The taint key to be applied to a device. Must be a label name.