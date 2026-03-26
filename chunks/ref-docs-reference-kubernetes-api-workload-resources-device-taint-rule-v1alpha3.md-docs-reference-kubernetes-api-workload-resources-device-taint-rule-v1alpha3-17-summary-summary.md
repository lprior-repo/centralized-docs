---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#17-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 105
summary: * **deviceSelector.driver** (string) If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver. * **deviceSelector.pool** (string) If pool is set,...
---

* **deviceSelector.driver** (string)
If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.
* **deviceSelector.pool** (string)
If pool is set, only devices in that pool are selected.
Also setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.