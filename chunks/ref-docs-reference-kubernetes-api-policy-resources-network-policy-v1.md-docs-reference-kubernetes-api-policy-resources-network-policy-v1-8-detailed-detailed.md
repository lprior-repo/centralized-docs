---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-network-policy-v1.md/docs-reference-kubernetes-api-policy-resources-network-policy-v1#8-detailed
chunk_level: detailed
chunk_type: prose
heading: NetworkPolicySpec
token_count: 701
summary: #### Response 200 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): OK 201...
---

#### Response
200 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): OK
201 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the NetworkPolicy
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
200 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): OK
201 ([NetworkPolicy](https://kubernetes.io/docs/reference/kubernetes-api/policy-resources/network-policy-v1/#NetworkPolicy)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the NetworkPolicy
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