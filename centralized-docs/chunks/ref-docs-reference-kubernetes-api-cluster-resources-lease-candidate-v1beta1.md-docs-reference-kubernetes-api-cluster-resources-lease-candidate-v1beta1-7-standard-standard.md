---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#7-standard
chunk_level: standard
chunk_type: prose
heading: LeaseCandidateList
token_count: 299
summary: #### Parameters * **name** (*in path*): string, required name of the LeaseCandidate * **namespace** (*in path*): string, required...
---

#### Parameters
* **name** (*in path*): string, required
name of the LeaseCandidate
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)): OK
201 ([LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)): Created
401: Unauthorized