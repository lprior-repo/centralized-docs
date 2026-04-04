---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#9-standard
chunk_level: standard
chunk_type: prose
heading: LeaseCandidateList
token_count: 403
summary: #### Response 200 ([LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)): OK 201...
---

#### Response
200 ([LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)): OK
201 ([LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the LeaseCandidate
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