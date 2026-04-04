---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#51-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 94
summary: * **metrics.pods** (PodsMetricSource) pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together...
---

* **metrics.pods** (PodsMetricSource)
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
*PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.*