---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1#14-summary
chunk_level: summary
chunk_type: prose
heading: CSINodeSpec
token_count: 68
summary: * **drivers.topologyKeys** ([]string) *Atomic: will be replaced during a merge* topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set...
---

* **drivers.topologyKeys** ([]string)
*Atomic: will be replaced during a merge*
topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"