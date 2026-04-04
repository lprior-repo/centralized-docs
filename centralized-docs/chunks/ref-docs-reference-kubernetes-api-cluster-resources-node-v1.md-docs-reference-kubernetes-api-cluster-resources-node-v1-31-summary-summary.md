---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#31-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 110
summary: * **config.active.configMap.kubeletConfigKey** (string), required KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is...
---

* **config.active.configMap.kubeletConfigKey** (string), required
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
* **config.active.configMap.name** (string), required
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
* **config.active.configMap.namespace** (string), required
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.