---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#3-standard
chunk_level: standard
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 510
summary: * **taint** (DeviceTaint), required The taint that gets applied to matching devices. *The device this taint is attached to has the \"effect\" on any claim which does not tolerate the taint and, through...
---

* **taint** (DeviceTaint), required
The taint that gets applied to matching devices.
*The device this taint is attached to has the "effect" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.*
* **taint.effect** (string), required
The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them.
Valid effects are None, NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here. More effects may get added in the future. Consumers must treat unknown effects like None.
Possible enum values:
* `"NoExecute"` Evict any already-running pods that do not tolerate the device taint.
* `"NoSchedule"` Do not allow new pods to schedule which use a tainted device unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running.
* `"None"` No effect, the taint is purely informational.
* **taint.key** (string), required
The taint key to be applied to a device. Must be a label name.
* **taint.timeAdded** (Time)
TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **taint.value** (string)
The taint value corresponding to the taint key. Must be a label value.
* **deviceSelector** (DeviceTaintSelector)
DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satisfied for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.
*DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.*
* **deviceSelector.device** (string)
If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices[].name.
Setting also driver and pool may be required to avoid ambiguity, but is not required.
* **deviceSelector.driver** (string)
If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.