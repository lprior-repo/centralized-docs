---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#12-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 100
summary: * **taint.effect** (string), required The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them. Valid effects are None, NoSchedule and...
---

* **taint.effect** (string), required
The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them.
Valid effects are None, NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here. More effects may get added in the future. Consumers must treat unknown effects like None.
Possible enum values:
* `"NoExecute"` Evict any already-running pods that do not tolerate the device taint.