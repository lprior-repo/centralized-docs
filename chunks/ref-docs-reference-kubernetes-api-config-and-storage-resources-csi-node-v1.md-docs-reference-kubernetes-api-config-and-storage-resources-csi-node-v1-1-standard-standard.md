---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: CSINode
token_count: 280
summary: # CSINode CSINode holds information about all CSI drivers installed on a node. `apiVersion: storage.k8s.io/v1` `import \"k8s.io/api/storage/v1\"` ## CSINode CSINode holds information about all CSI...
---

# CSINode
CSINode holds information about all CSI drivers installed on a node.
`apiVersion: storage.k8s.io/v1`
`import "k8s.io/api/storage/v1"`
## CSINode
CSINode holds information about all CSI drivers installed on a node. CSI drivers do not need to create the CSINode object directly. As long as they use the node-driver-registrar sidecar container, the kubelet will automatically populate the CSINode object for the CSI driver as part of kubelet plugin registration. CSINode has the same name as a node. If the object is missing, it means either there are no CSI Drivers available on the node, or the Kubelet version is low enough that it doesn't create this object. CSINode has an OwnerReference that points to the corresponding node object.
* **apiVersion**: storage.k8s.io/v1
* **kind**: CSINode
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. metadata.name must be the Kubernetes node name.
* **spec** ([CSINodeSpec](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/csi-node-v1/#CSINodeSpec)), required
spec is the specification of CSINode