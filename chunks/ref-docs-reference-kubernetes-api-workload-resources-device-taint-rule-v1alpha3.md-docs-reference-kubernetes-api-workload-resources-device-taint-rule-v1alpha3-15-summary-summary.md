---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#15-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 113
summary: * **taint.value** (string) The taint value corresponding to the taint key. Must be a label value. * **deviceSelector** (DeviceTaintSelector) DeviceSelector defines which device(s) the taint is...
---

* **taint.value** (string)
The taint value corresponding to the taint key. Must be a label value.
* **deviceSelector** (DeviceTaintSelector)
DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satisfied for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.
*DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.*