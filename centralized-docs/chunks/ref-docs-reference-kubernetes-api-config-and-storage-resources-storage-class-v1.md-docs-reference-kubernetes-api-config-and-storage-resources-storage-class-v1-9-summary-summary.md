---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: StorageClass
token_count: 92
summary: * `\"Delete\"` means the volume will be deleted from Kubernetes on release from its claim. The volume plugin must support Deletion. * `\"Recycle\"` means the volume will be recycled back into the pool of...
---

* `"Delete"` means the volume will be deleted from Kubernetes on release from its claim. The volume plugin must support Deletion.
* `"Recycle"` means the volume will be recycled back into the pool of unbound persistent volumes on release from its claim. The volume plugin must support Recycling.
* `"Retain"` means the volume will be left in its current phase (Released) for manual reclamation by the administrator. The default policy is Retain.