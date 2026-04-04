---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#5-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 76
summary: * **allowedTopologies** ([]TopologySelectorTerm) *Atomic: will be replaced during a merge* allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume...
---

* **allowedTopologies** ([]TopologySelectorTerm)
*Atomic: will be replaced during a merge*
allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.