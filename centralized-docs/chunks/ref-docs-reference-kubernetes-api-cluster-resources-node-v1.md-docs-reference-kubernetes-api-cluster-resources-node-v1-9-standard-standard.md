---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#9-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 496
summary: * **config.active.configMap.kubeletConfigKey** (string), required KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is...
---

* **config.active.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
* **config.active.configMap.name** (string), required
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
* **config.active.configMap.namespace** (string), required
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
* **config.active.configMap.resourceVersion** (string)
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **config.active.configMap.uid** (string)
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **config.assigned** (NodeConfigSource)
Assigned reports the checkpointed config the node will try to use. When Node.Spec.ConfigSource is updated, the node checkpoints the associated config payload to local disk, along with a record indicating intended config. The node refers to this record to choose its config checkpoint, and reports this record in Assigned. Assigned only updates in the status after the record has been checkpointed to disk. When the Kubelet is restarted, it tries to make the Assigned config the Active config by loading and validating the checkpointed payload identified by Assigned.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*
* **config.assigned.configMap** (ConfigMapNodeConfigSource)
ConfigMap is a reference to a Node's ConfigMap
*ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: [https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration](https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration)*
* **config.assigned.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.