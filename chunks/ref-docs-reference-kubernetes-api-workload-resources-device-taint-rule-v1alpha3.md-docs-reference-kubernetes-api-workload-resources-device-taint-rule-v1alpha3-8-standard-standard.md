---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#8-standard
chunk_level: standard
chunk_type: prose
heading: DeviceTaintRuleList
token_count: 237
summary: ## DeviceTaintRuleList DeviceTaintRuleList is a collection of DeviceTaintRules. * **apiVersion**: resource.k8s.io/v1alpha3 * **kind**: DeviceTaintRuleList * **metadata**...
---

## DeviceTaintRuleList
DeviceTaintRuleList is a collection of DeviceTaintRules.
* **apiVersion**: resource.k8s.io/v1alpha3
* **kind**: DeviceTaintRuleList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata
* **items** ([][DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)), required
Items is the list of DeviceTaintRules.
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the DeviceTaintRule
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)