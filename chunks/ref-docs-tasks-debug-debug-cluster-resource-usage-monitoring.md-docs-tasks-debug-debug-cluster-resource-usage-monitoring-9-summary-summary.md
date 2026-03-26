---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#9-summary
chunk_level: summary
chunk_type: prose
heading: Full metrics pipeline
token_count: 111
summary: A full metrics pipeline gives you access to richer metrics. Kubernetes can respond to these metrics by automatically scaling or adapting the cluster based on its current state, using mechanisms such...
---

A full metrics pipeline gives you access to richer metrics. Kubernetes can
respond to these metrics by automatically scaling or adapting the cluster
based on its current state, using mechanisms such as the Horizontal Pod
Autoscaler. The monitoring pipeline fetches metrics from the kubelet and
then exposes them to Kubernetes via an adapter by implementing either the
`custom.metrics.k8s.io` or `external.metrics.k8s.io` API.
Kubernetes is designed to work with [OpenMetrics](https://openmetrics.io/),
which is one of the