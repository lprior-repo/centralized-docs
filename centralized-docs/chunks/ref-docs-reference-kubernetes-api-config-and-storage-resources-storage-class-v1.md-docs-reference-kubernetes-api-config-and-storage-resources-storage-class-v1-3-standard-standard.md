---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: StorageClass
token_count: 492
summary: * **apiVersion**: storage.k8s.io/v1 * **kind**: StorageClass * **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta)) Standard...
---

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