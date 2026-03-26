---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#5-summary
chunk_level: summary
chunk_type: prose
heading: Resource metrics pipeline
token_count: 127
summary: The resource metrics pipeline provides a limited set of metrics related to cluster components such as the [Horizontal Pod Autoscaler](/docs/tasks/run-application/horizontal-pod-autoscale/)...
---

The resource metrics pipeline provides a limited set of metrics related to
cluster components such as the
[Horizontal Pod Autoscaler](/docs/tasks/run-application/horizontal-pod-autoscale/)
controller, as well as the `kubectl top` utility.
These metrics are collected by the lightweight, short-term, in-memory
[metrics-server](https://github.com/kubernetes-sigs/metrics-server) and
are exposed via the `metrics.k8s.io` API.
metrics-server discovers all nodes on the cluster and
queries each node's
[kubelet](/docs/reference/command-line-tools-reference/kubelet/) for CPU and