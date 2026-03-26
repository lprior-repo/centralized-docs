---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#8-summary
chunk_level: summary
chunk_type: prose
heading: DeviceTaintRule
token_count: 114
summary: * **apiVersion**: resource.k8s.io/v1alpha3 * **kind**: DeviceTaintRule * **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))...
---

* **apiVersion**: resource.k8s.io/v1alpha3
* **kind**: DeviceTaintRule
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object metadata
* **spec** ([DeviceTaintRuleSpec](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRuleSpec)), required
Spec specifies the selector and one taint.
Changing the spec automatically increments the metadata.generation number.