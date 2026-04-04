---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#22-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerList
token_count: 313
summary: ## HorizontalPodAutoscalerList HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects. * **apiVersion**: autoscaling/v2 * **kind**: HorizontalPodAutoscalerList * **metadata**...
---

## HorizontalPodAutoscalerList
HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
* **apiVersion**: autoscaling/v2
* **kind**: HorizontalPodAutoscalerList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
metadata is the standard list metadata.
* **items** ([][HorizontalPodAutoscaler](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/horizontal-pod-autoscaler-v2/#HorizontalPodAutoscaler)), required
items is the list of horizontal pod autoscaler objects.
#### Parameters
* **name** (*in path*): string, required
name of the HorizontalPodAutoscaler
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the HorizontalPodAutoscaler
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)