---
id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
title: Workload v1alpha1
category: ref
tags: ["contents", "ref", "table", "v1alpha1", "workload"]
---

## Table of Contents

* [Workload v1alpha1](#workload-v1alpha1)
  * [Workload](#workload)
  * [WorkloadSpec](#workloadspec)
  * [WorkloadList](#workloadlist)
    * [Parameters](#parameters)
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

# Workload v1alpha1



 > 
 > **Context**: Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including 



Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including scheduling, preemption, eviction and other phases.
`apiVersion: scheduling.k8s.io/v1alpha1`
`import "k8s.io/api/scheduling/v1alpha1"`

## Workload

Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including scheduling, preemption, eviction and other phases.

* **apiVersion**: scheduling.k8s.io/v1alpha1
* **kind**: Workload
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
  Standard object’s metadata. Name must be a DNS subdomain.
* **spec** ([WorkloadSpec](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#WorkloadSpec)), required
  Spec defines the desired behavior of a Workload.

## WorkloadSpec

WorkloadSpec defines the desired state of a Workload.

* **podGroups** (\[\]PodGroup), required
  *Map: unique values on key name will be kept during a merge*
  PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.
  *PodGroup represents a set of pods with a common scheduling policy.*
* **podGroups.name** (string), required
  Name is a unique identifier for the PodGroup within the Workload. It must be a DNS label. This field is immutable.
* **podGroups.policy** (PodGroupPolicy), required
  Policy defines the scheduling policy for this PodGroup.
  *PodGroupPolicy defines the scheduling configuration for a PodGroup.*
* **podGroups.policy.basic** (BasicSchedulingPolicy)
  Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.
  *BasicSchedulingPolicy indicates that standard Kubernetes scheduling behavior should be used.*
* **podGroups.policy.gang** (GangSchedulingPolicy)
  Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.
  *GangSchedulingPolicy defines the parameters for gang scheduling.*
* **podGroups.policy.gang.minCount** (int32), required
  MinCount is the minimum number of pods that must be schedulable or scheduled at the same time for the scheduler to admit the entire group. It must be a positive integer.
* **controllerRef** (TypedLocalObjectReference)
  ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.
  *TypedLocalObjectReference allows to reference typed object inside the same namespace.*
* **controllerRef.kind** (string), required
  Kind is the type of resource being referenced. It must be a path segment name.
* **controllerRef.name** (string), required
  Name is the name of resource being referenced. It must be a path segment name.
* **controllerRef.apiGroup** (string)
  APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.

## WorkloadList

WorkloadList contains a list of Workload resources.

* **apiVersion**: scheduling.k8s.io/v1alpha1
* **kind**: WorkloadList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
  Standard list metadata.
* **items** (\[\][Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)), required
  Items is the list of Workloads.

### Parameters

* **name** (*in path*): string, required
  name of the Workload
* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Parameters

* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
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

* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK
201 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Created
202 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Accepted
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the Workload
* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK
201 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the Workload
* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
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

200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK
201 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the Workload
* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
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

200 ([Status](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/#Status)): OK
202 ([Status](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/status/#Status)): Accepted
401: Unauthorized

#### Parameters

* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
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
Last modified December 21, 2025 at 5:37 PM PST: [Update resource docs for v1.35 (85b57273c5)](https://github.com/kubernetes/website/commit/85b57273c5db16377a2bec88f72f2cf5b9419524)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.

## Related Pages

* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [LeaseCandidate v1beta1](./ref-docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md-docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md)
* [Secret](./ref-docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md-docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md)
* [Node](./ref-docs-reference-kubernetes-api-cluster-resources-node-v1.md-docs-reference-kubernetes-api-cluster-resources-node-v1.md)
* [APIService](./ref-docs-reference-kubernetes-api-cluster-resources-api-service-v1.md-docs-reference-kubernetes-api-cluster-resources-api-service-v1.md)
## See Also

- [Documentation Index](./COMPASS.md)
