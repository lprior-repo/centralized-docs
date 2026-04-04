---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#55-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 91
summary: * **metrics.pods.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity)) averageValue is the target value of the average of the...
---

* **metrics.pods.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
* **metrics.pods.target.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the target value of the metric (as a quantity).