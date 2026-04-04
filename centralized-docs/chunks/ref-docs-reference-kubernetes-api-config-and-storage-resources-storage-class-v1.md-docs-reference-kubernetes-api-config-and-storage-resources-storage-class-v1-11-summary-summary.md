---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#11-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 103
summary: * `\"Immediate\"` indicates that PersistentVolumeClaims should be immediately provisioned and bound. This is the default mode. * `\"WaitForFirstConsumer\"` indicates that PersistentVolumeClaims should...
---

* `"Immediate"` indicates that PersistentVolumeClaims should be immediately provisioned and bound. This is the default mode.
* `"WaitForFirstConsumer"` indicates that PersistentVolumeClaims should not be provisioned and bound until the first Pod is created that references the PeristentVolumeClaim. The volume provisioning and binding will occur during Pod scheduing.## StorageClassList
StorageClassList is a collection of storage classes.
* **apiVersion**: storage.k8s.io/v1
* **kind**: StorageClassList