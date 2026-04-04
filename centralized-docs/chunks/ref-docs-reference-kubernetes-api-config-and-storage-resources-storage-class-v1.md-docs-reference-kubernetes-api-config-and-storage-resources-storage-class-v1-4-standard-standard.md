---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: StorageClass
token_count: 489
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
* `"Delete"` means the volume will be deleted from Kubernetes on release from its claim. The volume plugin must support Deletion.
* `"Recycle"` means the volume will be recycled back into the pool of unbound persistent volumes on release from its claim. The volume plugin must support Recycling.
* `"Retain"` means the volume will be left in its current phase (Released) for manual reclamation by the administrator. The default policy is Retain.
* **volumeBindingMode** (string)
volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound. When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
Possible enum values:
* `"Immediate"` indicates that PersistentVolumeClaims should be immediately provisioned and bound. This is the default mode.
* `"WaitForFirstConsumer"` indicates that PersistentVolumeClaims should not be provisioned and bound until the first Pod is created that references the PeristentVolumeClaim. The volume provisioning and binding will occur during Pod scheduing.## StorageClassList
StorageClassList is a collection of storage classes.
* **apiVersion**: storage.k8s.io/v1
* **kind**: StorageClassList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **items** ([][StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)), required
items is the list of StorageClasses