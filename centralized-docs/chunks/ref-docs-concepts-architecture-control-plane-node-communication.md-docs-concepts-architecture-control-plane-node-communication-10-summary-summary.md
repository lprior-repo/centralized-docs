---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#10-summary
chunk_level: summary
chunk_type: prose
heading: Control plane to node
token_count: 85
summary: ## Control plane to node There are two primary communication paths from the control plane (the API server) to the nodes. The first is from the API server to the...
---

## Control plane to node
There are two primary communication paths from the control plane (the API server) to the nodes.
The first is from the API server to the [kubelet](/docs/reference/command-line-tools-reference/kubelet) process which runs on each node in the cluster.
The second is from the API server to any node, pod, or service through the API server's *proxy*
functionality.