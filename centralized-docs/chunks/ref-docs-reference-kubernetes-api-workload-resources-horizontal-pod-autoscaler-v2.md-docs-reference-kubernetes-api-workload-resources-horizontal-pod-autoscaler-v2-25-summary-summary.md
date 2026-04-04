---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#25-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 102
summary: * double the number of pods per 60 seconds No stabilization is used. \*HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance....
---

* double the number of pods per 60 seconds
No stabilization is used.
\*HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.
Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.