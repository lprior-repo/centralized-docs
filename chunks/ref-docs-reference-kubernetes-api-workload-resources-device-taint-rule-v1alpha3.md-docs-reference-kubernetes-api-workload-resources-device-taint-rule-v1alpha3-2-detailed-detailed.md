---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#2-detailed
chunk_level: detailed
chunk_type: prose
heading: DeviceTaintRuleStatus
token_count: 573
summary: ## DeviceTaintRuleStatus DeviceTaintRuleStatus provides information about an on-going pod eviction. * **conditions** ([]Condition) *Patch strategy: merge on key `type`* *Map: unique values on key...
---

## DeviceTaintRuleStatus
DeviceTaintRuleStatus provides information about an on-going pod eviction.
* **conditions** ([]Condition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions provide information about the state of the DeviceTaintRule and the cluster at some point in time, in a machine-readable and human-readable format.
The following condition is currently defined as part of this API, more may get added: - Type: EvictionInProgress - Status: True if there are currently pods which need to be evicted, False otherwise
(includes the effects which don't cause eviction).
* Reason: not specified, may change - Message: includes information about number of pending pods and already evicted pods
in a human-readable format, updated periodically, may change
For `effect: None`, the condition above gets set once for each change to the spec, with the message containing information about what would happen if the effect was `NoExecute`. This feedback can be used to decide whether changing the effect to `NoExecute` will work as intended. It only gets set once to avoid having to constantly update the status.
Must have 8 or fewer entries.
*Condition contains details for one aspect of the current state of this API Resource.*
* **conditions.lastTransitionTime** (Time), required
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string), required
message is a human readable message indicating details about the transition. This may be an empty string.
* **conditions.reason** (string), required
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
* **conditions.status** (string), required
status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
type of condition in CamelCase or in foo.example.com/CamelCase.
* **conditions.observedGeneration** (int64)
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.