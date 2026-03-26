---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#16-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 79
summary: * **deviceSelector.device** (string) If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices[].name. Setting also driver and pool may be required to...
---

* **deviceSelector.device** (string)
If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices[].name.
Setting also driver and pool may be required to avoid ambiguity, but is not required.
* **deviceSelector.driver** (string)
If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.