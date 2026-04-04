---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#76-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 91
summary: * **currentMetrics.object.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity)) value is the current value of the metric (as a...
---

* **currentMetrics.object.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).
* **currentMetrics.object.describedObject** (CrossVersionObjectReference), required
DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
*CrossVersionObjectReference contains enough information to let you identify the referred resource.*