---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#3-standard
chunk_level: standard
chunk_type: prose
heading: LeaseCandidateList
token_count: 254
summary: ## LeaseCandidateList LeaseCandidateList is a list of Lease objects. * **apiVersion**: coordination.k8s.io/v1beta1 * **kind**: LeaseCandidateList * **metadata**...
---

## LeaseCandidateList
LeaseCandidateList is a list of Lease objects.
* **apiVersion**: coordination.k8s.io/v1beta1
* **kind**: LeaseCandidateList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **items** ([][LeaseCandidate](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidate)), required
items is a list of schema objects.
#### Parameters
* **name** (*in path*): string, required
name of the LeaseCandidate
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)