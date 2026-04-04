---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#4-detailed
chunk_level: detailed
chunk_type: prose
heading: NodeStatus
token_count: 987
summary: * **addresses** ([]NodeAddress) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* List of addresses reachable to the node. Queried from cloud...
---

* **addresses** ([]NodeAddress)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
List of addresses reachable to the node. Queried from cloud provider, if available. More info: [https://kubernetes.io/docs/reference/node/node-status/#addresses](https://kubernetes.io/docs/reference/node/node-status/#addresses) Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See [https://pr.k8s.io/79391](https://pr.k8s.io/79391) for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
*NodeAddress contains information for the node's address.*
* **addresses.address** (string), required
The node address.
* **addresses.type** (string), required
Node address type, one of Hostname, ExternalIP or InternalIP.
* **allocatable** (map[string][Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
* **capacity** (map[string][Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
Capacity represents the total resources of a node. More info: [https://kubernetes.io/docs/reference/node/node-status/#capacity](https://kubernetes.io/docs/reference/node/node-status/#capacity)
* **conditions** ([]NodeCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions is an array of current observed node conditions. More info: [https://kubernetes.io/docs/reference/node/node-status/#condition](https://kubernetes.io/docs/reference/node/node-status/#condition)
*NodeCondition contains condition information for a node.*
* **conditions.status** (string), required
Status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
Type of node condition.
* **conditions.lastHeartbeatTime** (Time)
Last time we got an update on a given condition.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.lastTransitionTime** (Time)
Last time the condition transit from one status to another.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
Human readable message indicating details about last transition.
* **conditions.reason** (string)
(brief) reason for the condition's last transition.
* **config** (NodeConfigStatus)
Status of the config assigned to the node via the dynamic Kubelet config feature.
*NodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.*
* **config.active** (NodeConfigSource)
Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*
* **config.active.configMap** (ConfigMapNodeConfigSource)
ConfigMap is a reference to a Node's ConfigMap
*ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: [https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration](https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration)*
* **config.active.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
* **config.active.configMap.name** (string), required
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
* **config.active.configMap.namespace** (string), required
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.