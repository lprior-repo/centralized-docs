---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: CSINodeSpec
token_count: 87
summary: * **drivers** ([]CSINodeDriver), required *Patch strategy: merge on key `name`* *Map: unique values on key name will be kept during a merge* drivers is a list of information of all CSI Drivers...
---

* **drivers** ([]CSINodeDriver), required
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
*CSINodeDriver holds information about the specification of one CSI driver installed on a node*