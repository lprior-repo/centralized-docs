---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#57-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 125
summary: * **runtimeHandlers.features.recursiveReadOnlyMounts** (boolean) RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts. *...
---

* **runtimeHandlers.features.recursiveReadOnlyMounts** (boolean)
RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
* **runtimeHandlers.features.userNamespaces** (boolean)
UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
* **runtimeHandlers.name** (string)
Runtime handler name. Empty for the default runtime handler.
* **volumesAttached** ([]AttachedVolume)
*Atomic: will be replaced during a merge*
List of volumes that are attached to the node.
*AttachedVolume describes a volume attached to a node*