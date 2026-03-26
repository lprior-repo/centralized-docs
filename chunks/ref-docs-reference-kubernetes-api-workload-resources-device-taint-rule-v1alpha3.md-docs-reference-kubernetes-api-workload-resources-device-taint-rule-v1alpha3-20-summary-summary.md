---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#20-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRuleStatus
token_count: 115
summary: * Reason: not specified, may change - Message: includes information about number of pending pods and already evicted pods in a human-readable format, updated periodically, may change For `effect:...
---

* Reason: not specified, may change - Message: includes information about number of pending pods and already evicted pods
in a human-readable format, updated periodically, may change
For `effect: None`, the condition above gets set once for each change to the spec, with the message containing information about what would happen if the effect was `NoExecute`. This feedback can be used to decide whether changing the effect to `NoExecute` will work as intended. It only gets set once to avoid having to constantly update the status.
Must have 8 or fewer entries.