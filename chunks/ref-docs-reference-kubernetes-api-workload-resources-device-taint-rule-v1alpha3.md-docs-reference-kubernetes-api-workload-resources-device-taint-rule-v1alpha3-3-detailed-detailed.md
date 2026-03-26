---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md/docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3#3-detailed
chunk_level: detailed
chunk_type: prose
heading: DeviceTaintRuleList
token_count: 910
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
#### Parameters
* **allowWatchBookmarks** (*in query*): boolean
[allowWatchBookmarks](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#allowWatchBookmarks)
* **continue** (*in query*): string
[continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **fieldSelector** (*in query*): string
[fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **labelSelector** (*in query*): string
[labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
[limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **resourceVersion** (*in query*): string
[resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
[resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
[sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
[timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)
* **watch** (*in query*): boolean
[watch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#watch)
#### Parameters
* **body**: [DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): OK
201 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Created
202 ([DeviceTaintRule](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/device-taint-rule-v1alpha3/#DeviceTaintRule)): Accepted
401: Unauthorized