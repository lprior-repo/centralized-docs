---
doc_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring
chunk_id: ref/docs-tasks-debug-debug-cluster-resource-usage-monitoring.md/docs-tasks-debug-debug-cluster-resource-usage-monitoring#6-summary
chunk_level: summary
chunk_type: prose
heading: Resource metrics pipeline
token_count: 128
summary: queries each node's [kubelet](/docs/reference/command-line-tools-reference/kubelet/) for CPU and memory usage. The kubelet acts as a bridge between the Kubernetes master and the nodes, managing the...
---

queries each node's
[kubelet](/docs/reference/command-line-tools-reference/kubelet/) for CPU and
memory usage. The kubelet acts as a bridge between the Kubernetes master and
the nodes, managing the pods and containers running on a machine. The kubelet
translates each pod into its constituent containers and fetches individual
container usage statistics from the container runtime through the container
runtime interface. If you use a container runtime that uses Linux cgroups and
namespaces to implement containers, and the container runtime does not publish
usage statistics, then the kubelet can look up those statistics directly
(using code from