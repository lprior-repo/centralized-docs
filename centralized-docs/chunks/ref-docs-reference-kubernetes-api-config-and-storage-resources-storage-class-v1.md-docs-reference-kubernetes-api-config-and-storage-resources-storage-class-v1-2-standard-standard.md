---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#2-standard
chunk_level: standard
chunk_type: prose
heading: StorageClass
token_count: 47
summary: ## StorageClass StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned. StorageClasses are non-namespaced; the name of the storage...
---

## StorageClass
StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.