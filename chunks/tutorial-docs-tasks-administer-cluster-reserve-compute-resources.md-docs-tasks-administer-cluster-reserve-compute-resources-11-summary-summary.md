---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#11-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 119
summary: ## Node Allocatable ![node capacity](/images/docs/node-capacity.svg) 'Allocatable' on a Kubernetes node is defined as the amount of compute resources that are available for pods. The scheduler does...
---

## Node Allocatable
![node capacity](/images/docs/node-capacity.svg)
'Allocatable' on a Kubernetes node is defined as the amount of compute resources
that are available for pods. The scheduler does not over-subscribe
'Allocatable'. 'CPU', 'memory' and 'ephemeral-storage' are supported as of now.
Node Allocatable is exposed as part of `v1.Node` object in the API and as part
of `kubectl describe node` in the CLI.
Resources can be reserved for two categories of system daemons in the `kubelet`.