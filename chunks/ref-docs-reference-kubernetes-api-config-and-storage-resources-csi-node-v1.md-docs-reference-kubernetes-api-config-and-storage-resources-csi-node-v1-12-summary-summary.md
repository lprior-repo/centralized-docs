---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#12-summary
chunk_level: summary
chunk_type: prose
heading: CSINodeSpec
token_count: 61
summary: \"nodeA\" instead of \"node1\". This field is required. * **drivers.allocatable** (VolumeNodeResources) allocatable represents the volume resources of a node that are available for scheduling. This field...
---

"nodeA" instead of "node1". This field is required.
* **drivers.allocatable** (VolumeNodeResources)
allocatable represents the volume resources of a node that are available for scheduling. This field is beta.
*VolumeNodeResources is a set of resource limits for scheduling of volumes.*