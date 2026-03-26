---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#5-standard
chunk_level: standard
chunk_type: prose
heading: CSINodeList
token_count: 221
summary: ## CSINodeList CSINodeList is a collection of CSINode objects. * **apiVersion**: storage.k8s.io/v1 * **kind**: CSINodeList * **metadata**...
---

## CSINodeList
CSINodeList is a collection of CSINode objects.
* **apiVersion**: storage.k8s.io/v1
* **kind**: CSINodeList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **items** ([][CSINode](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/#CSINode)), required
items is the list of CSINode
#### Parameters
* **name** (*in path*): string, required
name of the CSINode
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)