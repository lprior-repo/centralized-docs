---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#42-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 103
summary: * **metrics.external.metric.name** (string), required name is the name of the given metric * **metrics.external.metric.selector**...
---

* **metrics.external.metric.name** (string), required
name is the name of the given metric
* **metrics.external.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.