---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1.md/docs-reference-kubernetes-api-cluster-resources-lease-candidate-v1beta1#1-detailed
chunk_level: detailed
chunk_type: prose
heading: LeaseCandidateSpec
token_count: 693
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
## LeaseCandidateSpec
LeaseCandidateSpec is a specification of a Lease.
* **binaryVersion** (string), required
BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required.
* **leaseName** (string), required
LeaseName is the name of the lease for which this candidate is contending. The limits on this field are the same as on Lease.name. Multiple lease candidates may reference the same Lease.name. This field is immutable.
* **strategy** (string), required
Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the strategy provided by the candidate with the latest BinaryVersion will be used. If there is still conflict, this is a user error and coordinated leader election will not operate the Lease until resolved.
* **emulationVersion** (string)
EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is "OldestEmulationVersion"
* **pingTime** (MicroTime)
PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.
*MicroTime is version of Time with microsecond level precision.*
* **renewTime** (MicroTime)
RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.
*MicroTime is version of Time with microsecond level precision.*