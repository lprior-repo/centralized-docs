---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#10-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 75
summary: * **volumeBindingMode** (string) volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound. When unset, VolumeBindingImmediate is used. This field is only honored by...
---

* **volumeBindingMode** (string)
volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound. When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
Possible enum values:
* `"Immediate"` indicates that PersistentVolumeClaims should be immediately provisioned and bound. This is the default mode.