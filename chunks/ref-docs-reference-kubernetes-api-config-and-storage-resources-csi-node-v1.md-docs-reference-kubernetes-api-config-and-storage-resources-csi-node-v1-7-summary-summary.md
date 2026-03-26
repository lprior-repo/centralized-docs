---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: CSINode
token_count: 105
summary: * **apiVersion**: storage.k8s.io/v1 * **kind**: CSINode * **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta)) Standard object's...
---

* **apiVersion**: storage.k8s.io/v1
* **kind**: CSINode
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. metadata.name must be the Kubernetes node name.
* **spec** ([CSINodeSpec](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/#CSINodeSpec)), required
spec is the specification of CSINode