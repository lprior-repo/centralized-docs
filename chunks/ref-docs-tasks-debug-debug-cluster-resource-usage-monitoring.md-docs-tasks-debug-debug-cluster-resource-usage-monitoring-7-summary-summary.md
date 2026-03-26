---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#7-summary
chunk_level: summary
chunk_type: prose
heading: Resource metrics pipeline
token_count: 81
summary: usage statistics, then the kubelet can look up those statistics directly (using code from [cAdvisor](https://github.com/google/cadvisor)). No matter how those statistics arrive, the kubelet then...
---

usage statistics, then the kubelet can look up those statistics directly
(using code from [cAdvisor](https://github.com/google/cadvisor)).
No matter how those statistics arrive, the kubelet then exposes the aggregated pod
resource usage statistics through the metrics-server Resource Metrics API.
This API is served at `/metrics/resource/v1beta1` on the kubelet's authenticated and
read-only ports.