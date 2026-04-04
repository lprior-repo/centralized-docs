---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#6-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 111
summary: *A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality...
---

*A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.*
* **allowedTopologies.matchLabelExpressions** ([]TopologySelectorLabelRequirement)
*Atomic: will be replaced during a merge*
A list of topology selector requirements by labels.
*A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.*