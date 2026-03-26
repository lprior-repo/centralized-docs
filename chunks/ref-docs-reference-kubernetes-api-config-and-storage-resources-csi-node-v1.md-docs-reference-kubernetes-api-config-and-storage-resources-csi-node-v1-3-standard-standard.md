---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: CSINodeSpec
token_count: 399
summary: * **drivers** ([]CSINodeDriver), required *Patch strategy: merge on key `name`* *Map: unique values on key name will be kept during a merge* drivers is a list of information of all CSI Drivers...
---

* **drivers** ([]CSINodeDriver), required
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
*CSINodeDriver holds information about the specification of one CSI driver installed on a node*
* **drivers.name** (string), required
name represents the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
* **drivers.nodeID** (string), required
nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as "node1", but the storage system may refer to the same node as "nodeA". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. "nodeA" instead of "node1". This field is required.
* **drivers.allocatable** (VolumeNodeResources)
allocatable represents the volume resources of a node that are available for scheduling. This field is beta.
*VolumeNodeResources is a set of resource limits for scheduling of volumes.*
* **drivers.allocatable.count** (int32)
count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.