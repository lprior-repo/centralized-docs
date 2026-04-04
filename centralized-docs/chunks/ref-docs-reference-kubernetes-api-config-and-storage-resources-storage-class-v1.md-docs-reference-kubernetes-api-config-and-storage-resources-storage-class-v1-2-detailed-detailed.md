---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#2-detailed
chunk_level: detailed
chunk_type: prose
heading: StorageClass
token_count: 997
summary: ## StorageClass StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned. StorageClasses are non-namespaced; the name of the storage...
---

## StorageClass
StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
* **apiVersion**: storage.k8s.io/v1
* **kind**: StorageClass
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **provisioner** (string), required
provisioner indicates the type of the provisioner.
* **allowVolumeExpansion** (boolean)
allowVolumeExpansion shows whether the storage class allow volume expand.
* **allowedTopologies** ([]TopologySelectorTerm)
*Atomic: will be replaced during a merge*
allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
*A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.*
* **allowedTopologies.matchLabelExpressions** ([]TopologySelectorLabelRequirement)
*Atomic: will be replaced during a merge*
A list of topology selector requirements by labels.
*A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.*
* **allowedTopologies.matchLabelExpressions.key** (string), required
The label key that the selector applies to.
* **allowedTopologies.matchLabelExpressions.values** ([]string), required
*Atomic: will be replaced during a merge*
An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
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
#### Parameters
* **name** (*in path*): string, required
name of the StorageClass
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)