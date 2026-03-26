---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#12-summary
chunk_level: summary
chunk_type: prose
heading: Full metrics pipeline
token_count: 126
summary: mix of open-source software, paid-for software-as-a-service, and other commercial products. When you design and implement a full metrics pipeline you can make that monitoring data available back to...
---

mix of open-source software, paid-for software-as-a-service, and other commercial products.
When you design and implement a full metrics pipeline you can make that monitoring data
available back to Kubernetes. For example, a HorizontalPodAutoscaler can use the processed
metrics to work out how many Pods to run for a component of your workload.
Integration of a full metrics pipeline into your Kubernetes implementation is outside
the scope of Kubernetes documentation because of the very wide scope of possible
solutions.
The choice of monitoring platform depends heavily on your needs, budget, and technical resources.
Kubernetes does not recommend any specific metrics pipeline;