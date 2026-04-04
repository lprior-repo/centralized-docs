---
id: ref/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-storage-class-v1
title: StorageClass
category: ref
tags: ["contents", "ref", "storageclass", "table"]
---

## Table of Contents

* [StorageClass](#storageclass)
  * [StorageClass](#storageclass)
    * [Parameters](#parameters)
    * [Parameters](#parameters)
    * [Parameters](#parameters)
    * [Response](#response)
    * [Parameters](#parameters)
    * [Response](#response)
    * [Parameters](#parameters)
    * [Response](#response)
    * [Parameters](#parameters)
    * [Response](#response)
    * [Parameters](#parameters)
  * [Feedback](#feedback)

---

# StorageClass



 > 
 > **Context**: StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned. apiVersion: storage.k8s.io/v1



StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
`apiVersion: storage.k8s.io/v1`
`import "k8s.io/api/storage/v1"`

## StorageClass

StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.

* **apiVersion**: storage.k8s.io/v1
* **kind**: StorageClass
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
  Standard object’s metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **provisioner** (string), required
  provisioner indicates the type of the provisioner.
* **allowVolumeExpansion** (boolean)
  allowVolumeExpansion shows whether the storage class allow volume expand.
* **allowedTopologies** (\[\]TopologySelectorTerm)
  *Atomic: will be replaced during a merge*
  allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
  *A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.*
* **allowedTopologies.matchLabelExpressions** (\[\]TopologySelectorLabelRequirement)
  *Atomic: will be replaced during a merge*
  A list of topology selector requirements by labels.
  *A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.*
* **allowedTopologies.matchLabelExpressions.key** (string), required
  The label key that the selector applies to.
* **allowedTopologies.matchLabelExpressions.values** (\[\]string), required
  *Atomic: will be replaced during a merge*
  An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
* **mountOptions** (\[\]string)
  *Atomic: will be replaced during a merge*
  mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. \[“ro”, “soft”\]. Not validated - mount of the PVs will simply fail if one is invalid.
* **parameters** (map\[string\]string)
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
* **items** (\[\][StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)), required
  items is the list of StorageClasses

### Parameters

* **name** (*in path*): string, required
  name of the StorageClass
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Parameters

* **allowWatchBookmarks** (*in query*): boolean
  [allowWatchBookmarks](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#allowWatchBookmarks)
* **continue** (*in query*): string
  [continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **fieldSelector** (*in query*): string
  [fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **labelSelector** (*in query*): string
  [labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
  [limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **resourceVersion** (*in query*): string
  [resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
  [resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
  [sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
  [timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)
* **watch** (*in query*): boolean
  [watch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#watch)

#### Parameters

* **body**: [StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
201 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Created
202 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Accepted
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the StorageClass
* **body**: [StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
201 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the StorageClass
* **body**: [Patch](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/patch/#Patch), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **force** (*in query*): boolean
  [force](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#force)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
201 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the StorageClass
* **body**: [DeleteOptions](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/delete-options/#DeleteOptions)
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **gracePeriodSeconds** (*in query*): integer
  [gracePeriodSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#gracePeriodSeconds)
* **ignoreStoreReadErrorWithClusterBreakingPotential** (*in query*): boolean
  [ignoreStoreReadErrorWithClusterBreakingPotential](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#ignoreStoreReadErrorWithClusterBreakingPotential)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **propagationPolicy** (*in query*): string
  [propagationPolicy](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#propagationPolicy)

#### Response

200 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): OK
202 ([StorageClass](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/storage-class-v1/#StorageClass)): Accepted
401: Unauthorized

#### Parameters

* **body**: [DeleteOptions](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/delete-options/#DeleteOptions)
* **continue** (*in query*): string
  [continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldSelector** (*in query*): string
  [fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **gracePeriodSeconds** (*in query*): integer
  [gracePeriodSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#gracePeriodSeconds)
* **ignoreStoreReadErrorWithClusterBreakingPotential** (*in query*): boolean
  [ignoreStoreReadErrorWithClusterBreakingPotential](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#ignoreStoreReadErrorWithClusterBreakingPotential)
* **labelSelector** (*in query*): string
  [labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
  [limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **propagationPolicy** (*in query*): string
  [propagationPolicy](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#propagationPolicy)
* **resourceVersion** (*in query*): string
  [resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
  [resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
  [sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
  [timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)

## Feedback

Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified October 03, 2025 at 8:48 PM PST: [Update resource API 1.34 with enum values (24e17cbfa6)](https://github.com/kubernetes/website/commit/24e17cbfa680ea9e379774eec976b9fb4eea6e77)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.

## Related Pages

* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Node](./ref-docs-reference-kubernetes-api-cluster-resources-node-v1.md-docs-reference-kubernetes-api-cluster-resources-node-v1.md)
* [Workload v1alpha1](./ref-docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md-docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md)
* [LeaseCandidate v1beta1](./ref-docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md-docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md)
* [Secret](./ref-docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md-docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md)
## See Also

- [Documentation Index](./COMPASS.md)
