---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: CSINodeSpec
token_count: 248
summary: * **drivers.allocatable.count** (int32) count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node...
---

* **drivers.allocatable.count** (int32)
count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.
* **drivers.topologyKeys** ([]string)
*Atomic: will be replaced during a merge*
topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.