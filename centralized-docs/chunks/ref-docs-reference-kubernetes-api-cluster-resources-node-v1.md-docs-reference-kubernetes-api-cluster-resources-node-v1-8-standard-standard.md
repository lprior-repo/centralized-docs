---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#8-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 508
summary: * **conditions.status** (string), required Status of the condition, one of True, False, Unknown. * **conditions.type** (string), required Type of node condition. * **conditions.lastHeartbeatTime**...
---

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