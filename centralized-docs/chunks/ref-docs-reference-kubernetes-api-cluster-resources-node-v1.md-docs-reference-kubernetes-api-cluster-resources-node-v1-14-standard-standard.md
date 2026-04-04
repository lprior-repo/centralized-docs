---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#14-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 434
summary: * **nodeInfo.swap** (NodeSwapStatus) Swap Info reported by the node. *NodeSwapStatus represents swap memory information.* * **nodeInfo.swap.capacity** (int64) Total amount of swap memory in bytes. *...
---

* **nodeInfo.swap** (NodeSwapStatus)
Swap Info reported by the node.
*NodeSwapStatus represents swap memory information.*
* **nodeInfo.swap.capacity** (int64)
Total amount of swap memory in bytes.
* **phase** (string)
NodePhase is the recently observed lifecycle phase of the node. More info: [https://kubernetes.io/docs/concepts/nodes/node/#phase](https://kubernetes.io/docs/concepts/nodes/node/#phase) The field is never populated, and now is deprecated.
Possible enum values:
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
* **volumesAttached.devicePath** (string), required
DevicePath represents the device path where the volume should be available
* **volumesAttached.name** (string), required
Name of the attached volume
* **volumesInUse** ([]string)
*Atomic: will be replaced during a merge*
List of attachable volumes in use (mounted) by the node.