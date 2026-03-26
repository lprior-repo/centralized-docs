---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#11-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleSpec
token_count: 58
summary: * **taint** (DeviceTaint), required The taint that gets applied to matching devices. *The device this taint is attached to has the \"effect\" on any claim which does not tolerate the taint and, through...
---

* **taint** (DeviceTaint), required
The taint that gets applied to matching devices.
*The device this taint is attached to has the "effect" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.*