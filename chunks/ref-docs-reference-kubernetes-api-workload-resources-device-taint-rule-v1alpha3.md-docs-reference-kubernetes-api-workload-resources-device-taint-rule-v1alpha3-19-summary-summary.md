---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#19-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleStatus
token_count: 115
summary: * **conditions** ([]Condition) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* Conditions provide information about the state of the DeviceTaintRule...
---

* **conditions** ([]Condition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions provide information about the state of the DeviceTaintRule and the cluster at some point in time, in a machine-readable and human-readable format.
The following condition is currently defined as part of this API, more may get added: - Type: EvictionInProgress - Status: True if there are currently pods which need to be evicted, False otherwise
(includes the effects which don't cause eviction).