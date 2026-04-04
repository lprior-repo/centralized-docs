---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#8-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 121
summary: * **mountOptions** ([]string) *Atomic: will be replaced during a merge* mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. [\"ro\",...
---

* **mountOptions** ([]string)
*Atomic: will be replaced during a merge*
mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. ["ro", "soft"]. Not validated - mount of the PVs will simply fail if one is invalid.
* **parameters** (map[string]string)
parameters holds the parameters for the provisioner that should create volumes of this storage class.
* **reclaimPolicy** (string)
reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes of this storage class. Defaults to Delete.
Possible enum values: