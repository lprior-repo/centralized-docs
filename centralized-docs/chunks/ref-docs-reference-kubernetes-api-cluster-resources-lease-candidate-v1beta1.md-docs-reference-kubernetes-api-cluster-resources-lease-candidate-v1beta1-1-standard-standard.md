---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#1-standard
chunk_level: standard
chunk_type: prose
heading: LeaseCandidate
token_count: 283
summary: # LeaseCandidate v1beta1 LeaseCandidate defines a candidate for a Lease object. `apiVersion: coordination.k8s.io/v1beta1` `import \"k8s.io/api/coordination/v1beta1\"` ## LeaseCandidate LeaseCandidate...
---

# LeaseCandidate v1beta1
LeaseCandidate defines a candidate for a Lease object.
`apiVersion: coordination.k8s.io/v1beta1`
`import "k8s.io/api/coordination/v1beta1"`
## LeaseCandidate
LeaseCandidate defines a candidate for a Lease object. Candidates are created such that coordinated leader election will pick the best leader from the list of candidates.
* **apiVersion**: coordination.k8s.io/v1beta1
* **kind**: LeaseCandidate
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([LeaseCandidateSpec](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-candidate-v1beta1/#LeaseCandidateSpec))
spec contains the specification of the Lease. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status)