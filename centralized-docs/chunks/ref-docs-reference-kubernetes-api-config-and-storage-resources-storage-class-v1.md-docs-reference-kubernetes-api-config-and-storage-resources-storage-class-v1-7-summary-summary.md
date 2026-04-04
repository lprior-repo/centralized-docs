---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 73
summary: * **allowedTopologies.matchLabelExpressions.key** (string), required The label key that the selector applies to. * **allowedTopologies.matchLabelExpressions.values** ([]string), required *Atomic:...
---

* **allowedTopologies.matchLabelExpressions.key** (string), required
The label key that the selector applies to.
* **allowedTopologies.matchLabelExpressions.values** ([]string), required
*Atomic: will be replaced during a merge*
An array of string values. One value must match the label to be selected. Each entry in Values is ORed.