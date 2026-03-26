---
id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1
title: ServiceAccount
category: ref
tags: ["contents", "ref", "serviceaccount", "table"]
---

## Table of Contents

* [ServiceAccount](#serviceaccount)
  * [ServiceAccount](#serviceaccount)
  * [ServiceAccountList](#serviceaccountlist)
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

# ServiceAccount



 > 
 > **Context**: ServiceAccount binds together:  * a name, understood by users, and perhaps by peripheral systems, for an identity  * a principal that can be authentic



ServiceAccount binds together: \* a name, understood by users, and perhaps by peripheral systems, for an identity \* a principal that can be authenticated and authorized \* a set of secrets.
`apiVersion: v1`
`import "k8s.io/api/core/v1"`

## ServiceAccount

ServiceAccount binds together: \* a name, understood by users, and perhaps by peripheral systems, for an identity \* a principal that can be authenticated and authorized \* a set of secrets

* **apiVersion**: v1
* **kind**: ServiceAccount
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
  Standard object’s metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **automountServiceAccountToken** (boolean)
  AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.
* **imagePullSecrets** (\[\][LocalObjectReference](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/local-object-reference/#LocalObjectReference))
  *Atomic: will be replaced during a merge*
  ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: [https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod](https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod)
* **secrets** (\[\][ObjectReference](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-reference/#ObjectReference))
  *Patch strategy: merge on key `name`*
  *Map: unique values on key name will be kept during a merge*
  Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a “kubernetes.io/enforce-mountable-secrets” annotation set to “true”. The “kubernetes.io/enforce-mountable-secrets” annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: [https://kubernetes.io/docs/concepts/configuration/secret](https://kubernetes.io/docs/concepts/configuration/secret)

## ServiceAccountList

ServiceAccountList is a list of ServiceAccount objects

* **apiVersion**: v1
* **kind**: ServiceAccountList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
  Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **items** (\[\][ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)), required
  List of ServiceAccounts. More info: [https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)

### Parameters

* **name** (*in path*): string, required
  name of the ServiceAccount
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
* **body**: [ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): OK
201 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): Created
202 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): Accepted
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the ServiceAccount
* **namespace** (*in path*): string, required
  [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount), required
* **dryRun** (*in query*): string
  [dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
  [fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
  [fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
  [pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)

#### Response

200 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): OK
201 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the ServiceAccount
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

200 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): OK
201 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): Created
401: Unauthorized

#### Parameters

* **name** (*in path*): string, required
  name of the ServiceAccount
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

200 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): OK
202 ([ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)): Accepted
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
Last modified April 09, 2025 at 6:36 PM PST: [Update API reference docs for v1.32 (a3b579d035)](https://github.com/kubernetes/website/commit/a3b579d03512e440250c5153dacf982b9a364d2c)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.

## Related Pages

* [NetworkPolicy](./ref-docs-reference-kubernetes-api-policy-resources-network-policy-v1.md-docs-reference-kubernetes-api-policy-resources-network-policy-v1.md)
* [DeviceTaintRule v1alpha3](./ref-docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md-docs-reference-kubernetes-api-workload-resources-device-taint-rule-v1alpha3.md)
* [CSINode](./ref-docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md-docs-reference-kubernetes-api-config-and-storage-resources-csi-node-v1.md)
* [ValidatingAdmissionPolicy](./ref-docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md-docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md)
* [Secrets](./ref-docs-concepts-configuration-secret.md-docs-concepts-configuration-secret.md)
## See Also

- [Documentation Index](./COMPASS.md)
