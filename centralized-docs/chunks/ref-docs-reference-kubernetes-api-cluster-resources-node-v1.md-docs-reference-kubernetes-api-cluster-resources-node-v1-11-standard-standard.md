---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#11-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 510
summary: * **config.lastKnownGood** (NodeConfigSource) LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned...
---

* **config.lastKnownGood** (NodeConfigSource)
LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*
* **config.lastKnownGood.configMap** (ConfigMapNodeConfigSource)
ConfigMap is a reference to a Node's ConfigMap
*ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: [https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration](https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration)*
* **config.lastKnownGood.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
* **config.lastKnownGood.configMap.name** (string), required
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
* **config.lastKnownGood.configMap.namespace** (string), required
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
* **config.lastKnownGood.configMap.resourceVersion** (string)
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **config.lastKnownGood.configMap.uid** (string)
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.