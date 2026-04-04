---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#91-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerList
token_count: 123
summary: HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects. * **apiVersion**: autoscaling/v2 * **kind**: HorizontalPodAutoscalerList * **metadata**...
---

HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
* **apiVersion**: autoscaling/v2
* **kind**: HorizontalPodAutoscalerList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
metadata is the standard list metadata.
* **items** ([][HorizontalPodAutoscaler](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/#HorizontalPodAutoscaler)), required
items is the list of horizontal pod autoscaler objects.