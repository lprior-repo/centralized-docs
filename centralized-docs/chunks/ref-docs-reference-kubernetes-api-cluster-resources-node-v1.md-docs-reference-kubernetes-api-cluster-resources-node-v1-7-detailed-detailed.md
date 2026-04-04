---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#7-detailed
chunk_level: detailed
chunk_type: prose
heading: NodeStatus
token_count: 889
summary: * **nodeInfo** (NodeSystemInfo) Set of ids/uuids to uniquely identify the node. More info:...
---

* **nodeInfo** (NodeSystemInfo)
Set of ids/uuids to uniquely identify the node. More info: [https://kubernetes.io/docs/reference/node/node-status/#info](https://kubernetes.io/docs/reference/node/node-status/#info)
*NodeSystemInfo is a set of ids/uuids to uniquely identify the node.*
* **nodeInfo.architecture** (string), required
The Architecture reported by the node
* **nodeInfo.bootID** (string), required
Boot ID reported by the node.
* **nodeInfo.containerRuntimeVersion** (string), required
ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
* **nodeInfo.kernelVersion** (string), required
Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
* **nodeInfo.kubeProxyVersion** (string), required
Deprecated: KubeProxy Version reported by the node.
* **nodeInfo.kubeletVersion** (string), required
Kubelet Version reported by the node.
* **nodeInfo.machineID** (string), required
MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: [http://man7.org/linux/man-pages/man5/machine-id.5.html](http://man7.org/linux/man-pages/man5/machine-id.5.html)
* **nodeInfo.operatingSystem** (string), required
The Operating System reported by the node
* **nodeInfo.osImage** (string), required
OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
* **nodeInfo.systemUUID** (string), required
SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts [https://access.redhat.com/documentation/en-us/red\_hat\_subscription\_management/1/html/rhsm/uuid](https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid)
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