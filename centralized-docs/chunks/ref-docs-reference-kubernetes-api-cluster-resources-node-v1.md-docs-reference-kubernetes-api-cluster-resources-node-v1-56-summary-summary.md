---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#56-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 119
summary: * `\"Pending\"` means the node has been created/added by the system, but not configured. * `\"Running\"` means the node has been configured and has Kubernetes components running. * `\"Terminated\"` means...
---

* `"Pending"` means the node has been created/added by the system, but not configured.
* `"Running"` means the node has been configured and has Kubernetes components running.
* `"Terminated"` means the node has been removed from the cluster.
* **runtimeHandlers** ([]NodeRuntimeHandler)
*Atomic: will be replaced during a merge*
The available runtime handlers.
*NodeRuntimeHandler is a set of runtime handler information.*
* **runtimeHandlers.features** (NodeRuntimeHandlerFeatures)
Supported features.
*NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.*